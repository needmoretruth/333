//! `333 join` — ask a node that has the file to hand it over.
//!
//! This is the only way to become a member, and it is deliberately a thing one node
//! asks another for rather than a thing a client can do on its own. The binary carries
//! the hash of `333.txt` and not its contents, so it can tell the file from anything
//! else and cannot produce one.
//!
//! What it leaves behind: the file itself, and the two signed halves that say who
//! handed it over and when. Those halves are what everybody else reads as this node's
//! beginning.

use std::time::Duration;

use anyhow::Context as _;
use n333_core::Epoch;
use n333_core::enrollment;

use crate::commands::{Common, describe};
use crate::dial::Dialer;
use crate::node::Node;

/// Ask the node at `address` for the file.
///
/// # Errors
/// Fails if this node cannot be opened, the peer cannot be reached, it will not hand
/// the file over, or what it hands over is not the file.
pub(crate) async fn run(common: &Common, address: &n333_net::PeerAddress) -> anyhow::Result<()> {
    let (node, opened) = Node::open(&common.mistrust(), common.paths.root(), common.keeping)?;
    aloud_in!("join-name", name = node.identity().node_id().to_string());
    crate::commands::report_opening(&opened);
    ask(&node, &Dialer::new(common.clone()), common.timeout, address).await?;
    aloud_in!("join-vigil");
    Ok(())
}

/// Ask for the file, as a node that is already open.
///
/// Shared with the vigil, which holds the node and the dialler already. Opening either
/// a second time inside it would be a second writer to files it is writing.
///
/// # Errors
/// Fails if the peer cannot be reached, it will not hand the file over, or what it
/// hands over is not the file.
pub(crate) async fn ask(
    node: &Node,
    dialer: &Dialer,
    timeout: Duration,
    address: &n333_net::PeerAddress,
) -> anyhow::Result<()> {
    let typed = address.to_string();
    aloud_in!("join-knocking", address = &typed);

    let mut stream = match dialer.dial(address).await {
        Ok(stream) => stream,
        Err(e) => {
            // Not the end of anything. A door nobody opens is a door nobody opens. What
            // stood in the way is the failure below this: nobody answering is one of
            // several things it can be, and a name that does not resolve is another.
            aloud_in!("join-silence", address = &typed);
            return Err(e.context(words!("join-knocking-on", address = &typed)));
        }
    };
    let round = async {
        let exchange = n333_net::initiate(&mut stream, node.identity())
            .await
            .with_context(|| words!("join-exchanging"))?;
        crate::aloud::line(&describe(&exchange));
        // Kept once somebody answered there with a key, and not before: the vigil
        // knocks on every address this node was given, and one nobody answers at
        // would be knocked on for nothing.
        node.given_by_hand(&typed, Some(exchange.peer.node_id))
            .await?;
        n333_net::handover::ask(&mut stream, node.identity(), Epoch::now())
            .await
            .with_context(|| words!("join-asking"))
    };
    let taken = tokio::time::timeout(timeout, round).await.map_err(|_| {
        anyhow::anyhow!(words!(
            "join-no-answer",
            address = &typed,
            seconds = timeout.as_secs()
        ))
    })??;

    let joined = taken.handover.transfer.epoch();
    aloud_in!(
        "join-given",
        giver = taken.handover.transfer.giver().to_string()
    );
    crate::aloud::line(&crate::commands::what_was_signed(
        &taken.handover.transfer,
        false,
    ));
    aloud_in!("join-joined", epoch = joined.0);
    node.receive(taken.subject).await?;
    aloud_in!("join-holding");

    // The pair goes in last so that a giver who passed on nothing still leaves this
    // node with its own beginning written down.
    let mut passed = taken.tidings;
    passed.push(taken.handover.gave);
    passed.push(taken.handover.received);
    let from = crate::node::sources::Source::Peer {
        name: taken.handover.transfer.giver().to_string(),
    };
    let heard = node.hear(&passed, Epoch::now(), &from).await?;
    node.write_down_sources(Epoch::now()).await?;
    crate::commands::report_heard(&heard);
    aloud_in!("join-roll", members = node.roll().await.len());
    aloud_in!(
        "join-counted",
        epoch = enrollment::active_from(joined).0,
        least = EPOCH_MINUTES,
        most = 2 * EPOCH_MINUTES
    );
    Ok(())
}

/// How long one epoch is, in the minutes the wait for being counted is said in.
const EPOCH_MINUTES: u64 = n333_core::epoch::EPOCH_SECONDS / 60;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::words::count::Base;
    use crate::words::layout::{COLUMN, where_the_words_begin};

    const AT: &str = "127.0.0.1:3333";

    fn lines() -> Vec<String> {
        vec![
            words!("join-name", name = "333abc"),
            words!("join-knocking", address = AT),
            words!("join-silence", address = AT),
            words!("join-given", giver = "333def"),
            words!("join-joined", epoch = 89_612_u64),
            words!("join-holding"),
            words!("join-roll", members = 7_usize),
            words!(
                "join-counted",
                epoch = 89_614_u64,
                least = EPOCH_MINUTES,
                most = 2 * EPOCH_MINUTES
            ),
            words!("join-vigil"),
        ]
    }

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let (said, phrases) = crate::words::speaking("en", Base::Ten, || {
            let phrases = [
                (
                    words!("join-knocking-on", address = AT),
                    "knocking on 127.0.0.1:3333",
                ),
                (words!("join-exchanging"), "exchanging heartbeats"),
                (words!("join-asking"), "asking for the file"),
                (
                    words!("join-no-answer", address = AT, seconds = 30_u64),
                    "no answer from 127.0.0.1:3333 after 30 s",
                ),
            ];
            (lines(), phrases)
        });
        assert_eq!(
            said,
            [
                "name     333abc",
                "knocking 127.0.0.1:3333",
                "silence  nobody was reached at 127.0.0.1:3333. That is not proof that 333 is\n\
                 \x20        over. This client carries the hash of the file and not the file:\n\
                 \x20        there is no way in except from someone who holds it.",
                "given    by 333def",
                "joined   in epoch 89612",
                "holding  the file, and able to pass it on",
                "roll     7 of us",
                "counted  from epoch 89614, and not one epoch sooner: two boundaries away, between\n\
                 \x20        333 and 666 minutes, depending on where in this epoch you arrived.\n\
                 \x20        Until then, answer everything that is asked of you. What is witnessed\n\
                 \x20        in that time is the whole of the proof that you were ever here at all.",
                "vigil    run `333 serve` and stay awake. Nothing can be witnessed of a node\n\
                 \x20        nobody can reach, and this stretch is witnessed once or never.",
            ]
        );
        for (now, before) in phrases {
            assert_eq!(now, before);
        }
    }

    #[test]
    fn in_korean_every_line_of_join_begins_its_words_in_the_same_column() {
        for line in crate::words::speaking("ko", Base::Ten, lines) {
            assert!(!line.is_ascii(), "{line:?}");
            assert_eq!(where_the_words_begin(&line), COLUMN, "{line:?}");
        }
    }
}
