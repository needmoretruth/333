//! The question a verifier puts, and the answer a node gives.
//!
//! FROZEN. The field order is the wire format and the domains are part of the
//! protocol's identity.
//!
//! Each epoch three others are drawn to ask ([`crate::draw`]). Each sends a nonce;
//! the node signs it back. What that signature proves is narrow and worth stating: it
//! proves the node held its key and was awake after the nonce was chosen. It does not
//! prove the node is running this code, and nothing here claims to.
//!
//! WHAT THE ANSWER COMMITS TO, AND WHY EACH PART IS THERE.
//!
//! * The **nonce** makes it live. Without it a recording answers for ever.
//! * The **verifier** stops the answer being relayed to the other two as though the
//!   node had answered them as well, from one exchange.
//! * The **epoch** stops an answer being kept and produced against a later epoch.
//!   Both of these are inside the signature rather than checked alongside it: a rule
//!   applied by the reader is a rule some future reader forgets.
//! * The **chain head and length** are the only thing outside a node that is ever
//!   committed to the node's own record. Without them a node can rewrite its whole
//!   history and nothing contradicts it; with them, every answer it ever gave is a
//!   signed statement, held by somebody else, about what its record looked like at
//!   that moment.
//!
//! The record chain itself is not built yet. A node that has none sends a head of
//! zeros and a length of zero, which is a truthful statement about having no record —
//! and the fields have to be in the signed bytes from the beginning, because bytes
//! are the one thing that cannot be added by agreement later.

use serde::{Deserialize, Serialize};

use crate::epoch::Epoch;
use crate::heartbeat::PROTOCOL_VERSION;
use crate::identity::{Identity, NodeId, parse_public_key};
use crate::wire::{self, DOMAIN_LEN};

/// The domain a challenge is signed under. FROZEN.
pub const DOMAIN_CHALLENGE: &[u8; DOMAIN_LEN] = b"333.v1.challenge";

/// The domain an answer is signed under. FROZEN.
pub const DOMAIN_ANSWER: &[u8; DOMAIN_LEN] = b"333.v1.answer.to";

/// How long a node has to answer, in seconds.
///
/// FROZEN in the sense that two nodes must agree on it to read each other's records.
/// Three minutes out of every 333 is the whole cost of being counted, and it is set
/// by the slowest way a node can be reached rather than the fastest: an onion circuit
/// on a tired machine is seconds, and this leaves room for it to be many seconds.
pub const RESPONSE_WINDOW_SECONDS: u64 = 180;

/// A verifier asking a node to prove it is awake.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Challenge {
    /// Wire protocol version.
    pub protocol: u16,
    /// The key of the node asking.
    pub verifier: [u8; 32],
    /// The key of the node being asked.
    pub prover: [u8; 32],
    /// The epoch this challenge belongs to.
    pub epoch: u64,
    /// Fresh random bytes the answer must sign back.
    pub nonce: [u8; 32],
}

impl Challenge {
    /// Compose a challenge to `prover` for `epoch`.
    #[must_use]
    pub fn new(verifier: &Identity, prover: [u8; 32], epoch: Epoch) -> Self {
        let mut nonce = [0_u8; 32];
        rand_core::RngCore::fill_bytes(&mut rand_core::OsRng, &mut nonce);
        Self {
            protocol: PROTOCOL_VERSION,
            verifier: verifier.public_key(),
            prover,
            epoch: epoch.0,
            nonce,
        }
    }

    /// The epoch this challenge claims.
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
        wire::seal(DOMAIN_CHALLENGE, &body, verifier)
    }
}

/// A node's signed answer to one challenge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Answer {
    /// Wire protocol version.
    pub protocol: u16,
    /// The key of the node answering.
    pub prover: [u8; 32],
    /// The key of the node that asked. Inside the signature, so the answer cannot be
    /// shown to the other two verifiers as though they had been answered too.
    pub verifier: [u8; 32],
    /// The epoch answered for. Inside the signature, so an answer cannot be held back
    /// and produced against a later epoch.
    pub epoch: u64,
    /// The nonce from the challenge, signed back. This is what makes it live.
    pub nonce: [u8; 32],
    /// The head of the answering node's own record chain at this moment.
    ///
    /// Zero when it has no record yet, which is a true statement rather than a
    /// placeholder.
    pub chain_head: [u8; 32],
    /// How many entries that chain holds.
    pub chain_len: u64,
}

impl Answer {
    /// Compose an answer to `challenge`.
    ///
    /// The caller supplies its own chain head and length; nothing here can know them,
    /// and nothing here checks that they are true. What they buy is that the node
    /// cannot later present a different history without contradicting a signature
    /// somebody else is holding.
    #[must_use]
    pub fn to(
        challenge: &Challenge,
        prover: &Identity,
        chain_head: [u8; 32],
        chain_len: u64,
    ) -> Self {
        Self {
            protocol: PROTOCOL_VERSION,
            prover: prover.public_key(),
            verifier: challenge.verifier,
            epoch: challenge.epoch,
            nonce: challenge.nonce,
            chain_head,
            chain_len,
        }
    }

    /// The epoch this answer claims.
    #[must_use]
    pub const fn epoch(&self) -> Epoch {
        Epoch(self.epoch)
    }

    /// Encode and sign.
    ///
    /// # Errors
    /// Fails if the encoding is impossible or the frame would exceed the wire limit.
    pub fn seal(&self, prover: &Identity) -> Result<Vec<u8>, wire::Error> {
        let body = postcard::to_stdvec(self).map_err(|e| wire::Error::Decode(e.to_string()))?;
        wire::seal(DOMAIN_ANSWER, &body, prover)
    }
}

/// A challenge that arrived with a signature that checks out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedChallenge {
    /// What was asked.
    pub challenge: Challenge,
    /// The name derived from the verifier's key.
    pub verifier: NodeId,
}

/// An answer that arrived with a signature that checks out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedAnswer {
    /// What was answered.
    pub answer: Answer,
    /// The name derived from the prover's key.
    pub prover: NodeId,
}

/// Read a challenge frame.
///
/// # Errors
/// Fails if the frame is malformed, the version is unknown, the verifier's key is
/// unusable, or the signature does not match.
pub fn open_challenge(frame: &[u8]) -> Result<SignedChallenge, wire::Error> {
    let (signature, body) = wire::split(frame)?;
    let challenge: Challenge = wire::decode(body)?;
    check_version(challenge.protocol)?;
    parse_public_key(&challenge.verifier)?;
    wire::check_signature(DOMAIN_CHALLENGE, body, signature, &challenge.verifier)?;
    Ok(SignedChallenge {
        verifier: NodeId::from_public_key(&challenge.verifier),
        challenge,
    })
}

/// Read an answer frame.
///
/// # Errors
/// Fails if the frame is malformed, the version is unknown, the prover's key is
/// unusable, or the signature does not match.
pub fn open_answer(frame: &[u8]) -> Result<SignedAnswer, wire::Error> {
    let (signature, body) = wire::split(frame)?;
    let answer: Answer = wire::decode(body)?;
    check_version(answer.protocol)?;
    parse_public_key(&answer.prover)?;
    wire::check_signature(DOMAIN_ANSWER, body, signature, &answer.prover)?;
    Ok(SignedAnswer {
        prover: NodeId::from_public_key(&answer.prover),
        answer,
    })
}

/// Refuse a version this build does not speak, rather than guess at the fields.
fn check_version(got: u16) -> Result<(), wire::Error> {
    if got == PROTOCOL_VERSION {
        Ok(())
    } else {
        Err(wire::Error::Version {
            got,
            expected: PROTOCOL_VERSION,
        })
    }
}

/// Why an answer does not answer a challenge.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum NotAnAnswer {
    /// The nonce is not the one that was asked.
    #[error("the answer signs a nonce this challenge did not ask")]
    WrongNonce,
    /// The answer is addressed to a different verifier.
    #[error("the answer is addressed to a different verifier")]
    WrongVerifier,
    /// The answer is for a different epoch.
    #[error("the challenge is for epoch {asked}, the answer for epoch {answered}")]
    WrongEpoch {
        /// The epoch the challenge named.
        asked: u64,
        /// The epoch the answer named.
        answered: u64,
    },
    /// Somebody else's answer.
    #[error("the answer was signed by a node this challenge was not sent to")]
    WrongProver,
}

/// A challenge and the answer to it, each signed by a different node.
///
/// WHAT THIS PROVES: that the node held its key and answered, after the nonce was
/// chosen, for this epoch and this verifier. Anyone can check it from the two
/// signatures alone, without trusting either party.
///
/// WHAT IT DOES NOT PROVE: **when**. Nothing in these bytes carries a time, and a
/// timestamp inside them would be the answering node's own word about its own
/// promptness. Only the verifier saw whether the answer arrived inside
/// [`RESPONSE_WINDOW_SECONDS`]. A node holding one of these can show it answered
/// eventually, which is worth something when a verifier published silence and is not
/// the same claim as having answered in time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exchange {
    /// The verifier's question.
    pub challenge: SignedChallenge,
    /// The prover's answer.
    pub answer: SignedAnswer,
}

impl Exchange {
    /// Put a challenge and an answer together, or say why they do not fit.
    ///
    /// # Errors
    /// Fails if the answer does not answer this challenge.
    pub fn assemble(challenge: SignedChallenge, answer: SignedAnswer) -> Result<Self, NotAnAnswer> {
        if answer.answer.nonce != challenge.challenge.nonce {
            return Err(NotAnAnswer::WrongNonce);
        }
        if answer.answer.verifier != challenge.challenge.verifier {
            return Err(NotAnAnswer::WrongVerifier);
        }
        if answer.answer.prover != challenge.challenge.prover {
            return Err(NotAnAnswer::WrongProver);
        }
        if answer.answer.epoch != challenge.challenge.epoch {
            return Err(NotAnAnswer::WrongEpoch {
                asked: challenge.challenge.epoch,
                answered: answer.answer.epoch,
            });
        }
        Ok(Self { challenge, answer })
    }

    /// The epoch both sides named.
    #[must_use]
    pub const fn epoch(&self) -> Epoch {
        Epoch(self.challenge.challenge.epoch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity(seed: u8) -> Identity {
        Identity::from_seed(&[seed; 32])
    }

    /// One full round trip, sealed and reopened the way it travels.
    fn round(
        verifier: &Identity,
        prover: &Identity,
        epoch: u64,
    ) -> (SignedChallenge, SignedAnswer) {
        let challenge = Challenge::new(verifier, prover.public_key(), Epoch(epoch));
        let asked = open_challenge(&challenge.seal(verifier).expect("seals")).expect("opens");
        let answer = Answer::to(&asked.challenge, prover, [7_u8; 32], 42);
        let answered = open_answer(&answer.seal(prover).expect("seals")).expect("opens");
        (asked, answered)
    }

    #[test]
    fn the_frozen_values_are_the_agreed_ones() {
        assert_eq!(DOMAIN_CHALLENGE, b"333.v1.challenge");
        assert_eq!(DOMAIN_ANSWER, b"333.v1.answer.to");
        assert_eq!(DOMAIN_CHALLENGE.len(), DOMAIN_LEN);
        assert_eq!(DOMAIN_ANSWER.len(), DOMAIN_LEN);
        assert_ne!(DOMAIN_CHALLENGE, DOMAIN_ANSWER);
        assert_eq!(RESPONSE_WINDOW_SECONDS, 180);
    }

    #[test]
    fn a_challenge_and_its_answer_fit_together() {
        let (verifier, prover) = (identity(1), identity(2));
        let (asked, answered) = round(&verifier, &prover, 89_516);
        let exchange = Exchange::assemble(asked, answered).expect("assembles");
        assert_eq!(exchange.epoch(), Epoch(89_516));
        assert_eq!(exchange.answer.prover, prover.node_id());
        assert_eq!(exchange.challenge.verifier, verifier.node_id());
        assert_eq!(exchange.answer.answer.chain_head, [7_u8; 32]);
        assert_eq!(exchange.answer.answer.chain_len, 42);
    }

    #[test]
    fn an_answer_cannot_be_shown_to_the_other_two_verifiers() {
        // One exchange must not count as three. The verifier is inside the signature,
        // so the answer given to one is not an answer to another even though the
        // epoch and the prover are the same.
        let prover = identity(9);
        let (first, answered) = round(&identity(1), &prover, 7);
        let (second, _) = round(&identity(2), &prover, 7);
        assert!(Exchange::assemble(first, answered.clone()).is_ok());
        assert_eq!(
            Exchange::assemble(second, answered),
            Err(NotAnAnswer::WrongNonce)
        );
    }

    #[test]
    fn an_answer_kept_from_one_epoch_does_not_serve_another() {
        // A node cannot answer once and produce it every epoch after. Even with the
        // nonce and verifier forced to match, the epoch inside the signature does not.
        let (verifier, prover) = (identity(1), identity(2));
        let challenge = Challenge::new(&verifier, prover.public_key(), Epoch(10));
        let asked = open_challenge(&challenge.seal(&verifier).expect("seals")).expect("opens");

        let mut stale = Answer::to(&asked.challenge, &prover, [0_u8; 32], 0);
        stale.epoch = 9;
        let stale = open_answer(&stale.seal(&prover).expect("seals")).expect("opens");
        assert_eq!(
            Exchange::assemble(asked, stale),
            Err(NotAnAnswer::WrongEpoch {
                asked: 10,
                answered: 9
            })
        );
    }

    #[test]
    fn somebody_elses_answer_is_not_this_nodes_answer() {
        let verifier = identity(1);
        let (asked, _) = round(&verifier, &identity(2), 7);
        // A third node signs the same nonce, epoch and verifier.
        let stranger = identity(3);
        let mut theirs = Answer::to(&asked.challenge, &stranger, [0_u8; 32], 0);
        theirs.prover = stranger.public_key();
        let theirs = open_answer(&theirs.seal(&stranger).expect("seals")).expect("opens");
        assert_eq!(
            Exchange::assemble(asked, theirs),
            Err(NotAnAnswer::WrongProver)
        );
    }

    #[test]
    fn a_node_with_no_record_yet_says_so_truthfully() {
        let (verifier, prover) = (identity(1), identity(2));
        let challenge = Challenge::new(&verifier, prover.public_key(), Epoch(1));
        let asked = open_challenge(&challenge.seal(&verifier).expect("seals")).expect("opens");
        let answer = Answer::to(&asked.challenge, &prover, [0_u8; 32], 0);
        assert_eq!(answer.chain_head, [0_u8; 32]);
        assert_eq!(answer.chain_len, 0);
        assert!(
            Exchange::assemble(
                asked,
                open_answer(&answer.seal(&prover).expect("seals")).expect("opens")
            )
            .is_ok()
        );
    }

    #[test]
    fn each_kind_of_frame_is_refused_as_the_other() {
        // The domains keep them apart. A challenge frame read as an answer must fail
        // at the signature rather than decode into something plausible.
        let (verifier, prover) = (identity(1), identity(2));
        let challenge = Challenge::new(&verifier, prover.public_key(), Epoch(1));
        let frame = challenge.seal(&verifier).expect("seals");
        assert!(open_challenge(&frame).is_ok());
        assert!(open_answer(&frame).is_err());
    }

    #[test]
    fn padding_and_tampering_are_refused() {
        let (verifier, prover) = (identity(1), identity(2));
        let challenge = Challenge::new(&verifier, prover.public_key(), Epoch(1));
        let mut frame = challenge.seal(&verifier).expect("seals");
        frame.push(0);
        assert_eq!(
            open_challenge(&frame),
            Err(wire::Error::TrailingBytes { extra: 1 })
        );

        let answer = Answer::to(&challenge, &prover, [0_u8; 32], 0);
        let mut frame = answer.seal(&prover).expect("seals");
        frame[0] ^= 0x01;
        assert_eq!(open_answer(&frame), Err(wire::Error::BadSignature));
    }
}
