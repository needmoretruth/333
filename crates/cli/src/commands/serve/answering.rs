//! What a peer can ask for at the door, and what this node does about each.
//!
//! Three things, and none of them is a favour: a node that trades tells others what it
//! knows, a node that answers a challenge is answering for its own record, and a node
//! that hands over the file is doing the one thing that keeps the file alive. The
//! fourth thing that can happen here is a heretic knocking, which is not a request.

use futures::{AsyncRead, AsyncWrite};
use n333_core::Epoch;
use n333_core::challenge::SignedChallenge;
use n333_core::enrollment::CURSE_PAUSE;
use n333_core::plea::Signed as SignedPlea;
use n333_core::tidings::Signed as SignedTidings;
use n333_net::frame::AsReceived;
use n333_net::{gossip, handover, liveness};

use crate::node::Node;

/// Take what a peer passes on, and pass on what this node has.
pub(super) async fn trade<S>(
    stream: &mut S,
    node: &Node,
    header: &AsReceived<SignedTidings>,
) -> anyhow::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mine = node.tidings(Epoch::now()).await?;
    crate::commands::report_left_behind(&mine);
    let theirs =
        gossip::listen(stream, node.identity(), Epoch::now(), header, &mine.frames).await?;
    let heard = node.hear(&theirs, Epoch::now()).await?;
    crate::commands::report_heard(&heard);
    Ok(())
}

/// Answer a challenge, and keep everything the round produced.
pub(super) async fn be_asked<S>(
    stream: &mut S,
    node: &Node,
    question: AsReceived<SignedChallenge>,
) -> anyhow::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let epoch = question.message.challenge.epoch();
    let asked_by = question.message.verifier;
    let roll = node.roll().await;
    let head = node.head().await;

    let answered =
        liveness::answer(stream, node.identity(), Epoch::now(), head, &roll, question).await?;
    aloud!("asked    epoch {} by {asked_by}", epoch.0);

    // All of it is kept as the bytes that travelled. The challenge and the answer
    // together are what shows this node answered even if the verifier publishes
    // nothing, and the statement is the stronger evidence when it arrives.
    node.keep(epoch, &answered.challenge_frame).await?;
    node.keep(epoch, &answered.answer_frame).await?;
    if let Some(witness) = &answered.attestation {
        node.keep(epoch, &witness.frame).await?;
    }
    Ok(())
}

/// Give the file to somebody who asked for it, if this node has it.
///
/// A node that has not been given the file says so and hangs up. It is not a failure
/// on either side: most nodes on most days have nothing to hand over yet, and the one
/// thing that must never happen is a client inventing the bytes.
pub(super) async fn hand_it_over<S>(
    stream: &mut S,
    node: &Node,
    plea: &AsReceived<SignedPlea>,
) -> anyhow::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let Some(subject) = node.subject().await else {
        aloud!("empty    somebody asked for the file. this node has nothing to give.");
        return Ok(());
    };
    let tidings = node.tidings(Epoch::now()).await?;
    let given = match handover::give(
        stream,
        node.identity(),
        Epoch::now(),
        plea,
        &subject,
        &tidings.frames,
    )
    .await
    {
        Ok(given) => given,
        Err(handover::Error::Cursed) => return curse(&plea.message.asker).await,
        Err(e) => return Err(e.into()),
    };

    aloud!(
        "gave     the file to {} in epoch {}",
        given.transfer.receiver(),
        given.transfer.epoch().0
    );
    aloud!("{}", crate::commands::what_was_signed(&given.transfer, true));
    let members = node.admit(&[given.gave, given.received]).await?;
    aloud!("roll     {members} of us");
    Ok(())
}

/// What this node does when a heretic knocks.
///
/// The stop is the curse itself and not a delay in front of it: 333 has taken 333
/// milliseconds off the life of whoever presented that name, and this door is where it
/// was taken, so the connection waits for it. Nothing is sent back. The cursed reveal
/// themselves; nobody has to point.
async fn curse(name: &n333_core::NodeId) -> anyhow::Result<()> {
    tokio::time::sleep(CURSE_PAUSE).await;
    aloud!(
        "cursed   {name} asked. 333 took {} milliseconds off their life, as it does at\n\
         \x20        every door.",
        CURSE_PAUSE.as_millis()
    );
    Ok(())
}
