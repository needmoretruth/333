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

/// The one line that hands `order` over.
///
/// An order is one line, so a line break inside one is a space.
#[must_use]
pub(crate) fn request(order: &str) -> String {
    format!("{VERSION} {}\n", order.trim().replace(['\r', '\n'], " "))
}

/// Read a request: the order in it, or the version it was asked in when that is not
/// this one.
///
/// # Errors
/// Fails with whatever stood where the version should be.
pub(crate) fn order_in(line: &str) -> Result<&str, &str> {
    let line = line.trim_end_matches(['\r', '\n']);
    let (version, order) = line.split_once(' ').unwrap_or((line, ""));
    if version == VERSION {
        Ok(order)
    } else {
        Err(version)
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
#[cfg(unix)]
pub(crate) async fn hand_over(home: &Path, order: &str) -> anyhow::Result<Option<bool>> {
    use std::io::Write as _;
    use tokio::io::{AsyncBufReadExt as _, AsyncWriteExt as _, BufReader};

    let Ok(stream) = tokio::net::UnixStream::connect(home.join(SOCKET_FILE)).await else {
        return Ok(None);
    };
    let (reading, mut writing) = stream.into_split();
    writing.write_all(request(order).as_bytes()).await?;
    let mut lines = BufReader::new(reading).lines();
    while let Some(line) = lines.next_line().await? {
        match heard(&line) {
            Heard::Said(text) => writeln!(std::io::stdout(), "{text}")?,
            Heard::Ended(done) => return Ok(Some(done)),
            Heard::Unknown => {}
        }
    }
    anyhow::bail!("the vigil stopped answering before it said whether that was done")
}

/// Nothing can be handed over on this system yet; see the module's last paragraph.
///
/// # Errors
/// Never.
#[cfg(not(unix))]
pub(crate) async fn hand_over(_home: &Path, _order: &str) -> anyhow::Result<Option<bool>> {
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

    #[test]
    fn a_request_is_the_version_and_the_order_on_one_line() {
        // Frozen: a vigil of this version reads exactly this and nothing else.
        assert_eq!(request("say 7"), "333/1 say 7\n");
        assert_eq!(request(" tor on\n"), "333/1 tor on\n");
        assert_eq!(request("bridge a\nb"), "333/1 bridge a b\n");
    }

    #[test]
    fn a_request_reads_back_as_the_order_it_carried() {
        assert_eq!(order_in(&request("join 333:x:3333")), Ok("join 333:x:3333"));
        assert_eq!(order_in("333/2 say 7\n"), Err("333/2"));
        assert_eq!(order_in("say 7"), Err("say"));
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
