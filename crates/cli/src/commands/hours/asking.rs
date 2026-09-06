//! Talking to the other nodes, once an epoch: trading what each of us knows, and
//! putting the question to whoever this node was drawn to ask.
//!
//! Both steps are bounded in the same two ways, because both of them can be made slow
//! by peers this node has no control over: rounds run concurrently, and the whole step
//! has a deadline. A node that spends its epoch dialling has not kept the hours, it has
//! only been busy.

use std::time::Duration;

use anyhow::Context as _;
use futures::StreamExt as _;
use n333_core::challenge::RESPONSE_WINDOW_SECONDS;
use n333_core::presenting::Presenting;
use n333_core::{Epoch, draw};
use n333_net::{PeerAddress, gossip, liveness};

use crate::dial::Dialer;
use crate::node::Node;

/// How long one round with one peer may take.
///
/// The response window is what the protocol allows a node to answer within, so this
/// node stops waiting when the answer would no longer count anyway. Over Tor a round
/// can genuinely take tens of seconds, which is why the window is minutes.
pub(super) const ROUND_TIMEOUT: Duration = Duration::from_secs(RESPONSE_WINDOW_SECONDS);

/// How much of an epoch may go on trading statements before the rest of the hours run.
///
/// A third. The questions this node was drawn to put are what its neighbours are
/// waiting on, and they must not be reached only after every dead address in the
/// directory has been dialled to its own timeout. A node that runs out of budget says
/// so and goes on; it does not skip the epoch.
const TRADING_BUDGET: Duration = Duration::from_secs(n333_core::EPOCH_SECONDS / 3);

/// How many peers to speak to at once.
///
/// Enough that a directory of dead addresses costs one round rather than a hundred, and
/// small enough that a node on a domestic connection is not opening more sockets than
/// it has any use for. Over Tor each one is a circuit.
const AT_ONCE: usize = 16;

/// Trade statements with every node this one knows how to reach.
///
/// Every one of them, not a sample: at this cadence a node with a thousand neighbours
/// opens three connections a minute, and choosing a few would mean choosing, which
/// means a rule about whom to prefer, which is a thing this protocol does not have.
pub(super) async fn trade_news(node: &Node, dialer: &Dialer, now: Epoch) {
    let mine = match node.tidings(now).await {
        Ok(mine) => mine,
        Err(e) => {
            aloud!("failed   gathering what this node could pass on: {e:#}");
            return;
        }
    };
    crate::commands::report_left_behind(&mine);

    let addresses = node.where_others_are().await;
    if addresses.is_empty() {
        return;
    }
    // Before the rounds, not inside one: a round is bounded by the response window and
    // a bootstrap is allowed longer than that, so paying for it inside a round means
    // giving up on a peer for being slow when what was slow was this node starting.
    dialer.wake_for(&addresses).await;
    // Concurrently and under a deadline, because the alternative is a node that spends
    // its whole epoch dialling. A hundred and twelve addresses nobody answers, one after
    // another at three minutes each, is longer than an epoch — the questions this node
    // was drawn to put would never be reached, and everyone waiting on them would lose
    // that epoch too.
    let mine = &mine.frames;
    // Borrowed rather than spawned: these all belong to this one step and none of them
    // outlives it, so there is nothing here that needs its own lifetime.
    let round = futures::stream::iter(addresses)
        .map(|address| async move {
            match trade_with(node, dialer, &address, now, mine).await {
                Ok(heard) => crate::commands::report_heard(&heard),
                Err(e) => aloud!("quiet    {address}: {e:#}"),
            }
        })
        .buffer_unordered(AT_ONCE)
        .collect::<()>();
    if tokio::time::timeout(TRADING_BUDGET, round).await.is_err() {
        aloud!(
            "unfinished  the trading did not finish within {} of this epoch, and the\n\
             \x20           rest of the hours will not wait for it",
            super::minutes(TRADING_BUDGET)
        );
    }
}

/// Trade with one address now, without waiting for the next epoch.
///
/// For a node that has just turned up on this network. The hours come round every 333
/// minutes, and a person who starts a second node in the same house and watches
/// nothing happen for five hours has been told, correctly, that nothing is happening.
pub(crate) async fn trade_at_once(node: &Node, dialer: &Dialer, address: &str) -> bool {
    let now = Epoch::now();
    let mine = match node.tidings(now).await {
        Ok(mine) => mine,
        Err(e) => {
            aloud!("failed   gathering what this node could pass on: {e:#}");
            return false;
        }
    };
    match trade_with(node, dialer, address, now, &mine.frames).await {
        Ok(heard) => {
            crate::commands::report_heard(&heard);
            true
        }
        Err(e) => {
            aloud!("quiet    {address}: {e:#}");
            false
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
    node.hear(&theirs, now).await
}

/// Ask everybody this node was drawn to ask this epoch.
/// Go to whoever was drawn to ask this node, because nobody can come here.
///
/// The other side of [`ask_those_drawn`], for a node that answers on nothing a stranger
/// can dial. It is not a workaround. The draw takes the epoch, the prover's key and the
/// candidate's key and nothing else, so this node can work out which of us were drawn to
/// ask it as exactly as they can — and once the connection is open, the question travels
/// down it in the ordinary direction.
///
/// Nothing is claimed by arriving. A verifier that was not drawn says nothing and hangs
/// up, and the statement a verifier that was drawn ends up publishing is the one it would
/// have published had it dialled.
pub(super) async fn present_myself(node: &Node, dialer: &Dialer, now: Epoch) {
    let roll = node.roll().await;
    let me = node.identity().public_key();
    let verifiers: Vec<_> = roll
        .iter()
        .filter(|verifier| **verifier != me)
        .filter(|verifier| draw::is_entitled(now, &me, verifier, &roll))
        .copied()
        .collect();
    if verifiers.is_empty() {
        return;
    }
    aloud!(
        "going    nobody outside can open a connection to this node, so it goes to the\n\
         \x20        {} of us drawn to ask it this epoch. The draw falls out of the epoch\n\
         \x20        and the keys, so this node knows who they are without being told.",
        verifiers.len()
    );
    for verifier in verifiers {
        let Some(address) = node.address_of(&verifier).await else {
            aloud!(
                "unknown  drawn to be asked by one of us that nobody has said the whereabouts of"
            );
            continue;
        };
        if let Err(e) = present_to(node, dialer, &address, now).await {
            aloud!("unasked  epoch {}: {e:#}", now.0);
        }
    }
}

/// Arrive, say what for, and answer if there is a question waiting.
async fn present_to(node: &Node, dialer: &Dialer, address: &str, now: Epoch) -> anyhow::Result<()> {
    let address: PeerAddress = address.parse().context("reading a peer's address")?;
    let mut stream = tokio::time::timeout(ROUND_TIMEOUT, dialer.dial(&address))
        .await
        .with_context(|| format!("{address} did not answer in time"))??;
    tokio::time::timeout(
        ROUND_TIMEOUT,
        n333_net::initiate(&mut stream, node.identity()),
    )
    .await
    .with_context(|| format!("{address} did not finish the heartbeat in time"))?
    .context("exchanging heartbeats")?;

    let frame = Presenting::of(node.identity(), now)
        .seal(node.identity())
        .context("sealing what this node came to say")?;
    n333_net::frame::write_frame(&mut stream, &frame)
        .await
        .context("saying what this node came for")?;

    // A verifier that was not drawn hangs up, and that is this node reading `None`. It
    // is the ordinary outcome and not a failure of anything.
    let Some(question) = tokio::time::timeout(
        ROUND_TIMEOUT,
        n333_net::liveness::take_question(&mut stream),
    )
    .await
    .with_context(|| format!("{address} neither asked nor hung up in time"))??
    else {
        return Ok(());
    };
    crate::commands::serve::answering::be_asked(&mut stream, node, question).await
}

pub(super) async fn ask_those_drawn(node: &Node, dialer: &Dialer, now: Epoch) {
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
    // Said out loud because from the outside being drawn looks like a chore the client
    // performs, and it is the one moment in an epoch where this node is doing something
    // nobody, including this node, chose.
    aloud!(
        "drawn    epoch {} — to ask {} of us. Nobody chose that: the names fall out of\n\
         \x20        the epoch and the keys, identically on every machine.",
        now.0,
        asked.len()
    );

    for peer in asked {
        let Some(address) = node.address_of(&peer).await else {
            // Drawn to ask somebody nobody has said the whereabouts of. Not their
            // fault and not a silence worth publishing: this node simply cannot ask.
            aloud!("unknown  drawn to ask one of us that nobody has said the whereabouts of");
            continue;
        };
        match ask_one(node, dialer, &address, peer, now).await {
            Ok(()) => {}
            Err(e) => aloud!("unheard  epoch {}: {e:#}", now.0),
        }
    }
}

/// One round with one node: greet, ask, and say what came of it either way.
///
/// BOTH OUTCOMES ARE PUBLISHED. A verifier that simply gave up when nobody answered
/// would leave no trace of having asked, and absence would be a thing that no node ever
/// said out loud — which would make the two-thirds rule unable to bind on anybody, for
/// ever. So silence is signed for too, and it is deliberately the weaker statement: it
/// carries no signature from the node it is about, because silence cannot be signed.
async fn ask_one(
    node: &Node,
    dialer: &Dialer,
    address: &str,
    peer: [u8; 32],
    now: Epoch,
) -> anyhow::Result<()> {
    let address: PeerAddress = address.parse().context("reading a peer's address")?;
    let mut stream = tokio::time::timeout(ROUND_TIMEOUT, dialer.dial(&address))
        .await
        .with_context(|| format!("{address} did not answer in time"))??;
    tokio::time::timeout(
        ROUND_TIMEOUT,
        n333_net::initiate(&mut stream, node.identity()),
    )
    .await
    .with_context(|| format!("{address} did not finish the heartbeat in time"))?
    .context("exchanging heartbeats")?;

    // From here on the peer has been reached, so silence is the peer's silence and not
    // the road's. Everything before this point says nothing about anybody and is
    // reported without a statement being made.
    let question = liveness::put(&mut stream, node.identity(), peer, now)
        .await
        .context("putting the question")?;
    match tokio::time::timeout(ROUND_TIMEOUT, question.hear(&mut stream, node.identity())).await {
        Ok(Ok(witnessed)) => {
            aloud!(
                "witness  epoch {} answered by {}",
                now.0,
                witnessed.exchange.answer.prover
            );
            node.keep(now, &witnessed.attestation).await
        }
        Ok(Err(e)) => unanswered(node, &question, now, &e.to_string()).await,
        Err(_elapsed) => {
            unanswered(
                node,
                &question,
                now,
                &format!(
                    "nothing within the {} s the window allows",
                    ROUND_TIMEOUT.as_secs()
                ),
            )
            .await
        }
    }
}

/// Publish and keep the statement that says nobody answered.
async fn unanswered(
    node: &Node,
    question: &liveness::Question,
    now: Epoch,
    why: &str,
) -> anyhow::Result<()> {
    let sealed = question
        .unanswered(node.identity())
        .context("sealing what did not happen")?;
    aloud!("silence  epoch {}: {why}", now.0);
    node.keep(now, &question.frame).await?;
    node.keep(now, &sealed).await
}
