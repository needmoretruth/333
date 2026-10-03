//! `333-site observe`: the site node, written down for the public pages.
//!
//! Run by the node's owner (the site process cannot enter the node's directory), every
//! 15 seconds, from a timer. One run reads the node's files read-only, asks the running
//! node for `status --json`, and replaces `--out` atomically, readable by everybody.
//!
//! WHAT IT NEVER DOES. Open `identity.key`; write, create or repair anything in the
//! node's directory; publish an address other than the site node's own.
//!
//! # The file (`format` 1)
//!
//! Fields are only ever added while `format` is 1.
//!
//! ```text
//! {
//!   "format": 1,
//!   "as_of": <ms since 1970, when this was written>,
//!   "epoch": <the epoch it was written in>,
//!   "epoch_ends": <ms since 1970 when that epoch ends>,
//!   "running": <did the node answer on its control socket>,
//!   "site_node": <the site node's name, 64 hex> | null,
//!             `--site-node`, or else the author of its own chain
//!   "invitation": "333:host:port" | null,
//!             `--invitation`, or else the site node's own newest statement
//!   "status": <the node's `status --json`, byte for byte> | null,
//!             the previous run's when the node is down, null if never seen
//!   "nodes": [{
//!     "id": <name, 64 hex>,
//!     "founder": <handed the file on, never handed it>,
//!     "site": <the node this site runs>,
//!     "admitted": <epoch> | null,
//!     "sponsor": <name> | null,
//!     "counts_from": <epoch> | null,
//!     "last_heartbeat": <epoch> | null,
//!     "last_answered": <newest epoch of a positive attestation about it> | null,
//!     "answered_now": <a positive attestation or heartbeat this epoch or the last>,
//!     "signal": <what it said this epoch, 0..332> | null,
//!     "reach": "direct" | "onion" | null
//!   }],                                       sorted by id
//!   "edges": [{
//!     "from": <name>, "to": <name>,
//!     "kind": "handover" | "answered" | "silent",
//!     "epoch": <epoch>
//!   }]                                        attestations from the last 3 epochs only
//! }
//! ```
//!
//! Nodes come from the roll, from whereabouts, heartbeats, utterances and attestations
//! the node holds, so a node known only from those is in it too. Only records whose
//! signatures verify add a node or an edge. `handover` runs from sponsor to member;
//! `answered` and `silent` run from verifier to prover.

mod control;
mod given;
mod graph;
mod node_files;

use std::path::{Path, PathBuf};

use anyhow::Context as _;
use n333_core::Epoch;
use n333_core::epoch::unix_now_millis;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use crate::atomic;

/// Which shape of file this writes.
const FORMAT: u32 = 1;

/// The file is public: the site reads it, and so may anybody on the machine.
const OUT_MODE: u32 = 0o644;

/// What `observe` is told.
#[derive(clap::Args)]
pub(crate) struct Args {
    /// The node's directory. Read, never written.
    #[arg(long)]
    node: PathBuf,
    /// Where the observation goes.
    #[arg(long)]
    out: PathBuf,
    /// The site node's name (64 hex digits beginning with 333). Wins over the guess
    /// from the node's chain, which a new founder does not have yet. Empty is unset.
    #[arg(long, env = "THE333_SITE_NODE")]
    site_node: Option<String>,
    /// The invitation pages give out, `333:host:port`. Wins over the site node's own
    /// newest statement. Empty is unset.
    #[arg(long, env = "THE333_INVITATION")]
    invitation: Option<String>,
}

/// The file, as documented above.
#[derive(Serialize)]
struct Observation {
    format: u32,
    as_of: u64,
    epoch: u64,
    epoch_ends: u64,
    running: bool,
    site_node: Option<String>,
    invitation: Option<String>,
    status: Option<Box<RawValue>>,
    nodes: Vec<graph::Node>,
    edges: Vec<graph::Edge>,
}

/// The one field of the previous file that outlives a node going down.
#[derive(Deserialize)]
struct Previous {
    #[serde(default)]
    status: Option<Box<RawValue>>,
}

/// Observe once and write the file.
///
/// # Errors
/// Fails if `--site-node` or `--invitation` is malformed, or the file cannot be
/// written. Anything unreadable in the node's
/// directory is shown as absent rather than stopping the page from updating.
pub(crate) fn run(args: &Args) -> anyhow::Result<()> {
    if !args.node.is_dir() {
        tracing::warn!(
            "{} is not a directory; observing nothing",
            args.node.display()
        );
    }
    let given = given::Given::checked(args.site_node.as_deref(), args.invitation.as_deref())?;
    let observation = observe(&args.node, &args.out, &given, unix_now_millis());
    let bytes = serde_json::to_vec(&observation).context("writing the observation as JSON")?;
    atomic::write(&args.out, &bytes, OUT_MODE)
        .with_context(|| format!("writing {}", args.out.display()))
}

/// What the node at `home` looks like at `now_ms`.
fn observe(home: &Path, out: &Path, given: &given::Given, now_ms: u64) -> Observation {
    let epoch = Epoch::at_unix_seconds(now_ms / 1000);
    let held = node_files::read(home, epoch);
    let drawn = graph::build(&held, epoch, given);
    let (running, status) = match control::status(home) {
        control::Asked::Answered(Some(status)) => (true, Some(status)),
        control::Asked::Answered(None) => (true, previous_status(out)),
        control::Asked::Down => (false, previous_status(out)),
    };
    Observation {
        format: FORMAT,
        as_of: now_ms,
        epoch: epoch.0,
        epoch_ends: Epoch(epoch.0 + 1)
            .starts_at_unix_seconds()
            .saturating_mul(1000),
        running,
        site_node: drawn.site_node,
        invitation: drawn.invitation,
        status,
        nodes: drawn.nodes,
        edges: drawn.edges,
    }
}

/// The status the last run wrote, kept while the node cannot be asked.
fn previous_status(out: &Path) -> Option<Box<RawValue>> {
    let bytes = std::fs::read(out).ok()?;
    serde_json::from_slice::<Previous>(&bytes).ok()?.status
}

#[cfg(test)]
mod tests {
    use n333_core::Identity;
    use n333_core::subject::DIGEST;
    use n333_core::transfer::{Half, Record};

    use super::*;
    use crate::frames;

    /// Every file under `dir`, with its bytes and modification time.
    fn listing(dir: &Path) -> Vec<(PathBuf, Vec<u8>, std::time::SystemTime)> {
        let mut found: Vec<_> = std::fs::read_dir(dir)
            .unwrap()
            .map(|entry| {
                let path = entry.unwrap().path();
                let modified = std::fs::metadata(&path).unwrap().modified().unwrap();
                (path.clone(), std::fs::read(&path).unwrap(), modified)
            })
            .collect();
        found.sort();
        found
    }

    #[test]
    fn observing_writes_nothing_into_the_node_and_draws_its_founder() {
        let base = std::env::temp_dir().join(format!("n333-site-observe-{}", std::process::id()));
        let home = base.join("node");
        std::fs::create_dir_all(&home).unwrap();
        let (founder, member) = (Identity::from_seed(&[1; 32]), Identity::from_seed(&[2; 32]));
        let gave = Record::new(&founder, member.public_key(), Epoch(900), DIGEST)
            .seal(Half::Gave, &founder)
            .unwrap();
        let taken = Record::new(&member, founder.public_key(), Epoch(900), DIGEST)
            .seal(Half::Received, &member)
            .unwrap();
        let mut admissions = frames::join([gave.as_slice(), taken.as_slice()].into_iter());
        admissions.extend_from_slice(&[0, 0, 1]); // a torn tail, which must stay torn
        std::fs::write(home.join("admissions.log"), &admissions).unwrap();
        let before = listing(&home);

        let out = base.join("network.json");
        run(&Args {
            node: home.clone(),
            out: out.clone(),
            site_node: None,
            invitation: None,
        })
        .unwrap();

        assert_eq!(listing(&home), before);
        let written: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&out).unwrap()).unwrap();
        assert_eq!(written["running"], false);
        let founder_id = founder.node_id().to_string();
        let first = written["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|node| node["id"] == founder_id.as_str())
            .unwrap();
        assert_eq!(first["founder"], true);
        assert_eq!(written["edges"][0]["kind"], "handover");
        std::fs::remove_dir_all(&base).unwrap();
    }
}
