//! `333 id` — show this node's identity, creating one on first run.

use crate::commands::Common;
use crate::identity_file::{self, Origin};

/// Print this node's name, and how it came to have one.
///
/// # Errors
/// Fails if the identity file cannot be read or written.
pub(crate) fn run(common: &Common) -> anyhow::Result<()> {
    let home = common.paths.root();
    let (identity, origin) = identity_file::load_or_create(&common.mistrust(), home)?;

    println!("name     {}", identity.node_id());
    if let Origin::Created { not_called } = origin {
        println!("{}", crate::commands::naming(not_called));
    }
    println!("home     {}", home.display());
    if matches!(origin, Origin::Created { .. }) {
        // Said once, on the run that makes the name, because it is the only run on
        // which the warning can still be acted on.
        println!(
            "keep     that directory. lose it and you lose this name, every hour\n\
             \x20        anyone ever witnessed for you, and any way of proving you\n\
             \x20        were here. There is no recovery and there is no appeal."
        );
    }
    Ok(())
}
