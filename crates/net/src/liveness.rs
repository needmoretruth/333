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
use n333_core::chain::Head;
use n333_core::challenge::{self, Answer, Challenge, Exchange, SignedChallenge};
use n333_core::epoch::MAX_CLOCK_SKEW_EPOCHS;
use n333_core::{Epoch, Identity, draw};

use crate::frame::{self, AsReceived};

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
    /// The question is about an epoch that is not now.
    #[error("asked about epoch {asked} while it is epoch {now}")]
    NotNow {
        /// The epoch the question named.
        asked: u64,
        /// The epoch this node believes it is.
        now: u64,
    },
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
    put(stream, verifier, prover, epoch)
        .await?
        .hear(stream, verifier)
        .await
}

/// A question that has gone out, and can still be answered for either way.
///
/// It exists because the two outcomes of asking are both statements somebody has to
/// sign, and one of them is signed when nothing at all comes back. A verifier that
/// simply gave up would leave no trace, and absence would be a thing nobody ever said —
/// which is exactly what happens if the only path through this module is the happy one.
#[derive(Debug, Clone)]
pub struct Question {
    /// The question, opened, so the answer can be checked against it.
    challenge: SignedChallenge,
    /// The nonce that has to appear in both the answer and the statement.
    nonce: [u8; 32],
    /// The question exactly as it went out.
    pub frame: Vec<u8>,
}

/// Put the question, and do not wait for anything.
///
/// # Errors
/// Fails if the question cannot be sealed or the stream fails.
pub async fn put<S>(
    stream: &mut S,
    verifier: &Identity,
    prover: [u8; 32],
    epoch: Epoch,
) -> Result<Question, Error>
where
    S: AsyncWrite + Unpin,
{
    let challenge = Challenge::new(verifier, prover, epoch);
    let frame = challenge.seal(verifier)?;
    frame::write_frame(stream, &frame).await?;
    Ok(Question {
        challenge: challenge::open_challenge(&frame)?,
        nonce: challenge.nonce,
        frame,
    })
}

impl Question {
    /// Read the answer, publish the statement, and keep both.
    ///
    /// # Errors
    /// Fails if the stream fails, or the answer is not an answer to this question.
    pub async fn hear<S>(&self, stream: &mut S, verifier: &Identity) -> Result<Witnessed, Error>
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        // The answer's bytes are kept exactly as they arrived and go into the statement
        // unchanged. Re-encoding them would make every later reader's signature check
        // depend on this encoder still producing the same bytes.
        let answer_frame = frame::read_frame(stream).await?;
        let exchange = Exchange::assemble(
            self.challenge.clone(),
            challenge::open_answer(&answer_frame)?,
        )?;

        let sealed = Attestation::answered(
            verifier,
            exchange.answer.answer.prover,
            exchange.answer.answer.epoch(),
            self.nonce,
            answer_frame,
        )
        .seal(verifier)?;
        // Handed straight back, because the node it is about is the one node that
        // cannot obtain it any other way and the one that most needs it.
        frame::write_frame(stream, &sealed).await?;

        Ok(Witnessed {
            attestation: sealed,
            exchange,
        })
    }

    /// Say that nothing came back.
    ///
    /// WHAT THIS IS WORTH, WHICH IS LESS THAN THE OTHER ONE. It carries no signature
    /// from the node it is about, because there is none to carry: silence cannot be
    /// signed. So it is only ever half of an accusation — a reader needs one of these
    /// from every verifier that was drawn, and a single answer to any of them beats all
    /// of them. See [`n333_core::attestation::judge`].
    ///
    /// # Errors
    /// Fails if the statement cannot be sealed.
    pub fn unanswered(&self, verifier: &Identity) -> Result<Vec<u8>, Error> {
        Ok(Attestation::silent(
            verifier,
            self.challenge.challenge.prover,
            self.challenge.challenge.epoch(),
            self.nonce,
        )
        .seal(verifier)?)
    }
}

/// Read whatever the peer says after the heartbeat, if anything.
///
/// `None` means the peer said its piece and hung up, which is the ordinary shape of a
/// node that only wanted to exchange heartbeats.
///
/// # Errors
/// Fails if the stream fails mid-frame, or the frame is not a challenge.
pub async fn take_question<S>(stream: &mut S) -> Result<Option<AsReceived<SignedChallenge>>, Error>
where
    S: AsyncRead + Unpin,
{
    match frame::read_frame(stream).await {
        Ok(frame) => Ok(Some(AsReceived {
            message: challenge::open_challenge(&frame)?,
            frame,
        })),
        Err(frame::Error::Io(_)) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// What the prover ends a round holding.
#[derive(Debug, Clone)]
pub struct Answered {
    /// The challenge and this node's answer: two signatures, from two nodes.
    pub receipt: Exchange,
    /// The verifier's challenge, exactly as it arrived.
    pub challenge_frame: Vec<u8>,
    /// This node's answer, exactly as it was sent.
    pub answer_frame: Vec<u8>,
    /// The verifier's statement, if it handed one back.
    pub attestation: Option<AsReceived<SignedAttestation>>,
}

/// Read the question, answer it, and take the statement handed back.
///
/// `roll` is this node's own membership roll: a challenge from somebody the draw did
/// not choose is refused, because otherwise one node could make this one sign as many
/// statements as it liked.
///
/// `now` is this node's own epoch, and a question about any other one is refused. The
/// draw is computable for every epoch that will ever exist, so without this a node can
/// be asked about a year from now, or a year ago, and made to sign an answer that says
/// it was awake then. The only epoch a node can honestly answer for is the one it is
/// in — with one epoch of slack, because boundaries pass mid-round on their own.
///
/// # Errors
/// Fails if the stream fails, the question is malformed, the asker was not drawn, or
/// the question is about some other epoch.
pub async fn answer<S>(
    stream: &mut S,
    prover: &Identity,
    now: Epoch,
    head: Head,
    roll: &BTreeSet<[u8; 32]>,
    question: AsReceived<SignedChallenge>,
) -> Result<Answered, Error>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let epoch = question.message.challenge.epoch();
    if epoch.0.abs_diff(now.0) > MAX_CLOCK_SKEW_EPOCHS {
        return Err(Error::NotNow {
            asked: epoch.0,
            now: now.0,
        });
    }
    if !draw::is_entitled(
        epoch,
        &prover.public_key(),
        &question.message.challenge.verifier,
        roll,
    ) {
        return Err(Error::NotEntitled { epoch: epoch.0 });
    }

    let reply = Answer::to(
        &question.message.challenge,
        prover,
        head.digest,
        head.length,
    );
    let answer_frame = reply.seal(prover)?;
    frame::write_frame(stream, &answer_frame).await?;
    let receipt = Exchange::assemble(question.message, challenge::open_answer(&answer_frame)?)?;

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
            Some(AsReceived {
                message: signed,
                frame,
            })
        }
        Err(frame::Error::Io(_)) => None,
        Err(e) => return Err(e.into()),
    };

    Ok(Answered {
        receipt,
        challenge_frame: question.frame,
        answer_frame,
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
            let question = take_question(&mut b).await?.expect("a question arrives");
            answer(&mut b, &prover, epoch, head, &roll, question).await
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
        assert!(held.message.is_positive());
        assert_eq!(held.message.attestation.prover, prover_key);
        assert_eq!(held.message.attestation.verifier, verifier_key);
        assert_eq!(answered.receipt.epoch(), epoch);
        // Kept as the bytes that arrived, not as something re-encoded here.
        assert_eq!(held.frame, witnessed.attestation);

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
    async fn a_verifier_that_reached_a_node_and_heard_nothing_says_so_and_it_binds() {
        // The path that makes the two-thirds rule able to bind on anybody at all. If
        // no negative is ever published, Attendance::Absent is unreachable and nobody
        // can lose standing by not being there.
        //
        // A roll of exactly four draws all three of the others, so every verifier here
        // is drawn and the judgement is not left to depend on the draw.
        let epoch = Epoch(89_516);
        let prover = identity(1);
        let key = prover.public_key();
        let others: Vec<Identity> = (2..=4).map(identity).collect();
        let roll: BTreeSet<[u8; 32]> = std::iter::once(key)
            .chain(others.iter().map(Identity::public_key))
            .collect();
        let drawn = draw::verifiers_for(epoch, &key, &roll);
        assert_eq!(drawn.len(), 3, "a roll of four draws the other three");

        let mut nothing = futures::io::Cursor::new(Vec::new());
        let mut sealed = Vec::new();
        for who in &others {
            let question = put(&mut nothing, who, key, epoch).await.expect("puts");
            sealed.push(question.unanswered(who).expect("seals"));
        }
        let opened: Vec<_> = sealed
            .iter()
            .map(|frame| attestation::open(frame).expect("opens"))
            .collect();
        for signed in &opened {
            assert!(!signed.is_positive(), "silence is not an answer");
            assert_eq!(signed.attestation.prover, key);
            assert_eq!(signed.attestation.epoch, epoch.0);
        }

        let evidence = Evidence {
            attestations: opened.iter().collect(),
            receipt: None,
        };
        assert_eq!(judge(epoch, &key, &roll, &evidence), Attendance::Absent);

        // And one of the three keeping quiet is enough to stop it, because a verifier
        // that published nothing said nothing.
        let two_of_three = Evidence {
            attestations: opened.iter().take(2).collect(),
            receipt: None,
        };
        assert_eq!(
            judge(epoch, &key, &roll, &two_of_three),
            Attendance::Excluded
        );
    }

    #[tokio::test]
    async fn a_question_about_some_other_epoch_is_refused_however_it_was_drawn() {
        // Without this, the draw being computable for every epoch that will ever exist
        // means a node can be asked about next year and made to sign that it was awake
        // then. One epoch of slack, because boundaries pass mid-round on their own.
        let epoch = Epoch(89_516);
        let (prover, verifier, roll) = cast(epoch);

        // Two epochs away is refused before entitlement is even consulted.
        let (_, answered) = round_asking(prover, verifier, roll, epoch, Epoch(epoch.0 + 2)).await;
        assert!(matches!(
            answered,
            Err(Error::NotNow {
                asked,
                now
            }) if asked == epoch.0 + 2 && now == epoch.0
        ));
    }

    /// One round where the asker and the answerer disagree about what epoch it is.
    async fn round_asking(
        prover: Identity,
        verifier: Identity,
        roll: BTreeSet<[u8; 32]>,
        prover_thinks: Epoch,
        asked_about: Epoch,
    ) -> (Result<Witnessed, Error>, Result<Answered, Error>) {
        let prover_key = prover.public_key();
        let (a, b) = tokio::io::duplex(64 * 1024);
        let (mut a, mut b) = (a.compat(), b.compat());
        let answering = tokio::spawn(async move {
            let question = take_question(&mut b).await?.expect("a question arrives");
            answer(
                &mut b,
                &prover,
                prover_thinks,
                Head::default(),
                &roll,
                question,
            )
            .await
        });
        let asked = ask(&mut a, &verifier, prover_key, asked_about).await;
        (asked, answering.await.expect("task"))
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
            let question = take_question(&mut b).await?.expect("a question arrives");
            answer(&mut b, &prover, epoch, Head::default(), &roll, question).await
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
