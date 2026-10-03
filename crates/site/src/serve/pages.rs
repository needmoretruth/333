//! The files a browser asks for, and the pages rendered from them, in each language.
//!
//! Runs on a blocking thread: it reads files, and a slow disk must not stall the
//! threads answering the meeting point.
//!
//! An address that begins with a published language's segment (`/ko/about`) is that
//! page in that language; only pages are there, so `/ko/assets/…` is missing. An address
//! that begins with a language that is not published is missing too, in English, and is
//! never linked from anywhere. Nothing is chosen by `Accept-Language`: an address always
//! means the same page.

use std::path::Path;

use hyper::StatusCode;
use hyper::header::{CONTENT_LANGUAGE, HeaderValue};
use n333_core::epoch::unix_now_millis;

use super::respond::{self, NO_CACHE, Reply};
use super::state::State;
use super::status_view::{self, Status};
use super::{
    board_view, counted, head, map_view, network_view, sitemap, statics, template, values,
};
use crate::site::{self, ENGLISH, Language};

/// The board's route after a language's segment.
const BOARD_ROUTE: &str = "/333";

/// The template the board is.
const BOARD_PAGE: &str = "board.html";

/// Any `GET` that is not the meeting point's or the JSON's: a page, a file, or missing.
pub(crate) fn any(state: &State, request_path: &str) -> Reply {
    let rest = request_path.strip_prefix('/').unwrap_or(request_path);
    let (segment, tail) = rest.split_once('/').unwrap_or((rest, ""));
    let Some(language) = site::language_at(segment) else {
        return file(state, &ENGLISH, request_path);
    };
    if !state.words.is_published(language) {
        return not_found(state, &ENGLISH);
    }
    let tail = format!("/{tail}");
    if tail == BOARD_ROUTE {
        return board_page(state, language);
    }
    file(state, language, &tail)
}

/// A file under the site directory in `language`, or the not-found page.
///
/// Outside English only pages are served; the catalogs themselves are never a file.
fn file(state: &State, language: &'static Language, request_path: &str) -> Reply {
    if request_path.split('/').nth(1) == Some(site::WORDS) {
        return not_found(state, language);
    }
    let Some(path) = statics::resolve(&state.site, request_path) else {
        return not_found(state, language);
    };
    if !language.is_english() && !statics::is_html(&path) {
        return not_found(state, language);
    }
    send(
        state,
        language,
        &path,
        StatusCode::OK,
        statics::cache(request_path),
    )
    .unwrap_or_else(|| not_found(state, language))
}

/// The board as a page, for a person with a browser.
pub(crate) fn board_page(state: &State, language: &'static Language) -> Reply {
    let path = state.site.join(BOARD_PAGE);
    send(state, language, &path, StatusCode::OK, NO_CACHE)
        .unwrap_or_else(|| not_found(state, language))
}

/// `GET /sitemap.xml`.
pub(crate) fn sitemap(state: &State) -> Reply {
    respond::with(
        StatusCode::OK,
        "application/xml; charset=utf-8",
        NO_CACHE,
        sitemap::xml(state),
    )
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

/// `GET /api/status`: what the status page shows, as JSON.
pub(crate) fn status_api(state: &State) -> Reply {
    let now = unix_now_millis();
    let written = std::fs::read(&state.observation).unwrap_or_default();
    respond::json(status(state, &written, now).json())
}

/// The status page's numbers. The board is held only while it is counted; the history
/// file is read after, so a slow disk never keeps a statement waiting.
fn status(state: &State, written: &[u8], now: u64) -> Status {
    let observed = values::observation_in(written, now);
    let found = counted::count(&state.board(), observed.as_ref(), now);
    Status::gather(state, written, &found, now)
}

/// `404.html` in `language` with status 404, or plain words if there is none.
fn not_found(state: &State, language: &'static Language) -> Reply {
    let path = state.site.join(site::NOT_FOUND.template);
    send(state, language, &path, StatusCode::NOT_FOUND, NO_CACHE).unwrap_or_else(|| {
        respond::plain(
            StatusCode::NOT_FOUND,
            "Nothing is kept at this address.\n",
            None,
        )
    })
}

/// One file, templated in `language` if it is HTML. `None` if it cannot be read.
fn send(
    state: &State,
    language: &'static Language,
    path: &Path,
    status: StatusCode,
    cache: &'static str,
) -> Option<Reply> {
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
    let html = render(state, language, path, &page);
    let mut reply = respond::with(status, kind, NO_CACHE, html);
    if let Ok(tag) = HeaderValue::from_str(language.tag) {
        reply.headers_mut().insert(CONTENT_LANGUAGE, tag);
    }
    Some(reply)
}

/// A page with its values, its words and, where it asks for them, the parts the server
/// draws that its scripts also draw.
fn render(state: &State, language: &'static Language, path: &Path, page: &str) -> String {
    let now = unix_now_millis();
    let written = std::fs::read(&state.observation).unwrap_or_default();
    let observed = values::observation_in(&written, now);
    let words = state.words.speaker(language);
    let template = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    let indexed = site::page_of(template);
    let mut filled = {
        let board = state.board();
        let mut filled = values::for_page(&state.version, observed.as_ref(), &board, now, &words);
        if page.contains("{{html:board_") {
            board_view::fill(&mut filled, &board, observed.as_ref(), now, &words);
        }
        if page.contains("{{html:map_") {
            let found = counted::count(&board, observed.as_ref(), now);
            map_view::fill(&mut filled, &found, &words);
        }
        filled
    };
    if page.contains("{{html:network_") {
        network_view::fill(&mut filled, observed.as_ref(), &written, &words);
    }
    if page.contains("{{html:status_") {
        status_view::fill(&mut filled, &status(state, &written, now), &words);
    }
    filled.json("words", words.script().to_owned());
    filled.html(
        "head",
        indexed.map_or_else(String::new, |page| {
            head::links(&state.words, language, page)
        }),
    );
    filled.html("languages", head::languages(&state.words, &words, indexed));
    template::render(page, &filled, &words)
}
