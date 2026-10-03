//! Asking the running node for `status --json` over its control socket.
//!
//! The format is the client's (`crates/cli/src/control.rs`, version `333/1`): one line
//! in, then lines out — `: ` before each line the node said, `= done` or `= failed`
//! last. Asking this way means the observer never starts a second `333`, which would
//! open the node's key, and never reads anything the node would not say to its owner.

use std::path::Path;
use std::time::Duration;

use serde_json::value::RawValue;

/// The socket's name inside the node's directory.
const SOCKET_FILE: &str = "control.sock";

/// The request: version, then the order as the screen reads it.
const ASK: &[u8] = b"333/1 status --json\n";

/// How long the node may take to answer. A node that is up answers in milliseconds.
const PATIENCE: Duration = Duration::from_secs(10);

/// The most of an answer that is read. `status --json` is a few kilobytes.
const LONGEST: u64 = 1 << 20;

/// What asking found.
#[derive(Debug)]
pub(crate) enum Asked {
    /// Nothing answered on the socket: the node is not running.
    Down,
    /// The node answered. Its status, if it gave one that is JSON.
    Answered(Option<Box<RawValue>>),
}

/// Ask the node at `home` how it is.
#[cfg(unix)]
pub(crate) fn status(home: &Path) -> Asked {
    use std::io::{BufRead as _, BufReader, Read as _, Write as _};
    use std::os::unix::net::UnixStream;

    let Ok(mut stream) = UnixStream::connect(home.join(SOCKET_FILE)) else {
        return Asked::Down;
    };
    let timed = stream
        .set_read_timeout(Some(PATIENCE))
        .and_then(|()| stream.set_write_timeout(Some(PATIENCE)));
    if timed.is_err() || stream.write_all(ASK).is_err() {
        return Asked::Down;
    }
    let mut said = String::new();
    for line in BufReader::new(stream.take(LONGEST)).lines() {
        let Ok(line) = line else {
            break;
        };
        if let Some(text) = line.strip_prefix(": ") {
            said.push_str(text);
            said.push('\n');
        } else if line == "= done" {
            return Asked::Answered(serde_json::from_str(&said).ok());
        } else if line == "= failed" {
            break;
        }
    }
    // It took the question, so it is running; it just did not say how it is.
    Asked::Answered(None)
}

/// Nothing can be asked on this system: the client has no control socket here.
#[cfg(not(unix))]
pub(crate) fn status(_home: &Path) -> Asked {
    Asked::Down
}
