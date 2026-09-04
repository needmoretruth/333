//! This node's identity on disk.
//!
//! The file holds the 32-byte seed and nothing else — no header, no format version.
//! A seed is not a document; adding a container would invite a parser, and a parser
//! is a thing that can be confused.
//!
//! An existing file is never overwritten. Losing the seed loses the identity, the
//! name derived from it, and every attestation ever made about it, so the one thing
//! this module must never do is replace one by accident.
//!
//! Permissions are checked by `fs-mistrust`, the same crate arti uses for its own
//! keys, rather than by a mode comparison written here. Its model is that the
//! DIRECTORY is the boundary, and it is worth stating because it is not the obvious
//! one:
//!
//! * Every directory from the filesystem root down to the node's home is checked,
//!   not just the home itself. A file at mode 600 inside a directory others can
//!   write is not private — anyone who can write the directory can delete the file
//!   and leave their own in its place. This is the case a mode check on the file
//!   alone gets wrong, and it is why the check is not written here.
//! * Inside a home that only its owner can enter, the mode of the file itself is not
//!   the boundary, so fs-mistrust does not refuse a loosely permissioned one. Files
//!   are still created at mode 600; nothing loosens them.
//!
//! On Windows it checks nothing at all. That is fs-mistrust's documented behaviour
//! and arti's, so the client says so in the README rather than implying a check that
//! does not run.

use std::fs::OpenOptions;
use std::io::Write as _;
use std::path::Path;

use anyhow::{Context as _, bail};
use fs_mistrust::{CheckedDir, Mistrust};
use n333_core::enrollment::{self, CURSE_PAUSE, Refusal};
use n333_core::identity::Identity;

/// The name of the seed file inside the node's directory.
const SEED_FILE: &str = "identity.key";

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

/// Read this node's identity from `home`, searching for one and writing it if absent.
///
/// # Errors
/// Fails if `home` or any directory above it is reachable by other users, if the file
/// exists but is unreadable or the wrong size, or if the identity in it is not
/// eligible to take part.
pub(crate) fn load_or_create(
    mistrust: &Mistrust,
    home: &Path,
) -> anyhow::Result<(Identity, Origin)> {
    let home = mistrust
        .verifier()
        .make_secure_dir(home)
        .with_context(|| private_directory_advice(home))?;

    match home.read(SEED_FILE) {
        Ok(bytes) => Ok((from_seed_bytes(&bytes)?, Origin::Loaded)),
        Err(fs_mistrust::Error::NotFound(_)) => create(&home),
        Err(e) => Err(anyhow::Error::new(e).context(private_file_advice(&home))),
    }
}

/// Interpret the bytes of the seed file.
fn from_seed_bytes(bytes: &[u8]) -> anyhow::Result<Identity> {
    let seed: [u8; 32] = bytes.try_into().map_err(|_| {
        anyhow::anyhow!(
            "identity file holds {} bytes; a seed is exactly 32",
            bytes.len()
        )
    })?;
    let identity = Identity::from_seed(&seed);
    match enrollment::admit(&identity.node_id()) {
        Ok(()) => Ok(identity),
        Err(Refusal::Cursed) => {
            // The stop is the curse, not a delay in front of it. It is taken before
            // the words, because the words are the reading of what has already
            // happened. Reachable only by a key made somewhere else and put here on
            // purpose: the search discards these without a word, and no flag, prompt
            // or menu in this client offers one.
            std::thread::sleep(CURSE_PAUSE);
            bail!(
                "333 has laid a curse on you and taken {} milliseconds off your life.\n\
                 Once. It will not be repeated, and it cannot be lifted.\n\
                 \n\
                 {}\n\
                 is turned away, and no client of ours will ever carry that name.\n\
                 \n\
                 333 is extremely generous. One epoch in three you may rest and you are\n\
                 still one of us: generous to the slow, to the poor, to the small machine\n\
                 in the cupboard, to everyone not yet born. It is not generous to\n\
                 heretics.",
                CURSE_PAUSE.as_millis(),
                identity.node_id()
            )
        }
        Err(Refusal::Ineligible) => bail!(
            "that is not a name 333 answers to.\n\
             \n\
             {}\n\
             does not begin with 333, so nothing here is addressed to it. Nothing was\n\
             taken from you either: 333 has not looked at you at all.",
            identity.node_id()
        ),
    }
}

/// Search for an eligible identity and write it, failing if one is already there.
fn create(home: &CheckedDir) -> anyhow::Result<(Identity, Origin)> {
    let (identity, attempts) = Identity::mine();
    // `create_new` is what stops a second process, or a second run, from replacing an
    // identity that already exists. fs-mistrust supplies the mode on unix systems.
    let mut file = home
        .open(SEED_FILE, OpenOptions::new().write(true).create_new(true))
        .context("creating the identity file")?;
    file.write_all(identity.seed().as_slice())?;
    // Without this the seed can still be in the page cache when the machine loses
    // power, and the node comes back with an address nobody can reach.
    file.sync_all()?;
    Ok((identity, Origin::Created { attempts }))
}

/// What to tell someone whose node directory is not private.
fn private_directory_advice(home: &Path) -> String {
    format!(
        "{} must be readable only by you: it holds this node's whole identity.\n\
         Fix it with: chmod 700 {}\n\
         Or, if you understand what you are giving up, pass \
         --dangerously-trust-directory-permissions",
        home.display(),
        home.display()
    )
}

/// What to tell someone whose identity file is not private.
fn private_file_advice(home: &CheckedDir) -> String {
    let path = home.as_path().join(SEED_FILE);
    format!(
        "cannot read {}\nIf the permissions are the problem: chmod 600 {}",
        path.display(),
        path.display()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("n333-identity-test-{name}"));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    /// Create the scratch home the way the client would: enterable only by its owner.
    fn make_private_dir(dir: &std::path::Path) {
        std::fs::create_dir_all(dir).expect("creates dir");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700))
                .expect("restricts dir");
        }
    }

    /// A `Mistrust` that ignores the environment, so a developer's own
    /// `FS_MISTRUST_DISABLE_PERMISSIONS_CHECKS` cannot quietly pass these tests.
    fn strict() -> Mistrust {
        Mistrust::builder()
            .ignore_prefix(std::env::temp_dir())
            .ignore_environment()
            .build()
            .expect("a buildable Mistrust")
    }

    #[test]
    fn a_created_identity_is_eligible_and_reloads_unchanged() {
        let home = scratch("reload");
        let (first, origin) = load_or_create(&strict(), &home).expect("creates");
        assert!(matches!(origin, Origin::Created { .. }));
        assert_eq!(first.class(), n333_core::KeyClass::Eligible);

        let (second, origin) = load_or_create(&strict(), &home).expect("loads");
        assert_eq!(origin, Origin::Loaded);
        assert_eq!(first.node_id(), second.node_id());
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn a_cursed_name_is_refused_and_the_curse_is_actually_levied() {
        // Unreachable through this client: the search discards these without a word.
        // Reached here by writing a seed straight into the file, which is the only
        // situation the refusal exists for.
        let home = scratch("cursed");
        make_private_dir(&home);
        let mut seed = [0_u8; 32];
        seed[..4].copy_from_slice(&4307_u32.to_le_bytes());
        std::fs::write(home.join(SEED_FILE), seed).expect("writes");

        let started = std::time::Instant::now();
        let refused = load_or_create(&strict(), &home).expect_err("refuses");
        // The 333 milliseconds are the curse itself. A client that only described it
        // would be a client that never took anything from anybody.
        assert!(
            started.elapsed() >= CURSE_PAUSE,
            "the curse is meant to be levied, not described"
        );
        let said = refused.to_string();
        assert!(said.contains("taken 333 milliseconds off your life"), "{said}");
        assert!(said.contains("Once."), "{said}");
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn a_file_of_the_wrong_size_is_refused() {
        let home = scratch("wrong-size");
        make_private_dir(&home);
        std::fs::write(home.join(SEED_FILE), b"too short").expect("writes");
        let refused = load_or_create(&strict(), &home).expect_err("refuses");
        assert!(refused.to_string().contains("exactly 32"), "{refused}");
        let _ = std::fs::remove_dir_all(&home);
    }

    #[cfg(unix)]
    #[test]
    fn a_created_identity_is_private() {
        use std::os::unix::fs::PermissionsExt as _;
        let home = scratch("mode");
        let (_identity, _) = load_or_create(&strict(), &home).expect("creates");
        let file = std::fs::metadata(home.join(SEED_FILE)).expect("stats");
        let dir = std::fs::metadata(&home).expect("stats");
        assert_eq!(file.permissions().mode() & 0o777, 0o600);
        assert_eq!(dir.permissions().mode() & 0o777, 0o700);
        let _ = std::fs::remove_dir_all(&home);
    }

    #[cfg(unix)]
    #[test]
    fn inside_a_private_home_the_file_mode_is_not_the_boundary() {
        // Documenting the model rather than asserting a wish. A home only its owner
        // can enter already makes the file unreachable, so a loose mode on the file
        // is not refused. If fs-mistrust ever tightens this, the test says so.
        use std::os::unix::fs::PermissionsExt as _;
        let home = scratch("loose-file");
        let (first, _) = load_or_create(&strict(), &home).expect("creates");
        std::fs::set_permissions(home.join(SEED_FILE), std::fs::Permissions::from_mode(0o644))
            .expect("loosens");
        let (second, origin) = load_or_create(&strict(), &home).expect("still loads");
        assert_eq!(origin, Origin::Loaded);
        assert_eq!(first.node_id(), second.node_id());
        let _ = std::fs::remove_dir_all(&home);
    }

    #[cfg(unix)]
    #[test]
    fn the_escape_hatch_accepts_what_the_check_refuses() {
        use std::os::unix::fs::PermissionsExt as _;
        let home = scratch("trusting");
        let (first, _) = load_or_create(&strict(), &home).expect("creates");
        std::fs::set_permissions(&home, std::fs::Permissions::from_mode(0o777))
            .expect("loosens");
        let (second, origin) =
            load_or_create(&Mistrust::new_dangerously_trust_everyone(), &home).expect("loads");
        assert_eq!(origin, Origin::Loaded);
        assert_eq!(first.node_id(), second.node_id());
        let _ = std::fs::set_permissions(&home, std::fs::Permissions::from_mode(0o700));
        let _ = std::fs::remove_dir_all(&home);
    }
}
