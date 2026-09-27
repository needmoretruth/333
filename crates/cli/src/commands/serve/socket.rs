//! The socket listener, kept apart from the onion one so that each way in can be read
//! and changed without the other's feature flags in the way.

use std::sync::Arc;

use anyhow::Context as _;
use n333_net::direct;

use crate::node::Node;

use super::door::{Caller, Door, spawn_exchange};

/// Answer every peer that opens a socket to this node.
pub(super) async fn answer_direct(
    listener: direct::Listener,
    node: Arc<Node>,
    door: Door,
) -> anyhow::Result<()> {
    loop {
        let (stream, from) = listener.accept().await.context("accepting a peer")?;
        // A peer's address is not a name and is not recorded; it is shown so that the
        // operator of this node can see who is reaching it right now, and counted so
        // that one caller cannot be everybody at the door.
        spawn_exchange(stream, &node, &door, Caller::At(from));
    }
}
