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
use n333_core::{Epoch, enrollment, epoch};

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
        })
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

/// How long until the next epoch begins, in seconds.
pub(super) fn to_the_boundary(now: Epoch) -> u64 {
    Epoch(now.0.saturating_add(1))
        .starts_at_unix_seconds()
        .saturating_sub(epoch::unix_now_seconds())
}

/// A span of seconds, said the way a person waiting would say it.
pub(super) fn until(seconds: u64) -> String {
    let (hours, minutes) = (seconds / 3600, (seconds % 3600) / 60);
    if hours == 0 {
        return format!("{minutes}m {}s", seconds % 60);
    }
    format!("{hours}h {minutes:02}m")
}
