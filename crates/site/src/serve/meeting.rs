//! The meeting point, answering exactly as the Cloudflare Worker did.
//!
//! FROZEN AS FAR AS RELEASED CLIENTS SEE IT. Paths, methods, status codes, the words in
//! each answer, the `retry-after` header and the framing of the board are what 0.7.0
//! clients were written against (see `meeting/index.ts` and `crates/net/src/meeting.rs`).
//! The one difference is a limit that is gone: the Worker's 900 writes a day.

use std::collections::BTreeMap;

use http_body_util::{BodyExt as _, Limited};
use hyper::body::Body;
use hyper::{HeaderMap, Request, StatusCode};
use n333_core::epoch::unix_now_millis;
use serde::Serialize;

use super::respond::{self, NO_STORE, Reply};
use super::state::State;
use crate::board::{Line, Said};
use crate::place::{self, Place};

/// The largest statement accepted, in bytes. The same number the client refuses above.
pub(crate) const LONGEST: usize = 512;

/// The hexadecimal a name has to begin with before the protocol counts it.
const ELIGIBLE: &str = "333";

/// The header Cloudflare puts the visitor's address in. Only Cloudflare reaches Caddy,
/// and Caddy only passes requests on, so it is the edge's word.
pub(crate) const VISITOR: &str = "cf-connecting-ip";

/// `GET /333/where`: the address the caller arrived from, and a newline.
pub(crate) fn where_you_are(headers: &HeaderMap) -> Reply {
    let address = headers
        .get(VISITOR)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    respond::plain(StatusCode::OK, &format!("{address}\n"), None)
}

/// `GET /333` for a program: every live statement, framed.
pub(crate) fn read_the_board(state: &State) -> Reply {
    let framed = state.board().framed(unix_now_millis());
    respond::with(StatusCode::OK, "application/octet-stream", NO_STORE, framed)
}

/// Does the caller want a page rather than frames?
pub(crate) fn wants_html(headers: &HeaderMap) -> bool {
    headers
        .get(hyper::header::ACCEPT)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|accept| accept.contains("text/html"))
}

/// `PUT /333/<name>`: take one statement and put it in the slot its own key names.
///
/// Spam is turned away by the protocol rather than by anybody deciding: the statement
/// has to verify, its key has to be the slot's, and the name has to be one the protocol
/// counts, which costs about four thousand tries to find.
pub(crate) async fn speak<B>(state: &State, key: &str, request: Request<B>) -> Reply
where
    B: Body,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    if !named(key) {
        return respond::plain(
            StatusCode::BAD_REQUEST,
            "The slot is a node name in lower-case hex.\n",
            None,
        );
    }
    let headers = request.headers().clone();
    let Some(frame) = statement(request).await else {
        let words = format!("A statement is between 1 and {LONGEST} bytes.\n");
        return respond::plain(StatusCode::PAYLOAD_TOO_LARGE, &words, None);
    };
    let Some(said) = Said::of(&frame) else {
        let words = "That is not a signed statement about where a node is.\n";
        return respond::plain(StatusCode::BAD_REQUEST, words, None);
    };
    if said.node != key {
        let words = "A statement goes in the slot named after the key that signed it.\n";
        return respond::plain(StatusCode::FORBIDDEN, words, None);
    }
    if !key.starts_with(ELIGIBLE) {
        let words = format!("A name that 333 counts begins with {ELIGIBLE}. This one does not.\n");
        return respond::plain(StatusCode::FORBIDDEN, &words, None);
    }
    let place = place::of(&headers, &said.address);
    let visitor = headers.get(VISITOR).and_then(|value| value.to_str().ok());
    keep(state, said, frame, place, visitor).await
}

/// The checks that need the board, then the write.
async fn keep(
    state: &State,
    said: Said,
    frame: Vec<u8>,
    place: Option<Place>,
    visitor: Option<&str>,
) -> Reply {
    let now = unix_now_millis();
    let (generation, bytes) = {
        let mut board = state.board();
        // Byte for byte what is already there: checked before the wait, so a node
        // retrying after a lost answer is never told to come back later for a write that
        // already happened.
        if board.holds(&said.node, &frame, now) {
            return respond::plain(StatusCode::OK, "Already said.\n", None);
        }
        if let Some(seconds) = state.gate().too_soon(visitor, now) {
            let words = "Once an epoch is enough. Nothing here changes faster.\n";
            return respond::plain(StatusCode::TOO_MANY_REQUESTS, words, Some(seconds));
        }
        board.put(Line::new(said, frame, now, place), now);
        board.snapshot()
    };
    state.persist(generation, bytes).await;
    respond::plain(StatusCode::OK, "Said.\n", None)
}

/// Is this 64 lower-case hex digits?
fn named(key: &str) -> bool {
    key.len() == 64
        && key
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

/// The body, if it is between 1 and [`LONGEST`] bytes. Never reads more than that.
async fn statement<B>(request: Request<B>) -> Option<Vec<u8>>
where
    B: Body,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    let announced = request
        .headers()
        .get(hyper::header::CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok());
    if announced.is_some_and(|length| length > LONGEST as u64) {
        return None;
    }
    let body = Limited::new(request.into_body(), LONGEST)
        .collect()
        .await
        .ok()?;
    let bytes = body.to_bytes().to_vec();
    (!bytes.is_empty()).then_some(bytes)
}

/// How many of us are where, for the map. Counts only; names nobody.
#[derive(Serialize)]
struct Where {
    /// When this was counted, in milliseconds.
    as_of: u64,
    /// How many statements are held.
    saying: usize,
    /// How many name an onion address.
    tor: usize,
    /// How many the edge could not place.
    unplaced: usize,
    /// Countries, most first, then by code.
    countries: Vec<Country>,
    /// One `[x, y]` per placed statement, in whole degrees.
    dots: Vec<[i32; 2]>,
}

/// One country and how many statements came from it.
#[derive(Serialize)]
struct Country {
    /// Two letters.
    c: String,
    /// How many.
    n: usize,
}

/// `GET /333/where-we-are`.
pub(crate) fn where_we_are(state: &State) -> Reply {
    let now = unix_now_millis();
    let board = state.board();
    let mut counted: BTreeMap<String, usize> = BTreeMap::new();
    let mut found = Where {
        as_of: now,
        saying: 0,
        tor: 0,
        unplaced: 0,
        countries: Vec::new(),
        dots: Vec::new(),
    };
    for line in board.alive(now) {
        found.saying += 1;
        match line.place() {
            Some(Place::Tor) => found.tor += 1,
            None => found.unplaced += 1,
            Some(Place::At { country, y, x }) => {
                *counted.entry(country.clone()).or_default() += 1;
                found.dots.push([*x, *y]);
            }
        }
    }
    drop(board);
    found.countries = counted.into_iter().map(|(c, n)| Country { c, n }).collect();
    found
        .countries
        .sort_by(|one, two| two.n.cmp(&one.n).then_with(|| one.c.cmp(&two.c)));
    respond::json(serde_json::to_string(&found).unwrap_or_else(|_| "null".to_owned()))
}
