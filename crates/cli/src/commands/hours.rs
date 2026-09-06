//! Keeping the hours: what a node does at every epoch boundary, for ever.
//!
//! Four things, in this order, and each one is allowed to fail without stopping the
//! others. A node that could not reach anybody this epoch still has to judge the
//! epoch that has come of age, and a node that cannot write its record still has to
//! keep answering.
//!
//! 1. Say where it is, if it has an address worth telling anybody.
//! 2. Trade statements with every node it knows the whereabouts of.
//! 3. Ask the nodes it was drawn to ask.
//! 4. Judge the epoch that is now old enough to judge, and write the verdict down.
//! 5. Forget the statements that can no longer change anything.
//!
//! Trading comes before asking because it is what supplies the addresses asking needs.
//! A node that has just started knows where exactly one member is — whoever handed it
//! the file — and everything else it ever learns comes through that step.
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

use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use n333_core::whereabouts::Whereabouts;
use n333_core::{Epoch, epoch};
use n333_net::PeerAddress;

use crate::dial::Dialer;
use crate::node::Node;

use asking::{ask_those_drawn, trade_news};
use judging::judge_what_is_ready;

/// Run the hours until the process is stopped.
///
/// `announce_as` carries the address to tell other nodes about, once a listener knows
/// one worth telling. It stays empty for a node behind a wildcard bind, which
/// genuinely does not know which of its addresses a stranger can reach, and it fills
/// in later for a node whose onion address took minutes to come up.
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
) -> anyhow::Result<()> {
    loop {
        let now = Epoch::now();
        // Written first, and whether or not anything happens in this epoch. An epoch
        // nobody spoke in and an epoch this node was switched off for leave the same
        // empty disk otherwise, and telling them apart is the whole of this node's
        // right to ever say the network ended.
        if let Err(e) = node.keeping(now).await {
            aloud!("failed   marking this epoch as kept: {e:#}");
        }
        let address = announce_as.borrow().clone();
        if let Some(address) = address {
            say_where(&node, &address, now).await;
        }
        trade_news(&node, &dialer, now).await;
        ask_those_drawn(&node, &dialer, now).await;
        judge_what_is_ready(&node, now).await;
        forget_the_old(&node, now).await;
        sleep_until_the_next_boundary(now).await;
    }
}

/// Write down where this node can be reached, and keep it for others to pass on.
async fn say_where(node: &Node, address: &PeerAddress, now: Epoch) {
    let statement = Whereabouts::of(node.identity(), address.to_string(), now);
    match statement
        .seal(node.identity())
        .context("sealing this node's address")
    {
        Ok(frame) => {
            if let Err(e) = node.note_address(&frame).await {
                aloud!("failed   keeping this node's own address: {e:#}");
            }
        }
        Err(e) => aloud!("failed   saying where this node is: {e:#}"),
    }
}

/// Drop what can no longer change anybody's standing.
async fn forget_the_old(node: &Node, now: Epoch) {
    match node.forget_old(now).await {
        Ok(0) => {}
        Ok(dropped) => aloud!(
            "forgot   {dropped} epochs. nothing said about them now could change a verdict."
        ),
        Err(e) => aloud!("failed   forgetting old statements: {e:#}"),
    }
}

/// A duration in whole minutes, which is the scale a person keeps these hours on.
pub(super) fn minutes(span: Duration) -> String {
    let count = span.as_secs() / 60;
    if count == 1 {
        "1 minute".to_owned()
    } else {
        format!("{count} minutes")
    }
}

/// Wait for the next epoch to begin.
async fn sleep_until_the_next_boundary(now: Epoch) {
    let next = Epoch(now.0.saturating_add(1)).starts_at_unix_seconds();
    let seconds = next.saturating_sub(epoch::unix_now_seconds());
    // A clock that jumped forward past the boundary gives zero here. Waiting a second
    // rather than spinning is the difference between a hot loop and a late start.
    tokio::time::sleep(Duration::from_secs(seconds.max(1))).await;
}

/// "1 epoch" or "N epochs", said the way a person would.
pub(super) fn epochs(count: u64) -> String {
    if count == 1 {
        "1 epoch answered for".to_owned()
    } else {
        format!("{count} epochs answered for")
    }
}
