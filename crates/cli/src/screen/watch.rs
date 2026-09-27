//! One reading of everything the screen shows, taken off this node's own disk.
//!
//! Taken all at once and drawn from, rather than read while drawing: the screen is
//! redrawn whenever a line arrives or a second passes, and reading the whole window
//! that often would have a node spending its life answering its own screen. The
//! countdown is the exception — it is arithmetic on the clock and costs nothing.
//!
//! NOTHING HERE IS ASKED OF ANYBODY. Every number is what this one node has seen. The
//! machine next to it is looking at different numbers and neither of them is wrong.

use std::collections::BTreeSet;

use n333_core::extinction::Vigil;
use n333_core::presence::{self, Standing};
use n333_core::signal::Tally;
use n333_core::{Epoch, enrollment};

use crate::node::Node;

/// What this node had to say about itself at one moment.
pub(super) struct Watch {
    /// This node's name.
    pub(super) name: String,
    /// The epoch it was taken in.
    pub(super) epoch: Epoch,
    /// Whether this node has the file.
    pub(super) has_the_file: bool,
    /// Everyone this node holds a signed word from, this epoch or the last.
    pub(super) answering: usize,
    /// How many are on its roll.
    pub(super) roll: usize,
    /// How many nodes it knows an address for.
    pub(super) addresses: usize,
    /// How many statements others signed about it are held.
    pub(super) witnessed: usize,
    /// Where this node stands, and how it got there.
    pub(super) standing: Where,
    /// What was said this epoch, in index order, and what this node said.
    pub(super) said: Said,
    /// Whether anybody is here, and what is left if nobody is.
    pub(super) vigil: Vigil,
    /// Statements under this node's key that it did not make.
    pub(super) copies: Vec<crate::node::sources::Sighting>,
    /// Whether it holds the file and has been answering for a while, and nobody has
    /// ever signed anything about it: what `serve` says as `unseen` when it starts.
    pub(super) unseen: bool,
    /// How many of the addresses it holds were first heard of each way.
    pub(super) known: crate::node::sources::Counts,
}

/// Where this node stands, which is three different sentences.
pub(super) enum Where {
    /// Nobody has handed it the file.
    OnNobodysRoll,
    /// Admitted, and not yet counted.
    Waiting {
        /// The epoch somebody handed it the file.
        joined: Epoch,
        /// The first epoch its record covers.
        counted_from: Epoch,
    },
    /// Counted, with what its own record says.
    Counted {
        /// Present in this many of the epochs its record covers.
        standing: Standing,
        /// How many epochs of the window its record says nothing about.
        silent_on: u64,
    },
}

/// The shape of what everybody said this epoch.
pub(super) struct Said {
    /// Signal, how many said it, its share in per-mille, and whether it reached a third.
    pub(super) rows: Vec<(u16, u64, Option<u64>, bool)>,
    /// How many of the ones this node can see spoke.
    pub(super) spoken: u64,
    /// How many it can see.
    pub(super) observed: u64,
    /// What this node itself said, if it has.
    pub(super) mine: Option<u16>,
}

impl Watch {
    /// Read everything once.
    ///
    /// # Errors
    /// Fails if the node's own files cannot be read.
    pub(super) async fn of(node: &Node, now: Epoch) -> anyhow::Result<Self> {
        let answering = node.answering(now).await?;
        let me = node.identity().public_key();
        let mut everyone: BTreeSet<[u8; 32]> = answering.clone();
        everyone.insert(me);

        let heard = node.overheard(now).await?;
        let tally = Tally::of(heard.against(everyone.iter()));
        let said = Said {
            rows: tally
                .distribution()
                .filter(|(_, count)| *count > 0)
                .map(|(signal, count)| {
                    (
                        signal.index(),
                        count,
                        tally.share(signal),
                        tally.reached(signal),
                    )
                })
                .collect(),
            spoken: tally.spoken(),
            observed: tally.observed(),
            mine: heard.of(&me).map(n333_core::signal::Signal::index),
        };

        Ok(Self {
            name: node.identity().node_id().to_string(),
            epoch: now,
            has_the_file: node.subject().await.is_some(),
            answering: answering.len(),
            roll: node.roll().await.len(),
            addresses: node.where_others_are().await.len(),
            witnessed: node.witnessed().await,
            standing: stands(node, now).await?,
            said,
            vigil: node.watched(now).await?,
            copies: node.copies().await,
            unseen: crate::commands::unseen_now(node).await,
            known: node.known().await,
        })
    }
}

#[cfg(test)]
impl Watch {
    /// A node nobody has handed the file, that has seen nothing, in epoch 9.
    pub(super) fn quiet(copies: Vec<crate::node::sources::Sighting>) -> Self {
        Self {
            name: "333".into(),
            epoch: Epoch(9),
            has_the_file: false,
            answering: 0,
            roll: 0,
            addresses: 0,
            witnessed: 0,
            standing: Where::OnNobodysRoll,
            said: Said {
                rows: Vec::new(),
                spoken: 0,
                observed: 0,
                mine: None,
            },
            vigil: Vigil::new(),
            copies,
            unseen: false,
            known: crate::node::sources::Counts::default(),
        }
    }
}

/// Which of the three sentences about this node is the true one.
async fn stands(node: &Node, now: Epoch) -> anyhow::Result<Where> {
    let Some(joined) = node.joined_in().await else {
        return Ok(Where::OnNobodysRoll);
    };
    let counted_from = enrollment::active_from(joined);
    if now.0 < counted_from.0 {
        return Ok(Where::Waiting {
            joined,
            counted_from,
        });
    }
    let record = node.own_record().await?;
    let standing = presence::standing_at(now, record.iter().copied());
    let window = presence::window(now);
    let written = record
        .iter()
        .filter(|(epoch, _)| window.contains(&epoch.0))
        .count();
    Ok(Where::Counted {
        standing,
        silent_on: presence::WINDOW_EPOCHS.saturating_sub(written as u64),
    })
}
