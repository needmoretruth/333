//! Asking a node to prove it is awake, over a stream that is already open.
//!
//! One round, after the heartbeat exchange and on the same connection:
//!
//! ```text
//! verifier -> prover   challenge   (a nonce, for this epoch)
//! prover   -> verifier answer      (the nonce signed back, with its chain head)
//! verifier -> prover   attestation (the verifier's statement, carrying that answer)
//! ```
//!
//! WHY THE VERIFIER HANDS BACK ITS OWN STATEMENT. It has just made the strongest
//! piece of evidence that exists about this node, and the node is the one party
//! certain to want it spread. Sending it costs one frame on a connection that is
//! already open and puts the evidence where it will be carried furthest, without any
//! gossip protocol having to exist yet. The verifier still publishes it too — a
//! statement only the subject holds is a statement that vanishes when the subject
//! does.
//!
//! WHAT THE PROVER KEEPS EITHER WAY. The challenge it was sent and the answer it
//! made are two signatures from two different nodes, so even if the connection dies
//! before the attestation arrives — or if the verifier never publishes one — it can
//! show it answered. Not that it answered *in time*: only the verifier saw that.
//!
//! NOTHING HERE IS UNBOUNDED. The round is exactly three frames of known kinds, each
//! inside the wire's frame limit. A peer that says nothing is the caller's deadline
//! to handle, and a peer that says the wrong thing ends the round.

use std::collections::BTreeSet;

use futures::{AsyncRead, AsyncWrite};
use n333_core::attestation::{self, Attestation, SignedAttestation};
use n333_core::challenge::{self, Answer, Challenge, Exchange, SignedChallenge};
use n333_core::chain::Head;
use n333_core::{Epoch, Identity, draw};

use crate::frame;

/// Things that can go wrong during one round.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The stream failed, or the peer sent a frame this node will not read.
    #[error("frame: {0}")]
    Frame(#[from] frame::Error),
    /// The bytes arrived but are not the message this node was waiting for.
    #[error("message: {0}")]
    Message(#[from] n333_core::WireError),
    /// The peer's statement does not hold together.
    #[error("statement: {0}")]
    Statement(#[from] attestation::Invalid),
    /// The answer does not answer the challenge that was sent.
    #[error("answer: {0}")]
    Answer(#[from] challenge::NotAnAnswer),
    /// A node that was not drawn for this epoch asked the question.
    ///
    /// Refused rather than answered: answering everyone who asks would let one node
    /// make this one sign as many statements as it liked, and the draw is the only
    /// thing that says whose turn it is.
    #[error("this node was not drawn to ask about epoch {epoch}")]
    NotEntitled {
        /// The epoch that was asked about.
        epoch: u64,
    },
    /// The peer's statement is about somebody else, or some other epoch.
    #[error("the statement handed back is not about this exchange")]
    NotOurs,
}

/// What the verifier ends a round holding.
#[derive(Debug, Clone)]
pub struct Witnessed {
    /// The statement to publish. Carries the prover's answer inside it, so anybody
    /// can check it without trusting this node.
    pub attestation: Vec<u8>,
    /// The answer, already checked.
    pub exchange: Exchange,
}

/// Put the question, read the answer, and hand back the statement.
///
/// The caller supplies the deadline. Nothing here waits: a peer that has stopped
/// speaking looks exactly like a peer that is thinking, and only the caller knows how
/// long the response window has left.
///
/// # Errors
/// Fails if the stream fails, the answer is not an answer to this challenge, or the
/// statement cannot be sealed.
pub async fn ask<S>(
    stream: &mut S,
    verifier: &Identity,
    prover: [u8; 32],
    epoch: Epoch,
) -> Result<Witnessed, Error>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let question = Challenge::new(verifier, prover, epoch);
    let question_frame = question.seal(verifier)?;
    frame::write_frame(stream, &question_frame).await?;

    // The answer's bytes are kept exactly as they arrived and go into the statement
    // unchanged. Re-encoding them would make every later reader's signature check
    // depend on this encoder still producing the same bytes.
    let answer_frame = frame::read_frame(stream).await?;
    let exchange = Exchange::assemble(
        challenge::open_challenge(&question_frame)?,
        challenge::open_answer(&answer_frame)?,
    )?;

    let sealed = Attestation::answered(verifier, prover, epoch, question.nonce, answer_frame)
        .seal(verifier)?;
    frame::write_frame(stream, &sealed).await?;

    Ok(Witnessed {
        attestation: sealed,
        exchange,
    })
}

/// What the prover ends a round holding.
#[derive(Debug, Clone)]
pub struct Answered {
    /// The challenge and this node's answer: two signatures, from two nodes.
    pub receipt: Exchange,
    /// The verifier's statement, if it handed one back.
    pub attestation: Option<SignedAttestation>,
}

/// Read the question, answer it, and take the statement handed back.
///
/// `roll` is this node's own membership roll: a challenge from somebody the draw did
/// not choose is refused, because otherwise one node could make this one sign as many
/// statements as it liked.
///
/// # Errors
/// Fails if the stream fails, the question is malformed, or the asker was not drawn.
pub async fn answer<S>(
    stream: &mut S,
    prover: &Identity,
    head: Head,
    roll: &BTreeSet<[u8; 32]>,
    question: SignedChallenge,
) -> Result<Answered, Error>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let epoch = question.challenge.epoch();
    if !draw::is_entitled(
        epoch,
        &prover.public_key(),
        &question.challenge.verifier,
        roll,
    ) {
        return Err(Error::NotEntitled { epoch: epoch.0 });
    }

    let reply = Answer::to(&question.challenge, prover, head.digest, head.length);
    let sealed = reply.seal(prover)?;
    frame::write_frame(stream, &sealed).await?;
    let receipt = Exchange::assemble(question, challenge::open_answer(&sealed)?)?;

    // The statement is a courtesy, not a requirement: a verifier that hangs up here
    // has still been answered, and the receipt above is what survives that.
    let attestation = match frame::read_frame(stream).await {
        Ok(frame) => {
            let signed = attestation::open(&frame)?;
            if signed.attestation.epoch != epoch.0
                || signed.attestation.prover != prover.public_key()
            {
                return Err(Error::NotOurs);
            }
            Some(signed)
        }
        Err(frame::Error::Io(_)) => None,
        Err(e) => return Err(e.into()),
    };

    Ok(Answered {
        receipt,
        attestation,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use n333_core::attestation::{Evidence, judge};
    use n333_core::presence::Attendance;
    use tokio_util::compat::TokioAsyncReadCompatExt as _;

    fn identity(seed: u8) -> Identity {
        Identity::from_seed(&[seed; 32])
    }

    /// A roll of ten, and a verifier the draw actually chose for the prover.
    fn cast(epoch: Epoch) -> (Identity, Identity, BTreeSet<[u8; 32]>) {
        let prover = identity(1);
        let pool: Vec<_> = (2..12_u8).map(identity).collect();
        let mut roll: BTreeSet<_> = pool.iter().map(Identity::public_key).collect();
        roll.insert(prover.public_key());

        let drawn = draw::verifiers_for(epoch, &prover.public_key(), &roll);
        let verifier = pool
            .into_iter()
            .find(|i| drawn.contains(&i.public_key()))
            .expect("somebody was drawn");
        (prover, verifier, roll)
    }

    /// Run both sides of one round over a pipe.
    async fn round(
        prover: Identity,
        verifier: Identity,
        roll: BTreeSet<[u8; 32]>,
        epoch: Epoch,
        head: Head,
    ) -> (Result<Witnessed, Error>, Result<Answered, Error>) {
        let (a, b) = tokio::io::duplex(64 * 1024);
        let (mut a, mut b) = (a.compat(), b.compat());
        let prover_key = prover.public_key();

        let answering = tokio::spawn(async move {
            let question = challenge::open_challenge(&frame::read_frame(&mut b).await?)?;
            answer(&mut b, &prover, head, &roll, question).await
        });
        let asked = ask(&mut a, &verifier, prover_key, epoch).await;
        (asked, answering.await.expect("task"))
    }

    #[tokio::test]
    async fn one_round_leaves_both_sides_with_what_they_need() {
        let epoch = Epoch(89_516);
        let (prover, verifier, roll) = cast(epoch);
        let (prover_key, verifier_key) = (prover.public_key(), verifier.public_key());
        let head = Head {
            digest: [7_u8; 32],
            length: 42,
        };

        let (witnessed, answered) = round(prover, verifier, roll.clone(), epoch, head).await;
        let witnessed = witnessed.expect("the verifier finishes");
        let answered = answered.expect("the prover finishes");

        // The prover signed its own chain head back, which is the only thing outside
        // it that is ever committed to its record.
        assert_eq!(witnessed.exchange.answer.answer.chain_head, [7_u8; 32]);
        assert_eq!(witnessed.exchange.answer.answer.chain_len, 42);

        // The prover holds both the verifier's statement and its own receipt.
        let held = answered.attestation.expect("the statement was handed back");
        assert!(held.is_positive());
        assert_eq!(held.attestation.prover, prover_key);
        assert_eq!(held.attestation.verifier, verifier_key);
        assert_eq!(answered.receipt.epoch(), epoch);

        // And a third party, holding only the published bytes, reads a presence.
        let published = attestation::open(&witnessed.attestation).expect("opens");
        assert_eq!(
            judge(
                epoch,
                &prover_key,
                &roll,
                &Evidence {
                    attestations: vec![&published],
                    receipt: None,
                }
            ),
            Attendance::Present
        );
    }

    #[tokio::test]
    async fn a_node_that_was_not_drawn_is_refused() {
        // Otherwise one node could make this one sign as many statements as it liked
        // simply by asking over and over.
        let epoch = Epoch(11);
        let (prover, _, roll) = cast(epoch);
        let drawn = draw::verifiers_for(epoch, &prover.public_key(), &roll);
        let uninvited = (2..12_u8)
            .map(identity)
            .find(|i| !drawn.contains(&i.public_key()))
            .expect("somebody was not drawn");

        let (_, answered) = round(prover, uninvited, roll, epoch, Head::default()).await;
        assert!(matches!(answered, Err(Error::NotEntitled { epoch: 11 })));
    }

    #[tokio::test]
    async fn the_same_verifier_is_refused_for_an_epoch_it_was_not_drawn_for() {
        // Entitlement is about this epoch, not about that node: the very verifier
        // that may ask now may not ask about a different epoch.
        let epoch = Epoch(11);
        let (prover, verifier, roll) = cast(epoch);
        let elsewhere = (0..500_u64)
            .map(Epoch)
            .find(|e| !draw::is_entitled(*e, &prover.public_key(), &verifier.public_key(), &roll))
            .expect("some epoch does not draw them");

        let (_, answered) = round(prover, verifier, roll, elsewhere, Head::default()).await;
        assert!(matches!(answered, Err(Error::NotEntitled { .. })));
    }

    #[tokio::test]
    async fn a_prover_whose_verifier_hangs_up_still_holds_its_receipt() {
        // The case the receipt exists for. No statement arrives, and the prover can
        // still show two signatures from two nodes.
        let epoch = Epoch(11);
        let (prover, verifier, roll) = cast(epoch);
        let prover_key = prover.public_key();

        let (a, b) = tokio::io::duplex(64 * 1024);
        let (mut a, mut b) = (a.compat(), b.compat());
        let answering = tokio::spawn(async move {
            let question = challenge::open_challenge(&frame::read_frame(&mut b).await?)?;
            answer(&mut b, &prover, Head::default(), &roll, question).await
        });

        let question = Challenge::new(&verifier, prover_key, epoch);
        frame::write_frame(&mut a, &question.seal(&verifier).expect("seals"))
            .await
            .expect("writes");
        let _ = frame::read_frame(&mut a).await.expect("reads the answer");
        drop(a);

        let answered = answering.await.expect("task").expect("the prover finishes");
        assert!(answered.attestation.is_none(), "nothing was handed back");
        assert_eq!(answered.receipt.epoch(), epoch);
        assert_eq!(answered.receipt.answer.answer.prover, prover_key);
    }
}
