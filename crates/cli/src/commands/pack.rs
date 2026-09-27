//! `333 pack` — write this node into one file, to be carried to another machine.
//!
//! A node's name is its key, and a key in two places is one name in two places. So
//! packing is a move and not a copy: once the file is written, this directory is
//! marked, and nothing here acts as the node again. `333 unpack` on the other machine
//! is the other half. `333 pack --undo` takes the mark away for a move that was
//! abandoned, and is the only way back, on purpose: it is one deliberate command,
//! typed by somebody who knows the file was never unpacked anywhere.
//!
//! What goes into the file, what stays, and why it is not encrypted are in
//! [`crate::archive`].

use std::path::Path;

use anyhow::{Context as _, bail};

use crate::archive::{self, Manifest};
use crate::commands::{Common, in_threes};
use crate::dwelling;
use crate::identity_file;
use crate::node::{Node, Opened};

/// Pack this node into `file`, or with `undo`, take back a packing here.
///
/// # Errors
/// Fails if there is no node here, it was already packed, `file` already exists, or
/// anything cannot be read or written.
pub(crate) fn run(common: &Common, file: Option<&Path>, undo: bool) -> anyhow::Result<()> {
    match (file, undo) {
        (_, true) => unpack_here(common),
        (Some(file), false) => pack(common, file),
        (None, false) => bail!("name the file to pack this node into: 333 pack <FILE>"),
    }
}

/// Write the node into `file` and mark this directory.
fn pack(common: &Common, file: &Path) -> anyhow::Result<()> {
    let root = common.paths.root();
    if !identity_file::holds_a_name(root) {
        bail!(
            "there is no node in {} to pack. Nothing was written.",
            root.display()
        );
    }
    if std::fs::symlink_metadata(file).is_ok() {
        bail!(
            "{} already exists. Packing writes a new file and never over an old one; \
             name another.",
            file.display()
        );
    }
    // Opened the way every command opens it: refused if already packed, told if it
    // has moved, and its own record verified before any of it is carried.
    let (node, opened) = Node::open(&common.mistrust(), root, common.keeping)?;
    let name = node.identity().node_id().to_string();
    drop(node);
    say_what_goes(&name, &opened, archive::has_onion_key(root), file);

    let manifest = Manifest {
        format: archive::FORMAT,
        name,
        created: crate::began::utc(std::time::SystemTime::now()),
    };
    let size = write_new(root, &manifest, file)?;
    let into = dwelling::canonical(file)?;
    let home = identity_file::secure(&common.mistrust(), root)?;
    dwelling::mark_packed(&home, std::time::SystemTime::now(), &into).with_context(|| {
        format!(
            "{} was written, and this directory could not be marked as packed. Until it \
             is, this node lives in both: delete that file before anything runs here.",
            into.display()
        )
    })?;
    say_it_is_packed(root, &into, size);
    Ok(())
}

/// Create `file`, readable by its owner alone, and write the node into it.
///
/// A file that could not be finished is removed: half a node is not a node, and a
/// file that looked like one would be carried somewhere and fail there.
fn write_new(root: &Path, manifest: &Manifest, file: &Path) -> anyhow::Result<u64> {
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut options, 0o600);
    let out = options
        .open(file)
        .with_context(|| format!("creating {}", file.display()))?;
    match archive::write(root, manifest, out) {
        Ok(out) => Ok(out.metadata().map(|meta| meta.len()).unwrap_or_default()),
        Err(e) => {
            let _ = std::fs::remove_file(file);
            Err(e)
        }
    }
}

/// Name what is about to leave, before it leaves.
fn say_what_goes(name: &str, opened: &Opened, onion: bool, file: &Path) {
    aloud!("name     {name}");
    match opened.chain_length {
        0 => aloud!("record   none yet"),
        1 => aloud!("record   1 epoch, going with it"),
        n => aloud!("record   {} epochs, going with them", in_threes(n)),
    }
    if opened.witnessed != 0 {
        aloud!(
            "witness  {} statements other keys signed about it, going with it",
            opened.witnessed
        );
    }
    if opened.has_the_file {
        aloud!("holding  the file, going with it");
    }
    if onion {
        aloud!("unseen   the key to its onion address, so the address goes with it");
    }
    aloud!(
        "carrying this node, into {}.\n\
         \x20        That file IS this node: whoever holds it can answer as this name.\n\
         \x20        Carry it, unpack it, then delete it; it is not a backup to keep.\n\
         \x20        It is not encrypted, because a password would be one more thing to\n\
         \x20        lose, and losing it would lose the name as surely as losing the file.\n\
         \x20        It is readable by you alone, as this directory is.",
        file.display()
    );
}

/// Say what was written and what to type next, at each end.
fn say_it_is_packed(root: &Path, into: &Path, size: u64) {
    aloud!(
        "packed   {} bytes: the files as they are, and half a kilobyte for each.\n\
         \x20        nothing in {} will act as this node again.",
        in_threes(size),
        root.display()
    );
    let carried = into.file_name().map_or_else(
        || into.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    );
    aloud!(
        "next     on the other machine: 333 unpack {carried}\n\
         \x20        if the move is abandoned: {}",
        dwelling::command(root, "pack --undo")
    );
}

/// Take away the mark, for a move that was abandoned.
fn unpack_here(common: &Common) -> anyhow::Result<()> {
    let root = common.paths.root();
    let Some(packed) = dwelling::packed(root) else {
        aloud!(
            "here     this node was not packed, so there is nothing to undo in {}",
            root.display()
        );
        return Ok(());
    };
    let home = identity_file::secure(&common.mistrust(), root)?;
    dwelling::unmark_packed(&home)?;
    aloud!(
        "restored this node lives in {} again.\n\
         \x20        The file it was packed into is still this name. If it was unpacked\n\
         \x20        anywhere, one of the two has to go before either runs; if it was not,\n\
         \x20        delete {}",
        root.display(),
        packed.into
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("n333-pack-test-{name}"));
        let _ = std::fs::remove_dir_all(&dir);
        let _ = std::fs::remove_file(&dir);
        dir
    }

    fn common(root: &Path) -> Common {
        Common {
            paths: crate::paths::NodePaths::at(root.to_path_buf()),
            timeout: std::time::Duration::from_secs(1),
            keeping: crate::node::Keeping::TheWindow,
            bridges: std::sync::Arc::default(),
            trust_directory_permissions: true,
        }
    }

    #[test]
    fn packing_refuses_to_write_over_a_file_that_is_there() {
        let root = scratch("over-home");
        let _ = identity_file::load_or_create(&common(&root).mistrust(), &root).expect("names");
        let file = scratch("over.333");
        std::fs::write(&file, b"somebody's").expect("writes");
        let refused = run(&common(&root), Some(&file), false).expect_err("refuses");
        assert!(refused.to_string().contains("already exists"), "{refused}");
        assert_eq!(std::fs::read(&file).expect("reads"), b"somebody's");
        assert!(dwelling::packed(&root).is_none(), "nothing was packed");
        let _ = std::fs::remove_dir_all(&root);
        let _ = std::fs::remove_file(&file);
    }

    #[test]
    fn packing_an_empty_directory_does_not_make_a_name_to_pack() {
        let root = scratch("empty");
        let file = scratch("empty.333");
        assert!(run(&common(&root), Some(&file), false).is_err());
        assert!(!identity_file::holds_a_name(&root));
        assert!(!file.exists());
    }

    #[cfg(unix)]
    #[test]
    fn the_packed_file_is_readable_by_its_owner_alone() {
        use std::os::unix::fs::PermissionsExt as _;
        let root = scratch("mode-home");
        let _ = identity_file::load_or_create(&common(&root).mistrust(), &root).expect("names");
        let file = scratch("mode.333");
        run(&common(&root), Some(&file), false).expect("packs");
        let mode = std::fs::metadata(&file)
            .expect("stats")
            .permissions()
            .mode();
        assert_eq!(mode & 0o777, 0o600);
        let _ = std::fs::remove_dir_all(&root);
        let _ = std::fs::remove_file(&file);
    }
}
