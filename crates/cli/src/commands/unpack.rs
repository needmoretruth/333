//! `333 unpack` — put a packed node into this machine's node directory.
//!
//! The other half of `333 pack`. Nothing is written into the node directory until
//! the file has been read through and found to be a node: the seed makes the name
//! the manifest says it holds, and the record verifies. It is written first into a
//! directory beside the destination and renamed into place in one step, so a failure
//! halfway — a full disk, a pulled plug — leaves no half-node where a node is looked
//! for. A directory left beside it by a failure that stopped the process itself is
//! named `.<name>.unpacking-<number>`, and is safe to delete.
//!
//! It never unpacks over a node. A directory that already holds a name holds
//! somebody's record, and replacing it would lose that for good, so it is refused and
//! what would have been lost is named.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context as _, bail};
use fs_mistrust::CheckedDir;
use n333_core::Identity;
use n333_core::chain;
use n333_store::Log;

use crate::archive;
use crate::claim::{self, Taken};
use crate::commands::elsewhere::{self, Wanted};
use crate::commands::{Common, in_threes};
use crate::dwelling::{self, How};
use crate::identity_file;
use crate::node::Node;

/// Why nothing is unpacked into a directory another 333 has.
pub(crate) const KEPT: &str = "a node already lives in this directory. To unpack beside it\n\
                               \x20        instead, give it a directory of its own with --data-dir.";

/// Unpack the node in `file` into this machine's node directory.
///
/// The destination is taken for this process when it is there, so that nothing else
/// starts a node in it meanwhile, and the directory staged beside it is taken too, so
/// that the node is held from the moment it is renamed into place until this exits.
///
/// # Errors
/// Fails if a node already lives there, if `file` is not a packed node or its seed
/// and record do not agree with it, or if anything cannot be written.
pub(crate) async fn run(common: &Common, file: &Path) -> anyhow::Result<ExitCode> {
    let target = common.paths.root();
    let held = if target.exists() {
        match claim::take(&common.mistrust(), target)? {
            Taken::Ours(claim) => Some(claim),
            Taken::Theirs(holder) => {
                return elsewhere::run(common, holder, Wanted::Kept(KEPT)).await;
            }
        }
    } else {
        None
    };
    refuse_if_occupied(common, target, file)?;

    let looked = archive::look(file)?;
    let seed = format!("the {} in {}", identity_file::SEED_FILE, file.display());
    let identity = identity_file::from_seed_bytes(&looked.seed, Path::new(&seed))?;
    let name = identity.node_id().to_string();
    if name != looked.manifest.name {
        bail!(
            "that file says it holds {}, and the key inside it is {}. It is not one node, \
             and nothing was unpacked.",
            looked.manifest.name,
            name
        );
    }

    let staged = Staging::beside(common, target)?;
    let Taken::Ours(_staged_claim) = claim::take(&common.mistrust(), &staged.path)? else {
        bail!("another 333 took the directory this was unpacking into. Nothing was unpacked.");
    };
    archive::extract(file, &staged.dir)?;
    let epochs = verify_record(staged.dir.as_path(), &identity)?;
    let placed = staged.final_path()?;
    dwelling::record_here(&staged.dir, How::Unpacked, &placed)?;
    // Let go of the empty destination, whose lock file is the one thing in it, so that
    // it can be removed and the staged node, still held, renamed into its place.
    drop(held);
    staged.put_in_place()?;

    say_it_is_here(target, file, &name, epochs, &looked.manifest.created);
    Ok(ExitCode::SUCCESS)
}

/// Refuse, naming what would be lost, if a node already lives in `target`.
fn refuse_if_occupied(common: &Common, target: &Path, file: &Path) -> anyhow::Result<()> {
    let elsewhere = format!(
        "333 --data-dir <another directory> unpack {}",
        file.display()
    );
    if identity_file::holds_a_name(target) {
        let (node, opened) =
            Node::open(&common.mistrust(), target, common.keeping).with_context(|| {
                format!(
                    "a node already lives in {}, and it could not be opened to say what \
                     it holds. Nothing was unpacked. To unpack beside it instead: {elsewhere}",
                    target.display()
                )
            })?;
        let epochs = match opened.chain_length {
            0 => "no record yet".to_owned(),
            1 => "1 epoch of record".to_owned(),
            n => format!("{} epochs of record", in_threes(n)),
        };
        let holding = if opened.has_the_file {
            "holding the file"
        } else {
            "not holding the file"
        };
        bail!(
            "a node already lives in {}:\n\
             {}, {epochs}, {holding}.\n\
             Unpacking over it would lose all of that for good, so nothing was unpacked.\n\
             To unpack beside it instead, give it a directory of its own:\n\
             {elsewhere}",
            target.display(),
            node.identity().node_id()
        );
    }
    // The lock this process took on it is not something somebody else left there.
    let holds_anything = std::fs::read_dir(target).is_ok_and(|dir| {
        dir.flatten()
            .any(|entry| entry.file_name() != claim::LOCK_FILE)
    });
    if holds_anything {
        bail!(
            "{} holds files and no node. A node is unpacked into a directory of its own, \
             so nothing was unpacked. To unpack elsewhere: {elsewhere}",
            target.display()
        );
    }
    Ok(())
}

/// Check the unpacked record: that it verifies, is whole, and is this node's own.
///
/// # Errors
/// Fails if the record is torn, does not verify, or was written by another key.
fn verify_record(home: &Path, identity: &Identity) -> anyhow::Result<u64> {
    let (mut log, opened) =
        Log::open(&home.join(crate::node::CHAIN_FILE)).context("opening the record")?;
    if opened.truncated != 0 {
        bail!("the record in that file is torn, so it is not a whole node. Nothing was unpacked.");
    }
    let frames = log.read_all().context("reading the record")?;
    let head = chain::verify(&frames)
        .context("the record in that file does not verify. Nothing was unpacked.")?;
    if let Some(first) = frames.first() {
        let author = chain::open(first).context("reading the record")?.author;
        if author != identity.node_id() {
            bail!("the record in that file was written by another key. Nothing was unpacked.");
        }
    }
    Ok(head.length)
}

/// A directory beside the destination, removed unless it is put in place.
struct Staging {
    /// Where it is.
    path: PathBuf,
    /// The same, checked private.
    dir: CheckedDir,
    /// Where it is going.
    target: PathBuf,
    /// Whether it got there; if not, it is removed.
    placed: bool,
}

impl Staging {
    /// Make one beside `target`, private, with every directory above it checked.
    fn beside(common: &Common, target: &Path) -> anyhow::Result<Self> {
        let leaf = target
            .file_name()
            .with_context(|| {
                format!(
                    "{} is not a directory a node can be put in",
                    target.display()
                )
            })?
            .to_string_lossy()
            .into_owned();
        let path = parent_of(target).join(format!(".{leaf}.unpacking-{}", std::process::id()));
        let dir = identity_file::secure(&common.mistrust(), &path)?;
        Ok(Self {
            path,
            dir,
            target: target.to_path_buf(),
            placed: false,
        })
    }

    /// The canonical path the node will have once it is in place.
    fn final_path(&self) -> anyhow::Result<PathBuf> {
        let parent = dwelling::canonical(&parent_of(&self.target))?;
        Ok(match self.target.file_name() {
            Some(leaf) => parent.join(leaf),
            None => parent,
        })
    }

    /// Rename it into place, in one step.
    fn put_in_place(mut self) -> anyhow::Result<()> {
        // Only ever an empty directory by now, but for the lock this process took on
        // it: `refuse_if_occupied` said so, and `remove_dir` removes nothing that is not.
        if self.target.exists() {
            match std::fs::remove_file(self.target.join(claim::LOCK_FILE)) {
                Err(e) if e.kind() != std::io::ErrorKind::NotFound => {
                    return Err(e)
                        .with_context(|| format!("making room at {}", self.target.display()));
                }
                _ => {}
            }
            std::fs::remove_dir(&self.target)
                .with_context(|| format!("making room at {}", self.target.display()))?;
        }
        std::fs::rename(&self.path, &self.target)
            .with_context(|| format!("putting the node in {}", self.target.display()))?;
        self.placed = true;
        #[cfg(unix)]
        if let Ok(parent) = std::fs::File::open(parent_of(&self.target)) {
            // The rename is only durable once the directory holding it is.
            let _ = parent.sync_all();
        }
        Ok(())
    }
}

impl Drop for Staging {
    fn drop(&mut self) {
        if !self.placed {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }
}

/// The directory `path` is in, reading a bare name as the current directory.
fn parent_of(path: &Path) -> PathBuf {
    match path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent.to_path_buf(),
        _ => PathBuf::from("."),
    }
}

/// Say what arrived, and what to type next.
fn say_it_is_here(target: &Path, file: &Path, name: &str, epochs: u64, packed: &str) {
    aloud!("name     {name}");
    match epochs {
        0 => aloud!("record   none yet"),
        1 => aloud!("record   1 epoch, verified"),
        n => aloud!("record   {} epochs, verified", in_threes(n)),
    }
    if n333_core::Subject::recognise(
        &std::fs::read(target.join(n333_core::subject::FILENAME)).unwrap_or_default(),
    )
    .is_ok()
    {
        aloud!("holding  the file");
    }
    if archive::has_onion_key(target) {
        aloud!("unseen   the key to its onion address, so the address came with it");
    }
    aloud!(
        "{}",
        crate::began::describe(crate::began::read(target).as_ref())
    );
    aloud!(
        "unpacked into {},\n\
         \x20        from a file packed at {packed}.\n\
         \x20        This is the node now, and so is that file: delete {}",
        target.display(),
        file.display()
    );
    aloud!("next     {}", dwelling::command(target, "serve"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use n333_core::Epoch;
    use n333_core::presence::Attendance;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("n333-unpack-test-{name}"));
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

    /// A node with a name, three epochs of record, and the file.
    async fn a_node_with_a_record(root: &Path) -> (String, chain::Head) {
        let (node, _) = Node::open(
            &common(root).mistrust(),
            root,
            crate::node::Keeping::TheWindow,
        )
        .expect("opens");
        for epoch in [10, 11, 12] {
            let _ = node
                .record(Epoch(epoch), Attendance::Present, [0; 32])
                .await
                .expect("records");
        }
        std::fs::write(root.join(n333_core::subject::FILENAME), b"333").expect("is given it");
        (node.identity().node_id().to_string(), node.head().await)
    }

    #[tokio::test]
    async fn a_packed_node_unpacks_elsewhere_as_the_same_name_and_record() {
        let (from, to, file) = (scratch("rt-from"), scratch("rt-to"), scratch("rt.333"));
        let (name, head) = a_node_with_a_record(&from).await;
        crate::commands::pack::run(&common(&from), Some(&file), false).expect("packs");

        run(&common(&to), &file).await.expect("unpacks");
        let (node, opened) = Node::open(
            &common(&to).mistrust(),
            &to,
            crate::node::Keeping::TheWindow,
        )
        .expect("opens");
        assert_eq!(node.identity().node_id().to_string(), name);
        assert_eq!(node.head().await, head);
        assert_eq!(opened.chain_length, 3);
        assert!(opened.has_the_file);
        assert_eq!(
            crate::began::read(&to),
            crate::began::read(&from),
            "the day it began travels with it"
        );
        // The source is refused now; that is the other half of the same move.
        let refused = Node::open(
            &common(&from).mistrust(),
            &from,
            crate::node::Keeping::TheWindow,
        )
        .err()
        .expect("the source refuses")
        .to_string();
        assert!(refused.contains("packed for moving"), "{refused}");
        for dir in [&from, &to] {
            let _ = std::fs::remove_dir_all(dir);
        }
        let _ = std::fs::remove_file(&file);
    }

    #[tokio::test]
    async fn unpacking_over_a_node_is_refused_and_names_what_would_be_lost() {
        let (from, there, file) = (scratch("ov-from"), scratch("ov-there"), scratch("ov.333"));
        let _ = a_node_with_a_record(&from).await;
        crate::commands::pack::run(&common(&from), Some(&file), false).expect("packs");
        let (resident, _) = a_node_with_a_record(&there).await;

        let refused = run(&common(&there), &file)
            .await
            .expect_err("refuses")
            .to_string();
        assert!(refused.contains(&resident), "names the node: {refused}");
        assert!(refused.contains("3 epochs of record"), "{refused}");
        assert!(refused.contains("holding the file"), "{refused}");
        assert!(
            refused.contains("--data-dir"),
            "says the way through: {refused}"
        );
        let (node, _) = Node::open(
            &common(&there).mistrust(),
            &there,
            crate::node::Keeping::TheWindow,
        )
        .expect("the resident is untouched");
        assert_eq!(node.identity().node_id().to_string(), resident);
        for dir in [&from, &there] {
            let _ = std::fs::remove_dir_all(dir);
        }
        let _ = std::fs::remove_file(&file);
    }

    #[tokio::test]
    async fn a_file_whose_key_is_not_the_name_it_claims_leaves_nothing_behind() {
        let (from, to, file) = (scratch("lie-from"), scratch("lie-to"), scratch("lie.333"));
        let (name, _) = a_node_with_a_record(&from).await;
        let manifest = archive::Manifest {
            format: archive::FORMAT,
            name: name.replacen("333", "334", 1),
            created: String::new(),
        };
        let out = std::fs::File::create_new(&file).expect("creates");
        let _ = archive::write(&from, &manifest, out).expect("writes");

        let refused = run(&common(&to), &file)
            .await
            .expect_err("refuses")
            .to_string();
        assert!(refused.contains("not one node"), "{refused}");
        assert!(!to.exists(), "no half-node");
        let _ = std::fs::remove_dir_all(&from);
        let _ = std::fs::remove_file(&file);
    }

    #[tokio::test]
    async fn a_torn_record_is_found_after_staging_and_nothing_is_left_behind() {
        let (from, to, file) = (
            scratch("torn-from"),
            scratch("torn-to"),
            scratch("torn.333"),
        );
        let (name, _) = a_node_with_a_record(&from).await;
        let mut chain = std::fs::OpenOptions::new()
            .append(true)
            .open(from.join(crate::node::CHAIN_FILE))
            .expect("opens");
        std::io::Write::write_all(&mut chain, &[0xff; 3]).expect("tears it");
        let manifest = archive::Manifest {
            format: archive::FORMAT,
            name,
            created: String::new(),
        };
        let out = std::fs::File::create_new(&file).expect("creates");
        let _ = archive::write(&from, &manifest, out).expect("writes");

        let refused = run(&common(&to), &file)
            .await
            .expect_err("refuses")
            .to_string();
        assert!(refused.contains("torn"), "{refused}");
        assert!(!to.exists(), "no half-node");
        let staged = std::fs::read_dir(std::env::temp_dir())
            .expect("lists")
            .filter_map(Result::ok)
            .any(|entry| {
                entry
                    .file_name()
                    .to_string_lossy()
                    .starts_with(".n333-unpack-test-torn-to")
            });
        assert!(!staged, "the staging directory was left behind");
        let _ = std::fs::remove_dir_all(&from);
        let _ = std::fs::remove_file(&file);
    }
}
