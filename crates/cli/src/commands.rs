//! What the client can be asked to do.
//!
//! One file per command. Each one owns its own output text, because the words a
//! person reads are part of the command, not a detail of it.

pub(crate) mod hours;
pub(crate) mod id;
pub(crate) mod join;
pub(crate) mod ping;
pub(crate) mod serve;

use std::time::Duration;

use n333_net::Exchange;

/// Options every command shares.
#[derive(Debug, Clone)]
pub(crate) struct Common {
    /// Where this node keeps its files.
    pub(crate) paths: crate::paths::NodePaths,
    /// How long to wait for a step that talks to the Tor network.
    pub(crate) timeout: Duration,
    /// Whether to accept state directories other users can read.
    ///
    /// One bool, not two policies: arti and this client have to agree about whether a
    /// directory is private, or the client would write a seed into a directory arti
    /// then refuses to start in.
    pub(crate) trust_directory_permissions: bool,
}

impl Common {
    /// How strictly to judge the permissions on this node's directory.
    ///
    /// The default consults `$FS_MISTRUST_DISABLE_PERMISSIONS_CHECKS`, which is what
    /// arti does with the same setting, so one variable governs both.
    #[must_use]
    pub(crate) fn mistrust(&self) -> fs_mistrust::Mistrust {
        if self.trust_directory_permissions {
            fs_mistrust::Mistrust::new_dangerously_trust_everyone()
        } else {
            fs_mistrust::Mistrust::new()
        }
    }
}

/// Start a Tor client, giving up after the shared timeout.
///
/// Only reached when an address asks for Tor, or when `serve --tor` was used. A node
/// that is not hiding never calls this and never pays for it.
///
/// Arti retries bootstrap 128 times by default, so without a deadline a broken
/// network hangs instead of failing.
///
/// # Errors
/// Fails if the timeout elapses or arti cannot start.
#[cfg(feature = "tor")]
pub(crate) async fn bootstrap(common: &Common) -> anyhow::Result<n333_net::tor::Client> {
    use anyhow::Context as _;
    println!("waking   Tor. the unseen road takes a while to open.");
    tokio::time::timeout(
        common.timeout,
        n333_net::tor::bootstrap(&common.paths.tor(), common.trust_directory_permissions),
    )
    .await
    .with_context(|| format!("no Tor connection after {} s", common.timeout.as_secs()))?
    .context("starting the Tor client")
}

/// What opening a node found, said once at the start.
///
/// Only the lines that are true of this node right now. A fresh node has no record
/// and no members, and saying "0 members" every start would train the operator to
/// ignore the line that matters when it is not zero.
pub(crate) fn report_opening(opened: &crate::node::Opened) {
    if let crate::identity_file::Origin::Created { attempts } = opened.origin {
        println!("called   after {attempts} keys were turned away");
    }
    if opened.chain_truncated != 0 {
        println!(
            "torn     {} bytes of an unfinished entry were dropped from the record",
            opened.chain_truncated
        );
    }
    if opened.chain_length != 0 {
        println!("record   {} epochs judged", opened.chain_length);
    }
    if opened.members != 0 {
        println!("roll     {} members", opened.members);
    }
    if opened.addresses != 0 {
        println!("known    where {} of them said to look", opened.addresses);
    }
    if opened.has_the_file {
        println!("holding  the file");
    }
    if opened.read.unreadable != 0 {
        println!(
            "ignored  {} admissions that could not be read",
            opened.read.unreadable
        );
    }
}

/// One line describing what a completed exchange showed.
///
/// The parenthesis is the part that matters and the part most easily overstated: one
/// of these two exchanges proves the peer was awake and the other does not.
#[must_use]
pub(crate) fn describe(exchange: &Exchange) -> String {
    let liveness = if exchange.proves_peer_was_live {
        "answered the challenge we chose"
    } else {
        "spoke first, which proves only that it spoke"
    };
    format!(
        "witness  {}  epoch {}  skew {:+}  ({liveness})",
        exchange.peer.node_id, exchange.peer.heartbeat.epoch, exchange.epoch_skew
    )
}
