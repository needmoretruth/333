//! The record of the file being handed from one node to another.
//!
//! FROZEN: the field order below is the wire format, and the two domains are part of
//! the protocol's identity.
//!
//! WHY THIS EXISTS AT ALL. The file is three bytes. Nothing about the bytes proves
//! where they came from, and no amount of signing them would change that — anyone can
//! sign a copy of something everyone has. So what gets recorded is not the file but
//! the act: one side signs *I handed it to you in epoch N*, the other signs *I
//! received it from you in epoch N*, and a transfer is the pair.
//!
//! The pair is the whole point. Either half alone is one node's unsupported claim
//! about another, and this protocol never lets a node's own claim about itself count
//! for anything. Both halves together need both private keys, which is the one thing
//! a single node cannot manufacture. It still cannot prove the file was really sent —
//! two nodes can agree to sign a lie — but a lie now costs the cooperation of the
//! node being lied about, and it is written down under both names.
//!
//! Each node's first received half names whoever gave it the file. Following those
//! back is a lineage, and it is the only history this protocol keeps.

use serde::{Deserialize, Serialize};

use crate::epoch::Epoch;
use crate::identity::{Identity, NodeId, parse_public_key};
use crate::wire::{self, DOMAIN_LEN};

/// The domain for the giving half.
///
/// FROZEN, and exactly [`DOMAIN_LEN`] bytes like every other domain, so that no
/// domain can be a prefix of another and no signature can be replayed as the other
/// half of its own transfer.
pub const DOMAIN_GAVE: &[u8; DOMAIN_LEN] = b"333.v1.sent.file";

/// The domain for the receiving half. FROZEN.
pub const DOMAIN_RECEIVED: &[u8; DOMAIN_LEN] = b"333.v1.recv.file";

/// Which half of a transfer a record is.
///
/// Not a field of the record: it selects the domain, so it is already part of what
/// was signed. A record cannot be reinterpreted as the other half, because the
/// signature would no longer verify.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Half {
    /// *I handed the file to you.*
    Gave,
    /// *I received the file from you.*
    Received,
}

impl Half {
    /// The domain a record of this half is signed under.
    #[must_use]
    pub const fn domain(self) -> &'static [u8; DOMAIN_LEN] {
        match self {
            Self::Gave => DOMAIN_GAVE,
            Self::Received => DOMAIN_RECEIVED,
        }
    }

    /// The other half, the one the counterparty signs.
    #[must_use]
    pub const fn other(self) -> Self {
        match self {
            Self::Gave => Self::Received,
            Self::Received => Self::Gave,
        }
    }
}

/// One side's signed statement about one handover.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    /// Wire protocol version.
    pub protocol: u16,
    /// The public key of whoever signed this half.
    pub author: [u8; 32],
    /// The public key of the other side.
    pub counterparty: [u8; 32],
    /// The epoch the author believes the handover happened in.
    pub epoch: u64,
    /// The digest of what was handed over.
    ///
    /// There is only one file, so this is always [`crate::subject::DIGEST`] today. It
    /// is written down anyway: a record that does not say what it is about is a
    /// record of nothing, and a reader a century from now should not have to know
    /// what the only file was.
    pub subject: [u8; 32],
}

impl Record {
    /// Compose this node's half of a handover.
    #[must_use]
    pub fn new(
        identity: &Identity,
        counterparty: [u8; 32],
        epoch: Epoch,
        subject: [u8; 32],
    ) -> Self {
        Self {
            protocol: crate::heartbeat::PROTOCOL_VERSION,
            author: identity.public_key(),
            counterparty,
            epoch: epoch.0,
            subject,
        }
    }

    /// The epoch this record claims.
    #[must_use]
    pub const fn epoch(&self) -> Epoch {
        Epoch(self.epoch)
    }

    /// Encode and sign as one half of a transfer.
    ///
    /// # Errors
    /// Fails if the encoding is impossible or the frame would exceed the wire limit.
    pub fn seal(&self, half: Half, identity: &Identity) -> Result<Vec<u8>, wire::Error> {
        let body = postcard::to_stdvec(self).map_err(|e| wire::Error::Decode(e.to_string()))?;
        wire::seal(half.domain(), &body, identity)
    }
}

/// A half that arrived, decoded, and carried a signature that checks out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signed {
    /// What was said.
    pub record: Record,
    /// Which half it is. Established by which domain the signature verified under,
    /// not by anything the sender wrote in the body.
    pub half: Half,
    /// The name derived from the author's key.
    pub author: NodeId,
}

/// Read one half of a transfer as it arrived.
///
/// The `half` argument says which one to expect. Reading the same bytes as the other
/// half fails at the signature, because the domain is part of what was signed.
///
/// # Errors
/// Fails if the frame is malformed, the version is unknown, the author's key is
/// unusable, or the signature does not match.
pub fn open(frame: &[u8], half: Half) -> Result<Signed, wire::Error> {
    let (signature, body) = wire::split(frame)?;
    let record: Record = wire::decode(body)?;

    if record.protocol != crate::heartbeat::PROTOCOL_VERSION {
        return Err(wire::Error::Version {
            got: record.protocol,
            expected: crate::heartbeat::PROTOCOL_VERSION,
        });
    }
    parse_public_key(&record.author)?;
    wire::check_signature(half.domain(), body, signature, &record.author)?;

    let author = NodeId::from_public_key(&record.author);
    Ok(Signed {
        record,
        half,
        author,
    })
}

/// Why two halves do not make a transfer.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Mismatch {
    /// Both halves say the same thing, so one node signed both. Nothing about a
    /// transfer is established by one key.
    #[error("both halves are the same half")]
    SameHalf,
    /// One node signed both halves, naming itself as its own counterparty. Both
    /// signatures verify and the record still means nothing: the pair is only worth
    /// something because it takes two keys, and here it took one.
    #[error("both halves were signed by the same node")]
    SameNode,
    /// Each half names somebody other than the one that signed the other.
    #[error("the two halves do not name each other")]
    NotEachOther,
    /// The two sides put the handover in different epochs.
    #[error("one half says epoch {gave}, the other says epoch {received}")]
    DifferentEpoch {
        /// The epoch the giver signed.
        gave: u64,
        /// The epoch the receiver signed.
        received: u64,
    },
    /// The two sides are talking about different files.
    #[error("the two halves name different files")]
    DifferentSubject,
}

/// A handover both sides signed.
///
/// Constructing one is the only way to assert that a transfer happened, and it needs
/// both signatures, which is the property the whole record exists for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transfer {
    /// The half signed by whoever handed the file over.
    pub gave: Signed,
    /// The half signed by whoever received it.
    pub received: Signed,
}

impl Transfer {
    /// Put two verified halves together, or say why they do not fit.
    ///
    /// Each half is checked against the other rather than against anything this node
    /// believes, so a third party reading a record years later reaches the same
    /// answer with nothing but the two frames.
    ///
    /// # Errors
    /// Fails if the halves are the same half, do not name each other, or disagree
    /// about the epoch or the file.
    pub fn assemble(gave: Signed, received: Signed) -> Result<Self, Mismatch> {
        if gave.half == received.half {
            return Err(Mismatch::SameHalf);
        }
        let (gave, received) = match gave.half {
            Half::Gave => (gave, received),
            Half::Received => (received, gave),
        };
        if gave.record.author == received.record.author {
            return Err(Mismatch::SameNode);
        }
        if gave.record.author != received.record.counterparty
            || gave.record.counterparty != received.record.author
        {
            return Err(Mismatch::NotEachOther);
        }
        if gave.record.epoch != received.record.epoch {
            return Err(Mismatch::DifferentEpoch {
                gave: gave.record.epoch,
                received: received.record.epoch,
            });
        }
        if gave.record.subject != received.record.subject {
            return Err(Mismatch::DifferentSubject);
        }
        Ok(Self { gave, received })
    }

    /// The epoch both sides agreed the handover happened in.
    #[must_use]
    pub const fn epoch(&self) -> Epoch {
        Epoch(self.gave.record.epoch)
    }

    /// Who handed the file over: the name a lineage follows back through.
    #[must_use]
    pub const fn giver(&self) -> &NodeId {
        &self.gave.author
    }

    /// Who received it.
    #[must_use]
    pub const fn receiver(&self) -> &NodeId {
        &self.received.author
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subject::DIGEST;

    fn identity(seed: u8) -> Identity {
        Identity::from_seed(&[seed; 32])
    }

    /// Both halves of one handover, sealed and reopened the way they travel.
    fn handover(giver: &Identity, receiver: &Identity, epoch: u64) -> (Signed, Signed) {
        let epoch = Epoch(epoch);
        let gave = Record::new(giver, receiver.public_key(), epoch, DIGEST)
            .seal(Half::Gave, giver)
            .expect("seals");
        let received = Record::new(receiver, giver.public_key(), epoch, DIGEST)
            .seal(Half::Received, receiver)
            .expect("seals");
        (
            open(&gave, Half::Gave).expect("opens"),
            open(&received, Half::Received).expect("opens"),
        )
    }

    #[test]
    fn the_domains_are_the_agreed_ones() {
        assert_eq!(DOMAIN_GAVE, b"333.v1.sent.file");
        assert_eq!(DOMAIN_RECEIVED, b"333.v1.recv.file");
        assert_eq!(DOMAIN_GAVE.len(), DOMAIN_LEN);
        assert_eq!(DOMAIN_RECEIVED.len(), DOMAIN_LEN);
        assert_ne!(DOMAIN_GAVE, DOMAIN_RECEIVED);
    }

    #[test]
    fn two_halves_that_agree_make_a_transfer() {
        let (giver, receiver) = (identity(1), identity(2));
        let (gave, received) = handover(&giver, &receiver, 89_516);
        let transfer = Transfer::assemble(gave, received).expect("assembles");
        assert_eq!(transfer.epoch(), Epoch(89_516));
        assert_eq!(transfer.giver(), &giver.node_id());
        assert_eq!(transfer.receiver(), &receiver.node_id());
    }

    #[test]
    fn the_order_the_halves_arrive_in_does_not_matter() {
        let (giver, receiver) = (identity(1), identity(2));
        let (gave, received) = handover(&giver, &receiver, 7);
        let forwards = Transfer::assemble(gave.clone(), received.clone()).expect("assembles");
        let backwards = Transfer::assemble(received, gave).expect("assembles");
        assert_eq!(forwards, backwards);
    }

    #[test]
    fn one_node_cannot_sign_both_halves() {
        // The whole reason the record is a pair. A single key can seal a giving half
        // and a receiving half naming itself, and both signatures verify — so the
        // refusal has to be here, not at the signature.
        let me = identity(1);
        let epoch = Epoch(7);
        let [gave, received] = [Half::Gave, Half::Received].map(|half| {
            let frame = Record::new(&me, me.public_key(), epoch, DIGEST)
                .seal(half, &me)
                .expect("seals");
            open(&frame, half).expect("opens")
        });
        assert_eq!(
            Transfer::assemble(gave.clone(), received),
            Err(Mismatch::SameNode)
        );
        assert_eq!(
            Transfer::assemble(gave.clone(), gave),
            Err(Mismatch::SameHalf)
        );
    }

    #[test]
    fn a_node_cannot_borrow_a_second_key_it_also_holds() {
        // Two keys held by one person still assemble. Nothing here can tell that
        // apart from two people, and nothing claims to: what the pair buys is that a
        // node cannot write a transfer with a node that did not agree to it.
        let (a, b) = (identity(1), identity(2));
        let (gave, received) = handover(&a, &b, 7);
        assert!(Transfer::assemble(gave, received).is_ok());
    }

    #[test]
    fn a_half_cannot_be_read_as_the_other_half() {
        // The domain is inside the signature, so reinterpreting a giving half as a
        // receiving one fails at the signature rather than producing a valid record.
        let (giver, receiver) = (identity(1), identity(2));
        let frame = Record::new(&giver, receiver.public_key(), Epoch(7), DIGEST)
            .seal(Half::Gave, &giver)
            .expect("seals");
        assert!(open(&frame, Half::Gave).is_ok());
        assert_eq!(open(&frame, Half::Received), Err(wire::Error::BadSignature));
    }

    #[test]
    fn halves_that_do_not_name_each_other_are_refused() {
        let (giver, receiver, stranger) = (identity(1), identity(2), identity(3));
        let (gave, _) = handover(&giver, &receiver, 7);
        let (_, received_from_stranger) = handover(&stranger, &receiver, 7);
        assert_eq!(
            Transfer::assemble(gave, received_from_stranger),
            Err(Mismatch::NotEachOther)
        );
    }

    #[test]
    fn halves_that_disagree_about_when_or_what_are_refused() {
        let (giver, receiver) = (identity(1), identity(2));
        let (gave, _) = handover(&giver, &receiver, 7);
        let (_, later) = handover(&giver, &receiver, 8);
        assert_eq!(
            Transfer::assemble(gave.clone(), later),
            Err(Mismatch::DifferentEpoch {
                gave: 7,
                received: 8
            })
        );

        let other_file = Record::new(&receiver, giver.public_key(), Epoch(7), [0_u8; 32])
            .seal(Half::Received, &receiver)
            .expect("seals");
        assert_eq!(
            Transfer::assemble(gave, open(&other_file, Half::Received).expect("opens")),
            Err(Mismatch::DifferentSubject)
        );
    }

    #[test]
    fn a_padded_half_is_refused() {
        let (giver, receiver) = (identity(1), identity(2));
        let mut frame = Record::new(&giver, receiver.public_key(), Epoch(7), DIGEST)
            .seal(Half::Gave, &giver)
            .expect("seals");
        assert!(open(&frame, Half::Gave).is_ok());

        frame.push(0);
        assert_eq!(
            open(&frame, Half::Gave),
            Err(wire::Error::TrailingBytes { extra: 1 })
        );
    }

    #[test]
    fn a_tampered_half_does_not_open() {
        let (giver, receiver) = (identity(1), identity(2));
        let mut frame = Record::new(&giver, receiver.public_key(), Epoch(7), DIGEST)
            .seal(Half::Gave, &giver)
            .expect("seals");
        let last = frame.len() - 1;
        frame[last] ^= 0x01;
        assert_eq!(open(&frame, Half::Gave), Err(wire::Error::BadSignature));
    }
}
