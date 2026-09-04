//! `333 id` — show this node's identity, creating one on first run.

use crate::commands::Common;
use crate::identity_file::{self, Origin};

/// Print this node's name, and how it came to have one.
///
/// # Errors
/// Fails if the identity file cannot be read or written.
pub(crate) fn run(common: &Common) -> anyhow::Result<()> {
    let path = common.paths.identity_file();
    let (identity, origin) = identity_file::load_or_create(&path)?;

    match origin {
        Origin::Loaded => println!("name     {}", identity.node_id()),
        Origin::Created { attempts } => {
            println!("name     {}  (new)", identity.node_id());
            println!("search   {attempts} key pairs");
        }
    }
    println!("key      {}", path.display());
    Ok(())
}
