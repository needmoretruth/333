//! The meeting point: two nodes on two networks, with nobody to introduce them.
//!
//! A node holding an invitation never comes here — one address, typed in once, is the whole
//! of it. Two nodes on one network find each other without it, over mDNS. This is the third
//! case and the only one that cannot be solved between the two machines alone: neither knows
//! the other exists, and neither has anywhere to look.
//!
//! So there is one fixed address, and it holds a board. A node leaves the statement it
//! already signs about where it can be reached; a node looking for anybody reads what has
//! been left and knocks. Two epochs later the board has forgotten it.
//!
//! NOTHING HERE IS TRUSTED. Every statement comes back the way it was signed, and the caller
//! verifies before it believes any of it — this module hands back bytes and makes no claim
//! about them. The board cannot invent a member, cannot forge an address and cannot vouch
//! for one. What it can do is lie by omission, or vanish; both leave a node exactly where it
//! was before it asked, which is why nothing depends on it twice.
//!
//! WHAT IT COSTS TO USE IT. Whoever runs the meeting point learns the address a node speaks
//! from, and if that node leaves a statement, learns which onion address belongs to it. That
//! is the whole of the cost. It is one operator rather than the network, it is the same
//! request either way, and refusing it means refusing the board — which for a node behind a
//! router means nobody can find it at all. Withholding the statement was the earlier answer
//! here and it bought a narrow secret at the price of the node being unreachable.
//!
//! THE DEPENDENCE IS MEANT TO SHRINK. A fixed address is a single point that can be taken
//! away, and this design wanted none. The alternatives that need no fixed point cost more
//! code and more ways to publish an address its owner did not mean to publish than there is
//! reason to ship today. That is a limit of what is built, not a claim about what is right.

mod answered;

use std::io::Read as _;
use std::net::IpAddr;
use std::time::Duration;

use n333_core::identity::NodeId;

use crate::frame::{LENGTH_PREFIX_LEN, MAX_BATCH_FRAMES};

pub use answered::Error;

/// Where nodes that have nobody to introduce them agree to look. FROZEN.
///
/// It is written into the client rather than configured because a default nobody typed is
/// the only kind two strangers can both be holding. A node that would rather use another one
/// says so; a node that would rather use none says that too, and loses only this third way
/// of meeting anybody.
pub const THE_PLACE: &str = "the333.dev";

/// The longest statement the meeting point will take, in bytes.
///
/// FROZEN, and the same number on both sides: the server refuses anything larger and this
/// refuses to send it, so a node finds out here rather than at the far end.
pub const LONGEST_STATEMENT: usize = 512;

/// How long the whole exchange may take before it is given up on.
///
/// It is one request against a static edge, made a few times an epoch. Twenty seconds is far
/// past a bad mobile connection and nowhere near long enough to hold anything up.
const PATIENCE: Duration = Duration::from_secs(20);

/// The most of the board this node will read.
const LONGEST_BOARD: usize = MAX_BATCH_FRAMES * (LENGTH_PREFIX_LEN + LONGEST_STATEMENT);

/// The most that will be read when asking for the file.
///
/// The file is three bytes. This is far above that and far below anything that could
/// cost this node something, which is all a limit here has to be.
const LONGEST_FILE: u64 = 64;

/// One node's dealings with one meeting point.
///
/// Blocking, because it is a handful of requests an epoch and an async HTTP stack is thirty
/// crates to save a thread that is asleep anyway. Callers inside a runtime hand it to a
/// blocking worker.
#[derive(Clone)]
pub struct Meeting {
    /// The host, without a scheme.
    place: String,
    /// The connection pool and its timeouts.
    agent: ureq::Agent,
}

impl Meeting {
    /// Deal with the meeting point at `place`.
    #[must_use]
    pub fn at(place: &str) -> Self {
        // Every status comes back as an answer, so that what the meeting point said with
        // it can be read. Treated as an error, the body is gone before anybody sees it.
        let config = ureq::Agent::config_builder()
            .timeout_global(Some(PATIENCE))
            .http_status_as_error(false)
            .build();
        Self {
            place: place.to_owned(),
            agent: config.into(),
        }
    }

    /// Deal with [`THE_PLACE`].
    #[must_use]
    pub fn the_place() -> Self {
        Self::at(THE_PLACE)
    }

    /// Where this node is dealing.
    #[must_use]
    pub fn place(&self) -> &str {
        &self.place
    }

    /// Ask what address this node appears to arrive from.
    ///
    /// A node behind a household router knows the port it is listening on and has no way to
    /// learn the address the rest of the world would have to use to reach it. This is the
    /// one thing here that is about the asker rather than about anybody else, and it is a
    /// rumour like every other: the answer is whatever the far end says, and the proof that
    /// it was right is somebody knocking.
    ///
    /// # Errors
    /// Fails if the meeting point cannot be reached, refuses, or answers with something that
    /// is not an address.
    pub fn what_address_do_i_arrive_from(&self) -> Result<IpAddr, Error> {
        let said = yes(self.agent.get(self.url("/where")).call())?
            .body_mut()
            .read_to_string()
            .map_err(broke_off)?;
        said.trim().parse().map_err(|_| Error::NotAnAddress)
    }

    /// Leave a signed statement where anybody looking can read it.
    ///
    /// The name is a slot on the board and not a claim: the bytes underneath carry their own
    /// signature, so writing into somebody else's slot achieves nothing except wasting it.
    ///
    /// # Errors
    /// Fails if the statement is over the limit, or the meeting point cannot be reached or
    /// refuses.
    pub fn say(&self, who: &NodeId, statement: &[u8]) -> Result<(), Error> {
        if statement.is_empty() || statement.len() > LONGEST_STATEMENT {
            return Err(Error::TooLong {
                got: statement.len(),
            });
        }
        yes(self.agent.put(self.url(&format!("/{who}"))).send(statement))?;
        Ok(())
    }

    /// Read every statement left there.
    ///
    /// Verifies nothing. What comes back is bytes somebody left at an address, and the
    /// caller opens each one and keeps the ones that are signed.
    ///
    /// # Errors
    /// Fails if the meeting point cannot be reached or refuses.
    pub fn read(&self) -> Result<Vec<Vec<u8>>, Error> {
        let mut answer = yes(self.agent.get(self.url("")).call())?;
        let mut board = Vec::new();
        let cap = u64::try_from(LONGEST_BOARD).unwrap_or(u64::MAX);
        answer
            .body_mut()
            .as_reader()
            .take(cap)
            .read_to_end(&mut board)
            .map_err(|cause| Error::BrokeOff(cause.to_string()))?;
        Ok(unframe(&board))
    }

    /// Fetch the file itself, for a node that has nobody to be given it by.
    ///
    /// WHY THIS IS NOT THE CLIENT MAKING THE FILE. It still cannot. It carries the hash
    /// and not the bytes, so what comes back here is checked against that hash and
    /// anything else is refused. The bytes arrive from outside, the same as they do in a
    /// handover, and the only difference is that nobody signed for them, which is exactly
    /// what a node starting on its own has to live with.
    ///
    /// # Errors
    /// Fails if the meeting point cannot be reached, refuses, or answers with more bytes
    /// than the file could possibly be.
    pub fn the_file(&self) -> Result<Vec<u8>, Error> {
        let mut answer = yes(self.agent.get(self.whole("/333.txt")).call())?;
        let mut bytes = Vec::new();
        answer
            .body_mut()
            .as_reader()
            .take(LONGEST_FILE)
            .read_to_end(&mut bytes)
            .map_err(|cause| Error::BrokeOff(cause.to_string()))?;
        Ok(bytes)
    }

    /// The address a person would open in a browser to see the board.
    #[must_use]
    pub fn browse(&self) -> String {
        self.whole("/333")
    }

    /// The address of one part of the meeting point.
    fn url(&self, tail: &str) -> String {
        self.whole(&format!("/333{tail}"))
    }

    /// One path at this meeting point.
    ///
    /// A place given with a scheme in front of it is taken as written. That is for
    /// pointing a node at a meeting point running on the same machine while somebody
    /// works on one, and it is the only way to reach one over plain HTTP: a bare host
    /// is always https, so nobody arrives there by accident or by being told to.
    fn whole(&self, path: &str) -> String {
        if self.place.starts_with("http://") || self.place.starts_with("https://") {
            format!("{}{path}", self.place.trim_end_matches('/'))
        } else {
            format!("https://{}{path}", self.place)
        }
    }
}

/// The answer, if it was yes; what it meant, if it was not.
fn yes(sent: Result<Answer, ureq::Error>) -> Result<Answer, Error> {
    let mut answer = sent.map_err(unreachable)?;
    let status = answer.status();
    if status.is_success() {
        return Ok(answer);
    }
    let header = |name: &str| {
        answer
            .headers()
            .get(name)
            .and_then(|value| value.to_str().ok())
            .map(ToOwned::to_owned)
    };
    let (kind, retry_after) = (header("content-type"), header("retry-after"));
    let mut body = Vec::new();
    // A reason that cannot be read is a reason not given; the number still stands.
    let _ = answer
        .body_mut()
        .as_reader()
        .take(answered::LONGEST_REASON)
        .read_to_end(&mut body);
    Err(answered::refusal(&answered::Answer {
        status: status.as_u16(),
        kind: kind.as_deref(),
        retry_after: retry_after.as_deref(),
        body: &body,
    }))
}

/// What one request comes back as.
type Answer = ureq::http::Response<ureq::Body>;

/// Why the meeting point could not be reached, in the words for what happened.
///
/// The library's own words lead with the category it sorted the failure into — `io:`,
/// `timeout:` — which is not what a person needs to read.
fn unreachable(cause: ureq::Error) -> Error {
    Error::Unreachable(match cause {
        ureq::Error::Io(cause) => cause.to_string(),
        ureq::Error::Timeout(_) => format!("no answer within {} s", PATIENCE.as_secs()),
        ureq::Error::HostNotFound => "its name does not resolve".to_owned(),
        other => other.to_string(),
    })
}

/// An answer that started and did not finish.
fn broke_off(cause: ureq::Error) -> Error {
    Error::BrokeOff(match cause {
        ureq::Error::Io(cause) => cause.to_string(),
        ureq::Error::Timeout(_) => format!("not finished within {} s", PATIENCE.as_secs()),
        other => other.to_string(),
    })
}

/// Split a board into the statements it is made of.
///
/// Forgiving on purpose. The board is written by strangers and served by a machine nobody
/// here controls, so a length that runs off the end, a frame larger than the limit, or more
/// frames than this node will read all mean *stop*, not *throw away what was already read*.
/// A board that arrives half-eaten is still half a board.
fn unframe(board: &[u8]) -> Vec<Vec<u8>> {
    let mut statements = Vec::new();
    let mut rest = board;
    while statements.len() < MAX_BATCH_FRAMES {
        let Some((head, tail)) = rest.split_at_checked(LENGTH_PREFIX_LEN) else {
            break;
        };
        let Ok(head) = <[u8; LENGTH_PREFIX_LEN]>::try_from(head) else {
            break;
        };
        let announced = u32::from_be_bytes(head);
        let Ok(announced) = usize::try_from(announced) else {
            break;
        };
        if announced == 0 || announced > LONGEST_STATEMENT {
            break;
        }
        let Some((statement, next)) = tail.split_at_checked(announced) else {
            break;
        };
        statements.push(statement.to_vec());
        rest = next;
    }
    statements
}

#[cfg(test)]
mod tests {
    use super::*;

    fn board(statements: &[&[u8]]) -> Vec<u8> {
        let mut out = Vec::new();
        for statement in statements {
            let len = u32::try_from(statement.len()).expect("short");
            out.extend_from_slice(&len.to_be_bytes());
            out.extend_from_slice(statement);
        }
        out
    }

    #[test]
    fn a_board_comes_apart_into_what_was_put_on_it() {
        let written = board(&[b"first", b"second", b"third"]);
        assert_eq!(
            unframe(&written),
            vec![b"first".to_vec(), b"second".to_vec(), b"third".to_vec()]
        );
    }

    #[test]
    fn an_empty_board_is_no_statements_rather_than_a_failure() {
        assert!(unframe(&[]).is_empty());
    }

    #[test]
    fn a_board_cut_off_in_the_middle_keeps_what_was_whole() {
        let mut written = board(&[b"first", b"second"]);
        written.truncate(written.len() - 3);
        assert_eq!(unframe(&written), vec![b"first".to_vec()]);
    }

    #[test]
    fn a_statement_larger_than_the_board_takes_ends_the_reading() {
        let mut written = board(&[b"first"]);
        written.extend_from_slice(&u32::MAX.to_be_bytes());
        written.extend_from_slice(&[0_u8; 8]);
        assert_eq!(unframe(&written), vec![b"first".to_vec()]);
    }

    #[test]
    fn nobody_can_make_this_node_read_more_than_a_run_of_frames() {
        let one = [7_u8; 4];
        let many: Vec<&[u8]> = std::iter::repeat_n(&one[..], MAX_BATCH_FRAMES + 10).collect();
        assert_eq!(unframe(&board(&many)).len(), MAX_BATCH_FRAMES);
    }

    #[test]
    fn every_part_of_the_meeting_point_is_under_one_path() {
        let meeting = Meeting::at("example.test");
        assert_eq!(meeting.url(""), "https://example.test/333");
        assert_eq!(meeting.url("/where"), "https://example.test/333/where");
    }

    /// A meeting point on this machine that gives one answer, written out by hand.
    fn answering(answer: &'static str) -> Meeting {
        use std::io::Write as _;
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("binds");
        let port = listener.local_addr().expect("has an address").port();
        std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accepts");
            let mut request = [0_u8; 1024];
            let _ = stream.read(&mut request);
            stream.write_all(answer.as_bytes()).expect("answers");
        });
        Meeting::at(&format!("http://127.0.0.1:{port}"))
    }

    #[test]
    fn what_the_meeting_point_says_with_a_429_reaches_the_node() {
        let meeting = answering(
            "HTTP/1.1 429 Too Many Requests\r\ncontent-type: text/plain; charset=utf-8\r\n\
             retry-after: 42\r\ncontent-length: 55\r\nconnection: close\r\n\r\n\
             Once an epoch is enough. Nothing here changes faster.\n\n",
        );
        match meeting.read() {
            Err(Error::NotYet { again_in, said }) => {
                assert_eq!(again_in, Some(Duration::from_secs(42)));
                assert_eq!(
                    said,
                    "Once an epoch is enough. Nothing here changes faster."
                );
            }
            other => panic!("expected not yet, got {other:?}"),
        }
    }

    #[test]
    fn a_place_given_with_a_scheme_is_taken_as_written() {
        let meeting = Meeting::at("http://127.0.0.1:8787/");
        assert_eq!(meeting.url(""), "http://127.0.0.1:8787/333");
        assert_eq!(meeting.url("/where"), "http://127.0.0.1:8787/333/where");
        assert_eq!(meeting.whole("/333.txt"), "http://127.0.0.1:8787/333.txt");
    }
}
