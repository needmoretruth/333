//! Telling a running vigil something from another terminal.
//!
//! The vigil holds its node's directory for as long as it runs, and nothing else may
//! write there. So a person who types `333 say 7` beside it, or asks a node kept by a
//! service manager to raise an onion address, is not refused: what they asked for is
//! handed to the vigil, which carries it out with the connections and the files it
//! already has open, and what it said about it comes back to them.
//!
//! WHERE IT GOES IN. A Unix socket inside the node's directory, which only its owner
//! can enter, set so only its owner can open it as well. Nothing listens on a network
//! for this, not even on this machine's own loopback address: a port there can be
//! reached by every user and every program on the machine, and a file inside a private
//! directory cannot.
//!
//! THE FORMAT. One line in, lines out, and a version at the front so that a vigil and a
//! client of different versions say so instead of misreading each other:
//!
//! ```text
//! 333/1 say 7
//! : said     #7 in epoch 89615
//! : ...
//! = done
//! ```
//!
//! What follows the version is an order in the screen's own words, read by the same
//! code the screen reads it with, so there is one set of words and not two.
//!
//! Between the two, words of the form `name=value` say how whoever asks reads, so
//! that what comes back is in their language and their base rather than the vigil's:
//!
//! ```text
//! 333/1 language=ko count-in=twelve say 10
//! ```
//!
//! No order begins with a word that has `=` in it, so they cannot be mistaken for one.
//! A vigil reads `language` and `count-in` and passes over any other name, so a later
//! client can say more; a request with none of them is answered in the vigil's words,
//! as it always was.
//!
//! NOT ON WINDOWS. A named pipe there is readable by every account on the machine
//! unless it is created with a security descriptor written by hand, and that cannot be
//! written without code this workspace forbids. Until it can, a Windows node refuses a
//! second 333 and says so.

// The format is read and written only where there is a socket to carry it; on Windows
// it is compiled, tested, and not yet used.
#![cfg_attr(not(unix), allow(dead_code))]

use std::path::Path;

/// Which version of this exchange this client speaks.
///
/// FROZEN for 333/1. A change to the lines below is a new number, not an edit.
pub(crate) const VERSION: &str = "333/1";

/// The name before the language whoever asks reads.
const LANGUAGE: &str = "language";

/// The name before the base whoever asks counts in.
const COUNT_IN: &str = "count-in";

/// The socket's name inside the node's directory.
pub(crate) const SOCKET_FILE: &str = "control.sock";

/// The longest order a vigil will read, in bytes.
///
/// A bridge line is the longest thing anybody types into one, and a few hundred bytes.
pub(crate) const LONGEST_ORDER: usize = 4096;

/// Put in front of every line the vigil said.
const SAID: &str = ": ";

/// The last line, when the order was carried out.
const DONE: &str = "= done";

/// The last line, when it was not.
const FAILED: &str = "= failed";

/// The one line that hands `order` over, saying how whoever asks reads.
///
/// An order is one line, so a line break inside one is a space. A value that is not
/// one word, or is empty, is left out, and the vigil speaks its own there.
#[must_use]
pub(crate) fn request_read_as(order: &str, language: &str, count_in: &str) -> String {
    let one_word = |word: &str| !word.is_empty() && !word.contains([' ', '\t', '\r', '\n', '=']);
    let mut line = VERSION.to_owned();
    for (name, value) in [(LANGUAGE, language), (COUNT_IN, count_in)] {
        if one_word(value) {
            line.push_str(&format!(" {name}={value}"));
        }
    }
    format!("{line} {}\n", one_line(order))
}

/// An order as one line.
fn one_line(order: &str) -> String {
    order.trim().replace(['\r', '\n'], " ")
}

/// One request, read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Request<'a> {
    /// The language whoever asked reads, if they said.
    pub(crate) language: Option<&'a str>,
    /// The base whoever asked counts in, if they said.
    pub(crate) count_in: Option<&'a str>,
    /// The order, in the screen's words.
    pub(crate) order: &'a str,
}

/// Read a request, or say the version it was asked in when that is not this one.
///
/// # Errors
/// Fails with whatever stood where the version should be.
pub(crate) fn order_in(line: &str) -> Result<Request<'_>, &str> {
    let line = line.trim_end_matches(['\r', '\n']);
    let (version, order) = line.split_once(' ').unwrap_or((line, ""));
    if version != VERSION {
        return Err(version);
    }
    let mut request = Request {
        language: None,
        count_in: None,
        order,
    };
    loop {
        let (word, after) = request.order.split_once(' ').unwrap_or((request.order, ""));
        let Some((name, value)) = word.split_once('=') else {
            return Ok(request);
        };
        match name {
            LANGUAGE => request.language = Some(value),
            COUNT_IN => request.count_in = Some(value),
            _ => {}
        }
        request.order = after;
    }
}

/// Something the vigil said, ready to be sent: one line of reply for each line of it.
#[must_use]
pub(crate) fn said(text: &str) -> String {
    text.split('\n').fold(String::new(), |mut reply, line| {
        reply.push_str(SAID);
        reply.push_str(line);
        reply.push('\n');
        reply
    })
}

/// The last line of a reply.
#[must_use]
pub(crate) fn ended(done: bool) -> String {
    format!("{}\n", if done { DONE } else { FAILED })
}

/// One line of a reply, read back.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Heard<'a> {
    /// A line the vigil said about the order.
    Said(&'a str),
    /// The end, and whether it was carried out.
    Ended(bool),
    /// Nothing this version knows. Skipped, so a later vigil can add lines.
    Unknown,
}

/// Read one line of a reply.
#[must_use]
pub(crate) fn heard(line: &str) -> Heard<'_> {
    let line = line.trim_end_matches(['\r', '\n']);
    if let Some(text) = line.strip_prefix(SAID) {
        Heard::Said(text)
    } else if line == DONE {
        Heard::Ended(true)
    } else if line == FAILED {
        Heard::Ended(false)
    } else {
        Heard::Unknown
    }
}

/// Hand `order` to the vigil keeping `home`, and print what it says about it.
///
/// `None` when no vigil answers there: whoever holds the directory is some other
/// command, or a vigil too old to be handed anything.
///
/// # Errors
/// Fails if the vigil stops answering before it has said whether the order was done,
/// or if what it said cannot be printed.
pub(crate) async fn hand_over(home: &Path, order: &str) -> anyhow::Result<Option<bool>> {
    use std::io::Write as _;
    exchange(home, order, |text| writeln!(std::io::stdout(), "{text}")).await
}

/// Hand `order` to the vigil keeping `home`, and keep what it says instead of printing
/// it: whether it was done, and every line it said.
///
/// # Errors
/// Fails if the vigil stops answering before it has said whether the order was done.
pub(crate) async fn ask(home: &Path, order: &str) -> anyhow::Result<Option<(bool, String)>> {
    let mut kept = String::new();
    let done = exchange(home, order, |text| {
        kept.push_str(text);
        kept.push('\n');
        Ok(())
    })
    .await?;
    Ok(done.map(|done| (done, kept)))
}

/// One order in, every line of the reply to `each`, and how it ended.
#[cfg(unix)]
async fn exchange(
    home: &Path,
    order: &str,
    mut each: impl FnMut(&str) -> std::io::Result<()>,
) -> anyhow::Result<Option<bool>> {
    use tokio::io::{AsyncBufReadExt as _, AsyncWriteExt as _, BufReader};

    let Ok(stream) = tokio::net::UnixStream::connect(home.join(SOCKET_FILE)).await else {
        return Ok(None);
    };
    let (reading, mut writing) = stream.into_split();
    // In this terminal's words, which may not be the vigil's.
    let words = crate::words::current();
    let asked = request_read_as(order, words.tag(), words.base().name());
    writing.write_all(asked.as_bytes()).await?;
    let mut lines = BufReader::new(reading).lines();
    while let Some(line) = lines.next_line().await? {
        match heard(&line) {
            Heard::Said(text) => each(text)?,
            Heard::Ended(done) => return Ok(Some(done)),
            Heard::Unknown => {}
        }
    }
    anyhow::bail!(words!("control-stopped-answering"))
}

/// Nothing can be handed over on this system yet; see the module's last paragraph.
#[cfg(not(unix))]
async fn exchange(
    _home: &Path,
    _order: &str,
    _each: impl FnMut(&str) -> std::io::Result<()>,
) -> anyhow::Result<Option<bool>> {
    Ok(None)
}

/// Is a vigil answering in `home`? Asks nothing of it.
#[cfg(unix)]
pub(crate) async fn answering(home: &Path) -> bool {
    tokio::net::UnixStream::connect(home.join(SOCKET_FILE))
        .await
        .is_ok()
}

/// Nothing can be answering on this system; see the module's last paragraph.
#[cfg(not(unix))]
pub(crate) async fn answering(_home: &Path) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The request of a client that does not say how it reads.
    fn request(order: &str) -> String {
        request_read_as(order, "", "")
    }

    #[test]
    fn in_english_the_moved_line_says_exactly_what_it_said_before() {
        let said = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            words!("control-stopped-answering")
        });
        assert_eq!(
            said,
            "the running node stopped answering before it said whether that was done"
        );
    }

    #[test]
    fn a_request_is_the_version_and_the_order_on_one_line() {
        // Frozen: a vigil of this version reads exactly this and nothing else.
        assert_eq!(request("say 7"), "333/1 say 7\n");
        assert_eq!(request(" tor on\n"), "333/1 tor on\n");
        assert_eq!(request("bridge a\nb"), "333/1 bridge a b\n");
    }

    #[test]
    fn a_request_reads_back_as_the_order_it_carried() {
        fn order(line: &str) -> Result<&str, &str> {
            order_in(line).map(|read| read.order)
        }
        assert_eq!(order(&request("join 333:x:3333")), Ok("join 333:x:3333"));
        assert_eq!(order("333/2 say 7\n"), Err("333/2"));
        assert_eq!(order("say 7"), Err("say"));
    }

    #[test]
    fn a_request_says_how_whoever_asks_reads_between_the_version_and_the_order() {
        let line = request_read_as("say 10", "ko", "twelve");
        assert_eq!(line, "333/1 language=ko count-in=twelve say 10\n");
        assert_eq!(
            order_in(&line),
            Ok(Request {
                language: Some("ko"),
                count_in: Some("twelve"),
                order: "say 10",
            })
        );
        // What asks without saying is read as it always was.
        let plain = order_in("333/1 say 7\n").unwrap();
        assert_eq!((plain.language, plain.count_in), (None, None));
    }

    #[test]
    fn a_name_this_vigil_does_not_know_is_passed_over_and_an_order_is_never_one() {
        let read = order_in("333/1 colour=blue language=es bridge obfs4 cert=x\n").unwrap();
        assert_eq!(read.language, Some("es"));
        assert_eq!(read.order, "bridge obfs4 cert=x");
        assert_eq!(order_in("333/1 language=ko\n").unwrap().order, "");
        // A value that is not one word is not sent at all.
        assert_eq!(
            request_read_as("status", "a b", "ten"),
            "333/1 count-in=ten status\n"
        );
    }

    #[test]
    fn a_reply_carries_every_line_and_then_how_it_ended() {
        let reply = format!(
            "{}{}",
            said("said     #7\n         it goes out"),
            ended(true)
        );
        let read: Vec<Heard<'_>> = reply.lines().map(heard).collect();
        assert_eq!(
            read,
            [
                Heard::Said("said     #7"),
                Heard::Said("         it goes out"),
                Heard::Ended(true),
            ]
        );
        assert_eq!(heard(&ended(false)), Heard::Ended(false));
        assert_eq!(heard("? something newer"), Heard::Unknown);
    }
}
