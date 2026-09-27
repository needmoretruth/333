//! `333 service check` — what the service manager runs every hour, to find out whether
//! the vigil is being kept, and to say so on this machine when it is not.
//!
//! A person who has stopped being counted should hear it from their own computer, not
//! discover it weeks later from their standing. Two things are looked for: a vigil
//! that is not running, which the awake stamp shows, and a vigil that runs and is
//! never asked about, which is the `unseen` line `serve` says when it starts and
//! nobody running it as a service ever reads.
//!
//! Silent when all is well. When it is not, the line goes to standard output, which is
//! the service's log, and to the desktop if this system has a way to put it there.

use std::path::Path;

use n333_core::epoch::unix_now_seconds;

use crate::claim::{self, Taken};
use crate::commands::Common;
use crate::node::Node;

use super::{awake, notify, say};

/// What the check says when the vigil runs and nothing is ever signed about this node.
const UNSEEN: &str = "unseen   the vigil is kept, and nothing has been signed about this node in any\n\
                      \x20        epoch: it reaches out and nobody reaches it. `333 status` says why\n\
                      \x20        and what to do.";

/// Look, and say what is wrong if anything is.
///
/// # Errors
/// Fails if the node is being kept and its directory cannot be read.
pub(crate) async fn run(common: &Common) -> anyhow::Result<()> {
    let root = common.paths.root();
    let line = match awake::not_kept(awake::read(root), true, unix_now_seconds()) {
        Some(line) => Some(line),
        // Only once the vigil is known to be running, so that a node directory that
        // is not there is never made by the thing checking on it.
        None => unseen(common).await?.then(|| UNSEEN.to_owned()),
    };
    if let Some(line) = line {
        say(format_args!("{line}"))?;
        notify::raise(&line);
    }
    Ok(())
}

/// Whether nothing has been signed about this node, read without disturbing whoever
/// holds its directory.
///
/// Opening a node repairs its records, which is only safe for the one process that
/// holds the directory. When that is the vigil, as it is whenever the service is
/// keeping it, the vigil is asked through its socket instead, and answers from the node
/// it has open. When it is some other command, nothing is opened or said this hour.
async fn unseen(common: &Common) -> anyhow::Result<bool> {
    let root = common.paths.root();
    let Taken::Ours(_claim) = claim::take(&common.mistrust(), root)? else {
        return Ok(the_vigil_says_unseen(root).await.unwrap_or(false));
    };
    let (_, opened) = Node::open(&common.mistrust(), root, common.keeping)?;
    Ok(crate::commands::unseen(&opened))
}

/// Ask the vigil keeping `root` for `status --json`, and read `unseen` out of it.
///
/// `None` when no vigil answers, it did not do it, or what came back is not the page.
async fn the_vigil_says_unseen(root: &Path) -> Option<bool> {
    let (true, page) = crate::control::ask(root, "status --json").await.ok()?? else {
        return None;
    };
    serde_json::from_str::<serde_json::Value>(&page)
        .ok()?
        .get("unseen")?
        .as_bool()
}
