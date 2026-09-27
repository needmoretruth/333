//! `333 id` — show this node's identity, creating one on first run.

use crate::commands::Common;
use crate::identity_file::{self, Origin};

/// Print this node's name, how it came to have one, and when.
///
/// # Errors
/// Fails if the identity file cannot be read or written, or the node was packed for
/// moving.
pub(crate) fn run(common: &Common) -> anyhow::Result<()> {
    let home = common.paths.root();
    let (identity, origin) = identity_file::load_or_create(&common.mistrust(), home)?;

    aloud!("name     {}", identity.node_id());
    // The naming says where home is, once, beside the warning to keep it.
    if matches!(origin, Origin::Loaded) {
        aloud!("home     {}", home.display());
    }
    crate::named::report(origin, home);
    aloud!(
        "{}",
        crate::began::describe(crate::began::read(home).as_ref())
    );
    Ok(())
}
