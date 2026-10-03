//! The files a browser asks for, and the pages rendered from them.
//!
//! Runs on a blocking thread: it reads files, and a slow disk must not stall the
//! threads answering the meeting point.

use std::path::Path;

use hyper::StatusCode;
use n333_core::epoch::unix_now_millis;

use super::respond::{self, NO_CACHE, Reply};
use super::state::State;
use super::{statics, template, values};

/// The page shown for a path that names nothing.
const NOT_FOUND_PAGE: &str = "404.html";

/// The page `GET /333` gives a browser.
const BOARD_PAGE: &str = "board.html";

/// A file under the site directory, or the not-found page.
pub(crate) fn file(state: &State, request_path: &str) -> Reply {
    let Some(path) = statics::resolve(&state.site, request_path) else {
        return not_found(state);
    };
    send(state, &path, StatusCode::OK, statics::cache(request_path))
        .unwrap_or_else(|| not_found(state))
}

/// The board as a page, for a person with a browser.
pub(crate) fn board_page(state: &State) -> Reply {
    let path = state.site.join(BOARD_PAGE);
    send(state, &path, StatusCode::OK, NO_CACHE).unwrap_or_else(|| not_found(state))
}

/// `GET /api/network`: the observation as `observe` wrote it (marked as
/// [`values::mark_if_stale`] says when it is old), or `null` if it cannot be
/// read as JSON, so a page polling it never has to tell an error page from data.
pub(crate) fn network(state: &State) -> Reply {
    let Some(bytes) = std::fs::read(&state.observation).ok() else {
        return respond::json_bytes(b"null".to_vec());
    };
    let Ok(mut observed) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
        return respond::json_bytes(b"null".to_vec());
    };
    // Fresh, it goes out byte for byte; stale, it goes out marked.
    if values::mark_if_stale(&mut observed, unix_now_millis()) {
        return respond::json(observed.to_string());
    }
    respond::json_bytes(bytes)
}

/// `404.html` with status 404, or plain words if there is none.
fn not_found(state: &State) -> Reply {
    let path = state.site.join(NOT_FOUND_PAGE);
    send(state, &path, StatusCode::NOT_FOUND, NO_CACHE).unwrap_or_else(|| {
        respond::plain(
            StatusCode::NOT_FOUND,
            "Nothing is kept at this address.\n",
            None,
        )
    })
}

/// One file, templated if it is HTML. `None` if it cannot be read.
fn send(state: &State, path: &Path, status: StatusCode, cache: &'static str) -> Option<Reply> {
    let bytes = std::fs::read(path).ok()?;
    let kind = statics::kind(path);
    if !statics::is_html(path) {
        return Some(respond::with(status, kind, cache, bytes));
    }
    // A page that is not UTF-8 is sent as it is rather than refused.
    let page = match String::from_utf8(bytes) {
        Ok(page) => page,
        Err(not_text) => return Some(respond::with(status, kind, cache, not_text.into_bytes())),
    };
    let now = unix_now_millis();
    let observed = values::observation(&state.observation, now);
    let filled = {
        let board = state.board();
        values::for_page(&state.version, observed.as_ref(), &board, now)
    };
    Some(respond::with(
        status,
        kind,
        NO_CACHE,
        template::render(&page, &filled),
    ))
}
