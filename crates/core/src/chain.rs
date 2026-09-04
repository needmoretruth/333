//! A node's own record: one entry per epoch, each linked to the one before it.
//!
//! FROZEN. The field order is the wire format and the domain is part of the
//! protocol's identity.
//!
//! WHAT THE LINKING BUYS, AND WHAT IT DOES NOT. Each entry names the digest of the
//! one before it, so changing anything in the middle changes every digest after it
//! and the chain stops verifying. On its own that stops nothing: a node can throw the
//! whole chain away and sign a new one from scratch, and a fresh chain is as valid as
//! any other. What makes it costly is that every answer this node ever gave committed
//! to its head and length at that moment ([`crate::challenge::Answer`]), so a rewritten
//! chain contradicts signed statements other people are holding. The chain is the
//! record; the answers are what anchor it.
//!
//! WHAT AN ENTRY DOES NOT CARRY. Not the attestations that produced the verdict —
//! only a digest over them. Keeping them all would make an entry grow with the size
//! of the network and would put another node's bytes inside this node's record, and
//! the raw statements are kept separately and discarded after the window closes. The
//! consequence is written down rather than hidden: once the raw statements are gone,
//! a reader can check that this node signed the verdict and that nothing has been
//! altered since, and cannot re-derive the verdict from evidence. That is a real
//! limit and there is no version of this that does not have it — the design says the
//! same thing about the witnesses themselves, who will have left the network long
//! before anyone reads a decade-old chain.

use serde::{Deserialize, Serialize};

use crate::epoch::Epoch;
use crate::heartbeat::PROTOCOL_VERSION;
use crate::identity::{Identity, NodeId, parse_public_key};
use crate::presence::Attendance;
use crate::subject::digest_of;
use crate::wire::{self, DOMAIN_LEN};

/// The domain an entry is signed under. FROZEN.
pub const DOMAIN_ENTRY: &[u8; DOMAIN_LEN] = b"333.v1.chainlink";

/// What the first entry names as its predecessor: nothing.
pub const NO_PREVIOUS: [u8; 32] = [0; 32];

/// One epoch of one node's record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    /// Wire protocol version.
    pub protocol: u16,
    /// Whose record this is. Every entry names it, so a chain cannot be assembled
    /// out of entries from two nodes without the mismatch being visible.
    pub author: [u8; 32],
    /// Position in the chain, from zero.
    pub index: u64,
    /// The digest of the previous entry's frame, or [`NO_PREVIOUS`] at index zero.
    pub previous: [u8; 32],
    /// The epoch this entry judges. Strictly greater than the previous entry's, so
    /// one epoch cannot be recorded twice with two different verdicts.
    pub epoch: u64,
    /// What the node concluded about that epoch.
    pub attendance: Attendance,
    /// A digest over the statements the verdict was read from, in the order they
    /// were sorted. Zero when there were none.
    pub evidence: [u8; 32],
}

impl Entry {
    /// Compose the entry that follows `head`.
    ///
    /// `head` is `None` for the first entry of a chain.
    #[must_use]
    pub fn following(
        head: Option<&Head>,
        author: &Identity,
        epoch: Epoch,
        attendance: Attendance,
        evidence: [u8; 32],
    ) -> Self {
        Self {
            protocol: PROTOCOL_VERSION,
            author: author.public_key(),
            index: head.map_or(0, |h| h.length),
            previous: head.map_or(NO_PREVIOUS, |h| h.digest),
            epoch: epoch.0,
            attendance,
            evidence,
        }
    }

    /// The epoch this entry judges.
    #[must_use]
    pub const fn epoch(&self) -> Epoch {
        Epoch(self.epoch)
    }

    /// Encode and sign.
    ///
    /// # Errors
    /// Fails if the encoding is impossible or the frame would exceed the wire limit.
    pub fn seal(&self, author: &Identity) -> Result<Vec<u8>, wire::Error> {
        let body = postcard::to_stdvec(self).map_err(|e| wire::Error::Decode(e.to_string()))?;
        wire::seal(DOMAIN_ENTRY, &body, author)
    }
}

/// A digest over a set of statements, for an entry to point at.
///
/// The frames are sorted before hashing, so two nodes that received the same
/// statements in different orders commit to the same digest.
#[must_use]
pub fn evidence_digest(frames: &[Vec<u8>]) -> [u8; 32] {
    if frames.is_empty() {
        return NO_PREVIOUS;
    }
    let mut sorted: Vec<&Vec<u8>> = frames.iter().collect();
    sorted.sort_unstable();
    let mut input = Vec::new();
    for frame in sorted {
        // Length-prefixed, so two frames cannot be run together into a third
        // arrangement that hashes the same.
        input.extend_from_slice(&(frame.len() as u64).to_be_bytes());
        input.extend_from_slice(frame);
    }
    digest_of(&input)
}

/// Where a chain currently ends: what an answer commits to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Head {
    /// The digest of the last entry's frame, or zeros for an empty chain.
    pub digest: [u8; 32],
    /// How many entries the chain holds.
    pub length: u64,
}

/// An entry that arrived with a signature that checks out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedEntry {
    /// What was recorded.
    pub entry: Entry,
    /// The name derived from the author's key.
    pub author: NodeId,
    /// The digest of this entry's frame: what the next entry names.
    pub digest: [u8; 32],
}

/// Read one entry frame.
///
/// # Errors
/// Fails if the frame is malformed, the version is unknown, the author's key is
/// unusable, or the signature does not match.
pub fn open(frame: &[u8]) -> Result<SignedEntry, wire::Error> {
    let (signature, body) = wire::split(frame)?;
    let entry: Entry = wire::decode(body)?;
    if entry.protocol != PROTOCOL_VERSION {
        return Err(wire::Error::Version {
            got: entry.protocol,
            expected: PROTOCOL_VERSION,
        });
    }
    parse_public_key(&entry.author)?;
    wire::check_signature(DOMAIN_ENTRY, body, signature, &entry.author)?;
    Ok(SignedEntry {
        author: NodeId::from_public_key(&entry.author),
        digest: digest_of(frame),
        entry,
    })
}

/// Why a sequence of entries is not a chain.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Broken {
    /// An entry does not open, or its signature does not check out.
    #[error("entry {index}: {source}")]
    Entry {
        /// Which position in the sequence.
        index: u64,
        /// What was wrong with it.
        source: wire::Error,
    },
    /// Two different nodes' entries in one chain.
    #[error("entry {index} was written by a different node")]
    DifferentAuthor {
        /// Which position.
        index: u64,
    },
    /// The positions do not run from zero without gaps.
    #[error("entry at position {position} says it is number {claimed}")]
    OutOfOrder {
        /// Where it actually sits.
        position: u64,
        /// What it claims.
        claimed: u64,
    },
    /// An entry names a predecessor that is not the entry before it.
    #[error("entry {index} does not follow the one before it")]
    NotLinked {
        /// Which position.
        index: u64,
    },
    /// Epochs do not increase.
    ///
    /// Two verdicts about one epoch, or a chain that goes backwards in time, is a
    /// node writing its history twice.
    #[error("entry {index} judges epoch {epoch}, not after {previous}")]
    EpochNotAfter {
        /// Which position.
        index: u64,
        /// The epoch it judges.
        epoch: u64,
        /// The epoch before it.
        previous: u64,
    },
}

/// Check a whole chain and say where it ends.
///
/// Everything is checked against the entries themselves: one author throughout,
/// positions from zero without gaps, each entry naming the digest of the one before
/// it, epochs strictly increasing, and every signature. A reader needs nothing but
/// the frames.
///
/// # Errors
/// Fails at the first entry that does not fit, naming which one.
pub fn verify(frames: &[Vec<u8>]) -> Result<Head, Broken> {
    let mut head = Head::default();
    let mut author: Option<[u8; 32]> = None;
    let mut previous_epoch: Option<u64> = None;

    for (position, frame) in frames.iter().enumerate() {
        let position = position as u64;
        let signed = open(frame).map_err(|source| Broken::Entry {
            index: position,
            source,
        })?;
        let entry = &signed.entry;

        match author {
            None => author = Some(entry.author),
            Some(first) if first != entry.author => {
                return Err(Broken::DifferentAuthor { index: position });
            }
            Some(_) => {}
        }
        if entry.index != position {
            return Err(Broken::OutOfOrder {
                position,
                claimed: entry.index,
            });
        }
        if entry.previous != head.digest {
            return Err(Broken::NotLinked { index: position });
        }
        if let Some(previous) = previous_epoch
            && entry.epoch <= previous
        {
            return Err(Broken::EpochNotAfter {
                index: position,
                epoch: entry.epoch,
                previous,
            });
        }
        previous_epoch = Some(entry.epoch);
        head = Head {
            digest: signed.digest,
            length: position + 1,
        };
    }
    Ok(head)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity(seed: u8) -> Identity {
        Identity::from_seed(&[seed; 32])
    }

    /// Build a chain of `epochs`, all Present, and return the frames.
    fn chain_of(author: &Identity, epochs: &[u64]) -> Vec<Vec<u8>> {
        let mut frames = Vec::new();
        let mut head: Option<Head> = None;
        for epoch in epochs {
            let entry = Entry::following(
                head.as_ref(),
                author,
                Epoch(*epoch),
                Attendance::Present,
                NO_PREVIOUS,
            );
            let frame = entry.seal(author).expect("seals");
            head = Some(Head {
                digest: digest_of(&frame),
                length: frames.len() as u64 + 1,
            });
            frames.push(frame);
        }
        frames
    }

    #[test]
    fn the_frozen_values_are_the_agreed_ones() {
        assert_eq!(DOMAIN_ENTRY, b"333.v1.chainlink");
        assert_eq!(DOMAIN_ENTRY.len(), DOMAIN_LEN);
        assert_eq!(NO_PREVIOUS, [0_u8; 32]);
    }

    #[test]
    fn an_empty_chain_has_the_head_a_node_with_no_record_reports() {
        // The same zeros and zero an answer carries before anything has happened.
        let head = verify(&[]).expect("an empty chain is a chain");
        assert_eq!(head, Head::default());
        assert_eq!(head.digest, [0_u8; 32]);
        assert_eq!(head.length, 0);
    }

    #[test]
    fn a_chain_verifies_and_its_head_is_what_an_answer_would_carry() {
        let me = identity(1);
        let frames = chain_of(&me, &[10, 11, 12]);
        let head = verify(&frames).expect("verifies");
        assert_eq!(head.length, 3);
        assert_eq!(head.digest, digest_of(frames.last().expect("non-empty")));
    }

    #[test]
    fn changing_anything_in_the_middle_breaks_everything_after_it() {
        let me = identity(1);
        let mut frames = chain_of(&me, &[10, 11, 12]);
        // Re-sign entry 1 with a different verdict. Its own signature is valid; the
        // entry after it still names the digest of the old one.
        let replacement = Entry {
            protocol: PROTOCOL_VERSION,
            author: me.public_key(),
            index: 1,
            previous: digest_of(&frames[0]),
            epoch: 11,
            attendance: Attendance::Absent,
            evidence: NO_PREVIOUS,
        };
        frames[1] = replacement.seal(&me).expect("seals");
        assert_eq!(verify(&frames), Err(Broken::NotLinked { index: 2 }));
    }

    #[test]
    fn an_entry_removed_from_the_middle_is_caught() {
        let me = identity(1);
        let mut frames = chain_of(&me, &[10, 11, 12]);
        frames.remove(1);
        assert_eq!(
            verify(&frames),
            Err(Broken::OutOfOrder {
                position: 1,
                claimed: 2
            })
        );
    }

    #[test]
    fn one_epoch_cannot_be_judged_twice() {
        // Two verdicts about the same epoch is a node writing its history twice, and
        // it is caught even though both entries are correctly signed and linked.
        let me = identity(1);
        let frames = chain_of(&me, &[10, 10]);
        assert_eq!(
            verify(&frames),
            Err(Broken::EpochNotAfter {
                index: 1,
                epoch: 10,
                previous: 10
            })
        );
        let backwards = chain_of(&me, &[10, 9]);
        assert!(matches!(
            verify(&backwards),
            Err(Broken::EpochNotAfter { .. })
        ));
    }

    #[test]
    fn gaps_between_epochs_are_fine() {
        // A node that was off for a month records the epochs it judged, not one per
        // epoch that passed. Only the order has to hold.
        let me = identity(1);
        assert!(verify(&chain_of(&me, &[10, 400, 90_000])).is_ok());
    }

    #[test]
    fn two_nodes_entries_do_not_make_one_chain() {
        let (me, them) = (identity(1), identity(2));
        let mut frames = chain_of(&me, &[10]);
        let theirs = Entry {
            protocol: PROTOCOL_VERSION,
            author: them.public_key(),
            index: 1,
            previous: digest_of(&frames[0]),
            epoch: 11,
            attendance: Attendance::Present,
            evidence: NO_PREVIOUS,
        };
        frames.push(theirs.seal(&them).expect("seals"));
        assert_eq!(verify(&frames), Err(Broken::DifferentAuthor { index: 1 }));
    }

    #[test]
    fn an_entry_signed_by_somebody_else_does_not_open() {
        let (me, them) = (identity(1), identity(2));
        let entry = Entry::following(None, &me, Epoch(1), Attendance::Present, NO_PREVIOUS);
        // `them` signs an entry claiming `me` wrote it.
        let frame = entry.seal(&them).expect("seals");
        assert_eq!(open(&frame), Err(wire::Error::BadSignature));
    }

    #[test]
    fn a_padded_entry_does_not_open() {
        let me = identity(1);
        let mut frame = Entry::following(None, &me, Epoch(1), Attendance::Present, NO_PREVIOUS)
            .seal(&me)
            .expect("seals");
        frame.push(0);
        assert_eq!(open(&frame), Err(wire::Error::TrailingBytes { extra: 1 }));
    }

    #[test]
    fn the_evidence_digest_does_not_depend_on_the_order_things_arrived_in() {
        let a = b"first statement".to_vec();
        let b = b"second statement".to_vec();
        assert_eq!(
            evidence_digest(&[a.clone(), b.clone()]),
            evidence_digest(&[b.clone(), a.clone()])
        );
        assert_ne!(
            evidence_digest(std::slice::from_ref(&a)),
            evidence_digest(&[a, b])
        );
        assert_eq!(evidence_digest(&[]), NO_PREVIOUS);
    }

    #[test]
    fn two_statements_cannot_be_rearranged_into_a_third_with_the_same_digest() {
        // Without the length prefix, ["ab", "c"] and ["a", "bc"] hash identically and
        // a node could claim to have read evidence it never had.
        let split_one = vec![b"ab".to_vec(), b"c".to_vec()];
        let split_two = vec![b"a".to_vec(), b"bc".to_vec()];
        assert_ne!(evidence_digest(&split_one), evidence_digest(&split_two));
    }

    #[test]
    fn a_chain_carries_the_verdicts_a_reader_needs() {
        let me = identity(1);
        let mut frames = Vec::new();
        let mut head: Option<Head> = None;
        for (epoch, attendance) in [
            (10, Attendance::Present),
            (11, Attendance::Absent),
            (12, Attendance::Excluded),
        ] {
            let entry =
                Entry::following(head.as_ref(), &me, Epoch(epoch), attendance, NO_PREVIOUS);
            let frame = entry.seal(&me).expect("seals");
            head = Some(Head {
                digest: digest_of(&frame),
                length: frames.len() as u64 + 1,
            });
            frames.push(frame);
        }
        assert!(verify(&frames).is_ok());
        let read: Vec<_> = frames
            .iter()
            .map(|f| open(f).expect("opens").entry.attendance)
            .collect();
        assert_eq!(
            read,
            vec![
                Attendance::Present,
                Attendance::Absent,
                Attendance::Excluded
            ]
        );
    }
}
