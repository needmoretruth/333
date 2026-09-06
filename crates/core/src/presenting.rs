//! Coming to be asked, for a node nobody can reach.
//!
//! FROZEN. The field order is the wire format and the domain is part of the
//! protocol's identity.
//!
//! WHY THIS EXISTS. A challenge normally travels one way: the verifier was drawn, the
//! verifier dials, the prover answers. That works for as long as the prover can be
//! dialled, and on an ordinary home connection it cannot be — the router in front of it
//! drops what nobody inside asked for. Such a node is awake, holds its key and answers
//! everything it is asked, and it is nevertheless counted absent for ever, because
//! nobody can put the question to it.
//!
//! So it goes to them. The draw takes three inputs — the epoch, the prover's key and
//! the candidate's key — and nothing else, which means a node can work out for itself
//! which of us were drawn to ask it. It opens the connection, says this, and the
//! verifier puts the question down the connection that is now open.
//!
//! NOTHING ABOUT THE ANSWER CHANGES. The nonce is still the verifier's, the answer is
//! still signed by the prover over that nonce, and the statement the verifier publishes
//! is the same statement it would have published had it dialled. Who paid for the TCP
//! connection is not one of the things a signature covers, and it was never evidence of
//! anything: a prover that answers is a prover that was awake, which is the whole of
//! what the challenge asks.
//!
//! WHY IT IS SIGNED, GIVEN THE HEARTBEAT ALREADY PROVED WHO IS CALLING. Because the
//! epoch is in it. The verifier is being asked to spend a challenge on a particular
//! epoch, and the caller should have to say which one under its own signature rather
//! than leave the verifier to assume.

use serde::{Deserialize, Serialize};

use crate::epoch::Epoch;
use crate::heartbeat::PROTOCOL_VERSION;
use crate::identity::{Identity, NodeId, parse_public_key};
use crate::wire::{self, DOMAIN_LEN};

/// The domain this is signed under. FROZEN.
pub const DOMAIN_PRESENTING: &[u8; DOMAIN_LEN] = b"333.v1.here.i.am";

/// A node offering itself to whoever was drawn to ask it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Presenting {
    /// Wire protocol version.
    pub protocol: u16,
    /// Who has come. The key the challenge will name as the prover.
    pub prover: [u8; 32],
    /// The epoch it is offering itself for.
    pub epoch: u64,
}

impl Presenting {
    /// Come to be asked.
    #[must_use]
    pub fn of(prover: &Identity, epoch: Epoch) -> Self {
        Self {
            protocol: PROTOCOL_VERSION,
            prover: prover.public_key(),
            epoch: epoch.0,
        }
    }

    /// The epoch it is offering itself for.
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
        wire::seal(DOMAIN_PRESENTING, &body, prover)
    }
}

/// One that arrived with a signature that checks out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signed {
    /// What was said.
    pub presenting: Presenting,
    /// The name derived from the caller's key.
    pub prover: NodeId,
}

/// Read a frame that says somebody has come to be asked.
///
/// # Errors
/// Fails if the frame is malformed, the version is unknown, the key is unusable, or
/// the signature does not match.
pub fn open(frame: &[u8]) -> Result<Signed, wire::Error> {
    let (signature, body) = wire::split(frame)?;
    let presenting: Presenting = wire::decode(body)?;
    if presenting.protocol != PROTOCOL_VERSION {
        return Err(wire::Error::Version {
            got: presenting.protocol,
            expected: PROTOCOL_VERSION,
        });
    }
    parse_public_key(&presenting.prover)?;
    wire::check_signature(DOMAIN_PRESENTING, body, signature, &presenting.prover)?;
    Ok(Signed {
        prover: NodeId::from_public_key(&presenting.prover),
        presenting,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity(seed: u8) -> Identity {
        Identity::from_seed(&[seed; 32])
    }

    #[test]
    fn the_domain_is_frozen() {
        assert_eq!(DOMAIN_PRESENTING, b"333.v1.here.i.am");
        assert_eq!(DOMAIN_PRESENTING.len(), DOMAIN_LEN);
    }

    #[test]
    fn it_opens_as_what_was_sealed() {
        let node = identity(1);
        let frame = Presenting::of(&node, Epoch(90_000))
            .seal(&node)
            .expect("seals");
        let opened = open(&frame).expect("opens");
        assert_eq!(opened.presenting.prover, node.public_key());
        assert_eq!(opened.presenting.epoch(), Epoch(90_000));
        assert_eq!(opened.prover, node.node_id());
    }

    #[test]
    fn somebody_elses_signature_does_not_carry_it() {
        let (node, other) = (identity(1), identity(2));
        // Sealed by `other` while claiming to be `node`: the signature is checked
        // against the key inside the message, so this is the one forgery that matters
        // here — arriving as somebody who was drawn when you were not.
        let mut presenting = Presenting::of(&node, Epoch(7));
        presenting.prover = node.public_key();
        let body = postcard::to_stdvec(&presenting).expect("encodes");
        let frame = wire::seal(DOMAIN_PRESENTING, &body, &other).expect("seals");
        assert!(open(&frame).is_err());
    }

    #[test]
    fn a_plea_is_not_read_as_one_of_these() {
        let node = identity(1);
        let frame = crate::plea::Plea::of(&node, Epoch(7))
            .seal(&node)
            .expect("seals");
        assert!(open(&frame).is_err());
    }
}
