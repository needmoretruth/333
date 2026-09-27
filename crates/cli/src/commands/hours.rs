//! Keeping the hours: what a node does at every epoch boundary, for ever.
//!
//! Four things, in this order, and each one is allowed to fail without stopping the
//! others. A node that could not reach anybody this epoch still has to judge the
//! epoch that has come of age, and a node that cannot write its record still has to
//! keep answering.
//!
//! 1. Say where it is, if it has an address worth telling anybody.
//! 2. Leave that where a stranger could find it, and read what strangers left.
//! 3. Trade statements with every node it knows the whereabouts of.
//! 4. Ask the nodes it was drawn to ask.
//! 5. Judge the epoch that is now old enough to judge, and write the verdict down.
//! 6. Forget the statements that can no longer change anything.
//! 7. Write down where every address it holds came from.
//!
//! The two that find people come before asking because they are what supplies the
//! addresses asking needs. A node that has just started knows where exactly one member
//! is — whoever handed it the file — and everything else it ever learns arrives
//! through one of them.
//!
//! NOTHING HERE DECIDES ANYTHING ABOUT ANYBODY ELSE'S STANDING. It writes down what
//! this node concluded about **itself** from what it was given, which is the only
//! record it is entitled to keep. What it concluded about others lives in the
//! statements it published, and those are judged by whoever reads them.
//!
//! WHY IT SLEEPS TO THE BOUNDARY RATHER THAN FOR AN EPOCH. Sleeping for 333 minutes
//! drifts: every restart, every slow round and every scheduler hiccup pushes the next
//! one later, and after a while a node is doing its epoch's work in the middle of the
//! next one. The boundary is a property of the clock, not of when this started.

mod asking;
mod judging;
mod meeting;

use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use n333_core::whereabouts::Whereabouts;
use n333_core::{Epoch, epoch};
use n333_net::PeerAddress;

use crate::dial::Dialer;
use crate::node::Node;
use crate::words::Arg;

pub(crate) use asking::trade_at_once;
use asking::{ask_those_drawn, present_myself, trade_news};
pub(crate) use judging::judge_what_is_ready;
pub(crate) use meeting::Board;

/// Run the hours until the process is stopped.
///
/// `announce_as` carries the address to tell other nodes about, once a listener knows
/// one worth telling. It stays empty for a node behind a wildcard bind, which
/// genuinely does not know which of its addresses a stranger can reach, and it fills
/// in later for a node whose onion address took minutes to come up.
///
/// `board` is the meeting point, if this node is using one.
///
/// # Errors
/// Never, as written: it runs until the process is stopped. The result type is here
/// because everything that could fail inside is reported and stepped over — a node
/// that stopped keeping the hours because one round failed is a node that stopped
/// being counted.
pub(crate) async fn keep(
    node: Arc<Node>,
    dialer: Dialer,
    announce_as: tokio::sync::watch::Receiver<Option<PeerAddress>>,
    board: Option<Board>,
) -> anyhow::Result<()> {
    // Beside the hours rather than inside them: the hours sleep for 333 minutes at a
    // time, and a vigil that says it is awake once an epoch cannot be told from one
    // that stopped.
    tokio::spawn(crate::commands::service::awake::keep_saying(
        node.home().to_path_buf(),
    ));
    let mut announce_as = announce_as;
    loop {
        let now = Epoch::now();
        let address = announce_as.borrow().clone();
        one_round(&node, &dialer, address.clone(), board.as_ref(), now).await;
        // A node that had no address to give out when the round ran would otherwise
        // wait out the whole epoch before anybody could be told where it is, and both
        // of the ways an address arrives take their time: an onion address is minutes
        // of Tor waking up, and a socket is not known to be reachable until the knock
        // has come back. Neither is worth 333 minutes of being unfindable.
        if address.is_none() && announce_as.changed().await.is_ok() {
            let arrived = announce_as.borrow().clone();
            if arrived.is_some() {
                say_where_i_am(&node, arrived.as_ref(), board.as_ref(), Epoch::now()).await;
            }
        }
        sleep_until_the_next_boundary(now).await;
    }
}

/// Everything a node does at one boundary, in order.
///
/// One function rather than five calls in the loop, so that what a node does in an
/// epoch is a thing that can be run — from the loop with the clock's answer, and from
/// a test with an epoch of its own. A round that only ever exists inside a loop that
/// sleeps for 333 minutes is a round nobody watches from beginning to end.
pub(crate) async fn one_round(
    node: &Node,
    dialer: &Dialer,
    address: Option<PeerAddress>,
    board: Option<&Board>,
    now: Epoch,
) {
    // Written first, and whether or not anything happens in this epoch. An epoch
    // nobody spoke in and an epoch this node was switched off for leave the same
    // empty disk otherwise, and telling them apart is the whole of this node's right
    // to ever say the network ended.
    if let Err(e) = node.keeping(now).await {
        aloud_in!("hours-failed-marking", why = format!("{e:#}"));
    }
    say_where_i_am(node, address.as_ref(), board, now).await;
    trade_news(node, dialer, now).await;
    ask_those_drawn(node, dialer, now).await;
    // A node with no address a stranger could dial cannot be asked, and an epoch nobody
    // asked it about is an epoch it is not in. So it goes to them instead. A node that
    // can be reached does not do this: its verifiers are already on their way.
    if !address.is_some_and(|address| address.worth_telling_a_stranger()) {
        present_myself(node, dialer, now).await;
    }
    judge_what_is_ready(node, now).await;
    forget_the_old(node, now).await;
    // Last, so that everything this round heard of is in it. Nothing reads it but the
    // person running this node, so a round that could not write it has lost nothing
    // it needed.
    if let Err(e) = node.write_down_sources(now).await {
        aloud_in!("hours-failed-sources", why = format!("{e:#}"));
    }
}

/// Write down where this node can be reached, and go and look where others said to.
///
/// Both halves of one visit, because a node with nothing to say still has everybody
/// else to read. Called once a round, and again the moment an address turns up in the
/// middle of a round that began without one.
async fn say_where_i_am(
    node: &Node,
    address: Option<&PeerAddress>,
    board: Option<&Board>,
    now: Epoch,
) {
    let mine = match address {
        Some(address) => say_where(node, address, now).await,
        None => None,
    };
    let Some(board) = board else { return };
    // Kept for the neighbours, withheld from the strangers. An address that only means
    // something on this wire is a correct thing to have written down and a wrong thing to
    // leave where somebody on the other side of the world will dial it.
    let worth = address.is_some_and(PeerAddress::worth_telling_a_stranger);
    if mine.is_some() && !worth {
        aloud_in!("hours-not-leaving", place = board.place());
    }
    board.visit(node, mine.filter(|_| worth)).await;
}

/// Write down where this node can be reached, and keep it for others to pass on.
///
/// Hands back the signed statement, because whoever wants to leave it somewhere else
/// should be leaving the same bytes this node kept rather than signing a second copy.
async fn say_where(node: &Node, address: &PeerAddress, now: Epoch) -> Option<Vec<u8>> {
    let statement = Whereabouts::of(node.identity(), address.to_string(), now);
    match statement
        .seal(node.identity())
        .with_context(|| words!("hours-sealing"))
    {
        Ok(frame) => {
            if let Err(e) = node.note_own_address(&frame, now).await {
                aloud_in!("hours-failed-keeping-address", why = format!("{e:#}"));
            }
            Some(frame)
        }
        Err(e) => {
            aloud_in!("hours-failed-saying-where", why = format!("{e:#}"));
            None
        }
    }
}

/// Drop what can no longer change anybody's standing.
async fn forget_the_old(node: &Node, now: Epoch) {
    match node.forget_old(now).await {
        Ok(0) => {}
        Ok(dropped) => aloud_in!("hours-forgot", epochs = dropped),
        Err(e) => aloud_in!("hours-failed-forgetting", why = format!("{e:#}")),
    }
}

/// A duration in whole minutes, which is the scale a person keeps these hours on.
pub(super) fn minutes(span: Duration) -> String {
    words!("hours-minutes", minutes = span.as_secs() / 60)
}

/// How long until the next epoch begins, in seconds.
pub(crate) fn to_the_boundary(now: Epoch) -> u64 {
    Epoch(now.0.saturating_add(1))
        .starts_at_unix_seconds()
        .saturating_sub(epoch::unix_now_seconds())
}

/// A span of seconds, said the way a person waiting would say it.
pub(crate) fn until(seconds: u64) -> String {
    let (hours, minutes) = (seconds / 3600, (seconds % 3600) / 60);
    if hours == 0 {
        return words!(
            "hours-minutes-and-seconds",
            minutes = minutes,
            seconds = seconds % 60
        );
    }
    words!(
        "hours-hours-and-minutes",
        hours = hours,
        minutes = Arg::padded(minutes, 2)
    )
}

/// Wait for the next epoch to begin.
async fn sleep_until_the_next_boundary(now: Epoch) {
    let seconds = to_the_boundary(now);
    // A clock that jumped forward past the boundary gives zero here. Waiting a second
    // rather than spinning is the difference between a hot loop and a late start.
    tokio::time::sleep(Duration::from_secs(seconds.max(1))).await;
}

/// "1 epoch" or "N epochs", said the way a person would.
pub(super) fn epochs(count: u64) -> String {
    words!("hours-epochs-answered-for", epochs = count)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::words::count::Base;

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let pairs = crate::words::speaking("en", Base::Ten, || {
            [
                (
                    words!("hours-failed-marking", why = "disk full"),
                    "failed   marking this epoch as kept: disk full".to_owned(),
                ),
                (
                    words!("hours-failed-sources", why = "disk full"),
                    "failed   writing down where addresses came from: disk full".to_owned(),
                ),
                (
                    words!("hours-not-leaving", place = "the333.dev"),
                    "meet     not leaving this address at the333.dev. It reaches this node from here\n\
                     \x20        and from nowhere else, and a stranger who dialled it would\n\
                     \x20        reach something of their own."
                        .to_owned(),
                ),
                (
                    words!("hours-sealing"),
                    "sealing this node's address".to_owned(),
                ),
                (
                    words!("hours-failed-keeping-address", why = "disk full"),
                    "failed   keeping this node's own address: disk full".to_owned(),
                ),
                (
                    words!("hours-failed-saying-where", why = "no key"),
                    "failed   saying where this node is: no key".to_owned(),
                ),
                (
                    words!("hours-forgot", epochs = 3_usize),
                    "forgot   3 epochs. nothing said about them now could change a verdict."
                        .to_owned(),
                ),
                (
                    words!("hours-failed-forgetting", why = "disk full"),
                    "failed   forgetting old statements: disk full".to_owned(),
                ),
                (minutes(Duration::from_secs(60)), "1 minute".to_owned()),
                (minutes(Duration::from_secs(6_660)), "111 minutes".to_owned()),
                (until(185), "3m 5s".to_owned()),
                (until(11_520), "3h 12m".to_owned()),
                (until(3_900), "1h 05m".to_owned()),
                (epochs(1), "1 epoch answered for".to_owned()),
                (epochs(40), "40 epochs answered for".to_owned()),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

    #[test]
    fn one_epoch_forgotten_is_one_epoch() {
        let said =
            crate::words::speaking("en", Base::Ten, || words!("hours-forgot", epochs = 1_usize));
        assert_eq!(
            said,
            "forgot   1 epoch. nothing said about them now could change a verdict."
        );
    }

    #[test]
    fn in_twelve_a_wait_is_counted_in_twelve() {
        let said = crate::words::speaking("en", Base::TwelveAscii, || until(11_520));
        assert_eq!(said, "3h 10m");
    }
}
