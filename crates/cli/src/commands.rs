//! What the client can be asked to do.
//!
//! One file per command. Each one owns its own output text, because the words a
//! person reads are part of the command, not a detail of it.

pub(crate) mod id;
pub(crate) mod ping;
pub(crate) mod serve;

use std::time::Duration;

use anyhow::Context as _;
use n333_net::Exchange;

/// Options every command shares.
#[derive(Debug, Clone)]
pub(crate) struct Common {
    /// Where this node keeps its files.
    pub(crate) paths: crate::paths::NodePaths,
    /// How long to wait for a step that talks to the Tor network.
    pub(crate) timeout: Duration,
    /// Whether to accept state directories other users can read.
    pub(crate) trust_directory_permissions: bool,
}

/// Start a Tor client, giving up after the shared timeout.
///
/// Arti retries bootstrap 128 times by default, so without a deadline a broken
/// network hangs instead of failing.
///
/// # Errors
/// Fails if the timeout elapses or arti cannot start.
pub(crate) async fn bootstrap(common: &Common) -> anyhow::Result<n333_net::tor::Client> {
    println!("connecting to the Tor network...");
    tokio::time::timeout(
        common.timeout,
        n333_net::tor::bootstrap(&common.paths.tor(), common.trust_directory_permissions),
    )
    .await
    .with_context(|| format!("no Tor connection after {} s", common.timeout.as_secs()))?
    .context("starting the Tor client")
}

/// One line describing what a completed exchange showed.
#[must_use]
pub(crate) fn describe(exchange: &Exchange) -> String {
    let liveness = if exchange.proves_peer_was_live {
        "answered our nonce"
    } else {
        "spoke first"
    };
    format!(
        "peer {}  epoch {}  skew {:+}  ({liveness})",
        exchange.peer.node_id, exchange.peer.heartbeat.epoch, exchange.epoch_skew
    )
}
