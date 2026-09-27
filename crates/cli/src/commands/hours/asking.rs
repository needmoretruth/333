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
            aloud_in!("hours-asking-failed-gathering", why = format!("{e:#}"));
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
                Err(e) => aloud_in!(
                    "hours-asking-quiet",
                    address = &address,
                    why = format!("{e:#}")
                ),
            }
        })
        .buffer_unordered(AT_ONCE)
        .collect::<()>();
    if tokio::time::timeout(TRADING_BUDGET, round).await.is_err() {
        aloud_in!(
            "hours-asking-unended",
            time = super::minutes(TRADING_BUDGET)
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
            aloud_in!("hours-asking-failed-gathering", why = format!("{e:#}"));
            return false;
        }
    };
    match trade_with(node, dialer, address, now, &mine.frames).await {
        Ok(heard) => {
            crate::commands::report_heard(&heard);
            true
        }
        Err(e) => {
            aloud_in!(
                "hours-asking-quiet",
                address = address,
                why = format!("{e:#}")
            );
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
    let typed = address;
    let address: PeerAddress = address
        .parse()
        .with_context(|| words!("hours-asking-reading-address"))?;
    let (teller, theirs) = tokio::time::timeout(ROUND_TIMEOUT, async {
        let mut stream = dialer.dial(&address).await?;
        let exchange = n333_net::initiate(&mut stream, node.identity())
            .await
            .with_context(|| words!("hours-asking-exchanging-heartbeats"))?;
        let theirs = gossip::tell(&mut stream, node.identity(), now, mine)
            .await
            .with_context(|| words!("hours-asking-trading"))?;
        anyhow::Ok((exchange.peer.node_id, theirs))
    })
    .await
    .map_err(|_| within(&words!("hours-asking-no-answer"), ROUND_TIMEOUT))??;
    node.answered_at(typed, teller).await;
    let from = crate::node::sources::Source::Peer {
        name: teller.to_string(),
    };
    node.hear(&theirs, now, &from).await
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
    aloud_in!("hours-asking-going", drawn = verifiers.len());
    for verifier in verifiers {
        let Some(address) = node.address_of(&verifier).await else {
            aloud_in!("hours-asking-unknown-drawn-by");
            continue;
        };
        if let Err(e) = present_to(node, dialer, &address, now).await {
            aloud_in!(
                "hours-asking-unasked",
                epoch = now.0,
                why = format!("{e:#}")
            );
        }
    }
}

/// Arrive, say what for, and answer if there is a question waiting.
async fn present_to(node: &Node, dialer: &Dialer, address: &str, now: Epoch) -> anyhow::Result<()> {
    let address: PeerAddress = address
        .parse()
        .with_context(|| words!("hours-asking-reading-address"))?;
    let mut stream = tokio::time::timeout(ROUND_TIMEOUT, dialer.dial(&address))
        .await
        .map_err(|_| {
            let what = words!("hours-asking-did-not-answer", address = address.to_string());
            within(&what, ROUND_TIMEOUT)
        })??;
    tokio::time::timeout(
        ROUND_TIMEOUT,
        n333_net::initiate(&mut stream, node.identity()),
    )
    .await
    .map_err(|_| {
        let what = words!("hours-asking-did-not-finish", address = address.to_string());
        within(&what, ROUND_TIMEOUT)
    })?
    .with_context(|| words!("hours-asking-exchanging-heartbeats"))?;

    let frame = Presenting::of(node.identity(), now)
        .seal(node.identity())
        .with_context(|| words!("hours-asking-sealing-presenting"))?;
    n333_net::frame::write_frame(&mut stream, &frame)
        .await
        .with_context(|| words!("hours-asking-saying-what-for"))?;

    // A verifier that was not drawn hangs up, and that is this node reading `None`. It
    // is the ordinary outcome and not a failure of anything.
    let Some(question) = tokio::time::timeout(
        ROUND_TIMEOUT,
        n333_net::liveness::take_question(&mut stream),
    )
    .await
    .map_err(|_| {
        let what = words!("hours-asking-neither", address = address.to_string());
        within(&what, ROUND_TIMEOUT)
    })??
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
    aloud_in!("hours-asking-drawn", epoch = now.0, asked = asked.len());

    for peer in asked {
        let Some(address) = node.address_of(&peer).await else {
            // Drawn to ask somebody nobody has said the whereabouts of. Not their
            // fault and not a silence worth publishing: this node simply cannot ask.
            aloud_in!("hours-asking-unknown-drawn-to-ask");
            continue;
        };
        match ask_one(node, dialer, &address, peer, now).await {
            Ok(()) => {}
            Err(e) => aloud_in!(
                "hours-asking-unheard",
                epoch = now.0,
                why = format!("{e:#}")
            ),
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
    let address: PeerAddress = address
        .parse()
        .with_context(|| words!("hours-asking-reading-address"))?;
    let mut stream = tokio::time::timeout(ROUND_TIMEOUT, dialer.dial(&address))
        .await
        .map_err(|_| {
            let what = words!("hours-asking-did-not-answer", address = address.to_string());
            within(&what, ROUND_TIMEOUT)
        })??;
    tokio::time::timeout(
        ROUND_TIMEOUT,
        n333_net::initiate(&mut stream, node.identity()),
    )
    .await
    .map_err(|_| {
        let what = words!("hours-asking-did-not-finish", address = address.to_string());
        within(&what, ROUND_TIMEOUT)
    })?
    .with_context(|| words!("hours-asking-exchanging-heartbeats"))?;

    // From here on the peer has been reached, so silence is the peer's silence and not
    // the road's. Everything before this point says nothing about anybody and is
    // reported without a statement being made.
    let question = liveness::put(&mut stream, node.identity(), peer, now)
        .await
        .with_context(|| words!("hours-asking-putting"))?;
    match tokio::time::timeout(ROUND_TIMEOUT, question.hear(&mut stream, node.identity())).await {
        Ok(Ok(witnessed)) => {
            aloud_in!(
                "hours-asking-witness",
                epoch = now.0,
                prover = witnessed.exchange.answer.prover.to_string()
            );
            node.keep(now, &witnessed.attestation).await
        }
        // The whole of it: the outermost sentence of a liveness failure is only which
        // part of the exchange it was in.
        Ok(Err(e)) => {
            unanswered(
                node,
                &question,
                now,
                &format!("{:#}", anyhow::Error::new(e)),
            )
            .await
        }
        Err(_elapsed) => {
            unanswered(
                node,
                &question,
                now,
                &words!(
                    "hours-asking-nothing-within",
                    seconds = ROUND_TIMEOUT.as_secs()
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
        .with_context(|| words!("hours-asking-sealing-silence"))?;
    aloud_in!("hours-asking-silence", epoch = now.0, why = why);
    node.keep(now, &question.frame).await?;
    node.keep(now, &sealed).await
}

/// A deadline that passed, said as what did not happen and how long it was given.
///
/// The deadline's own words are "deadline has elapsed", which says the same thing again
/// and not how long.
fn within(what: &str, deadline: Duration) -> anyhow::Error {
    anyhow::anyhow!(words!(
        "hours-asking-within",
        what = what,
        seconds = deadline.as_secs()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let pairs = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            [
                (
                    words!("hours-asking-failed-gathering", why = "disk full"),
                    "failed   gathering what this node could pass on: disk full".to_owned(),
                ),
                (
                    words!("hours-asking-quiet", address = "192.0.2.7:3333", why = "refused"),
                    "quiet    192.0.2.7:3333: refused".to_owned(),
                ),
                (
                    words!("hours-asking-going", drawn = 2_usize),
                    "going    nobody outside can open a connection to this node, so it goes to the\n\
                     \x20        2 of us drawn to ask it this epoch. The draw falls out of the epoch\n\
                     \x20        and the keys, so this node knows who they are without being told."
                        .to_owned(),
                ),
                (
                    words!("hours-asking-unknown-drawn-by"),
                    "unknown  drawn to be asked by one of us that nobody has said the whereabouts of"
                        .to_owned(),
                ),
                (
                    words!("hours-asking-unasked", epoch = 9_u64, why = "no"),
                    "unasked  epoch 9: no".to_owned(),
                ),
                (
                    words!("hours-asking-drawn", epoch = 9_u64, asked = 3_usize),
                    "drawn    epoch 9 — to ask 3 of us. Nobody chose that: the names fall out of\n\
                     \x20        the epoch and the keys, identically on every machine."
                        .to_owned(),
                ),
                (
                    words!("hours-asking-unknown-drawn-to-ask"),
                    "unknown  drawn to ask one of us that nobody has said the whereabouts of"
                        .to_owned(),
                ),
                (
                    words!("hours-asking-unheard", epoch = 9_u64, why = "no"),
                    "unheard  epoch 9: no".to_owned(),
                ),
                (
                    words!("hours-asking-witness", epoch = 9_u64, prover = "333ab"),
                    "witness  epoch 9 answered by 333ab".to_owned(),
                ),
                (
                    words!("hours-asking-silence", epoch = 9_u64, why = "no"),
                    "silence  epoch 9: no".to_owned(),
                ),
                (
                    words!("hours-asking-nothing-within", seconds = 180_u64),
                    "nothing within the 180 s the window allows".to_owned(),
                ),
                (
                    within(
                        &words!("hours-asking-did-not-answer", address = "192.0.2.7:3333"),
                        ROUND_TIMEOUT,
                    )
                    .to_string(),
                    format!(
                        "192.0.2.7:3333 did not answer within the {} s the window allows",
                        ROUND_TIMEOUT.as_secs()
                    ),
                ),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

    #[test]
    fn a_trading_that_ran_out_of_time_says_so_in_a_keyword_that_fits_the_column() {
        // It was `unfinished  `, three columns too wide, with its next line to match.
        let said = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            words!("hours-asking-unended", time = "111 minutes")
        });
        assert_eq!(
            said,
            "unended  the trading did not finish within 111 minutes of this epoch, and the\n\
             \x20        rest of the hours will not wait for it"
        );
    }
}
