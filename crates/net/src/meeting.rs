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
//! from. That is the whole of the cost and it is not nothing: a node that is hiding behind an
//! onion address must not leave a statement here, because doing so ties the onion address it
//! is publishing to the address it is speaking from. Reading is different — a reader sends
//! nothing about itself — so a hidden node may read the board and must not write to it.
//!
//! THE DEPENDENCE IS MEANT TO SHRINK. A fixed address is a single point that can be taken
//! away, and this design wanted none. The alternatives that need no fixed point cost more
//! code and more ways to publish an address its owner did not mean to publish than there is
//! reason to ship today. That is a limit of what is built, not a claim about what is right.

use std::io::Read as _;
use std::net::IpAddr;
use std::time::Duration;

use n333_core::identity::NodeId;

use crate::frame::{LENGTH_PREFIX_LEN, MAX_BATCH_FRAMES};

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

/// Reasons a visit to the meeting point came to nothing.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// It could not be reached at all: no route, no name, no answer, no certificate.
    #[error("could not reach the meeting point: {0}")]
    Unreachable(String),
    /// It answered, and said no.
    #[error("the meeting point answered {status}")]
    Refused {
        /// What it answered with.
        status: u16,
    },
    /// A statement this node was asked to leave is larger than the board takes.
    #[error("a statement of {got} bytes is over the {LONGEST_STATEMENT} the board holds")]
    TooLong {
        /// How large it was.
        got: usize,
    },
    /// It was asked where this node arrives from and answered with something else.
    #[error("the meeting point did not answer with an address")]
    NotAnAddress,
}

/// One node's dealings with one meeting point.
///
/// Blocking, because it is a handful of requests an epoch and an async HTTP stack is thirty
/// crates to save a thread that is asleep anyway. Callers inside a runtime hand it to a
/// blocking worker.
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
        let config = ureq::Agent::config_builder()
            .timeout_global(Some(PATIENCE))
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
        let said = self
            .agent
            .get(self.url("/where"))
            .call()
            .map_err(went_wrong)?
            .body_mut()
            .read_to_string()
            .map_err(went_wrong)?;
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
        self.agent
            .put(self.url(&format!("/{who}")))
            .send(statement)
            .map_err(went_wrong)?;
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
        let mut answer = self.agent.get(self.url("")).call().map_err(went_wrong)?;
        let mut board = Vec::new();
        let cap = u64::try_from(LONGEST_BOARD).unwrap_or(u64::MAX);
        answer
            .body_mut()
            .as_reader()
            .take(cap)
            .read_to_end(&mut board)
            .map_err(|cause| Error::Unreachable(cause.to_string()))?;
        Ok(unframe(&board))
    }

    /// The address of one part of the meeting point.
    ///
    /// A place given with a scheme in front of it is taken as written. That is for
    /// pointing a node at a meeting point running on the same machine while somebody
    /// works on one, and it is the only way to reach one over plain HTTP: a bare host
    /// is always https, so nobody arrives there by accident or by being told to.
    fn url(&self, tail: &str) -> String {
        if self.place.starts_with("http://") || self.place.starts_with("https://") {
            format!("{}/333{tail}", self.place.trim_end_matches('/'))
        } else {
            format!("https://{}/333{tail}", self.place)
        }
    }
}

/// Turn a failed request into the two things a caller can do something about.
fn went_wrong(cause: ureq::Error) -> Error {
    match cause {
        ureq::Error::StatusCode(status) => Error::Refused { status },
        other => Error::Unreachable(other.to_string()),
    }
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

    #[test]
    fn a_place_given_with_a_scheme_is_taken_as_written() {
        let meeting = Meeting::at("http://127.0.0.1:8787/");
        assert_eq!(meeting.url(""), "http://127.0.0.1:8787/333");
        assert_eq!(meeting.url("/where"), "http://127.0.0.1:8787/333/where");
    }
}
