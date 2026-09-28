//! The third way a node meets anybody, and the only one that needs somewhere fixed.
//!
//! An invitation needs nobody. The local network needs nobody. Two machines on two
//! networks that have never heard of each other need somewhere both of them already
//! know to look, and there is no arrangement between the two of them that produces
//! one. So: one address, a board of signed statements, and a node that stops needing
//! it the moment it has met somebody.
//!
//! WHAT IS BELIEVED FROM IT: nothing. Every statement read here goes through the same
//! door as a statement a peer handed over — it opens under its own signature or it is
//! dropped — and the meeting point is never told, and never learns, whether any of it
//! was true.
//!
//! WHO LEAVES A STATEMENT: whoever has an address a stranger could actually dial. For
//! most people that is an onion address and not a socket, because a socket on a home
//! network is behind a router that was never told to let anybody in, and no amount of
//! publishing it makes it answer. An onion address needs nothing opened.
//!
//! WHAT LEAVING ONE COSTS: whoever runs the meeting point sees the address the request
//! came from, and so learns which machine published which onion address. Reading costs
//! the same request and does not link the two, so this is a real difference and a
//! narrow one: it is one operator, not the network, and `--no-meet` refuses the whole
//! arrangement. Saying nothing was the older answer and it left everybody behind a
//! router unreachable, which is worse than the thing it was avoiding.

use std::sync::Arc;
use std::time::Duration;

use n333_core::Epoch;
use n333_core::identity::NodeId;
use n333_core::whereabouts;
use n333_net::{Meeting, meeting};

use super::{to_the_boundary, until};
use crate::node::Node;

/// One node's dealings with one meeting point.
#[derive(Clone)]
pub(crate) struct Board {
    /// Where it is, and the connection to it.
    place: Arc<Meeting>,
}

impl Board {
    /// Deal with the meeting point at `place`.
    pub(crate) fn at(place: &str) -> Self {
        Self {
            place: Arc::new(Meeting::at(place)),
        }
    }

    /// Where this node is dealing.
    pub(crate) fn place(&self) -> &str {
        self.place.place()
    }

    /// Ask what address this node appears to arrive from.
    ///
    /// Used once, at the start, for a node that is listening on every interface and
    /// therefore cannot say which of its addresses a stranger could reach. The answer
    /// is a suggestion to a person, never a statement this node signs: an address that
    /// arrives at a router is not an address that reaches this machine, and only
    /// whoever set the router up knows whether it does.
    pub(crate) async fn what_address_do_i_arrive_from(&self) -> Option<std::net::IpAddr> {
        let place = Arc::clone(&self.place);
        tokio::task::spawn_blocking(move || place.what_address_do_i_arrive_from())
            .await
            .ok()?
            .ok()
    }

    /// Leave this node's address if it has one to leave, and read everyone else's.
    ///
    /// Nothing here is allowed to stop an epoch. A meeting point that is down, slow,
    /// blocked by somebody's firewall or gone for good costs this node the addresses
    /// it did not already have, and nothing else.
    ///
    /// The board is read before the leaving is reported, so that a meeting point that
    /// says *not yet* can be told apart from one that has lost this node: what it is
    /// still holding is on the board.
    pub(crate) async fn visit(&self, node: &Node, mine: Option<Vec<u8>>) {
        let who = node.identity().node_id();
        let said = match mine {
            Some(statement) => Some((self.say(&who, statement.clone()).await, statement)),
            None => None,
        };
        let board = self.read().await;
        if let Some((outcome, statement)) = said {
            let held = board.as_deref().map(|board| held_for(board, &who));
            let again_in = say_what_was_said(self.place(), &outcome, held, true);
            if let Some(wait) = again_in {
                self.say_again(who, statement, wait);
            }
        }
        let Some(board) = board else { return };
        let mut fresh = 0_usize;
        let from = crate::node::sources::Source::MeetingPoint {
            place: self.place().to_owned(),
        };
        for statement in &board {
            // Only addresses. The board is one thing and gossip is another, and a
            // meeting point that could hand out admissions would be a meeting point
            // whose operator could decide who this node hears about being admitted.
            let noted = node.note_address(statement, &from, n333_core::Epoch::now());
            if noted.await.unwrap_or(false) {
                fresh = fresh.saturating_add(1);
            }
        }
        say_what_was_there(self.place(), board.len(), fresh);
    }

    /// Put this node's own address on the board.
    async fn say(&self, who: &NodeId, statement: Vec<u8>) -> Said {
        let place = Arc::clone(&self.place);
        let who = *who;
        match tokio::task::spawn_blocking(move || place.say(&who, &statement)).await {
            Ok(said) => Said::Answered(said),
            // A blocking request is only ever cancelled by the runtime shutting down,
            // which is this node stopping.
            Err(stopped) if stopped.is_cancelled() => Said::Stopped(None),
            Err(stopped) => Said::Stopped(Some(stopped.to_string())),
        }
    }

    /// Leave the same statement once more, once the meeting point has said it would take it.
    ///
    /// Once. A second *not yet* waits for the next epoch like any other failure, so two
    /// machines behind one address cannot keep each other writing.
    fn say_again(&self, who: NodeId, statement: Vec<u8>, wait: Duration) {
        let board = self.clone();
        tokio::spawn(async move {
            tokio::time::sleep(wait).await;
            let outcome = board.say(&who, statement).await;
            say_what_was_said(board.place(), &outcome, None, false);
        });
    }

    /// Read every statement left on the board, or say why it could not be read.
    async fn read(&self) -> Option<Vec<Vec<u8>>> {
        let place = Arc::clone(&self.place);
        match tokio::task::spawn_blocking(move || place.read()).await {
            Ok(Ok(board)) => Some(board),
            Ok(Err(e)) => {
                aloud_in!(
                    "hours-meeting-unreadable",
                    place = self.place(),
                    why = e.to_string()
                );
                None
            }
            // Cut short by this node stopping, which loses nothing worth a line.
            Err(e) if e.is_cancelled() => None,
            Err(e) => {
                aloud_in!(
                    "hours-meeting-read-failed-inside",
                    place = self.place(),
                    why = e.to_string()
                );
                None
            }
        }
    }
}

/// How a request to leave a statement ended.
enum Said {
    /// The meeting point answered, or could not be reached.
    Answered(Result<(), meeting::Error>),
    /// The request never ended: this node stopped first, or it failed inside this node
    /// for this reason.
    Stopped(Option<String>),
}

/// What the board holds for this node: the newest epoch it holds an address from.
fn held_for(board: &[Vec<u8>], who: &NodeId) -> Held {
    let newest = board
        .iter()
        .filter_map(|frame| whereabouts::open(frame).ok())
        .filter(|signed| &signed.node == who)
        .map(|signed| signed.whereabouts.epoch)
        .max();
    newest.map_or(Held::Nothing, Held::From)
}

/// What a board was seen holding for this node.
#[derive(Debug, Clone, Copy)]
enum Held {
    /// Its address, signed in that epoch.
    From(u64),
    /// Nothing from it.
    Nothing,
}

/// Say how leaving this node's address went, and hand back how long to wait before leaving
/// it once more, when that is worth doing.
///
/// `held` is what the board was seen holding for this node afterwards, when it could be
/// read. `may_wait` is false on the second try, which does not get a third.
fn say_what_was_said(
    place: &str,
    outcome: &Said,
    held: Option<Held>,
    may_wait: bool,
) -> Option<Duration> {
    let (line, again_in) = what_was_said(
        place,
        outcome,
        held,
        may_wait,
        to_the_boundary(Epoch::now()),
    );
    crate::aloud::line(&line);
    again_in
}

/// The line itself, and the wait it promises, from everything that decides them.
///
/// The minute is the meeting point's own rule (`BETWEEN_WORDS` in its source): one
/// statement a minute from each address a request arrives from. Several machines behind
/// one router share that address, which is why the line says what the board holds for
/// this node rather than assuming it.
fn what_was_said(
    place: &str,
    outcome: &Said,
    held: Option<Held>,
    may_wait: bool,
    to_next_epoch: u64,
) -> (String, Option<Duration>) {
    let next_epoch = words!(
        "hours-meeting-at-the-next-epoch",
        wait = until(to_next_epoch)
    );
    let e = match outcome {
        Said::Answered(Ok(())) => return (words!("hours-meeting-left", place = place), None),
        Said::Stopped(None) => return (words!("hours-meeting-stopped", place = place), None),
        Said::Stopped(Some(why)) => {
            let line = words!(
                "hours-meeting-leaving-failed-inside",
                place = place,
                why = why
            );
            return (line, None);
        }
        Said::Answered(Err(e)) => e,
    };
    // A line of its own, which the layout indents like every other further line.
    let holding = match held {
        Some(Held::From(epoch)) => {
            format!("\n{}", words!("hours-meeting-holds-from", epoch = epoch))
        }
        Some(Held::Nothing) => format!("\n{}", words!("hours-meeting-holds-nothing")),
        None => String::new(),
    };
    match e {
        meeting::Error::NotYet { again_in, .. } => {
            let wait = again_in.filter(|_| may_wait);
            let when = wait.map_or_else(
                || next_epoch.clone(),
                |wait| words!("hours-meeting-in", wait = seconds(wait.as_secs())),
            );
            let line = words!(
                "hours-meeting-not-yet",
                place = place,
                holding = holding,
                when = when
            );
            (line, wait)
        }
        meeting::Error::FullForToday { again_in, .. } => {
            let line = match again_in {
                Some(wait) => words!(
                    "hours-meeting-full-until",
                    place = place,
                    midnight = until(wait.as_secs()),
                    holding = holding,
                    next_epoch = next_epoch
                ),
                None => words!(
                    "hours-meeting-full",
                    place = place,
                    holding = holding,
                    next_epoch = next_epoch
                ),
            };
            (line, None)
        }
        meeting::Error::Unreachable(_)
        | meeting::Error::Silent { .. }
        | meeting::Error::BrokeOff(_)
        | meeting::Error::NotAnAddress => (
            words!(
                "hours-meeting-did-not-reach",
                place = place,
                why = crate::failed::net::meeting(&e)
            ),
            None,
        ),
        meeting::Error::Refused { .. } | meeting::Error::TooLong { .. } => (
            words!(
                "hours-meeting-not-taken",
                place = place,
                why = crate::failed::net::meeting(&e)
            ),
            None,
        ),
    }
}

/// A wait of under a minute in seconds, and anything longer the way the screen says it.
fn seconds(count: u64) -> String {
    match count {
        0..60 => words!("hours-meeting-seconds", seconds = count),
        _ => until(count),
    }
}

/// Report a visit, without a line about nothing having happened.
///
/// A board that has not changed since the last epoch is the ordinary case for a node
/// that has been running a while, and a line every 333 minutes saying so is a line
/// that teaches a person to stop reading.
fn say_what_was_there(place: &str, saying: usize, fresh: usize) {
    match (saying, fresh) {
        (0, _) => aloud_in!("hours-meeting-nobody", place = place),
        (_, 0) => {}
        (_, fresh) => aloud_in!("hours-meeting-newer", fresh = fresh, place = place),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLACE: &str = "the333.dev";

    fn not_yet(again_in: Option<u64>) -> Said {
        Said::Answered(Err(meeting::Error::NotYet {
            again_in: again_in.map(Duration::from_secs),
            said: "Once an epoch is enough. Nothing here changes faster.".to_owned(),
        }))
    }

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let full = |again_in| {
            Said::Answered(Err(meeting::Error::FullForToday {
                again_in,
                said: "Enough for today.".to_owned(),
            }))
        };
        let pairs = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            [
                (
                    words!("hours-meeting-unreadable", place = PLACE, why = "down"),
                    "meet     the333.dev could not be read: down".to_owned(),
                ),
                (
                    words!("hours-meeting-read-failed-inside", place = PLACE, why = "panic"),
                    "meet     reading the333.dev failed inside this node: panic".to_owned(),
                ),
                (
                    what_was_said(PLACE, &Said::Answered(Ok(())), None, true, 60).0,
                    "meet     left this node's address at the333.dev".to_owned(),
                ),
                (
                    what_was_said(PLACE, &Said::Stopped(None), None, true, 60).0,
                    "meet     this node stopped before the333.dev answered".to_owned(),
                ),
                (
                    what_was_said(PLACE, &Said::Stopped(Some("panic".into())), None, true, 60).0,
                    "meet     leaving this node's address at the333.dev failed inside this node: panic"
                        .to_owned(),
                ),
                (
                    what_was_said(PLACE, &full(None), Some(Held::Nothing), true, 11_520).0,
                    "meet     the333.dev has taken all the statements it takes in a day and takes\n\
                     \x20        more after midnight UTC. It can still be read.\n\
                     \x20        It holds nothing from this node.\n\
                     \x20        This node leaves its address again at the next epoch, in 3h 12m."
                        .to_owned(),
                ),
                (
                    what_was_said(PLACE, &full(Some(Duration::from_secs(3_900))), None, true, 185).0,
                    "meet     the333.dev has taken all the statements it takes in a day and takes\n\
                     \x20        more after midnight UTC, in 1h 05m. It can still be read.\n\
                     \x20        This node leaves its address again at the next epoch, in 3m 5s."
                        .to_owned(),
                ),
                (seconds(1), "1 second".to_owned()),
                (seconds(59), "59 seconds".to_owned()),
                (seconds(185), "3m 5s".to_owned()),
                (
                    words!("hours-meeting-nobody", place = PLACE),
                    "meet     nobody is saying where they are at the333.dev".to_owned(),
                ),
                (
                    words!("hours-meeting-newer", fresh = 1_usize, place = PLACE),
                    "meet     1 newer address at the333.dev".to_owned(),
                ),
                (
                    words!("hours-meeting-newer", fresh = 4_usize, place = PLACE),
                    "meet     4 newer addresses at the333.dev".to_owned(),
                ),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

    #[test]
    fn not_yet_is_waiting_and_says_how_long_the_meeting_point_said() {
        let (line, wait) =
            what_was_said(PLACE, &not_yet(Some(42)), Some(Held::From(7)), true, 11_520);
        assert_eq!(
            line,
            "meet     the333.dev takes one statement a minute from each internet address,\n\
             \x20        and had one from this address less than a minute ago.\n\
             \x20        It still holds this node's address from epoch 7.\n\
             \x20        This node leaves its address again in 42 seconds."
        );
        assert_eq!(wait, Some(Duration::from_secs(42)));
    }

    #[test]
    fn not_yet_without_a_wait_is_the_next_epoch() {
        // What a meeting point from before `Retry-After` answers, and the second try.
        for (said, may_wait) in [(not_yet(None), true), (not_yet(Some(42)), false)] {
            let (line, wait) = what_was_said(PLACE, &said, None, may_wait, 11_520);
            assert!(
                line.ends_with("This node leaves its address again at the next epoch, in 3h 12m."),
                "{line}"
            );
            assert!(
                !line.contains("It still holds"),
                "the board was not read: {line}"
            );
            assert_eq!(wait, None);
        }
    }

    #[test]
    fn a_board_holding_nothing_from_this_node_is_said() {
        let (line, _) = what_was_said(PLACE, &not_yet(Some(9)), Some(Held::Nothing), true, 60);
        assert!(
            line.contains("\n\x20        It holds nothing from this node.\n"),
            "{line}"
        );
    }

    #[test]
    fn a_refusal_is_not_waiting() {
        let refused = Said::Answered(Err(meeting::Error::Refused {
            status: 403,
            said: Some("A name that 333 counts begins with 333. This one does not.".to_owned()),
        }));
        let (line, wait) = what_was_said(PLACE, &refused, Some(Held::Nothing), true, 60);
        assert_eq!(
            line,
            "meet     the333.dev did not take this node's address: the meeting point answered \
             403: A name that 333 counts begins with 333. This one does not."
        );
        assert_eq!(wait, None);
    }

    #[test]
    fn a_meeting_point_nobody_reached_did_not_refuse_anything() {
        let down = Said::Answered(Err(meeting::Error::Unreachable(
            "Connection refused (os error 111)".to_owned(),
        )));
        let (line, _) = what_was_said(PLACE, &down, None, true, 60);
        assert_eq!(
            line,
            "meet     this node's address did not reach the333.dev: could not reach the \
             meeting point: Connection refused (os error 111)"
        );
    }
}
