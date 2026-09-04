//! A node saying where it can be reached.
//!
//! FROZEN. The field order is the wire format and the domain is part of the
//! protocol's identity.
//!
//! The roll says who is a member. It does not say where anybody is, and it must not:
//! a member who moves house has not stopped being a member, and an admission is signed
//! once and never again. So whereabouts are a separate, short-lived statement — this
//! node, this address, this epoch — that a node reissues as it likes and that anybody
//! can pass on.
//!
//! WHAT IT PROVES. That whoever holds this key said, at that epoch, to look there.
//! Nothing else. Not that the address works, not that it still works, and not that
//! the node is there now. The way to find out is to go and knock, and the exchange at
//! the far end is where a node proves anything.
//!
//! WHY THE EPOCH IS INSIDE THE SIGNATURE. Without it, an address a node used years ago
//! is indistinguishable from the one it uses today, and anybody holding the old
//! statement can keep sending people to it for ever. With it, a reader keeps the
//! newest and can see how old the newest is.
//!
//! THIS IS THE ONE PLACE A NODE'S LOCATION IS WRITTEN DOWN, and a node that does not
//! want its location written down does not issue one for a clearnet address: it
//! publishes an onion address instead, or none at all and reaches out rather than
//! being reached.

use serde::{Deserialize, Serialize};

use crate::epoch::Epoch;
use crate::heartbeat::PROTOCOL_VERSION;
use crate::identity::{Identity, NodeId, parse_public_key};
use crate::wire::{self, DOMAIN_LEN};

/// The domain a whereabouts statement is signed under. FROZEN.
pub const DOMAIN_WHERE: &[u8; DOMAIN_LEN] = b"333.v1.where.iam";

/// The longest address this will accept.
///
/// A hostname is at most 253 characters and an onion address is 62. The limit is here
/// so that a signed statement cannot be made large by whoever wrote it.
pub const MAX_ADDRESS_LEN: usize = 300;

/// A node's signed statement about where to find it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Whereabouts {
    /// Wire protocol version.
    pub protocol: u16,
    /// Whose address this is.
    pub node: [u8; 32],
    /// Where to look, in the one canonical spelling.
    ///
    /// Kept as text rather than parsed, because this crate has no idea what a
    /// transport is. Whoever dials it decides what the text means, and refuses it if
    /// it is not an address.
    pub address: String,
    /// The epoch this was said in.
    pub epoch: u64,
}

impl Whereabouts {
    /// Say where this node can be reached.
    #[must_use]
    pub fn of(node: &Identity, address: String, epoch: Epoch) -> Self {
        Self {
            protocol: PROTOCOL_VERSION,
            node: node.public_key(),
            address,
            epoch: epoch.0,
        }
    }

    /// The epoch this was said in.
    #[must_use]
    pub const fn epoch(&self) -> Epoch {
        Epoch(self.epoch)
    }

    /// Encode and sign.
    ///
    /// # Errors
    /// Fails if the address is too long, the encoding is impossible, or the frame
    /// would exceed the wire limit.
    pub fn seal(&self, node: &Identity) -> Result<Vec<u8>, wire::Error> {
        if self.address.len() > MAX_ADDRESS_LEN {
            return Err(wire::Error::TooLong {
                got: self.address.len(),
            });
        }
        let body = postcard::to_stdvec(self).map_err(|e| wire::Error::Decode(e.to_string()))?;
        wire::seal(DOMAIN_WHERE, &body, node)
    }
}

/// A whereabouts statement that arrived with a signature that checks out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signed {
    /// What was said.
    pub whereabouts: Whereabouts,
    /// The name derived from the key.
    pub node: NodeId,
}

/// Read a whereabouts frame.
///
/// # Errors
/// Fails if the frame is malformed, the version is unknown, the address is over the
/// limit, the key is unusable, or the signature does not match.
pub fn open(frame: &[u8]) -> Result<Signed, wire::Error> {
    let (signature, body) = wire::split(frame)?;
    let whereabouts: Whereabouts = wire::decode(body)?;
    if whereabouts.protocol != PROTOCOL_VERSION {
        return Err(wire::Error::Version {
            got: whereabouts.protocol,
            expected: PROTOCOL_VERSION,
        });
    }
    if whereabouts.address.len() > MAX_ADDRESS_LEN {
        return Err(wire::Error::TooLong {
            got: whereabouts.address.len(),
        });
    }
    parse_public_key(&whereabouts.node)?;
    wire::check_signature(DOMAIN_WHERE, body, signature, &whereabouts.node)?;
    Ok(Signed {
        node: NodeId::from_public_key(&whereabouts.node),
        whereabouts,
    })
}

/// The newest statement each node has made about itself.
///
/// Not a directory and not authoritative: it is one node's notes, built from whatever
/// it happened to be handed.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Directory {
    /// The newest statement per key, and the frame it arrived as.
    entries: std::collections::BTreeMap<[u8; 32], (Whereabouts, Vec<u8>)>,
}

impl Directory {
    /// An empty directory.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// How many nodes it knows an address for.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Does it know of anybody?
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Where a node last said it could be found.
    #[must_use]
    pub fn address_of(&self, node: &[u8; 32]) -> Option<&str> {
        self.entries.get(node).map(|(w, _)| w.address.as_str())
    }

    /// The statement behind an address, as it arrived, to pass on unchanged.
    #[must_use]
    pub fn frame_for(&self, node: &[u8; 32]) -> Option<&[u8]> {
        self.entries.get(node).map(|(_, frame)| frame.as_slice())
    }

    /// Every node and where it said to look, in key order.
    pub fn entries(&self) -> impl Iterator<Item = (&[u8; 32], &str)> {
        self.entries
            .iter()
            .map(|(key, (w, _))| (key, w.address.as_str()))
    }

    /// Every statement held, as it arrived, to pass on unchanged.
    pub fn frames(&self) -> impl Iterator<Item = &[u8]> {
        self.entries.values().map(|(_, frame)| frame.as_slice())
    }

    /// Take a statement, keeping it only if it is newer than what is held.
    ///
    /// Returns whether anything changed. An older statement is dropped rather than
    /// applied, so replaying a node's past addresses cannot move it back to one.
    pub fn note(&mut self, signed: Signed, frame: Vec<u8>) -> bool {
        let key = signed.whereabouts.node;
        match self.entries.get(&key) {
            Some((held, _)) if held.epoch >= signed.whereabouts.epoch => false,
            _ => {
                self.entries.insert(key, (signed.whereabouts, frame));
                true
            }
        }
    }

    /// Build a directory out of frames, in any order, from anywhere.
    ///
    /// Anything that does not open is counted and left out.
    #[must_use]
    pub fn from_frames(frames: &[Vec<u8>]) -> (Self, usize) {
        let mut directory = Self::new();
        let mut unreadable = 0;
        for frame in frames {
            match open(frame) {
                Ok(signed) => {
                    directory.note(signed, frame.clone());
                }
                Err(_) => unreadable += 1,
            }
        }
        (directory, unreadable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity(seed: u8) -> Identity {
        Identity::from_seed(&[seed; 32])
    }

    fn stated(node: &Identity, address: &str, epoch: u64) -> Vec<u8> {
        Whereabouts::of(node, address.to_owned(), Epoch(epoch))
            .seal(node)
            .expect("seals")
    }

    #[test]
    fn the_frozen_values_are_the_agreed_ones() {
        assert_eq!(DOMAIN_WHERE, b"333.v1.where.iam");
        assert_eq!(DOMAIN_WHERE.len(), DOMAIN_LEN);
        assert_eq!(MAX_ADDRESS_LEN, 300);
    }

    #[test]
    fn a_node_can_say_where_it_is_and_anybody_can_read_it() {
        let me = identity(1);
        let signed = open(&stated(&me, "node.example:3333", 100)).expect("opens");
        assert_eq!(signed.node, me.node_id());
        assert_eq!(signed.whereabouts.address, "node.example:3333");
        assert_eq!(signed.whereabouts.epoch(), Epoch(100));
    }

    #[test]
    fn nobody_can_say_where_somebody_else_is() {
        // The statement is signed by the node it is about, so moving a peer to an
        // address of your choosing needs their key.
        let (me, them) = (identity(1), identity(2));
        let mut forged = Whereabouts::of(&me, "trap.example:3333".into(), Epoch(1));
        forged.node = them.public_key();
        let frame = forged.seal(&me).expect("seals");
        assert_eq!(open(&frame), Err(wire::Error::BadSignature));
    }

    #[test]
    fn the_newest_statement_wins_and_an_older_one_cannot_move_a_node_back() {
        let me = identity(1);
        let mut directory = Directory::new();
        let old = stated(&me, "old.example:3333", 10);
        let new = stated(&me, "new.example:3333", 20);

        assert!(directory.note(open(&new).expect("opens"), new.clone()));
        assert_eq!(
            directory.address_of(&me.public_key()),
            Some("new.example:3333")
        );
        // Replaying the older one changes nothing.
        assert!(!directory.note(open(&old).expect("opens"), old));
        assert_eq!(
            directory.address_of(&me.public_key()),
            Some("new.example:3333")
        );
        assert_eq!(directory.len(), 1);
    }

    #[test]
    fn a_statement_from_the_same_epoch_does_not_flap() {
        // Two statements from one epoch would otherwise let whoever spoke last decide,
        // which makes a directory depend on arrival order.
        let me = identity(1);
        let mut directory = Directory::new();
        let first = stated(&me, "a.example:3333", 5);
        let second = stated(&me, "b.example:3333", 5);
        assert!(directory.note(open(&first).expect("opens"), first));
        assert!(!directory.note(open(&second).expect("opens"), second));
        assert_eq!(directory.address_of(&me.public_key()), Some("a.example:3333"));
    }

    #[test]
    fn the_frame_is_kept_so_it_can_be_passed_on_unchanged() {
        let me = identity(1);
        let frame = stated(&me, "node.example:3333", 1);
        let (directory, _) = Directory::from_frames(std::slice::from_ref(&frame));
        assert_eq!(directory.frame_for(&me.public_key()), Some(frame.as_slice()));
    }

    #[test]
    fn an_address_longer_than_the_limit_is_refused_at_both_ends() {
        let me = identity(1);
        let long = "a".repeat(MAX_ADDRESS_LEN + 1);
        let refused = Whereabouts::of(&me, long, Epoch(1)).seal(&me);
        assert!(matches!(refused, Err(wire::Error::TooLong { .. })));
    }

    #[test]
    fn a_directory_is_built_from_whatever_arrives_and_rubbish_is_counted() {
        let frames = vec![
            stated(&identity(1), "one.example:3333", 1),
            stated(&identity(2), "two.example:3333", 1),
            b"not a statement".to_vec(),
        ];
        let (directory, unreadable) = Directory::from_frames(&frames);
        assert_eq!(directory.len(), 2);
        assert_eq!(unreadable, 1);
        assert_eq!(directory.entries().count(), 2);
    }

    #[test]
    fn a_padded_statement_does_not_open() {
        let me = identity(1);
        let mut frame = stated(&me, "node.example:3333", 1);
        frame.push(0);
        assert_eq!(open(&frame), Err(wire::Error::TrailingBytes { extra: 1 }));
    }
}
