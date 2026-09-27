//! Where each address this node holds came from, for the person who runs it.
//!
//! Every address arrives one of four ways: somebody typed it, this network announced
//! it, a meeting point had it on its board, or another node handed it over. None of
//! that changes what an address is worth — whoever answers there proves who they are
//! by holding a key — and none of it goes on the wire. It is kept because a node that
//! is supposed to be depending less on one fixed address has to be able to say how
//! much it still does, and the only way to say it is to have written it down.
//!
//! Two more things live here because they are the same kind of note: the whereabouts
//! this node signed itself, and any statement under this node's key that it did not
//! sign. The second is somebody else running this node's name.
//!
//! KEPT FOR THE WINDOW AND NO LONGER. An address last heard of more than
//! [`WINDOW_EPOCHS`] ago is forgotten with everything else about those epochs, so this
//! is bounded by what the node heard in the window, not by how long it has run.

mod file;

use std::collections::{BTreeMap, BTreeSet};

use n333_core::Epoch;
use n333_core::presence::WINDOW_EPOCHS;
use serde::{Deserialize, Serialize};

pub(crate) use file::{Loaded, load, save, typed};

/// How an address reached this node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Source {
    /// Typed by the person running it: an invitation given to `ping` or `join`, on the
    /// command line or in the screen.
    ByHand,
    /// Announced on the network this machine is on.
    ThisNetwork,
    /// Read off the board at a meeting point.
    MeetingPoint {
        /// Which one.
        place: String,
    },
    /// Handed over by another node in a trade.
    Peer {
        /// That node's name.
        name: String,
    },
}

impl std::fmt::Display for Source {
    /// How it reached this node, said so that it follows "it arrived".
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ByHand => write!(f, "by hand"),
            Self::ThisNetwork => write!(f, "on this network"),
            Self::MeetingPoint { place } => write!(f, "at the meeting point {place}"),
            Self::Peer { name } => write!(f, "from {}", crate::commands::shorten(name)),
        }
    }
}

/// One occasion an address was heard of.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Heard {
    /// From where.
    pub(crate) from: Source,
    /// In which epoch, by this node's clock.
    pub(crate) epoch: u64,
}

/// What is known about where one address came from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Learned {
    /// The name that signed for this address or answered at it, once one has.
    pub(crate) name: Option<String>,
    /// The first time it was heard of.
    pub(crate) first: Heard,
    /// The latest time.
    pub(crate) last: Heard,
}

/// A statement under this node's key that this node did not make.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Sighting {
    /// Where it says this node is.
    pub(crate) address: String,
    /// The epoch it was signed for.
    pub(crate) said_in: u64,
    /// How it reached this node, and when.
    pub(crate) heard: Heard,
}

impl Sighting {
    /// How it arrived, said so that it follows "it arrived".
    ///
    /// A copy that knocks on this node hands its statement over itself, under this
    /// node's own name, `me`, and that is said as the thing it is.
    pub(crate) fn arrived(&self, me: &str) -> String {
        match &self.heard.from {
            Source::Peer { name } if name == me => "from the other copy itself".to_owned(),
            other => other.to_string(),
        }
    }
}

/// Whether a statement under this node's own key is one it made.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mine {
    /// It is: this node signed exactly that.
    Yes,
    /// It is not, and it is recent enough that this node would have written it down.
    No,
    /// From before this node kept count, so nothing here can say.
    CannotTell,
}

/// Everything this node has noted about where things came from.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Sources {
    /// The epoch this node began writing down what it signed. Anything under its key
    /// from that epoch or earlier cannot be told apart from what it said before.
    kept_since: u64,
    /// Every address heard of in the window, by address.
    addresses: BTreeMap<String, Learned>,
    /// Every whereabouts this node signed in the window: the epoch and the address.
    said: BTreeSet<(u64, String)>,
    /// Statements under this node's key that it did not make.
    sightings: Vec<Sighting>,
}

impl Sources {
    /// Nothing noted yet, keeping count from `now`.
    pub(crate) fn from(now: Epoch) -> Self {
        Self {
            kept_since: now.0,
            ..Self::default()
        }
    }

    /// Note that `address` was heard of, and by which name if one signed for it.
    ///
    /// Says whether the address was new.
    pub(crate) fn heard(&mut self, address: &str, name: Option<String>, heard: Heard) -> bool {
        match self.addresses.get_mut(address) {
            Some(learned) => {
                // The latest to arrive is the one heard last. The epoch is what is
                // shown beside it, and never what decides.
                learned.last = heard;
                if name.is_some() {
                    learned.name = name;
                }
                false
            }
            None => {
                let learned = Learned {
                    name,
                    first: heard.clone(),
                    last: heard,
                };
                self.addresses.insert(address.to_owned(), learned);
                true
            }
        }
    }

    /// Put a name to an address, because whoever answered there holds that key.
    pub(crate) fn answered(&mut self, address: &str, name: String) {
        if let Some(learned) = self.addresses.get_mut(address) {
            learned.name = Some(name);
        }
    }

    /// Note a whereabouts this node signed.
    pub(crate) fn said(&mut self, address: &str, epoch: Epoch) {
        self.said.insert((epoch.0, address.to_owned()));
    }

    /// Did this node sign that it was at `address` in `epoch`?
    pub(crate) fn is_mine(&self, address: &str, epoch: Epoch) -> Mine {
        if self.said.contains(&(epoch.0, address.to_owned())) {
            Mine::Yes
        } else if epoch.0 <= self.kept_since {
            Mine::CannotTell
        } else {
            Mine::No
        }
    }

    /// Note a statement under this node's key that it did not make.
    ///
    /// Says whether it is one not seen before, which is when it is worth saying.
    pub(crate) fn sighted(&mut self, sighting: Sighting) -> bool {
        let held = self
            .sightings
            .iter_mut()
            .find(|held| held.address == sighting.address && held.said_in == sighting.said_in);
        match held {
            Some(held) => {
                held.heard = sighting.heard;
                false
            }
            None => {
                self.sightings.push(sighting);
                true
            }
        }
    }

    /// Where an address came from, if it was noted.
    pub(crate) fn of(&self, address: &str) -> Option<&Learned> {
        self.addresses.get(address)
    }

    /// Every address somebody typed, with the name that answered there if one has.
    pub(crate) fn given_by_hand(&self) -> impl Iterator<Item = (&str, &Learned)> {
        self.addresses
            .iter()
            .filter(|(_, learned)| learned.first.from == Source::ByHand)
            .map(|(address, learned)| (address.as_str(), learned))
    }

    /// Every address first heard of because this network announced it.
    pub(crate) fn heard_on_this_network(&self) -> impl Iterator<Item = &str> {
        self.addresses
            .iter()
            .filter(|(_, learned)| learned.first.from == Source::ThisNetwork)
            .map(|(address, _)| address.as_str())
    }

    /// Every statement seen under this node's key that it did not make.
    pub(crate) fn sightings(&self) -> &[Sighting] {
        &self.sightings
    }

    /// Forget whatever was last heard of before the window.
    pub(crate) fn prune(&mut self, now: Epoch) {
        let oldest = now.0.saturating_sub(WINDOW_EPOCHS);
        self.addresses
            .retain(|_, learned| learned.last.epoch >= oldest);
        self.said.retain(|(epoch, _)| *epoch >= oldest);
        self.sightings
            .retain(|sighting| sighting.heard.epoch >= oldest);
    }

    /// Take in what another copy of this record holds.
    ///
    /// Another process on this directory — `333 ping` while the vigil is running —
    /// writes the same file, so what is on disk is folded in before it is written
    /// over. The earliest first hearing and the latest last one win, and nothing
    /// either side noted is dropped.
    pub(crate) fn merge(&mut self, other: Self) {
        self.kept_since = self.kept_since.min(other.kept_since);
        for (address, theirs) in other.addresses {
            match self.addresses.get_mut(&address) {
                Some(ours) => {
                    if theirs.first.epoch < ours.first.epoch {
                        ours.first = theirs.first;
                    }
                    if theirs.last.epoch > ours.last.epoch {
                        ours.last = theirs.last;
                    }
                    if ours.name.is_none() {
                        ours.name = theirs.name;
                    }
                }
                None => {
                    self.addresses.insert(address, theirs);
                }
            }
        }
        self.said.extend(other.said);
        for sighting in other.sightings {
            self.sighted(sighting);
        }
    }
}

/// How many of the addresses held came each way, counted by how each was first heard.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub(crate) struct Counts {
    /// Typed by the person running this node.
    pub(crate) by_hand: usize,
    /// Announced on this network.
    pub(crate) this_network: usize,
    /// Read at a meeting point.
    pub(crate) meeting_point: usize,
    /// Handed over by other nodes.
    pub(crate) from_peers: usize,
    /// How many different nodes handed those over.
    pub(crate) peers: usize,
    /// Held from before this node wrote down where things came from.
    pub(crate) unrecorded: usize,
}

impl Counts {
    /// Count `held` by where each was first heard.
    pub(crate) fn of<'a>(sources: &Sources, held: impl Iterator<Item = &'a str>) -> Self {
        let mut counts = Self::default();
        let mut peers = BTreeSet::new();
        for address in held {
            match sources.of(address).map(|learned| &learned.first.from) {
                None => counts.unrecorded += 1,
                Some(Source::ByHand) => counts.by_hand += 1,
                Some(Source::ThisNetwork) => counts.this_network += 1,
                Some(Source::MeetingPoint { .. }) => counts.meeting_point += 1,
                Some(Source::Peer { name }) => {
                    counts.from_peers += 1;
                    peers.insert(name.as_str());
                }
            }
        }
        counts.peers = peers.len();
        counts
    }
}
