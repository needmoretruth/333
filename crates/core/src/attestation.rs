//! What a verifier publishes about a node, and what a reader makes of it.
//!
//! FROZEN. The field order is the wire format and the domain is part of the
//! protocol's identity.
//!
//! A POSITIVE CARRIES ITS OWN PROOF. When a verifier says a node answered, it
//! publishes the node's answer verbatim inside its own statement, so anyone can check
//! the answer's signature themselves. The verifier is not believed; it is merely the
//! one that happened to be holding the evidence.
//!
//! A NEGATIVE IS ONLY A CLAIM. "Nothing arrived" has nothing inside it — an absence
//! cannot be signed by the party that is absent. That asymmetry is the whole shape of
//! the judgement below: one checkable positive outweighs any number of negatives, and
//! a negative counts for anything only when every verifier drawn that epoch published
//! one.
//!
//! SILENCE IN THE RECORD IS NOT AN ACCUSATION. A verifier that crashed, or that
//! simply never published, leaves the epoch outside the count rather than counting
//! against the node. Reading missing evidence as evidence is how a network punishes
//! its own outages.

use serde::{Deserialize, Serialize};

use crate::challenge::{self, Exchange, SignedAnswer};
use crate::draw;
use crate::epoch::Epoch;
use crate::heartbeat::PROTOCOL_VERSION;
use crate::identity::{Identity, NodeId, parse_public_key};
use crate::presence::Attendance;
use crate::wire::{self, DOMAIN_LEN};

/// The domain an attestation is signed under. FROZEN.
pub const DOMAIN_WITNESS: &[u8; DOMAIN_LEN] = b"333.v1.witnessed";

/// How many epochs after an epoch ends before a node judges it.
///
/// ⚖ Chosen here rather than specified. Attestations have to reach a node before it
/// can read them, and a node that judges the instant an epoch closes judges on
/// whatever happened to have arrived. Three epochs is about seventeen hours — under
/// one percent of the 333-epoch window, and far longer than gossip needs among nodes
/// that are answering at all.
///
/// **A judgement is made once and never revised.** The verifiers drawn for an epoch
/// depend on the roll, and the roll grows; re-judging an old epoch against today's
/// roll would quietly drift both presences and absences towards nothing at all.
pub const JUDGEMENT_DELAY_EPOCHS: u64 = 3;

/// What a verifier saw.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Testimony {
    /// The node answered. Carries its signed answer exactly as it arrived.
    Answered(Vec<u8>),
    /// Nothing arrived inside the response window.
    Silent,
}

/// One verifier's published statement about one node in one epoch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attestation {
    /// Wire protocol version.
    pub protocol: u16,
    /// The key of the node that asked.
    pub verifier: [u8; 32],
    /// The key of the node that was asked.
    pub prover: [u8; 32],
    /// The epoch this is about.
    pub epoch: u64,
    /// The nonce the verifier issued, so its own challenge can be matched to this.
    pub nonce: [u8; 32],
    /// What it saw.
    pub testimony: Testimony,
}

impl Attestation {
    /// Compose a statement that the node answered, carrying the answer itself.
    #[must_use]
    pub fn answered(
        verifier: &Identity,
        prover: [u8; 32],
        epoch: Epoch,
        nonce: [u8; 32],
        answer_frame: Vec<u8>,
    ) -> Self {
        Self {
            protocol: PROTOCOL_VERSION,
            verifier: verifier.public_key(),
            prover,
            epoch: epoch.0,
            nonce,
            testimony: Testimony::Answered(answer_frame),
        }
    }

    /// Compose a statement that nothing arrived.
    #[must_use]
    pub fn silent(
        verifier: &Identity,
        prover: [u8; 32],
        epoch: Epoch,
        nonce: [u8; 32],
    ) -> Self {
        Self {
            protocol: PROTOCOL_VERSION,
            verifier: verifier.public_key(),
            prover,
            epoch: epoch.0,
            nonce,
            testimony: Testimony::Silent,
        }
    }

    /// The epoch this is about.
    #[must_use]
    pub const fn epoch(&self) -> Epoch {
        Epoch(self.epoch)
    }

    /// Encode and sign.
    ///
    /// # Errors
    /// Fails if the encoding is impossible or the frame would exceed the wire limit.
    pub fn seal(&self, verifier: &Identity) -> Result<Vec<u8>, wire::Error> {
        let body = postcard::to_stdvec(self).map_err(|e| wire::Error::Decode(e.to_string()))?;
        wire::seal(DOMAIN_WITNESS, &body, verifier)
    }
}

/// An attestation whose signatures all check out.
///
/// For a positive, that means two signatures: the verifier's over the statement, and
/// the node's own over the answer inside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedAttestation {
    /// What was published.
    pub attestation: Attestation,
    /// The name derived from the verifier's key.
    pub verifier: NodeId,
    /// The answer inside a positive, already checked against this statement.
    pub answer: Option<SignedAnswer>,
}

impl SignedAttestation {
    /// Does this say the node answered?
    #[must_use]
    pub const fn is_positive(&self) -> bool {
        self.answer.is_some()
    }
}

/// Why a published attestation does not hold together.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Invalid {
    /// The frame or a signature is wrong.
    #[error("{0}")]
    Frame(#[from] wire::Error),
    /// The answer inside a positive does not answer this statement.
    ///
    /// A verifier attaching somebody else's answer, or an answer from another epoch,
    /// is caught here rather than being believed because the outer signature checked.
    #[error("the answer inside does not match this statement: {0}")]
    Mismatched(#[from] challenge::NotAnAnswer),
}

impl From<crate::identity::PublicKeyError> for Invalid {
    fn from(e: crate::identity::PublicKeyError) -> Self {
        Self::Frame(e.into())
    }
}

/// Read a published attestation, checking everything inside it.
///
/// # Errors
/// Fails if the frame is malformed, either signature does not match, or the answer
/// carried by a positive is not an answer to this verifier's own challenge.
pub fn open(frame: &[u8]) -> Result<SignedAttestation, Invalid> {
    let (signature, body) = wire::split(frame)?;
    let attestation: Attestation = wire::decode(body)?;
    if attestation.protocol != PROTOCOL_VERSION {
        return Err(wire::Error::Version {
            got: attestation.protocol,
            expected: PROTOCOL_VERSION,
        }
        .into());
    }
    parse_public_key(&attestation.verifier)?;
    wire::check_signature(DOMAIN_WITNESS, body, signature, &attestation.verifier)?;

    let answer = match &attestation.testimony {
        Testimony::Silent => None,
        Testimony::Answered(frame) => {
            let answer = challenge::open_answer(frame)?;
            check_answer_fits(&attestation, &answer)?;
            Some(answer)
        }
    };
    Ok(SignedAttestation {
        verifier: NodeId::from_public_key(&attestation.verifier),
        attestation,
        answer,
    })
}

/// The answer inside a positive has to be an answer to this exact challenge.
fn check_answer_fits(
    attestation: &Attestation,
    answer: &SignedAnswer,
) -> Result<(), challenge::NotAnAnswer> {
    if answer.answer.nonce != attestation.nonce {
        return Err(challenge::NotAnAnswer::WrongNonce);
    }
    if answer.answer.verifier != attestation.verifier {
        return Err(challenge::NotAnAnswer::WrongVerifier);
    }
    if answer.answer.prover != attestation.prover {
        return Err(challenge::NotAnAnswer::WrongProver);
    }
    if answer.answer.epoch != attestation.epoch {
        return Err(challenge::NotAnAnswer::WrongEpoch {
            asked: attestation.epoch,
            answered: answer.answer.epoch,
        });
    }
    Ok(())
}

/// Everything a reader has about one node in one epoch.
#[derive(Debug, Clone, Default)]
pub struct Evidence<'a> {
    /// Published statements, in any order, from anyone. Ones from nodes that were not
    /// drawn are ignored rather than refused: a node that was drawn on somebody
    /// else's roll is not misbehaving.
    pub attestations: Vec<&'a SignedAttestation>,
    /// The node's own kept challenge-and-answer pair, if it has one and produced it.
    pub receipt: Option<&'a Exchange>,
}

/// Judge one epoch for one node, once.
///
/// The rule, in the order it is applied:
///
/// 1. **Nobody was drawn** — a roll too small to hold a verifier for this node. The
///    epoch leaves the count. No question was put, so there is nothing to answer for.
/// 2. **Any drawn verifier published a positive** — [`Attendance::Present`]. One is
///    enough, and it does not matter what the other two said: a positive carries a
///    signature that could not be made without the node, and a negative carries
///    nothing.
/// 3. **Every drawn verifier published a negative** — [`Attendance::Absent`], unless
///    the node produced a receipt. Every one of them: a verifier that published
///    nothing is a verifier that said nothing, and reading its silence as agreement
///    would let a captured pair convict on one negative.
/// 4. **The node produced a receipt** — the epoch leaves the count. A receipt is a
///    challenge and an answer signed by two different nodes, so the node demonstrably
///    answered; it does not show *when*, and only the verifier saw that. So it does
///    not buy a presence, it withdraws an accusation. The cost, stated plainly: a
///    node that answers every verifier too late is never marked absent. That is
///    accepted, because "answered late" and "the verifiers agreed to lie" cannot be
///    told apart afterwards, and this protocol settles that kind of tie the same way
///    everywhere — in favour of the accused.
/// 5. **Anything else** — the epoch leaves the count.
#[must_use]
pub fn judge(
    epoch: Epoch,
    prover: &[u8; 32],
    roll: &std::collections::BTreeSet<[u8; 32]>,
    evidence: &Evidence<'_>,
) -> Attendance {
    read(epoch, prover, roll, evidence).attendance
}

/// A verdict and the reason it was reached.
///
/// The reason is for a screen and for nothing else. It is not serialised, it does not
/// go into a record, and it never travels: two nodes holding different evidence reach
/// the same verdict for different reasons all the time, and that is two observations
/// rather than a disagreement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Verdict {
    /// What the epoch counts as.
    pub attendance: Attendance,
    /// Which of the five ways it got there.
    pub because: Because,
}

/// The five ways [`judge`] reaches a verdict, in the order it tries them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Because {
    /// A verifier that was drawn published the node's own answer.
    Answered,
    /// Every verifier that was drawn swore nothing came back.
    Denounced,
    /// The roll was too small to draw anybody, so nothing was asked.
    NoneDrawn,
    /// The node kept a question and the answer it gave, which withdraws an accusation
    /// without earning a presence.
    ReceiptWithdrew,
    /// Some of those drawn said nothing at all, and silence is not agreement.
    NotAllSpoke,
}

/// Judge one epoch, and say which of the five ways it went.
///
/// [`judge`] is this with the reason dropped.
#[must_use]
pub fn read(
    epoch: Epoch,
    prover: &[u8; 32],
    roll: &std::collections::BTreeSet<[u8; 32]>,
    evidence: &Evidence<'_>,
) -> Verdict {
    let said = |attendance, because| Verdict {
        attendance,
        because,
    };
    let drawn = draw::verifiers_for(epoch, prover, roll);
    if drawn.is_empty() {
        return said(Attendance::Excluded, Because::NoneDrawn);
    }

    let relevant = |a: &SignedAttestation| {
        a.attestation.epoch == epoch.0
            && &a.attestation.prover == prover
            && drawn.contains(&a.attestation.verifier)
    };

    if evidence
        .attestations
        .iter()
        .filter(|a| relevant(a))
        .any(|a| a.is_positive())
    {
        return said(Attendance::Present, Because::Answered);
    }

    if let Some(receipt) = evidence.receipt
        && receipt.epoch() == epoch
        && &receipt.answer.answer.prover == prover
        && drawn.contains(&receipt.challenge.challenge.verifier)
    {
        return said(Attendance::Excluded, Because::ReceiptWithdrew);
    }

    // Distinct verifiers, so one verifier publishing the same negative twice does not
    // stand in for a second one.
    let mut denouncing: Vec<[u8; 32]> = evidence
        .attestations
        .iter()
        .filter(|a| relevant(a))
        .map(|a| a.attestation.verifier)
        .collect();
    denouncing.sort_unstable();
    denouncing.dedup();

    if denouncing.len() == drawn.len() {
        said(Attendance::Absent, Because::Denounced)
    } else {
        said(Attendance::Excluded, Because::NotAllSpoke)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenge::{Answer, Challenge, open_answer, open_challenge};
    use std::collections::BTreeSet;

    fn identity(seed: u8) -> Identity {
        Identity::from_seed(&[seed; 32])
    }

    /// A roll big enough that three verifiers are drawn.
    fn roll(members: &[&Identity]) -> BTreeSet<[u8; 32]> {
        members.iter().map(|m| m.public_key()).collect()
    }

    /// Everyone this test needs: a prover and eight possible verifiers.
    fn cast() -> (Identity, Vec<Identity>, BTreeSet<[u8; 32]>) {
        let prover = identity(1);
        let others: Vec<_> = (2..10_u8).map(identity).collect();
        let mut all: Vec<&Identity> = others.iter().collect();
        all.push(&prover);
        let roll = roll(&all);
        (prover, others, roll)
    }

    /// The identity behind a drawn key.
    fn drawn_identities<'a>(
        epoch: Epoch,
        prover: &Identity,
        roll: &BTreeSet<[u8; 32]>,
        pool: &'a [Identity],
    ) -> Vec<&'a Identity> {
        draw::verifiers_for(epoch, &prover.public_key(), roll)
            .into_iter()
            .map(|k| {
                pool.iter()
                    .find(|i| i.public_key() == k)
                    .expect("a drawn verifier is in the pool")
            })
            .collect()
    }

    /// One verifier's positive statement, sealed and reopened.
    fn positive(verifier: &Identity, prover: &Identity, epoch: Epoch) -> SignedAttestation {
        let challenge = Challenge::new(verifier, prover.public_key(), epoch);
        let asked = open_challenge(&challenge.seal(verifier).expect("seals")).expect("opens");
        let answer_frame = Answer::to(&asked.challenge, prover, [0_u8; 32], 0)
            .seal(prover)
            .expect("seals");
        let frame = Attestation::answered(
            verifier,
            prover.public_key(),
            epoch,
            challenge.nonce,
            answer_frame,
        )
        .seal(verifier)
        .expect("seals");
        open(&frame).expect("opens")
    }

    /// One verifier's negative statement, sealed and reopened.
    fn negative(verifier: &Identity, prover: &Identity, epoch: Epoch) -> SignedAttestation {
        let frame = Attestation::silent(verifier, prover.public_key(), epoch, [9_u8; 32])
            .seal(verifier)
            .expect("seals");
        open(&frame).expect("opens")
    }

    fn evidence<'a>(items: &[&'a SignedAttestation]) -> Evidence<'a> {
        Evidence {
            attestations: items.to_vec(),
            receipt: None,
        }
    }

    #[test]
    fn the_frozen_values_are_the_agreed_ones() {
        assert_eq!(DOMAIN_WITNESS, b"333.v1.witnessed");
        assert_eq!(DOMAIN_WITNESS.len(), DOMAIN_LEN);
        assert_eq!(JUDGEMENT_DELAY_EPOCHS, 3);
    }

    #[test]
    fn a_positive_carries_the_answer_and_is_checked_without_trusting_the_verifier() {
        let (prover, pool, _) = cast();
        let attested = positive(&pool[0], &prover, Epoch(7));
        assert!(attested.is_positive());
        let answer = attested.answer.as_ref().expect("carries the answer");
        assert_eq!(answer.prover, prover.node_id());
        assert_eq!(answer.answer.epoch, 7);
    }

    #[test]
    fn a_verifier_cannot_attach_somebody_elses_answer() {
        // The outer signature is the verifier's and checks out; the answer inside is
        // for a different challenge. Believing the outer one would be believing the
        // verifier, which is the thing this refuses to do.
        let (prover, pool, _) = cast();
        let (verifier, other) = (&pool[0], &pool[1]);

        let elsewhere = Challenge::new(other, prover.public_key(), Epoch(7));
        let asked = open_challenge(&elsewhere.seal(other).expect("seals")).expect("opens");
        let answer_frame = Answer::to(&asked.challenge, &prover, [0_u8; 32], 0)
            .seal(&prover)
            .expect("seals");

        let frame = Attestation::answered(
            verifier,
            prover.public_key(),
            Epoch(7),
            [1_u8; 32],
            answer_frame,
        )
        .seal(verifier)
        .expect("seals");
        assert!(matches!(open(&frame), Err(Invalid::Mismatched(_))));
    }

    #[test]
    fn one_positive_outweighs_every_negative() {
        let (prover, pool, roll) = cast();
        let epoch = Epoch(11);
        let drawn = drawn_identities(epoch, &prover, &roll, &pool);
        let good = positive(drawn[0], &prover, epoch);
        let bad: Vec<_> = drawn[1..]
            .iter()
            .map(|v| negative(v, &prover, epoch))
            .collect();
        let mut items = vec![&good];
        items.extend(bad.iter());
        assert_eq!(
            judge(epoch, &prover.public_key(), &roll, &evidence(&items)),
            Attendance::Present
        );
    }

    #[test]
    fn every_drawn_verifier_must_publish_a_negative_before_anyone_is_absent() {
        let (prover, pool, roll) = cast();
        let epoch = Epoch(11);
        let drawn = drawn_identities(epoch, &prover, &roll, &pool);
        let all: Vec<_> = drawn.iter().map(|v| negative(v, &prover, epoch)).collect();
        let refs: Vec<_> = all.iter().collect();
        assert_eq!(
            judge(epoch, &prover.public_key(), &roll, &evidence(&refs)),
            Attendance::Absent
        );

        // Two of the three published; the third said nothing at all. Its silence is
        // not agreement, so the epoch leaves the count instead of convicting.
        let two: Vec<_> = refs[..2].to_vec();
        assert_eq!(
            judge(epoch, &prover.public_key(), &roll, &evidence(&two)),
            Attendance::Excluded
        );
    }

    #[test]
    fn one_verifier_saying_it_twice_is_still_one_verifier() {
        let (prover, pool, roll) = cast();
        let epoch = Epoch(11);
        let drawn = drawn_identities(epoch, &prover, &roll, &pool);
        let once = negative(drawn[0], &prover, epoch);
        let again = negative(drawn[0], &prover, epoch);
        let second = negative(drawn[1], &prover, epoch);
        assert_eq!(
            judge(
                epoch,
                &prover.public_key(),
                &roll,
                &evidence(&[&once, &again, &second])
            ),
            Attendance::Excluded
        );
    }

    #[test]
    fn a_statement_from_a_verifier_that_was_not_drawn_is_ignored() {
        let (prover, pool, roll) = cast();
        let epoch = Epoch(11);
        let drawn_keys = draw::verifiers_for(epoch, &prover.public_key(), &roll);
        let uninvited = pool
            .iter()
            .find(|i| !drawn_keys.contains(&i.public_key()))
            .expect("somebody was not drawn");
        let claim = negative(uninvited, &prover, epoch);
        assert_eq!(
            judge(epoch, &prover.public_key(), &roll, &evidence(&[&claim])),
            Attendance::Excluded
        );

        // ...and neither does a positive from them create a presence.
        let flattery = positive(uninvited, &prover, epoch);
        assert_eq!(
            judge(epoch, &prover.public_key(), &roll, &evidence(&[&flattery])),
            Attendance::Excluded
        );
    }

    #[test]
    fn a_receipt_withdraws_an_accusation_without_claiming_to_have_been_prompt() {
        // Three captured verifiers all publish silence. The node holds a challenge
        // one of them signed and its own answer to it, so the accusation cannot
        // stand — but nothing here shows the answer was in time, so it does not
        // become a presence either.
        let (prover, pool, roll) = cast();
        let epoch = Epoch(11);
        let drawn = drawn_identities(epoch, &prover, &roll, &pool);
        let all: Vec<_> = drawn.iter().map(|v| negative(v, &prover, epoch)).collect();
        let refs: Vec<_> = all.iter().collect();

        let challenge = Challenge::new(drawn[0], prover.public_key(), epoch);
        let asked = open_challenge(&challenge.seal(drawn[0]).expect("seals")).expect("opens");
        let answered = open_answer(
            &Answer::to(&asked.challenge, &prover, [0_u8; 32], 0)
                .seal(&prover)
                .expect("seals"),
        )
        .expect("opens");
        let receipt = Exchange::assemble(asked, answered).expect("assembles");

        let with_receipt = Evidence {
            attestations: refs.clone(),
            receipt: Some(&receipt),
        };
        assert_eq!(
            judge(epoch, &prover.public_key(), &roll, &with_receipt),
            Attendance::Excluded
        );
        assert_eq!(
            judge(epoch, &prover.public_key(), &roll, &evidence(&refs)),
            Attendance::Absent,
            "without the receipt the same evidence convicts"
        );
    }

    #[test]
    fn a_node_nobody_could_be_drawn_for_is_never_absent() {
        let alone = identity(1);
        let roll: BTreeSet<_> = [alone.public_key()].into_iter().collect();
        assert_eq!(
            judge(
                Epoch(11),
                &alone.public_key(),
                &roll,
                &Evidence::default()
            ),
            Attendance::Excluded
        );
    }

    #[test]
    fn an_epoch_nobody_published_anything_about_leaves_the_count() {
        let (prover, _, roll) = cast();
        assert_eq!(
            judge(
                Epoch(11),
                &prover.public_key(),
                &roll,
                &Evidence::default()
            ),
            Attendance::Excluded
        );
    }

    #[test]
    fn statements_about_another_epoch_or_another_node_do_not_count_here() {
        let (prover, pool, roll) = cast();
        let epoch = Epoch(11);
        let drawn = drawn_identities(epoch, &prover, &roll, &pool);
        let elsewhen: Vec<_> = drawn
            .iter()
            .map(|v| negative(v, &prover, Epoch(12)))
            .collect();
        let refs: Vec<_> = elsewhen.iter().collect();
        assert_eq!(
            judge(epoch, &prover.public_key(), &roll, &evidence(&refs)),
            Attendance::Excluded
        );
    }

    #[test]
    fn a_padded_or_tampered_attestation_does_not_open() {
        let (prover, pool, _) = cast();
        let mut frame = Attestation::silent(&pool[0], prover.public_key(), Epoch(1), [0_u8; 32])
            .seal(&pool[0])
            .expect("seals");
        frame.push(0);
        assert!(matches!(
            open(&frame),
            Err(Invalid::Frame(wire::Error::TrailingBytes { extra: 1 }))
        ));
    }
}
