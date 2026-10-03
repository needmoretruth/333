//! Which handler a request goes to.
//!
//! The meeting point's paths come first and answer for any method the way the Worker did
//! (`/333/where` for any method, 405 with the expected usage for the rest). Everything
//! else is the JSON for the pages or a file, for `GET` and `HEAD` only.

use std::sync::Arc;

use hyper::body::Body;
use hyper::header::{ALLOW, HeaderValue};
use hyper::{Method, Request, StatusCode};
use n333_core::epoch::unix_now_millis;

use super::respond::{self, Reply};
use super::state::State;
use super::{meeting, pages, values};
use crate::site::ENGLISH;

/// Answer one request, with the headers every answer carries.
pub(crate) async fn handle<B>(state: Arc<State>, request: Request<B>) -> Reply
where
    B: Body,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    let mut reply = route(state, request).await;
    respond::secure(&mut reply);
    reply
}

/// Pick the handler.
async fn route<B>(state: Arc<State>, request: Request<B>) -> Reply
where
    B: Body,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    let path = request.uri().path().to_owned();
    let method = request.method().clone();
    let get = method == Method::GET;
    let readable = get || method == Method::HEAD;
    match path.as_str() {
        "/333/where" => meeting::where_you_are(request.headers()),
        "/333/where-we-are" if get => blocking(state, meeting::where_we_are).await,
        "/333/where-we-are" => refused("GET /333/where-we-are\n"),
        "/333" if get && meeting::wants_html(request.headers()) => {
            blocking(state, |state| pages::board_page(state, &ENGLISH)).await
        }
        "/333" if get => meeting::read_the_board(&state),
        "/333" => refused("GET /333\n"),
        _ if path.starts_with("/333/") => match path.strip_prefix("/333/") {
            Some(key) if method == Method::PUT => meeting::speak(&state, key, request).await,
            _ => refused("PUT /333/<node name in hex>\n"),
        },
        "/api/network" if readable => blocking(state, pages::network).await,
        "/api/status" if readable => blocking(state, pages::status_api).await,
        "/api/board" if readable => board_api(&state),
        "/sitemap.xml" if readable => blocking(state, pages::sitemap).await,
        _ if readable => blocking(state, move |state| pages::any(state, &path)).await,
        _ => {
            let mut reply = refused("GET or HEAD\n");
            reply
                .headers_mut()
                .insert(ALLOW, HeaderValue::from_static("GET, HEAD"));
            reply
        }
    }
}

/// `GET /api/board`: the live statements that verify, for a browser.
fn board_api(state: &State) -> Reply {
    let board = state.board();
    respond::json(values::board_json(&board, unix_now_millis()))
}

/// 405, with the usage the Worker answered with.
fn refused(usage: &str) -> Reply {
    respond::plain(StatusCode::METHOD_NOT_ALLOWED, usage, None)
}

/// Run file work on a blocking thread.
async fn blocking(state: Arc<State>, work: impl FnOnce(&State) -> Reply + Send + 'static) -> Reply {
    match tokio::task::spawn_blocking(move || work(&state)).await {
        Ok(reply) => reply,
        Err(error) => {
            tracing::error!("a page could not be made: {error}");
            respond::plain(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong here.\n",
                None,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::OnceLock;

    use http_body_util::{BodyExt as _, Full};
    use hyper::body::Bytes;
    use n333_core::{Epoch, Identity, Whereabouts};

    use super::*;

    /// One eligible identity for every test: finding one costs a few thousand tries.
    fn eligible() -> &'static Identity {
        static MINED: OnceLock<Identity> = OnceLock::new();
        MINED.get_or_init(|| Identity::mine().0)
    }

    fn statement(node: &Identity, address: &str) -> Vec<u8> {
        Whereabouts::of(node, address.to_owned(), Epoch::now())
            .seal(node)
            .unwrap()
    }

    fn state(name: &str) -> Arc<State> {
        let dir =
            std::env::temp_dir().join(format!("n333-site-routes-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("site")).unwrap();
        let state = State::open(
            &dir.join("site"),
            &dir.join("state"),
            dir.join("network.json"),
            "test".to_owned(),
        );
        Arc::new(state.unwrap())
    }

    async fn put(
        state: &Arc<State>,
        slot: &str,
        body: Vec<u8>,
    ) -> (StatusCode, Option<String>, String) {
        let request = Request::put(format!("/333/{slot}"))
            .header("cf-connecting-ip", "192.0.2.7")
            .body(Full::new(Bytes::from(body)))
            .unwrap();
        let reply = handle(Arc::clone(state), request).await;
        let wait = reply
            .headers()
            .get("retry-after")
            .map(|value| value.to_str().unwrap().to_owned());
        let status = reply.status();
        let body = reply.into_body().collect().await.unwrap().to_bytes();
        (status, wait, String::from_utf8(body.to_vec()).unwrap())
    }

    async fn board(state: &Arc<State>) -> Vec<u8> {
        let request = Request::get("/333").body(Full::new(Bytes::new())).unwrap();
        let reply = handle(Arc::clone(state), request).await;
        assert_eq!(reply.headers()["content-type"], "application/octet-stream");
        reply
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes()
            .to_vec()
    }

    #[tokio::test]
    async fn a_signed_statement_is_taken_and_handed_back_framed() {
        let state = state("said");
        let frame = statement(eligible(), "192.0.2.7:3333");
        let said = put(&state, &eligible().node_id().to_string(), frame.clone()).await;
        assert_eq!(said, (StatusCode::OK, None, "Said.\n".to_owned()));

        let mut expected = u32::try_from(frame.len()).unwrap().to_be_bytes().to_vec();
        expected.extend_from_slice(&frame);
        assert_eq!(board(&state).await, expected);
    }

    #[tokio::test]
    async fn a_tampered_statement_is_refused() {
        let state = state("tampered");
        let mut frame = statement(eligible(), "192.0.2.7:3333");
        *frame.last_mut().unwrap() ^= 1;
        let (status, _, _) = put(&state, &eligible().node_id().to_string(), frame).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn a_statement_in_somebody_elses_slot_is_refused() {
        let state = state("slot");
        let frame = statement(eligible(), "192.0.2.7:3333");
        let other = "3".repeat(64);
        let (status, _, words) = put(&state, &other, frame).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert_eq!(
            words,
            "A statement goes in the slot named after the key that signed it.\n"
        );
    }

    #[tokio::test]
    async fn a_name_333_does_not_count_is_refused() {
        let state = state("uncounted");
        let plain = Identity::from_seed(&[1; 32]);
        assert!(!plain.node_id().to_string().starts_with("333"));
        let (status, _, words) = put(
            &state,
            &plain.node_id().to_string(),
            statement(&plain, "192.0.2.7:3333"),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert_eq!(
            words,
            "A name that 333 counts begins with 333. This one does not.\n"
        );
    }

    #[tokio::test]
    async fn a_statement_over_512_bytes_is_too_large() {
        let state = state("large");
        let (status, _, words) = put(&state, &eligible().node_id().to_string(), vec![0; 513]).await;
        assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
        assert_eq!(words, "A statement is between 1 and 512 bytes.\n");
    }

    #[tokio::test]
    async fn the_same_bytes_again_are_already_said_even_inside_the_minute() {
        let state = state("again");
        let (slot, frame) = (
            eligible().node_id().to_string(),
            statement(eligible(), "192.0.2.7:3333"),
        );
        put(&state, &slot, frame.clone()).await;
        assert_eq!(
            put(&state, &slot, frame).await,
            (StatusCode::OK, None, "Already said.\n".to_owned())
        );
    }

    #[tokio::test]
    async fn a_different_statement_inside_the_minute_waits() {
        let state = state("wait");
        let slot = eligible().node_id().to_string();
        put(&state, &slot, statement(eligible(), "192.0.2.7:3333")).await;
        let (status, wait, words) =
            put(&state, &slot, statement(eligible(), "192.0.2.7:3334")).await;
        assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
        assert!(
            wait.unwrap()
                .parse::<u64>()
                .is_ok_and(|seconds| (1..=60).contains(&seconds))
        );
        assert_eq!(
            words,
            "Once an epoch is enough. Nothing here changes faster.\n"
        );
    }

    async fn network(state: &Arc<State>, as_of: u64) -> serde_json::Value {
        let written = format!(r#"{{"format":1,"as_of":{as_of},"running":true,"nodes":[]}}"#);
        std::fs::write(&state.observation, &written).unwrap();
        let request = Request::get("/api/network")
            .body(Full::new(Bytes::new()))
            .unwrap();
        let reply = handle(Arc::clone(state), request).await;
        let body = reply.into_body().collect().await.unwrap().to_bytes();
        if as_of + 1_000 > unix_now_millis() {
            assert_eq!(
                body,
                written.as_bytes(),
                "a fresh observation goes out as written"
            );
        }
        serde_json::from_slice(&body).unwrap()
    }

    #[tokio::test]
    async fn an_observation_older_than_ninety_seconds_goes_out_as_not_running_and_stale() {
        let state = state("stale");
        let fresh = network(&state, unix_now_millis()).await;
        assert_eq!(
            (fresh["running"].as_bool(), fresh.get("stale")),
            (Some(true), None)
        );

        let old = network(&state, unix_now_millis() - 91_000).await;
        assert_eq!(
            (old["running"].as_bool(), old["stale"].as_bool()),
            (Some(false), Some(true))
        );
    }

    async fn get(state: &Arc<State>, path: &str) -> (StatusCode, String) {
        let request = Request::get(path)
            .header("accept", "text/html")
            .body(Full::new(Bytes::new()))
            .unwrap();
        let reply = handle(Arc::clone(state), request).await;
        let status = reply.status();
        let body = reply.into_body().collect().await.unwrap().to_bytes();
        (status, String::from_utf8(body.to_vec()).unwrap())
    }

    #[tokio::test]
    async fn the_board_and_the_map_are_in_the_page_before_any_script_runs() {
        let dir = std::env::temp_dir().join(format!("n333-site-pages-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let site = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../site");
        let state = Arc::new(
            State::open(
                &site,
                &dir.join("state"),
                dir.join("network.json"),
                "t".to_owned(),
            )
            .unwrap(),
        );
        let placed = Request::put(format!("/333/{}", eligible().node_id()))
            .header("cf-connecting-ip", "192.0.2.8")
            .header("cf-ipcountry", "CA")
            .header("cf-iplatitude", "45.4")
            .header("cf-iplongitude", "-75.7")
            .body(Full::new(Bytes::from(statement(
                eligible(),
                "192.0.2.7:3333",
            ))))
            .unwrap();
        assert_eq!(
            handle(Arc::clone(&state), placed).await.status(),
            StatusCode::OK
        );
        let hidden = Identity::mine().0;
        let onion = format!("{}.onion:3333", "a".repeat(56));
        let (status, _, _) = put(
            &state,
            &hidden.node_id().to_string(),
            statement(&hidden, &onion),
        )
        .await;
        assert_eq!(status, StatusCode::OK);

        let (status, board) = get(&state, "/333").await;
        assert_eq!(status, StatusCode::OK);
        assert!(
            board.contains("<code>333 join 333:192.0.2.7:3333</code>"),
            "{board}"
        );
        assert!(board.contains(" · CA</p>") && board.contains(" · through Tor</p>"));
        assert!(board.contains("data-board-empty hidden"));

        let (_, map) = get(&state, "/map").await;
        assert!(map.contains(r#"<span data-c="CA">CA</span></td><td class="n">1</td>"#));
        assert!(map.contains(r#"<tr><td>Tor</td><td class="n">1</td></tr>"#));
        assert!(
            map.contains(r#"<tr class="sum"><td>All of us saying</td><td class="n">2</td></tr>"#)
        );
        assert!(
            map.contains(r#"<circle class="dot" cx="416" cy="180" r="6"><title>1 node</title>"#)
        );
        let tor = map.split(r#"<g id="tor-dots">"#).nth(1).unwrap();
        assert!(tor.starts_with(r#"<circle class="ring""#), "{tor}");

        let (status, korean) = get(&state, "/ko/333").await;
        assert_eq!(status, StatusCode::OK);
        assert!(korean.contains(r#"<html lang="ko""#), "{korean}");
        assert!(korean.contains("<code>333 join 333:192.0.2.7:3333</code>"));
        assert_eq!(get(&state, "/xx/about").await.0, StatusCode::NOT_FOUND);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
