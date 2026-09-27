//! What the meeting point said when it did not say yes.
//!
//! A status number is the category an answer was sorted into, and the meeting point says
//! more than that: a sentence in the body, and for the answers that mean *not yet*, how
//! long until it would take another. Both are kept here so the node can say what actually
//! happened instead of a number.
//!
//! ONLY ITS OWN WORDS ARE KEPT. The meeting point answers in plain text, and an answer in
//! anything else came from something in front of it — a proxy, the edge's own error page —
//! and is reported as the number alone. What is kept is printed on somebody's terminal, so
//! it is cut short and has every control character taken out of it first: a meeting point
//! is not trusted with the terminal any more than with anything else.

use std::time::Duration;

/// The longest reason that will be read and repeated, in bytes.
///
/// The meeting point's longest sentence is under a hundred bytes. This is room for it to
/// say more one day and nowhere near room to fill somebody's screen.
pub(super) const LONGEST_REASON: u64 = 240;

/// The longest wait a `Retry-After` is believed about.
///
/// A day. The meeting point's longest honest answer is the time until midnight, and a
/// number larger than a day is a number nobody should plan around.
const LONGEST_WAIT: Duration = Duration::from_secs(86_400);

/// Reasons a visit to the meeting point came to nothing.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// It could not be reached at all: no route, no name, no answer, no certificate.
    #[error("could not reach the meeting point: {0}")]
    Unreachable(String),
    /// It was reached, and its answer stopped before it was finished.
    #[error("the meeting point's answer broke off: {0}")]
    BrokeOff(String),
    /// It has taken a statement from this address too recently to take another.
    ///
    /// Not a refusal of the statement. The meeting point takes one statement a minute
    /// from each address a request can come from, which several machines behind one
    /// router share, and this is that rule answering.
    #[error("the meeting point is not taking another statement from this address yet: {said}")]
    NotYet {
        /// How long until it would, when it said.
        again_in: Option<Duration>,
        /// What it said, in its own words.
        said: String,
    },
    /// It has taken as many statements today as it takes in a day.
    ///
    /// Reading goes on as before; only writing has stopped, until midnight UTC.
    #[error("the meeting point has taken all the statements it takes today: {said}")]
    FullForToday {
        /// How long until it takes them again, when it said.
        again_in: Option<Duration>,
        /// What it said, in its own words.
        said: String,
    },
    /// It answered, and said no.
    #[error("the meeting point answered {status}{}", because(said.as_deref()))]
    Refused {
        /// What it answered with.
        status: u16,
        /// Why, in its own words, if the answer was its own.
        said: Option<String>,
    },
    /// A statement this node was asked to leave is larger than the board takes.
    #[error(
        "a statement of {got} bytes is over the {} the board holds",
        super::LONGEST_STATEMENT
    )]
    TooLong {
        /// How large it was.
        got: usize,
    },
    /// It was asked where this node arrives from and answered with something else.
    #[error("the meeting point did not answer with an address")]
    NotAnAddress,
}

/// The reason after the number, when there is one.
fn because(said: Option<&str>) -> String {
    said.map_or_else(String::new, |said| format!(": {said}"))
}

/// Everything the meeting point handed back with an answer that was not yes.
pub(super) struct Answer<'a> {
    /// The status.
    pub(super) status: u16,
    /// The `content-type` header, if it sent one.
    pub(super) kind: Option<&'a str>,
    /// The `retry-after` header, if it sent one.
    pub(super) retry_after: Option<&'a str>,
    /// The first [`LONGEST_REASON`] bytes of the body.
    pub(super) body: &'a [u8],
}

/// Turn an answer that was not yes into what it means.
///
/// 429 and 503 are the two answers the meeting point gives on purpose that mean *not now*
/// rather than *no*, and they are only read that way when the meeting point is the one
/// that said them: the edge in front of it answers 429 and 503 of its own, in HTML, and
/// those mean something else.
pub(super) fn refusal(answer: &Answer<'_>) -> Error {
    let its_own = answer
        .kind
        .is_some_and(|kind| kind.trim_start().starts_with("text/plain"));
    let said = its_own.then(|| printable(answer.body));
    let again_in = answer.retry_after.and_then(wait);
    match (answer.status, said) {
        (429, Some(said)) => Error::NotYet { again_in, said },
        (503, Some(said)) => Error::FullForToday { again_in, said },
        (status, said) => Error::Refused {
            status,
            said: said.filter(|said| !said.is_empty()),
        },
    }
}

/// A `Retry-After` in whole seconds, the only form the meeting point sends.
///
/// The other form the standard allows, a date, is read as not having said: a node whose
/// clock is wrong would read it wrong, and the one thing this is used for is telling a
/// person how long.
fn wait(header: &str) -> Option<Duration> {
    let seconds: u64 = header.trim().parse().ok()?;
    let wait = Duration::from_secs(seconds);
    (wait <= LONGEST_WAIT).then_some(wait)
}

/// The body as one line a terminal can print and nothing else.
fn printable(body: &[u8]) -> String {
    let text = String::from_utf8_lossy(body);
    let line: String = text
        .chars()
        .map(|c| if c.is_whitespace() { ' ' } else { c })
        .filter(|c| !c.is_control())
        .collect();
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn answer<'a>(status: u16, kind: &'a str, retry: Option<&'a str>, body: &'a str) -> Answer<'a> {
        Answer {
            status,
            kind: Some(kind),
            retry_after: retry,
            body: body.as_bytes(),
        }
    }

    const PLAIN: &str = "text/plain; charset=utf-8";

    #[test]
    fn too_soon_is_not_yet_and_carries_how_long() {
        let said = refusal(&answer(
            429,
            PLAIN,
            Some("42"),
            "Once an epoch is enough.\n",
        ));
        assert!(matches!(
            said,
            Error::NotYet { again_in: Some(wait), .. } if wait == Duration::from_secs(42)
        ));
    }

    #[test]
    fn too_soon_without_the_header_is_still_not_yet() {
        // What every meeting point before the header existed sends.
        let said = refusal(&answer(429, PLAIN, None, "Once an epoch is enough.\n"));
        assert_eq!(
            said.to_string(),
            "the meeting point is not taking another statement from this address yet: Once \
             an epoch is enough."
        );
        assert!(matches!(said, Error::NotYet { again_in: None, .. }));
    }

    #[test]
    fn a_full_board_says_so_in_its_own_words() {
        let body =
            "The board is full for today. It is still readable, and it empties at midnight.\n";
        let said = refusal(&answer(503, PLAIN, Some("3600"), body));
        assert_eq!(
            said.to_string(),
            "the meeting point has taken all the statements it takes today: The board is \
             full for today. It is still readable, and it empties at midnight."
        );
    }

    #[test]
    fn a_refusal_carries_the_reason_it_was_given() {
        let body = "A statement goes in the slot named after the key that signed it.\n";
        assert_eq!(
            refusal(&answer(403, PLAIN, None, body)).to_string(),
            "the meeting point answered 403: A statement goes in the slot named after the \
             key that signed it."
        );
    }

    #[test]
    fn an_answer_from_something_in_front_of_it_is_only_the_number() {
        // The edge's own rate limit is a 429 in HTML. It is not the meeting point's rule
        // and must not be reported as the meeting point waiting.
        let said = refusal(&answer(
            429,
            "text/html",
            Some("10"),
            "<html>slow down</html>",
        ));
        assert_eq!(said.to_string(), "the meeting point answered 429");
    }

    #[test]
    fn nothing_it_says_can_reach_the_terminal_as_anything_but_text() {
        let body = "Said.\x1b]0;owned\x07\x1b[2J\r\nagain";
        let said = refusal(&answer(400, PLAIN, None, body)).to_string();
        assert!(!said.chars().any(char::is_control), "{said:?}");
        assert!(said.ends_with("again"), "{said:?}");
    }

    #[test]
    fn a_wait_it_cannot_mean_is_not_believed() {
        assert_eq!(wait("60"), Some(Duration::from_secs(60)));
        assert_eq!(wait("Wed, 21 Oct 2015 07:28:00 GMT"), None);
        assert_eq!(wait("99999999999"), None);
    }
}
