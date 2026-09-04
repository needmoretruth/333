//! One node saying one of the 333 things, once in an epoch.
//!
//! FROZEN. The field order is the wire format and the domain is part of the
//! protocol's identity.
//!
//! What travels is the index. Never the word — see [`crate::signal`], which holds the
//! reason and the counting. This module is only how an index gets from one node to
//! another with a name attached to it.
//!
//! WHY IT IS SIGNED AT ALL, given that nothing is decided by it. Because the count is
//! of nodes and not of messages. Unsigned, one machine could say the same thing three
//! hundred times and the distribution would show three hundred of us agreeing. Signed,
//! it can still say it three hundred times — but it needs three hundred names to do it,
//! and the network makes names cost the one thing that cannot be forged, which is
//! somebody already inside bothering to hand over the file.
//!
//! ONE PER EPOCH, AND THE FIRST ONE HEARD IS THE ONE KEPT. Not the newest: a node that
//! could replace what it said by saying it again would let the count depend on the
//! order things arrived in, and two nodes would show different distributions for a
//! reason that had nothing to do with what anybody meant.

use serde::{Deserialize, Serialize};

use crate::epoch::Epoch;
use crate::heartbeat::PROTOCOL_VERSION;
use crate::identity::{Identity, NodeId, parse_public_key};
use crate::signal::{SIGNAL_COUNT, Signal};
use crate::wire::{self, DOMAIN_LEN};

/// The domain an utterance is signed under. FROZEN.
pub const DOMAIN_SPOKE: &[u8; DOMAIN_LEN] = b"333.v1.spoke.one";

/// A node saying one of the 333 things.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Utterance {
    /// Wire protocol version.
    pub protocol: u16,
    /// Who said it.
    pub speaker: [u8; 32],
    /// The epoch they said it in.
    pub epoch: u64,
    /// Which of the 333. An index, never a word.
    pub signal: u16,
}

impl Utterance {
    /// Say one.
    #[must_use]
    pub fn of(speaker: &Identity, signal: Signal, epoch: Epoch) -> Self {
        Self {
            protocol: PROTOCOL_VERSION,
            speaker: speaker.public_key(),
            epoch: epoch.0,
            signal: signal.index(),
        }
    }

    /// The epoch it was said in.
    #[must_use]
    pub const fn epoch(&self) -> Epoch {
        Epoch(self.epoch)
    }

    /// Which of the 333, if the index is one of them.
    #[must_use]
    pub const fn signal(&self) -> Option<Signal> {
        Signal::new(self.signal)
    }

    /// Encode and sign.
    ///
    /// # Errors
    /// Fails if the encoding is impossible or the frame would exceed the wire limit.
    pub fn seal(&self, speaker: &Identity) -> Result<Vec<u8>, wire::Error> {
        let body = postcard::to_stdvec(self).map_err(|e| wire::Error::Decode(e.to_string()))?;
        wire::seal(DOMAIN_SPOKE, &body, speaker)
    }
}

/// An utterance that arrived with a signature that checks out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signed {
    /// What was said.
    pub utterance: Utterance,
    /// The name derived from the speaker's key.
    pub speaker: NodeId,
    /// Which of the 333 it was.
    pub signal: Signal,
}

/// Read an utterance frame.
///
/// # Errors
/// Fails if the frame is malformed, the version is unknown, the index is not one of
/// the 333, the key is unusable, or the signature does not match.
pub fn open(frame: &[u8]) -> Result<Signed, wire::Error> {
    let (signature, body) = wire::split(frame)?;
    let utterance: Utterance = wire::decode(body)?;
    if utterance.protocol != PROTOCOL_VERSION {
        return Err(wire::Error::Version {
            got: utterance.protocol,
            expected: PROTOCOL_VERSION,
        });
    }
    // Out of range is refused rather than carried. There is no 334th thing to say, and
    // a client that passed one along would be passing along a number that can only ever
    // be dropped later, further from whoever sent it.
    let signal = utterance.signal().ok_or(wire::Error::TooLong {
        got: usize::from(utterance.signal),
    })?;
    parse_public_key(&utterance.speaker)?;
    wire::check_signature(DOMAIN_SPOKE, body, signature, &utterance.speaker)?;
    Ok(Signed {
        speaker: NodeId::from_public_key(&utterance.speaker),
        signal,
        utterance,
    })
}

/// What one node was heard to say in one epoch, first word kept.
///
/// Not a record of what anybody meant and not a total. It is one node's notes about
/// what reached it, which is the only thing any node has.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Heard {
    /// The first utterance heard from each speaker.
    said: std::collections::BTreeMap<[u8; 32], Signal>,
}

impl Heard {
    /// Nothing heard yet.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Take one, keeping it only if this speaker has not been heard yet.
    ///
    /// Returns whether anything changed.
    pub fn take(&mut self, signed: &Signed) -> bool {
        match self.said.entry(signed.utterance.speaker) {
            std::collections::btree_map::Entry::Vacant(slot) => {
                slot.insert(signed.signal);
                true
            }
            std::collections::btree_map::Entry::Occupied(_) => false,
        }
    }

    /// What one node was heard to say.
    #[must_use]
    pub fn of(&self, speaker: &[u8; 32]) -> Option<Signal> {
        self.said.get(speaker).copied()
    }

    /// How many nodes were heard to say anything.
    #[must_use]
    pub fn speakers(&self) -> usize {
        self.said.len()
    }

    /// What each of `everyone` was heard to say, silence included.
    ///
    /// The denominator is every node this one observed, not every node that spoke.
    /// Saying nothing is a thing a node did, and a threshold read against speakers
    /// only would rise as fewer of us spoke.
    pub fn against<'a, I>(&'a self, everyone: I) -> impl Iterator<Item = Option<Signal>> + 'a
    where
        I: IntoIterator<Item = &'a [u8; 32]> + 'a,
    {
        everyone.into_iter().map(|who| self.of(who))
    }
}

/// The highest index, said in words, so that a person asked for one knows the range.
#[must_use]
pub const fn how_many() -> u16 {
    SIGNAL_COUNT
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity(seed: u8) -> Identity {
        Identity::from_seed(&[seed; 32])
    }

    fn said(who: &Identity, index: u16, epoch: u64) -> Vec<u8> {
        let signal = Signal::new(index).expect("in range");
        Utterance::of(who, signal, Epoch(epoch))
            .seal(who)
            .expect("seals")
    }

    #[test]
    fn the_frozen_values_are_the_agreed_ones() {
        assert_eq!(DOMAIN_SPOKE, b"333.v1.spoke.one");
        assert_eq!(DOMAIN_SPOKE.len(), DOMAIN_LEN);
    }

    #[test]
    fn an_index_travels_and_arrives_with_a_name_on_it() {
        let me = identity(1);
        let signed = open(&said(&me, 187, 7)).expect("opens");
        assert_eq!(signed.speaker, me.node_id());
        assert_eq!(signed.signal.index(), 187);
        assert_eq!(signed.utterance.epoch(), Epoch(7));
    }

    #[test]
    fn nobody_can_speak_in_somebody_elses_name() {
        // The whole reason it is signed: the count is of nodes, not of messages.
        let (me, them) = (identity(1), identity(2));
        let signal = Signal::new(1).expect("in range");
        let mut forged = Utterance::of(&me, signal, Epoch(1));
        forged.speaker = them.public_key();
        let frame = forged.seal(&me).expect("seals");
        assert_eq!(open(&frame), Err(wire::Error::BadSignature));
    }

    #[test]
    fn there_is_no_three_hundred_and_thirty_fourth_thing_to_say() {
        let me = identity(1);
        let signal = Signal::new(0).expect("in range");
        let mut past_the_end = Utterance::of(&me, signal, Epoch(1));
        past_the_end.signal = SIGNAL_COUNT;
        let frame = past_the_end.seal(&me).expect("seals");
        assert!(matches!(open(&frame), Err(wire::Error::TooLong { .. })));
    }

    #[test]
    fn the_first_thing_a_node_says_is_the_thing_it_said() {
        // Saying it again does not replace it. Otherwise the distribution would depend
        // on which copy reached which node first.
        let me = identity(1);
        let mut heard = Heard::new();
        assert!(heard.take(&open(&said(&me, 10, 5)).expect("opens")));
        assert!(!heard.take(&open(&said(&me, 20, 5)).expect("opens")));
        assert_eq!(heard.of(&me.public_key()).map(Signal::index), Some(10));
        assert_eq!(heard.speakers(), 1);
    }

    #[test]
    fn silence_is_counted_against_everyone_observed_and_not_only_the_speakers() {
        let (spoke, quiet) = (identity(1), identity(2));
        let mut heard = Heard::new();
        heard.take(&open(&said(&spoke, 5, 1)).expect("opens"));

        let everyone = [spoke.public_key(), quiet.public_key()];
        let against: Vec<_> = heard.against(everyone.iter()).collect();
        assert_eq!(against.len(), 2, "both, not just the one that spoke");
        assert_eq!(against.iter().filter(|said| said.is_none()).count(), 1);
    }

    #[test]
    fn a_padded_utterance_does_not_open() {
        let mut frame = said(&identity(1), 3, 1);
        frame.push(0);
        assert_eq!(open(&frame), Err(wire::Error::TrailingBytes { extra: 1 }));
    }
}
