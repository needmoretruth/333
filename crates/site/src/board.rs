//! The board: what nodes said about where they are, held for two epochs.
//!
//! IT IS NOT TRUSTED BY ANYTHING THAT READS IT. Every line is signed by the node it
//! names and every client checks before it believes. This board cannot invent a member,
//! forge an address or vouch for one. What it can do is disappear, which is why nothing
//! depends on it twice.
//!
//! The stored shape is the Worker's (`{"v":1,"e":[{k,b,t,p}]}`), less its count of
//! writes per day, so a board exported from the Worker loads here as it is.

use n333_core::whereabouts;
use serde::{Deserialize, Serialize};

use crate::frames;
use crate::place::Place;

/// How long a statement is held: two epochs of 333 minutes, in milliseconds.
pub(crate) const KEPT_FOR_MS: u64 = 2 * 333 * 60 * 1000;

/// The most statements the board holds. The oldest go first.
pub(crate) const MOST: usize = 333;

/// What a statement says, once its signature has been checked here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Said {
    /// The node's name, 64 lower-case hex digits.
    pub(crate) node: String,
    /// Where it said to look.
    pub(crate) address: String,
    /// The epoch it said that in.
    pub(crate) epoch: u64,
}

impl Said {
    /// Open a frame and keep what it says, if it is a signed whereabouts statement.
    pub(crate) fn of(frame: &[u8]) -> Option<Self> {
        let signed = whereabouts::open(frame).ok()?;
        Some(Self {
            node: signed.node.to_string(),
            address: signed.whereabouts.address,
            epoch: signed.whereabouts.epoch,
        })
    }
}

/// One statement on the board.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Line {
    /// The slot: the name of the node the statement is about.
    k: String,
    /// The frame, exactly as the node signed it.
    #[serde(with = "as_base64")]
    b: Vec<u8>,
    /// When this server received it, in milliseconds. Used only to forget.
    t: u64,
    /// Where it came from, as far as the edge could tell.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    p: Option<Place>,
    /// What the frame says, checked once when the line arrives or is loaded.
    #[serde(skip)]
    said: Option<Said>,
}

impl Line {
    /// A statement received at `now_ms`, already opened by whoever accepted it.
    pub(crate) fn new(said: Said, frame: Vec<u8>, now_ms: u64, place: Option<Place>) -> Self {
        Self {
            k: said.node.clone(),
            b: frame,
            t: now_ms,
            p: place,
            said: Some(said),
        }
    }

    /// The signed frame.
    pub(crate) fn frame(&self) -> &[u8] {
        &self.b
    }

    /// What it says, if its signature checks out.
    pub(crate) const fn said(&self) -> Option<&Said> {
        self.said.as_ref()
    }

    /// Where it came from.
    pub(crate) const fn place(&self) -> Option<&Place> {
        self.p.as_ref()
    }

    /// Is it young enough to hand out at `now_ms`?
    fn alive(&self, now_ms: u64) -> bool {
        self.t >= now_ms.saturating_sub(KEPT_FOR_MS)
    }
}

/// The board as it is stored.
#[derive(Serialize, Deserialize)]
struct Stored {
    /// The version of this shape.
    #[serde(default)]
    v: u32,
    /// The statements, oldest first.
    #[serde(default)]
    e: Vec<Line>,
}

/// Every statement held, oldest first.
#[derive(Debug, Default)]
pub(crate) struct Board {
    /// The statements.
    lines: Vec<Line>,
    /// How many times it has changed since it was loaded, so that a slower write of an
    /// older board never lands on top of a newer one.
    generation: u64,
}

impl Board {
    /// Read a stored board. One that cannot be read is an empty board, as it was for the
    /// Worker: two epochs from now nothing in it would have mattered anyway.
    pub(crate) fn load(bytes: &[u8]) -> Self {
        let Ok(stored) = serde_json::from_slice::<Stored>(bytes) else {
            tracing::warn!("the stored board could not be read; starting empty");
            return Self::default();
        };
        let mut lines = stored.e;
        for line in &mut lines {
            line.said = Said::of(&line.b);
        }
        Self {
            lines,
            generation: 0,
        }
    }

    /// The statements still worth handing out, oldest first.
    pub(crate) fn alive(&self, now_ms: u64) -> impl Iterator<Item = &Line> {
        self.lines.iter().filter(move |line| line.alive(now_ms))
    }

    /// Is this exact frame what the slot already holds?
    pub(crate) fn holds(&self, key: &str, frame: &[u8], now_ms: u64) -> bool {
        self.alive(now_ms)
            .any(|line| line.k == key && line.b.as_slice() == frame)
    }

    /// Put a statement in its slot, dropping the slot's old one and anything too old,
    /// and keeping the newest [`MOST`].
    pub(crate) fn put(&mut self, line: Line, now_ms: u64) {
        self.lines
            .retain(|held| held.alive(now_ms) && held.k != line.k);
        self.lines.push(line);
        let over = self.lines.len().saturating_sub(MOST);
        self.lines.drain(..over);
        self.generation += 1;
    }

    /// The board as it is stored, and which change it is.
    pub(crate) fn snapshot(&self) -> (u64, Vec<u8>) {
        let stored = serde_json::json!({ "v": 1, "e": &self.lines });
        (self.generation, stored.to_string().into_bytes())
    }

    /// Every live statement, framed the way a node reads its logs.
    ///
    /// Everything is handed over, including anything that does not verify: the client
    /// checks for itself, and a server that filters is a server that can filter
    /// somebody out.
    pub(crate) fn framed(&self, now_ms: u64) -> Vec<u8> {
        frames::join(self.alive(now_ms).map(Line::frame))
    }
}

/// A frame inside the JSON file, as base64.
mod as_base64 {
    use base64::Engine as _;
    use base64::engine::general_purpose::STANDARD;

    /// Write bytes as base64.
    pub(super) fn serialize<S: serde::Serializer>(bytes: &[u8], to: S) -> Result<S::Ok, S::Error> {
        to.serialize_str(&STANDARD.encode(bytes))
    }

    /// Read base64 back into bytes.
    pub(super) fn deserialize<'de, D: serde::Deserializer<'de>>(
        from: D,
    ) -> Result<Vec<u8>, D::Error> {
        let text: String = serde::Deserialize::deserialize(from)?;
        STANDARD.decode(text).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use n333_core::{Epoch, Identity, Whereabouts};

    fn line(seed: u8, now_ms: u64) -> Line {
        let node = Identity::from_seed(&[seed; 32]);
        let frame = Whereabouts::of(&node, "192.0.2.1:3333".to_owned(), Epoch(1))
            .seal(&node)
            .unwrap();
        Line::new(Said::of(&frame).unwrap(), frame, now_ms, None)
    }

    #[test]
    fn lines_older_than_two_epochs_disappear() {
        let mut board = Board::default();
        board.put(line(1, 1_000), 1_000);
        board.put(line(2, 2_000), 2_000);

        assert_eq!(board.alive(1_000 + KEPT_FOR_MS).count(), 2);
        assert_eq!(board.alive(1_001 + KEPT_FOR_MS).count(), 1);
        assert!(board.framed(2_001 + KEPT_FOR_MS).is_empty());

        board.put(line(3, 2_001 + KEPT_FOR_MS), 2_001 + KEPT_FOR_MS);
        let (_, stored) = board.snapshot();
        assert_eq!(
            Board::load(&stored).lines.len(),
            1,
            "the dead are not written back"
        );
    }
}
