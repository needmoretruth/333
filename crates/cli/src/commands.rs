//! What the client can be asked to do.
//!
//! One file per command. Each one owns its own output text, because the words a
//! person reads are part of the command, not a detail of it.

pub(crate) mod hours;
pub(crate) mod id;
pub(crate) mod join;
pub(crate) mod ping;
pub(crate) mod serve;
pub(crate) mod status;

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
        let epochs = if opened.chain_length == 1 {
            "1 epoch".to_owned()
        } else {
            format!("{} epochs", opened.chain_length)
        };
        println!("record   {epochs} already answered for, none of them open to revision");
    }
    if opened.members != 0 {
        let us = if opened.members == 1 {
            "1 of us, which is this node".to_owned()
        } else {
            format!("{} of us", opened.members)
        };
        println!("roll     {us}");
    }
    if opened.addresses != 0 {
        println!("known    where {} of us said to look", opened.addresses);
    }
    if opened.has_the_file {
        println!("holding  the file, and able to pass it on");
    }
    if opened.read.unreadable != 0 {
        println!(
            "ignored  {} admissions that could not be read",
            opened.read.unreadable
        );
    }
}

/// What a trade of statements changed, when it changed anything.
///
/// Silent when it changed nothing, which is the ordinary case once a node has settled:
/// a line every time would be a line every 333 minutes per neighbour saying nothing
/// happened.
pub(crate) fn report_heard(heard: &crate::node::Heard) {
    if heard.addresses != 0 {
        println!("learned  where {} more of us are", heard.addresses);
    }
    if heard.members != 0 {
        println!("learned  {} more of us by name", heard.members);
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
