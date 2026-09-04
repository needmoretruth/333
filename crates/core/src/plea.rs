//! Asking for the file.
//!
//! FROZEN. The field order is the wire format and the domain is part of the
//! protocol's identity.
//!
//! A client cannot make `333.txt`. It carries the hash and can therefore recognise
//! the file, but the bytes have to come from somebody who already has them, and this
//! is how they are asked for. That is the whole of it: three fields, one signature,
//! no negotiation.
//!
//! WHY IT IS SIGNED. Not to prove anything about the asker — a key is free and this
//! proves only that whoever sent it holds one. It is signed because the giver's half
//! of the record names the asker's key, and a node should not put its signature on a
//! statement about a key that nobody has demonstrably used.
//!
//! WHY THERE IS NO NONCE. A replayed plea gets the file handed over again and gets
//! the giver to sign a second "I gave it to X" that says the same thing as the first.
//! Neither half of a transfer record means anything alone: it becomes an admission
//! only when the other node signs the matching half, which needs the other node's
//! key. So a replay costs the giver some bandwidth and buys the replayer nothing.
//!
//! WHOSE EPOCH IT IS. The asker states the epoch it believes it is in, and the giver
//! is free to disagree — see the giver's half of the record, which is the one that
//! decides. This field is here so a giver can see a clock that is wildly out before
//! it signs anything.

use serde::{Deserialize, Serialize};

use crate::epoch::Epoch;
use crate::heartbeat::PROTOCOL_VERSION;
use crate::identity::{Identity, NodeId, parse_public_key};
use crate::wire::{self, DOMAIN_LEN};

/// The domain a plea is signed under. FROZEN.
pub const DOMAIN_PLEA: &[u8; DOMAIN_LEN] = b"333.v1.asked.for";

/// Somebody asking to be given the file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plea {
    /// Wire protocol version.
    pub protocol: u16,
    /// Who is asking. The key the giver's half of the record will name.
    pub asker: [u8; 32],
    /// The epoch the asker believes it is in.
    pub epoch: u64,
}

impl Plea {
    /// Ask.
    #[must_use]
    pub fn of(asker: &Identity, epoch: Epoch) -> Self {
        Self {
            protocol: PROTOCOL_VERSION,
            asker: asker.public_key(),
            epoch: epoch.0,
        }
    }

    /// The epoch the asker believes it is in.
    #[must_use]
    pub const fn epoch(&self) -> Epoch {
        Epoch(self.epoch)
    }

    /// Encode and sign.
    ///
    /// # Errors
    /// Fails if the encoding is impossible or the frame would exceed the wire limit.
    pub fn seal(&self, asker: &Identity) -> Result<Vec<u8>, wire::Error> {
        let body = postcard::to_stdvec(self).map_err(|e| wire::Error::Decode(e.to_string()))?;
        wire::seal(DOMAIN_PLEA, &body, asker)
    }
}

/// A plea that arrived with a signature that checks out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signed {
    /// What was asked.
    pub plea: Plea,
    /// The name derived from the asker's key.
    pub asker: NodeId,
}

/// Read a plea frame.
///
/// # Errors
/// Fails if the frame is malformed, the version is unknown, the key is unusable, or
/// the signature does not match.
pub fn open(frame: &[u8]) -> Result<Signed, wire::Error> {
    let (signature, body) = wire::split(frame)?;
    let plea: Plea = wire::decode(body)?;
    if plea.protocol != PROTOCOL_VERSION {
        return Err(wire::Error::Version {
            got: plea.protocol,
            expected: PROTOCOL_VERSION,
        });
    }
    parse_public_key(&plea.asker)?;
    wire::check_signature(DOMAIN_PLEA, body, signature, &plea.asker)?;
    Ok(Signed {
        asker: NodeId::from_public_key(&plea.asker),
        plea,
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
        assert_eq!(DOMAIN_PLEA, b"333.v1.asked.for");
        assert_eq!(DOMAIN_PLEA.len(), DOMAIN_LEN);
    }

    #[test]
    fn a_plea_arrives_saying_who_asked_and_when_they_think_it_is() {
        let me = identity(1);
        let frame = Plea::of(&me, Epoch(500)).seal(&me).expect("seals");
        let signed = open(&frame).expect("opens");
        assert_eq!(signed.asker, me.node_id());
        assert_eq!(signed.plea.asker, me.public_key());
        assert_eq!(signed.plea.epoch(), Epoch(500));
    }

    #[test]
    fn nobody_can_ask_in_somebody_elses_name() {
        // The giver's half of the record names this key, so it has to be the key that
        // signed the asking.
        let (me, them) = (identity(1), identity(2));
        let mut forged = Plea::of(&me, Epoch(1));
        forged.asker = them.public_key();
        let frame = forged.seal(&me).expect("seals");
        assert_eq!(open(&frame), Err(wire::Error::BadSignature));
    }

    #[test]
    fn a_plea_from_another_version_is_refused_rather_than_guessed_at() {
        let me = identity(1);
        let mut ahead = Plea::of(&me, Epoch(1));
        ahead.protocol = PROTOCOL_VERSION + 1;
        let frame = ahead.seal(&me).expect("seals");
        assert!(matches!(open(&frame), Err(wire::Error::Version { .. })));
    }

    #[test]
    fn a_padded_plea_does_not_open() {
        let me = identity(1);
        let mut frame = Plea::of(&identity(1), Epoch(1)).seal(&me).expect("seals");
        frame.push(0);
        assert_eq!(open(&frame), Err(wire::Error::TrailingBytes { extra: 1 }));
    }
}
