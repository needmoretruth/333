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
    aloud!("name     {}", node.identity().node_id());
    crate::commands::report_opening(&opened);
    ask(&node, &Dialer::new(common.clone()), common.timeout, address).await?;
    aloud!(
        "vigil    run `333 serve` and stay awake. Nothing can be witnessed of a node\n\
         \x20        nobody can reach, and this stretch is witnessed once or never."
    );
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
    // Kept whether or not anybody answers: it is an address this node was given, and
    // the vigil goes on knocking there once it runs.
    node.given_by_hand(&address.to_string(), None).await?;
    aloud!("knocking {address}");

    let mut stream = match dialer.dial(address).await {
        Ok(stream) => stream,
        Err(e) => {
            // Not the end of anything. A door nobody opens is a door nobody opens. What
            // stood in the way is the failure below this: nobody answering is one of
            // several things it can be, and a name that does not resolve is another.
            aloud!(
                "silence  nobody was reached at {address}. That is not proof that 333 is\n\
                 \x20        over. This client carries the hash of the file and not the file:\n\
                 \x20        there is no way in except from someone who holds it."
            );
            return Err(e.context(format!("knocking on {address}")));
        }
    };
    let round = async {
        let exchange = n333_net::initiate(&mut stream, node.identity())
            .await
            .context("exchanging heartbeats")?;
        aloud!("{}", describe(&exchange));
        node.answered_at(&address.to_string(), exchange.peer.node_id)
            .await;
        n333_net::handover::ask(&mut stream, node.identity(), Epoch::now())
            .await
            .context("asking for the file")
    };
    let taken = tokio::time::timeout(timeout, round)
        .await
        .map_err(|_| anyhow::anyhow!("no answer from {address} after {} s", timeout.as_secs()))??;

    let joined = taken.handover.transfer.epoch();
    aloud!("given    by {}", taken.handover.transfer.giver());
    aloud!(
        "{}",
        crate::commands::what_was_signed(&taken.handover.transfer, false)
    );
    aloud!("joined   in epoch {}", joined.0);
    node.receive(taken.subject).await?;
    aloud!("holding  the file, and able to pass it on");

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
    aloud!("roll     {} of us", node.roll().await.len());
    aloud!(
        "counted  from epoch {}, and not one epoch sooner: two boundaries away, between\n\
         \x20        333 and 666 minutes, depending on where in this epoch you arrived.\n\
         \x20        Until then, answer everything that is asked of you. What is witnessed\n\
         \x20        in that time is the whole of the proof that you were ever here at all.",
        enrollment::active_from(joined).0
    );
    Ok(())
}
