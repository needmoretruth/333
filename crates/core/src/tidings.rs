//! Saying "here is what I have", before handing it over.
//!
//! FROZEN. The field order is the wire format and the domain is part of the
//! protocol's identity.
//!
//! A short signed header and then a run of frames. The header exists so that the run
//! can be told apart from the two other things a peer may open a conversation with,
//! and so that whoever passed the statements on is named — not because it matters who
//! carried them. It does not: every statement inside carries its own signature, and a
//! statement that opens is worth exactly the same whoever handed it over.
//!
//! WHAT IS NOT HERE. No list of what the other side already has, no digests to compare,
//! no request for particular epochs. Working out the difference between two nodes'
//! holdings costs a round trip and some bookkeeping to save re-sending a few hundred
//! signed statements that are a few hundred bytes each, and it introduces the one thing
//! this protocol will not have: two nodes having to agree about something.

use serde::{Deserialize, Serialize};

use crate::epoch::Epoch;
use crate::heartbeat::PROTOCOL_VERSION;
use crate::identity::{Identity, NodeId, parse_public_key};
use crate::wire::{self, DOMAIN_LEN};

/// The domain the header of a run is signed under. FROZEN.
pub const DOMAIN_TIDINGS: &[u8; DOMAIN_LEN] = b"333.v1.i.pass.on";

/// The header in front of a run of statements being passed on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tidings {
    /// Wire protocol version.
    pub protocol: u16,
    /// Who is passing them on. Named, not trusted.
    pub teller: [u8; 32],
    /// The epoch the teller believes it is in.
    pub epoch: u64,
}

impl Tidings {
    /// Say that a run is coming.
    #[must_use]
    pub fn from(teller: &Identity, epoch: Epoch) -> Self {
        Self {
            protocol: PROTOCOL_VERSION,
            teller: teller.public_key(),
            epoch: epoch.0,
        }
    }

    /// The epoch the teller believes it is in.
    #[must_use]
    pub const fn epoch(&self) -> Epoch {
        Epoch(self.epoch)
    }

    /// Encode and sign.
    ///
    /// # Errors
    /// Fails if the encoding is impossible or the frame would exceed the wire limit.
    pub fn seal(&self, teller: &Identity) -> Result<Vec<u8>, wire::Error> {
        let body = postcard::to_stdvec(self).map_err(|e| wire::Error::Decode(e.to_string()))?;
        wire::seal(DOMAIN_TIDINGS, &body, teller)
    }
}

/// A header that arrived with a signature that checks out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signed {
    /// What was said.
    pub tidings: Tidings,
    /// The name derived from the teller's key.
    pub teller: NodeId,
}

/// Read the header of a run.
///
/// # Errors
/// Fails if the frame is malformed, the version is unknown, the key is unusable, or
/// the signature does not match.
pub fn open(frame: &[u8]) -> Result<Signed, wire::Error> {
    let (signature, body) = wire::split(frame)?;
    let tidings: Tidings = wire::decode(body)?;
    if tidings.protocol != PROTOCOL_VERSION {
        return Err(wire::Error::Version {
            got: tidings.protocol,
            expected: PROTOCOL_VERSION,
        });
    }
    parse_public_key(&tidings.teller)?;
    wire::check_signature(DOMAIN_TIDINGS, body, signature, &tidings.teller)?;
    Ok(Signed {
        teller: NodeId::from_public_key(&tidings.teller),
        tidings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity(seed: u8) -> Identity {
        Identity::from_seed(&[seed; 32])
    }

    #[test]
    fn the_frozen_values_are_the_agreed_ones() {
        assert_eq!(DOMAIN_TIDINGS, b"333.v1.i.pass.on");
        assert_eq!(DOMAIN_TIDINGS.len(), DOMAIN_LEN);
    }

    #[test]
    fn a_header_says_who_is_passing_things_on_and_when() {
        let me = identity(1);
        let frame = Tidings::from(&me, Epoch(42)).seal(&me).expect("seals");
        let signed = open(&frame).expect("opens");
        assert_eq!(signed.teller, me.node_id());
        assert_eq!(signed.tidings.epoch(), Epoch(42));
    }

    #[test]
    fn nobody_can_pass_things_on_in_somebody_elses_name() {
        // It changes nothing about what the statements are worth. It is refused
        // because a name in a signed message should mean the thing it says.
        let (me, them) = (identity(1), identity(2));
        let mut forged = Tidings::from(&me, Epoch(1));
        forged.teller = them.public_key();
        let frame = forged.seal(&me).expect("seals");
        assert_eq!(open(&frame), Err(wire::Error::BadSignature));
    }

    #[test]
    fn a_padded_header_does_not_open() {
        let me = identity(1);
        let mut frame = Tidings::from(&me, Epoch(1)).seal(&me).expect("seals");
        frame.push(0);
        assert_eq!(open(&frame), Err(wire::Error::TrailingBytes { extra: 1 }));
    }
}
