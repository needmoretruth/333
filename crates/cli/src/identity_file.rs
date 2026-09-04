//! This node's identity on disk.
//!
//! The file holds the 32-byte seed and nothing else — no header, no format version.
//! A seed is not a document; adding a container would invite a parser, and a parser
//! is a thing that can be confused.
//!
//! An existing file is never overwritten. Losing the seed loses the identity, the
//! name derived from it, and every attestation ever made about it, so the one thing
//! this module must never do is replace one by accident.

use std::fs;
use std::io::Write as _;
use std::path::Path;

use anyhow::{Context as _, bail};
use n333_core::identity::{Identity, KeyClass};

/// How this node's identity came to be, for the caller to report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Origin {
    /// Read from a file that already existed.
    Loaded,
    /// Searched for and written, after this many key pairs were tried.
    Created {
        /// Key pairs generated before one qualified.
        attempts: u64,
    },
}

/// Read this node's identity, searching for one and writing it if the file is absent.
///
/// # Errors
/// Fails if the file exists but is unreadable, the wrong size, or readable by users
/// other than its owner; and if the directory cannot be created.
pub(crate) fn load_or_create(path: &Path) -> anyhow::Result<(Identity, Origin)> {
    match fs::read(path) {
        Ok(bytes) => {
            refuse_loose_permissions(path)?;
            let seed: [u8; 32] = bytes.as_slice().try_into().map_err(|_| {
                anyhow::anyhow!(
                    "identity file holds {} bytes; a seed is exactly 32",
                    bytes.len()
                )
            })?;
            let identity = Identity::from_seed(&seed);
            if identity.class() != KeyClass::Eligible {
                bail!(
                    "the identity in this file is not eligible: its name is {}",
                    identity.node_id()
                );
            }
            Ok((identity, Origin::Loaded))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => create(path),
        Err(e) => Err(e).with_context(|| format!("reading {}", path.display())),
    }
}

/// Search for an eligible identity and write it, failing if one is already there.
fn create(path: &Path) -> anyhow::Result<(Identity, Origin)> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
        restrict_directory(parent)?;
    }
    let (identity, attempts) = Identity::mine();
    let mut file = open_new_private(path)
        .with_context(|| format!("creating {}", path.display()))?;
    file.write_all(identity.seed().as_slice())?;
    // Without this the seed can still be in the page cache when the machine loses
    // power, and the node comes back with an address nobody can reach.
    file.sync_all()?;
    Ok((identity, Origin::Created { attempts }))
}

#[cfg(unix)]
fn open_new_private(path: &Path) -> std::io::Result<fs::File> {
    use std::os::unix::fs::OpenOptionsExt as _;
    // `create_new` is what stops a second process, or a second run, from replacing
    // an identity that already exists.
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
}

#[cfg(not(unix))]
fn open_new_private(path: &Path) -> std::io::Result<fs::File> {
    fs::OpenOptions::new().write(true).create_new(true).open(path)
}

#[cfg(unix)]
fn restrict_directory(dir: &Path) -> anyhow::Result<()> {
    use std::os::unix::fs::PermissionsExt as _;
    fs::set_permissions(dir, fs::Permissions::from_mode(0o700))
        .with_context(|| format!("restricting {}", dir.display()))
}

#[cfg(not(unix))]
fn restrict_directory(_dir: &Path) -> anyhow::Result<()> {
    Ok(())
}

#[cfg(unix)]
fn refuse_loose_permissions(path: &Path) -> anyhow::Result<()> {
    use std::os::unix::fs::PermissionsExt as _;
    let mode = fs::metadata(path)
        .with_context(|| format!("reading permissions of {}", path.display()))?
        .permissions()
        .mode();
    if mode & 0o077 != 0 {
        bail!(
            "{} is readable by other users (mode {:o}); this file is the node's whole \
             identity. Fix it with: chmod 600 {}",
            path.display(),
            mode & 0o777,
            path.display()
        );
    }
    Ok(())
}

#[cfg(not(unix))]
fn refuse_loose_permissions(_path: &Path) -> anyhow::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("n333-identity-test-{name}"));
        let _ = fs::remove_dir_all(&dir);
        dir.join("identity.key")
    }

    #[test]
    fn a_created_identity_is_eligible_and_reloads_unchanged() {
        let path = scratch("reload");
        let (first, origin) = load_or_create(&path).expect("creates");
        assert!(matches!(origin, Origin::Created { .. }));
        assert_eq!(first.class(), KeyClass::Eligible);

        let (second, origin) = load_or_create(&path).expect("loads");
        assert_eq!(origin, Origin::Loaded);
        assert_eq!(first.node_id(), second.node_id());
        let _ = fs::remove_dir_all(path.parent().expect("has a parent"));
    }

    #[test]
    fn a_file_of_the_wrong_size_is_refused() {
        let path = scratch("wrong-size");
        fs::create_dir_all(path.parent().expect("has a parent")).expect("creates dir");
        fs::write(&path, b"too short").expect("writes");
        assert!(load_or_create(&path).is_err());
        let _ = fs::remove_dir_all(path.parent().expect("has a parent"));
    }

    #[cfg(unix)]
    #[test]
    fn a_world_readable_identity_is_refused() {
        use std::os::unix::fs::PermissionsExt as _;
        let path = scratch("loose");
        let (_identity, _) = load_or_create(&path).expect("creates");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).expect("loosens");
        let refused = load_or_create(&path).expect_err("refuses");
        assert!(refused.to_string().contains("chmod 600"), "{refused}");
        let _ = fs::remove_dir_all(path.parent().expect("has a parent"));
    }

    #[cfg(unix)]
    #[test]
    fn a_created_identity_is_private() {
        use std::os::unix::fs::PermissionsExt as _;
        let path = scratch("mode");
        let (_identity, _) = load_or_create(&path).expect("creates");
        let mode = fs::metadata(&path).expect("stats").permissions().mode();
        assert_eq!(mode & 0o777, 0o600, "mode was {:o}", mode & 0o777);
        let _ = fs::remove_dir_all(path.parent().expect("has a parent"));
    }
}
