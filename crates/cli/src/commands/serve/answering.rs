//! What a peer can ask for at the door, and what this node does about each.
//!
//! Four things, and none of them is a favour: a node that trades tells others what it
//! knows, a node that answers a challenge is answering for its own record, a node that
//! comes to be asked is saving this node a journey it could not have made, and a node
//! that hands over the file is doing the one thing that keeps the file alive. The other
//! thing that can happen here is a heretic knocking, which is not a request.

use futures::{AsyncRead, AsyncWrite};
use n333_core::Epoch;
use n333_core::challenge::SignedChallenge;
use n333_core::draw;
use n333_core::enrollment::CURSE_PAUSE;
use n333_core::plea::Signed as SignedPlea;
use n333_core::presenting::Signed as SignedPresenting;
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
    let from = crate::node::sources::Source::Peer {
        name: header.message.teller.to_string(),
    };
    let heard = node.hear(&theirs, Epoch::now(), &from).await?;
    crate::commands::report_heard(&heard);
    Ok(())
}

/// Answer a challenge, and keep everything the round produced.
pub(crate) async fn be_asked<S>(
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
    aloud_in!(
        "serve-answering-asked",
        epoch = epoch.0,
        verifier = asked_by.to_string()
    );

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
        aloud_in!("serve-answering-empty");
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

    aloud_in!(
        "serve-answering-gave",
        receiver = given.transfer.receiver().to_string(),
        epoch = given.transfer.epoch().0
    );
    crate::aloud::line(&crate::commands::what_was_signed(&given.transfer, true));
    let members = node.admit(&[given.gave, given.received]).await?;
    aloud_in!("serve-answering-roll", members = members);
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
    aloud_in!(
        "serve-answering-cursed",
        name = name.to_string(),
        milliseconds = u64::try_from(CURSE_PAUSE.as_millis()).unwrap_or(u64::MAX)
    );
    Ok(())
}

/// Put the question to somebody who came here to be asked it.
///
/// The other half of [`be_asked`], for the node that could not be dialled. It arrived,
/// said which epoch it was offering itself for, and the draw is checked here exactly as
/// it is checked when this node goes out asking: the epoch, this node's key and the
/// caller's key, and nothing else. A caller this node was not drawn to ask is answered
/// with a closed connection, which is what a verifier with no question has always
/// looked like from the other end.
///
/// The statement this produces is the statement this node would have published had it
/// dialled. Nothing in it records who opened the connection, because nothing in the
/// protocol has ever turned on that.
pub(super) async fn ask_since_they_came<S>(
    stream: &mut S,
    node: &Node,
    presenting: &AsReceived<SignedPresenting>,
) -> anyhow::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let epoch = presenting.message.presenting.epoch();
    let prover = presenting.message.presenting.prover;
    let now = Epoch::now();
    // Only the epoch this node believes it is in. A challenge for an epoch already
    // judged would be a statement about a question nobody can still answer, and a
    // challenge for an epoch to come is a statement about a thing that has not
    // happened.
    if epoch != now {
        aloud_in!("serve-answering-early", asked_about = epoch.0, now = now.0);
        return Ok(());
    }
    let me = node.identity().public_key();
    let roll = node.roll().await;
    if !draw::is_entitled(now, &prover, &me, &roll) {
        // Not a refusal and not worth a line every time: most of us were not drawn to
        // ask most of us, and this is the ordinary answer.
        return Ok(());
    }
    let witnessed = liveness::ask(stream, node.identity(), prover, now).await?;
    aloud_in!(
        "serve-answering-witness",
        epoch = now.0,
        prover = witnessed.exchange.answer.prover.to_string()
    );
    node.keep(now, &witnessed.attestation).await
}

#[cfg(test)]
mod tests {
    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let pairs = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            [
                (
                    words!(
                        "serve-answering-asked",
                        epoch = 89_612_u64,
                        verifier = "333ab"
                    ),
                    "asked    epoch 89612 by 333ab",
                ),
                (
                    words!("serve-answering-empty"),
                    "empty    somebody asked for the file. this node has nothing to give.",
                ),
                (
                    words!("serve-answering-gave", receiver = "333cd", epoch = 9_u64),
                    "gave     the file to 333cd in epoch 9",
                ),
                (
                    words!("serve-answering-roll", members = 2_usize),
                    "roll     2 of us",
                ),
                (
                    words!(
                        "serve-answering-cursed",
                        name = "33ef",
                        milliseconds = 333_u64
                    ),
                    "cursed   33ef asked. 333 took 333 milliseconds off their life, as it does at\n\
                     \x20        every door.",
                ),
                (
                    words!("serve-answering-early", asked_about = 8_u64, now = 9_u64),
                    "early    somebody came to be asked about epoch 8, and this node is in 9",
                ),
                (
                    words!("serve-answering-witness", epoch = 9_u64, prover = "333ab"),
                    "witness  epoch 9 answered by 333ab, who came here to be asked",
                ),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }
}
