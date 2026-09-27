//! The one file a node becomes while it is carried to another machine.
//!
//! A tar archive, because every system can already list and unpack one: a person can
//! see what they are carrying without this client, and a client years from now can
//! read it without this one's code. Inside it, first a `manifest` — which format this
//! is, the name it holds, when it was made — and then every file the node is, under
//! `node/`, at the path it has inside the node's directory.
//!
//! WHAT GOES is everything that makes this node this node and cannot be had again:
//! the seed, when the name began, its own record, the admissions, what others signed
//! about it, where others said they are, the window's statements, `333.txt` if it
//! holds it, and arti's keystore, which is where the key to its onion address is, so
//! that the unseen address moves with it. WHAT STAYS is what describes the directory
//! rather than the node (`here`, `packed`) and what arti rebuilds on its own: the
//! directory cache, which costs one slow start, and the rest of arti's state — guard
//! choices and introduction points — which a new machine is better off choosing for
//! itself.
//!
//! IT IS NOT ENCRYPTED. Encrypting it would mean a password, and a password is a
//! second thing to lose with no recovery behind it: forgetting it would lose the name
//! as surely as losing the file. The directory it came from is not encrypted either,
//! and is protected the way this file is, by being readable by its owner alone. What
//! that means is plain: whoever can read this file can be this node, so it is made to
//! be carried and then deleted, not kept.
//!
//! WHAT IT COSTS is the files themselves, uncompressed, and half a kilobyte of header
//! and padding for each — a node with a full window is a few hundred files.

use std::io::Read as _;
use std::path::{Path, PathBuf};

use anyhow::{Context as _, bail};
use fs_mistrust::CheckedDir;

use crate::node::{ADMISSIONS_FILE, CHAIN_FILE, WHEREABOUTS_FILE, WINDOW_DIR, WITNESSED_FILE};

/// The version of this layout. A client reads every format up to its own.
///
/// FROZEN once released: an archive written today has to open in every later client.
pub(crate) const FORMAT: u32 = 1;

/// The first entry of every archive.
const MANIFEST: &str = "manifest";

/// The first line of the manifest, so that a person opening it knows what it is.
const HEADING: &str = "333 node";

/// Where the node's own files sit inside the archive.
const NODE: &str = "node/";

/// Files carried from the top of the node's directory, where there is one.
const FILES: [&str; 7] = [
    crate::identity_file::SEED_FILE,
    crate::began::BEGAN_FILE,
    CHAIN_FILE,
    ADMISSIONS_FILE,
    WITNESSED_FILE,
    WHEREABOUTS_FILE,
    n333_core::subject::FILENAME,
];

/// Arti's keystore, under the state directory `paths.rs` gives it.
///
/// arti puts it at `<state_dir>/keystore`; `paths.rs` pins that this matches.
pub(crate) const KEYSTORE: &str = "tor/state/keystore";

/// Directories carried whole.
const DIRS: [&str; 2] = [WINDOW_DIR, KEYSTORE];

/// What an archive says about itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Manifest {
    /// Which layout it was written in.
    pub(crate) format: u32,
    /// The name of the node inside it.
    pub(crate) name: String,
    /// When it was written: UTC, ISO 8601, to the second.
    pub(crate) created: String,
}

impl Manifest {
    /// Written as lines of `key value`, which a person can read and a later client
    /// can add to: a key it does not know is skipped.
    fn to_text(&self) -> String {
        format!(
            "{HEADING}\nformat {}\nname {}\ncreated {}\n",
            self.format, self.name, self.created
        )
    }

    /// Read one back, refusing a format newer than this client.
    fn from_text(text: &str) -> anyhow::Result<Self> {
        let mut lines = text.lines();
        if lines.next() != Some(HEADING) {
            bail!("that file is not a packed node: its manifest does not begin `{HEADING}`");
        }
        let (mut format, mut name, mut created) = (None, None, None);
        for line in lines {
            match line.split_once(' ') {
                Some(("format", value)) => format = value.parse::<u32>().ok(),
                Some(("name", value)) => name = Some(value.to_owned()),
                Some(("created", value)) => created = Some(value.to_owned()),
                _ => {}
            }
        }
        let format = format.context("the manifest does not say which format it is")?;
        if format > FORMAT {
            bail!(
                "that file was packed by a newer client, in format {format}. This one \
                 reads up to format {FORMAT}; unpack it with the newer one."
            );
        }
        Ok(Self {
            format,
            name: name.context("the manifest does not say which name it holds")?,
            created: created.unwrap_or_default(),
        })
    }
}

/// Every file of the node in `home` that goes, as paths relative to it with `/`.
///
/// # Errors
/// Fails if a directory that is there cannot be listed.
pub(crate) fn carried(home: &Path) -> anyhow::Result<Vec<String>> {
    let mut found: Vec<String> = FILES
        .iter()
        .filter(|name| is_file(&home.join(name)))
        .map(|name| (*name).to_owned())
        .collect();
    for dir in DIRS {
        walk(home, dir, &mut found)?;
    }
    Ok(found)
}

/// A regular file, not followed through a link.
fn is_file(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok_and(|meta| meta.is_file())
}

/// Add every regular file under `rel` to `found`, in a fixed order.
fn walk(home: &Path, rel: &str, found: &mut Vec<String>) -> anyhow::Result<()> {
    let dir = on_disk(home, rel);
    let Ok(listing) = std::fs::read_dir(&dir) else {
        return Ok(());
    };
    let mut names: Vec<String> = Vec::new();
    for entry in listing {
        let entry = entry.with_context(|| format!("listing {}", dir.display()))?;
        if let Ok(name) = entry.file_name().into_string() {
            names.push(name);
        }
    }
    names.sort();
    for name in names {
        let child = format!("{rel}/{name}");
        let meta = std::fs::symlink_metadata(on_disk(home, &child))?;
        if meta.is_dir() {
            walk(home, &child, found)?;
        } else if meta.is_file() {
            found.push(child);
        }
    }
    Ok(())
}

/// A `/`-separated relative path, as a path on this system.
fn on_disk(home: &Path, rel: &str) -> PathBuf {
    rel.split('/')
        .fold(home.to_path_buf(), |path, part| path.join(part))
}

/// Whether `home` holds the key to an onion address.
#[must_use]
pub(crate) fn has_onion_key(home: &Path) -> bool {
    let mut found = Vec::new();
    walk(home, &format!("{KEYSTORE}/hss"), &mut found).is_ok() && !found.is_empty()
}

/// Write the node in `home` into `out`, manifest first.
///
/// # Errors
/// Fails if a file cannot be read or the archive cannot be written.
pub(crate) fn write(
    home: &Path,
    manifest: &Manifest,
    out: std::fs::File,
) -> anyhow::Result<std::fs::File> {
    let mut builder = tar::Builder::new(out);
    let text = manifest.to_text();
    builder.append_data(&mut header(text.len() as u64), MANIFEST, text.as_bytes())?;
    for rel in carried(home)? {
        let path = on_disk(home, &rel);
        let file = std::fs::File::open(&path).with_context(|| format!("reading {rel}"))?;
        let size = file.metadata()?.len();
        builder
            .append_data(&mut header(size), format!("{NODE}{rel}"), file)
            .with_context(|| format!("packing {rel}"))?;
    }
    let out = builder.into_inner().context("finishing the archive")?;
    out.sync_all().context("writing the archive to disk")?;
    Ok(out)
}

/// A header for one file, readable by its owner alone and telling nothing else.
///
/// No owner, no group and no time: none of them is part of the node, and a user name
/// is a thing about the person that nobody asked this file to carry.
fn header(size: u64) -> tar::Header {
    let mut header = tar::Header::new_gnu();
    header.set_entry_type(tar::EntryType::Regular);
    header.set_size(size);
    header.set_mode(0o600);
    header.set_mtime(0);
    header
}

/// What reading an archive through once found, before anything is written.
#[derive(Debug)]
pub(crate) struct Looked {
    /// What it says about itself.
    pub(crate) manifest: Manifest,
    /// The seed it carries.
    pub(crate) seed: Vec<u8>,
}

/// Read `path` once: the manifest, the seed, and that every entry is a node's file.
///
/// # Errors
/// Fails if it cannot be read, is not an archive of a node, or holds anything a node
/// does not.
pub(crate) fn look(path: &Path) -> anyhow::Result<Looked> {
    let file = std::fs::File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let mut archive = tar::Archive::new(file);
    let (mut manifest, mut seed) = (None, None);
    for entry in archive.entries().context("reading the archive")? {
        let mut entry = entry.context("reading the archive")?;
        let name = entry_name(&entry)?;
        if name == MANIFEST {
            let mut text = String::new();
            entry
                .read_to_string(&mut text)
                .context("reading the manifest")?;
            manifest = Some(Manifest::from_text(&text)?);
        } else if let Some(rel) = inside(&name, &entry)?
            && rel == crate::identity_file::SEED_FILE
        {
            let mut bytes = Vec::new();
            entry.read_to_end(&mut bytes).context("reading the seed")?;
            seed = Some(bytes);
        }
    }
    Ok(Looked {
        manifest: manifest.context("that file has no manifest, so it is not a packed node")?,
        seed: seed.context("that file holds no seed, so there is no name in it")?,
    })
}

/// Write every node file in the archive at `path` into `into`.
///
/// # Errors
/// Fails if the archive cannot be read, holds anything a node does not, or a file
/// cannot be written.
pub(crate) fn extract(path: &Path, into: &CheckedDir) -> anyhow::Result<()> {
    let file = std::fs::File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let mut archive = tar::Archive::new(file);
    for entry in archive.entries().context("reading the archive")? {
        let mut entry = entry.context("reading the archive")?;
        let name = entry_name(&entry)?;
        if name == MANIFEST {
            continue;
        }
        let Some(rel) = inside(&name, &entry)? else {
            continue;
        };
        if let Some((parent, _)) = rel.rsplit_once('/') {
            into.make_directory(parent)
                .with_context(|| format!("making {parent}"))?;
        }
        let mut out = into
            .open(
                &rel,
                std::fs::OpenOptions::new().write(true).create_new(true),
            )
            .with_context(|| format!("writing {rel}"))?;
        std::io::copy(&mut entry, &mut out).with_context(|| format!("writing {rel}"))?;
        out.sync_all()?;
    }
    Ok(())
}

/// The name of an entry, which has to be text.
fn entry_name<R: std::io::Read>(entry: &tar::Entry<'_, R>) -> anyhow::Result<String> {
    let path = entry.path().context("reading a name in the archive")?;
    path.to_str()
        .map(str::to_owned)
        .context("the archive holds a name that is not text, which no node file has")
}

/// The path inside the node an entry belongs at, or `None` for a directory entry.
///
/// # Errors
/// Fails for anything that is not one of a node's files: a link, a device, a path
/// that climbs out, or a file no node has. An archive is refused whole rather than
/// unpacked in part.
fn inside<R: std::io::Read>(
    name: &str,
    entry: &tar::Entry<'_, R>,
) -> anyhow::Result<Option<String>> {
    let kind = entry.header().entry_type();
    if kind.is_dir() {
        return Ok(None);
    }
    match node_path(name) {
        Some(rel) if kind.is_file() => Ok(Some(rel.to_owned())),
        _ => bail!("that file holds {name}, which is not part of a node. Nothing was unpacked."),
    }
}

/// `node/<path>` for a path a node's file can have, as `<path>`.
fn node_path(name: &str) -> Option<&str> {
    let rel = name.strip_prefix(NODE)?;
    let sound = rel
        .split('/')
        .all(|part| !part.is_empty() && part != "." && part != ".." && !part.contains(['\\', ':']));
    let known = FILES.contains(&rel)
        || DIRS.iter().any(|dir| {
            rel.strip_prefix(dir)
                .is_some_and(|rest| rest.starts_with('/'))
        });
    (sound && known).then_some(rel)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("n333-archive-test-{name}"));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    fn put(home: &Path, rel: &str, bytes: &[u8]) {
        let path = on_disk(home, rel);
        std::fs::create_dir_all(path.parent().expect("has a parent")).expect("makes dirs");
        std::fs::write(path, bytes).expect("writes");
    }

    #[test]
    fn the_manifest_reads_back_and_a_newer_format_is_refused() {
        let manifest = Manifest {
            format: FORMAT,
            name: "333abc".to_owned(),
            created: "2026-09-27T10:12:00Z".to_owned(),
        };
        // Pinned: an archive written today is read by every later client.
        assert_eq!(
            manifest.to_text(),
            "333 node\nformat 1\nname 333abc\ncreated 2026-09-27T10:12:00Z\n"
        );
        assert_eq!(
            Manifest::from_text(&manifest.to_text()).expect("reads"),
            manifest
        );
        let newer = "333 node\nformat 2\nname 333abc\nsomething new\n";
        let refused = Manifest::from_text(newer).expect_err("refuses").to_string();
        assert!(refused.contains("newer client"), "{refused}");
    }

    #[test]
    fn only_a_nodes_own_files_have_a_place_inside_it() {
        assert_eq!(node_path("node/identity.key"), Some("identity.key"));
        assert_eq!(node_path("node/statements/9.seg"), Some("statements/9.seg"));
        assert!(node_path("node/tor/state/keystore/hss/n333/ks_hs_id.x").is_some());
        assert_eq!(node_path("node/../identity.key"), None);
        assert_eq!(node_path("node/statements/../../x"), None);
        assert_eq!(node_path("/etc/passwd"), None);
        assert_eq!(
            node_path("node/here"),
            None,
            "describes a directory, not a node"
        );
        assert_eq!(node_path("node/tor/cache/x"), None, "arti rebuilds it");
        assert_eq!(node_path("node/statements"), None);
    }

    #[test]
    fn a_node_is_carried_whole_and_nothing_rebuildable_goes_with_it() {
        let home = scratch("from");
        for (rel, bytes) in [
            ("identity.key", &[7_u8; 32][..]),
            ("chain.log", b"record"),
            ("333.txt", b"333"),
            ("statements/9.seg", b""),
            ("tor/state/keystore/hss/n333/ks_hs_id.key", b"onion"),
            ("tor/state/state/guards.json", b"guards"),
            ("tor/cache/consensus", b"cache"),
            ("here", b"made\n/somewhere\n"),
        ] {
            put(&home, rel, bytes);
        }
        let manifest = Manifest {
            format: FORMAT,
            name: "333abc".to_owned(),
            created: "2026-09-27T10:12:00Z".to_owned(),
        };
        let out = std::env::temp_dir().join("n333-archive-test-file.333");
        let _ = std::fs::remove_file(&out);
        let file = std::fs::File::create_new(&out).expect("creates");
        let _ = write(&home, &manifest, file).expect("packs");
        assert!(has_onion_key(&home));

        let looked = look(&out).expect("looks");
        assert_eq!(looked.manifest, manifest);
        assert_eq!(looked.seed, [7_u8; 32]);

        let to = scratch("to");
        let checked = fs_mistrust::Mistrust::new_dangerously_trust_everyone()
            .verifier()
            .make_secure_dir(&to)
            .expect("makes");
        extract(&out, &checked).expect("unpacks");
        assert_eq!(
            carried(&to).expect("lists"),
            [
                "identity.key",
                "chain.log",
                "333.txt",
                "statements/9.seg",
                "tor/state/keystore/hss/n333/ks_hs_id.key"
            ]
        );
        assert!(!to.join("here").exists());
        assert!(!on_disk(&to, "tor/cache/consensus").exists());
        assert_eq!(
            std::fs::read(to.join("chain.log")).expect("reads"),
            b"record"
        );
        for dir in [home, to] {
            let _ = std::fs::remove_dir_all(dir);
        }
        let _ = std::fs::remove_file(out);
    }
}
