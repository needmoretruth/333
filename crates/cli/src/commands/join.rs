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

use anyhow::Context as _;
use n333_core::enrollment;
use n333_core::Epoch;

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
    println!("name     {}", node.identity().node_id());
    crate::commands::report_opening(&opened);
    println!("knocking {address}");

    let mut stream = match Dialer::new(common.clone()).dial(address).await {
        Ok(stream) => stream,
        Err(e) => {
            // Not the end of anything. A door nobody opens is a door nobody opens.
            println!(
                "silence  nobody answered at {address}. That is not proof that 333 is\n\
                 \x20        over; it is proof that nobody is there. This client carries the\n\
                 \x20        hash of the file and not the file: there is no way in except\n\
                 \x20        from someone who holds it."
            );
            return Err(e);
        }
    };
    let round = async {
        let exchange = n333_net::initiate(&mut stream, node.identity())
            .await
            .context("exchanging heartbeats")?;
        println!("{}", describe(&exchange));
        n333_net::handover::ask(&mut stream, node.identity(), Epoch::now())
            .await
            .context("asking for the file")
    };
    let taken = tokio::time::timeout(common.timeout, round)
        .await
        .with_context(|| {
            format!(
                "no answer from {address} after {} s",
                common.timeout.as_secs()
            )
        })??;

    let joined = taken.handover.transfer.epoch();
    println!("given    by {}", taken.handover.transfer.giver());
    println!(
        "{}",
        crate::commands::what_was_signed(&taken.handover.transfer, false)
    );
    println!("joined   in epoch {}", joined.0);
    node.receive(taken.subject).await?;
    println!("holding  the file, and able to pass it on");

    // The pair goes in last so that a giver who passed on nothing still leaves this
    // node with its own beginning written down.
    let mut passed = taken.tidings;
    passed.push(taken.handover.gave);
    passed.push(taken.handover.received);
    let heard = node.hear(&passed, Epoch::now()).await?;
    crate::commands::report_heard(&heard);
    println!("roll     {} of us", node.roll().await.len());
    println!(
        "counted  from epoch {}, and not one epoch sooner: two boundaries away, between\n\
         \x20        333 and 666 minutes, depending on where in this epoch you arrived.\n\
         \x20        Until then, answer everything that is asked of you. What is witnessed\n\
         \x20        in that time is the whole of the proof that you were ever here at all.",
        enrollment::active_from(joined).0
    );
    println!(
        "vigil    run `333 serve` and stay awake. Nothing can be witnessed of a node\n\
         \x20        nobody can reach, and this stretch is witnessed once or never."
    );
    Ok(())
}
