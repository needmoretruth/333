//! Whether a node is reaching out and never being reached.
//!
//! What `serve` says as `unseen` when it starts, what `status --json` carries, and what
//! the hourly service check raises. Kept apart so that the three read one condition.

use crate::node::Node;

/// How many epochs of silence mean something is wrong rather than nothing has happened.
///
/// Three. Every member is asked by three verifiers every epoch, so a reachable node has
/// something signed about it within one. Three is long enough that a quiet start, a
/// restart, or an epoch where the people who drew this node were themselves asleep does
/// not raise it.
const EPOCHS_BEFORE_SILENCE_MEANS_SOMETHING: u64 = 3;

/// Whether this node is on the roll and has never had anything signed about it.
///
/// Only after a few epochs, and only when there is somebody who could have asked.
/// One function, because `serve` says it when it starts and the hourly check says it
/// when nobody is reading what `serve` said, and the two must not disagree.
pub(crate) const fn unseen(opened: &crate::node::Opened) -> bool {
    unseen_by(
        opened.has_the_file,
        opened.witnessed,
        opened.chain_length,
        opened.members,
    )
}

/// The same, for a node that is already open: the vigil's own, asked by `status --json`
/// and so by the hourly check while the vigil holds the directory.
pub(crate) async fn unseen_now(node: &Node) -> bool {
    unseen_by(
        node.subject().await.is_some(),
        node.witnessed().await,
        node.head().await.length,
        node.roll().await.len(),
    )
}

/// The condition itself, from the four things it is read from.
const fn unseen_by(
    has_the_file: bool,
    witnessed: usize,
    chain_length: u64,
    members: usize,
) -> bool {
    has_the_file
        && witnessed == 0
        && chain_length >= EPOCHS_BEFORE_SILENCE_MEANS_SOMETHING
        && members >= 2
}
