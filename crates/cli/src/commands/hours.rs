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

use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use n333_core::attestation::{self, Evidence, JUDGEMENT_DELAY_EPOCHS};
use n333_core::enrollment;
use n333_core::challenge::{self, Exchange, RESPONSE_WINDOW_SECONDS};
use n333_core::chain::evidence_digest;
use n333_core::presence::Attendance;
use n333_core::whereabouts::Whereabouts;
use n333_core::{Epoch, draw, epoch};
use n333_net::{PeerAddress, gossip, liveness};

use crate::dial::Dialer;
use crate::node::Node;

/// How long one challenge round may take.
///
/// The response window is what the protocol allows a node to answer within, so this
/// node stops waiting when the answer would no longer count anyway. Over Tor a round
/// can genuinely take tens of seconds, which is why the window is minutes and not
/// seconds.
const ROUND_TIMEOUT: Duration = Duration::from_secs(RESPONSE_WINDOW_SECONDS);

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
            println!("failed   marking this epoch as kept: {e:#}");
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
                println!("failed   keeping this node's own address: {e:#}");
            }
        }
        Err(e) => println!("failed   saying where this node is: {e:#}"),
    }
}

/// Trade statements with every node this one knows how to reach.
///
/// Every one of them, not a sample: at this cadence a node with a thousand neighbours
/// opens three connections a minute, and choosing a few would mean choosing, which
/// means a rule about whom to prefer, which is a thing this protocol does not have.
async fn trade_news(node: &Node, dialer: &Dialer, now: Epoch) {
    let mine = match node.tidings(now).await {
        Ok(mine) => mine,
        Err(e) => {
            println!("failed   gathering what this node could pass on: {e:#}");
            return;
        }
    };
    for address in node.where_others_are().await {
        match trade_with(node, dialer, &address, now, &mine).await {
            Ok(heard) => crate::commands::report_heard(&heard),
            Err(e) => println!("quiet    {address}: {e:#}"),
        }
    }
}

/// One round with one node: greet, trade, file whatever came back.
async fn trade_with(
    node: &Node,
    dialer: &Dialer,
    address: &str,
    now: Epoch,
    mine: &[Vec<u8>],
) -> anyhow::Result<crate::node::Heard> {
    let address: PeerAddress = address.parse().context("reading a peer's address")?;
    let theirs = tokio::time::timeout(ROUND_TIMEOUT, async {
        let mut stream = dialer.dial(&address).await?;
        n333_net::initiate(&mut stream, node.identity())
            .await
            .context("exchanging heartbeats")?;
        gossip::tell(&mut stream, node.identity(), now, mine)
            .await
            .context("trading statements")
    })
    .await
    .with_context(|| format!("no answer from {address} within the window"))??;
    node.hear(&theirs).await
}

/// Ask everybody this node was drawn to ask this epoch.
async fn ask_those_drawn(node: &Node, dialer: &Dialer, now: Epoch) {
    let roll = node.roll().await;
    let me = node.identity().public_key();
    let asked: Vec<_> = roll
        .iter()
        .filter(|peer| **peer != me)
        .filter(|peer| draw::is_entitled(now, peer, &me, &roll))
        .copied()
        .collect();
    if asked.is_empty() {
        return;
    }

    for peer in asked {
        let Some(address) = node.address_of(&peer).await else {
            // Drawn to ask somebody nobody has said the whereabouts of. Not their
            // fault and not a silence worth publishing: this node simply cannot ask.
            println!("unknown  drawn to ask one of us that nobody has said the whereabouts of");
            continue;
        };
        match ask_one(node, dialer, &address, peer, now).await {
            Ok(()) => {}
            Err(e) => println!("unheard  epoch {}: {e:#}", now.0),
        }
    }
}

/// One round with one node: greet, ask, keep the statement.
async fn ask_one(
    node: &Node,
    dialer: &Dialer,
    address: &str,
    peer: [u8; 32],
    now: Epoch,
) -> anyhow::Result<()> {
    let address: PeerAddress = address.parse().context("reading a peer's address")?;
    let witnessed = tokio::time::timeout(ROUND_TIMEOUT, async {
        let mut stream = dialer.dial(&address).await?;
        n333_net::initiate(&mut stream, node.identity())
            .await
            .context("exchanging heartbeats")?;
        liveness::ask(&mut stream, node.identity(), peer, now)
            .await
            .context("putting the question")
    })
    .await
    .with_context(|| format!("no answer from {address} within the window"))??;

    println!(
        "witness  epoch {} answered by {}",
        now.0, witnessed.exchange.answer.prover
    );
    // Published by keeping it: this node's own copy is what it will hand on. The peer
    // already has its own, handed back on the same connection.
    node.keep(now, &witnessed.attestation).await
}

/// Judge the epoch that has now had long enough for statements to arrive.
async fn judge_what_is_ready(node: &Node, now: Epoch) {
    let Some(ready) = now.0.checked_sub(JUDGEMENT_DELAY_EPOCHS).map(Epoch) else {
        return;
    };
    // A node nobody has admitted has nothing to judge: no verifier is ever drawn for
    // it, so every epoch would be an empty entry saying nothing.
    let Some(joined) = node.joined_in().await else {
        return;
    };
    // Epochs before this node counted are not absences and not excluded epochs. They
    // are epochs it was not here for, and its record has nothing to say about them.
    if !enrollment::covers(joined, ready) {
        return;
    }
    match node.last_judged().await {
        Ok(Some(last)) if last.0 >= ready.0 => return,
        Ok(_) => {}
        Err(e) => {
            println!("failed   reading this node's own record: {e:#}");
            return;
        }
    }
    if let Err(e) = judge_one(node, ready).await {
        println!("failed   judging epoch {}: {e:#}", ready.0);
    }
}

/// Read one epoch's statements, reach a verdict, and write it down.
async fn judge_one(node: &Node, epoch: Epoch) -> anyhow::Result<()> {
    let frames = node.statements(epoch).await?;
    let me = node.identity().public_key();
    let roll = node.roll().await;

    let published: Vec<_> = frames
        .iter()
        .filter_map(|frame| attestation::open(frame).ok())
        .collect();
    let receipt = receipt_in(&frames, &me);
    let evidence = Evidence {
        attestations: published.iter().collect(),
        receipt: receipt.as_ref(),
    };
    let attendance = attestation::judge(epoch, &me, &roll, &evidence);
    let head = node
        .record(epoch, attendance, evidence_digest(&frames))
        .await?;
    println!("judged   epoch {}: {}", epoch.0, said(attendance));
    println!("record   {}", epochs(head.length));
    Ok(())
}

/// This node's own proof that it answered somebody, if it kept one.
///
/// Two signatures by two different keys: a challenge somebody put, and the answer this
/// node gave to it. It does not say *when* the answer was given, which is why it
/// withdraws an accusation rather than earning a presence — see [`attestation::judge`].
fn receipt_in(frames: &[Vec<u8>], me: &[u8; 32]) -> Option<Exchange> {
    let mine: Vec<_> = frames
        .iter()
        .filter_map(|frame| challenge::open_answer(frame).ok())
        .filter(|answer| answer.answer.prover == *me)
        .collect();
    frames
        .iter()
        .filter_map(|frame| challenge::open_challenge(frame).ok())
        .find_map(|question| {
            mine.iter()
                .find_map(|answer| Exchange::assemble(question.clone(), answer.clone()).ok())
        })
}

/// The word for a verdict, in the register the rest of the output uses.
///
/// An excluded epoch is not a bad mark and not a good one. Nobody was drawn to ask,
/// so there was no question to answer and the epoch leaves the count entirely — which
/// is a different thing from having been asked and having answered.
const fn said(attendance: Attendance) -> &'static str {
    match attendance {
        Attendance::Present => "present. it will not be judged again.",
        Attendance::Absent => "absent. it will not be judged again.",
        Attendance::Excluded => "nobody was drawn to ask, so the epoch counts for nothing.",
    }
}

/// Drop what can no longer change anybody's standing.
async fn forget_the_old(node: &Node, now: Epoch) {
    match node.forget_old(now).await {
        Ok(0) => {}
        Ok(dropped) => println!(
            "forgot   {dropped} epochs. nothing said about them now could change a verdict."
        ),
        Err(e) => println!("failed   forgetting old statements: {e:#}"),
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
fn epochs(count: u64) -> String {
    if count == 1 {
        "1 epoch answered for".to_owned()
    } else {
        format!("{count} epochs answered for")
    }
}
