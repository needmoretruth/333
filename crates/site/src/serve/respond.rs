//! The shapes an answer takes, and the headers every answer carries.

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::header::{self, HeaderName, HeaderValue};
use hyper::{Response, StatusCode};

/// Every answer this server gives.
pub(crate) type Reply = Response<Full<Bytes>>;

/// HTML is checked again on every visit: it holds live numbers.
pub(crate) const NO_CACHE: &str = "no-cache";

/// Answers about the board are never kept: a stale board is a wrong board.
pub(crate) const NO_STORE: &str = "no-store";

/// Files under `/assets/` are named by version (`?v=`), so a copy never goes stale.
pub(crate) const IMMUTABLE: &str = "public, max-age=31536000, immutable";

/// What a browser may do with a page from here: load from here, frame nothing.
const POLICY: &str = "default-src 'self'; img-src 'self' data:; style-src 'self'; \
script-src 'self'; connect-src 'self'; frame-ancestors 'none'; base-uri 'none'; \
form-action 'self'";

/// An answer with a body, a type and a caching rule.
pub(crate) fn with(
    status: StatusCode,
    kind: &'static str,
    cache: &'static str,
    body: impl Into<Bytes>,
) -> Reply {
    let mut reply = Response::new(Full::new(body.into()));
    *reply.status_mut() = status;
    let headers = reply.headers_mut();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static(kind));
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static(cache));
    reply
}

/// An answer in words, as the Worker gave them.
///
/// `retry_after` is for the answers that mean "not yet": the seconds until the rule that
/// said no would say yes.
pub(crate) fn plain(status: StatusCode, words: &str, retry_after: Option<u64>) -> Reply {
    let mut reply = with(
        status,
        "text/plain; charset=utf-8",
        NO_STORE,
        words.to_owned(),
    );
    if let Some(seconds) = retry_after {
        reply
            .headers_mut()
            .insert(header::RETRY_AFTER, HeaderValue::from(seconds));
    }
    reply
}

/// JSON for a program, never cached.
pub(crate) fn json(body: String) -> Reply {
    json_bytes(body.into_bytes())
}

/// JSON that is already bytes, never cached.
pub(crate) fn json_bytes(body: Vec<u8>) -> Reply {
    with(
        StatusCode::OK,
        "application/json; charset=utf-8",
        NO_STORE,
        body,
    )
}

/// The headers every answer carries, whatever it is.
pub(crate) fn secure(reply: &mut Reply) {
    let headers = reply.headers_mut();
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("no-referrer"),
    );
    headers.insert(
        HeaderName::from_static("content-security-policy"),
        HeaderValue::from_static(POLICY),
    );
}
