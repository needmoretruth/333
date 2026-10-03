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
use crate::commands::Common;
use crate::dwelling;
use crate::identity_file;
use crate::node::{Node, Opened};
use crate::words::Arg;

/// Pack this node into `file`, or with `undo`, take back a packing here.
///
/// # Errors
/// Fails if there is no node here, it was already packed, `file` already exists, or
/// anything cannot be read or written.
pub(crate) fn run(common: &Common, file: Option<&Path>, undo: bool) -> anyhow::Result<()> {
    match (file, undo) {
        (_, true) => unpack_here(common),
        (Some(file), false) => pack(common, file),
        (None, false) => bail!(words!("pack-name-the-file")),
    }
}

/// Write the node into `file` and mark this directory.
fn pack(common: &Common, file: &Path) -> anyhow::Result<()> {
    let root = common.paths.root();
    if !identity_file::holds_a_name(root) {
        bail!(words!("pack-no-node", root = root.display().to_string()));
    }
    if std::fs::symlink_metadata(file).is_ok() {
        bail!(words!(
            "pack-already-exists",
            file = file.display().to_string()
        ));
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
    dwelling::mark_packed(&home, std::time::SystemTime::now(), &into)
        .with_context(|| words!("pack-not-marked", file = into.display().to_string()))?;
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
        .with_context(|| words!("pack-creating", file = file.display().to_string()))?;
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
    aloud_in!("pack-name", name = name);
    match opened.chain_length {
        0 => aloud_in!("pack-record-none"),
        n => aloud_in!("pack-record", epochs = Arg::grouped(n)),
    }
    if opened.witnessed != 0 {
        aloud_in!("pack-witnessed", statements = opened.witnessed);
    }
    if opened.has_the_file {
        aloud_in!("pack-holding");
    }
    if onion {
        aloud_in!("pack-onion-key");
    }
    aloud_in!("pack-carrying", file = file.display().to_string());
}

/// Say what was written and what to type next, at each end.
fn say_it_is_packed(root: &Path, into: &Path, size: u64) {
    aloud_in!(
        "pack-packed",
        bytes = Arg::grouped(size),
        root = root.display().to_string()
    );
    let carried = into.file_name().map_or_else(
        || into.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    );
    aloud_in!(
        "pack-next",
        carried = carried,
        undo = dwelling::command(root, "pack --undo")
    );
}

/// Take away the mark, for a move that was abandoned.
fn unpack_here(common: &Common) -> anyhow::Result<()> {
    let root = common.paths.root();
    let Some(packed) = dwelling::packed(root) else {
        aloud_in!("pack-not-packed", root = root.display().to_string());
        return Ok(());
    };
    let home = identity_file::secure(&common.mistrust(), root)?;
    dwelling::unmark_packed(&home)?;
    aloud_in!(
        "pack-restored",
        root = root.display().to_string(),
        file = packed.into
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::words::count::Base;
    use crate::words::layout::{COLUMN, where_the_words_begin};

    const ROOT: &str = "/tmp/333-node";
    const FILE: &str = "/tmp/node.333";

    fn lines() -> Vec<String> {
        vec![
            words!("pack-name", name = "333abc"),
            words!("pack-record-none"),
            words!("pack-record", epochs = Arg::grouped(1)),
            words!("pack-record", epochs = Arg::grouped(1_234)),
            words!("pack-witnessed", statements = 12_usize),
            words!("pack-holding"),
            words!("pack-onion-key"),
            words!("pack-carrying", file = FILE),
            words!("pack-packed", bytes = Arg::grouped(1_048_576), root = ROOT),
            words!(
                "pack-next",
                carried = "node.333",
                undo = "333 --data-dir /tmp/333-node pack --undo"
            ),
            words!("pack-not-packed", root = ROOT),
            words!("pack-restored", root = ROOT, file = FILE),
        ]
    }

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let (said, phrases) = crate::words::speaking("en", Base::Ten, || {
            let phrases = [
                (
                    words!("pack-name-the-file"),
                    "name the file to pack this node into: 333 pack <FILE>",
                ),
                (
                    words!("pack-no-node", root = ROOT),
                    "there is no node in /tmp/333-node to pack. Nothing was written.",
                ),
                (
                    words!("pack-already-exists", file = FILE),
                    "/tmp/node.333 already exists. Packing writes a new file and never over \
                     an old one; name another.",
                ),
                (
                    words!("pack-not-marked", file = FILE),
                    "/tmp/node.333 was written, and this directory could not be marked as \
                     packed. Until it is, this node lives in both: delete that file before \
                     anything runs here.",
                ),
                (
                    words!("pack-creating", file = FILE),
                    "creating /tmp/node.333",
                ),
            ];
            (lines(), phrases)
        });
        assert_eq!(
            said,
            [
                "name     333abc",
                "record   none yet",
                "record   1 epoch, going with it",
                "record   1,234 epochs, going with them",
                "witness  12 statements other keys signed about it, going with it",
                "holding  the file, going with it",
                "unseen   the key to its onion address, so the address goes with it",
                "carrying this node, into /tmp/node.333.\n\
                 \x20        That file IS this node: whoever holds it can answer as this name.\n\
                 \x20        Carry it, unpack it, then delete it; it is not a backup to keep.\n\
                 \x20        It is not encrypted, because a password would be one more thing to\n\
                 \x20        lose, and losing it would lose the name as surely as losing the file.\n\
                 \x20        It is readable by you alone, as this directory is.",
                "packed   1,048,576 bytes.\n\
                 \x20        nothing in /tmp/333-node will act as this node again.",
                "next     on the other machine: 333 unpack node.333\n\
                 \x20        if the move is abandoned: 333 --data-dir /tmp/333-node pack --undo",
                "here     this node was not packed, so there is nothing to undo in /tmp/333-node",
                "restored this node lives in /tmp/333-node again.\n\
                 \x20        The file it was packed into is still this name. If it was unpacked\n\
                 \x20        anywhere, one of the two has to go before either runs; if it was not,\n\
                 \x20        delete /tmp/node.333",
            ]
        );
        for (now, before) in phrases {
            assert_eq!(now, before);
        }
    }

    #[test]
    fn in_korean_every_line_of_pack_begins_its_words_in_the_same_column() {
        for line in crate::words::speaking("ko", Base::Ten, lines) {
            assert!(!line.is_ascii(), "{line:?}");
            assert_eq!(where_the_words_begin(&line), COLUMN, "{line:?}");
        }
    }

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
