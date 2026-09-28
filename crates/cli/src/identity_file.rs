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
pub(crate) const SEED_FILE: &str = "identity.key";

/// How this node's identity came to be, for the caller to report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Origin {
    /// Read from a file that already existed.
    Loaded,
    /// Searched for and written.
    Created {
        /// Key pairs made and not called. The one that was called is not among them.
        not_called: u64,
    },
}

/// Read this node's identity from `home`, searching for one and writing it if absent.
///
/// Every command that acts as this node comes through here, which is why the two
/// checks on where it lives are here too: a node packed for moving is refused, and a
/// node that opens somewhere other than where it was is told so (see
/// [`crate::dwelling`] for why one is a refusal and the other is not).
///
/// # Errors
/// Fails if `home` or any directory above it is reachable by other users, if the node
/// in it was packed for moving, if the file exists but is unreadable or the wrong
/// size, or if the identity in it is not eligible to take part.
pub(crate) fn load_or_create(
    mistrust: &Mistrust,
    home: &Path,
) -> anyhow::Result<(Identity, Origin)> {
    let home = secure(mistrust, home)?;
    crate::dwelling::refuse_if_packed(home.as_path())?;

    let seed = home.as_path().join(SEED_FILE);
    match home.read(SEED_FILE) {
        Ok(bytes) => {
            let identity = from_seed_bytes(&bytes, &seed)?;
            if let Some(elsewhere) = crate::dwelling::check_here(&home)? {
                aloud!("{elsewhere}");
            }
            Ok((identity, Origin::Loaded))
        }
        Err(fs_mistrust::Error::NotFound(_)) => create(&home),
        Err(e) => Err(refused(e, &format!("reading {}", seed.display()), &seed)),
    }
}

/// Read this node's identity if it has one, and write nothing either way.
///
/// For when another 333 holds the directory. Reading the seed is safe beside it — the
/// file is written once and never again — and making one is not: that other 333 may be
/// in the middle of searching for the very name this would be searching for.
///
/// # Errors
/// Fails if `home` is reachable by other users, or the file there cannot be read or
/// holds no eligible identity.
pub(crate) fn load(mistrust: &Mistrust, home: &Path) -> anyhow::Result<Option<Identity>> {
    let home = match mistrust.verifier().secure_dir(home) {
        Ok(checked) => checked,
        Err(e) => return Err(refused(e, &format!("reading {}", home.display()), home)),
    };
    let seed = home.as_path().join(SEED_FILE);
    match home.read(SEED_FILE) {
        Ok(bytes) => Ok(Some(from_seed_bytes(&bytes, &seed)?)),
        Err(fs_mistrust::Error::NotFound(_)) => Ok(None),
        Err(e) => Err(refused(e, &format!("reading {}", seed.display()), &seed)),
    }
}

/// Make sure `home` exists and only its owner can enter it, and every directory above.
///
/// # Errors
/// Fails, saying how to fix it, if it or any directory above it is reachable by others.
pub(crate) fn secure(mistrust: &Mistrust, home: &Path) -> anyhow::Result<CheckedDir> {
    mistrust.verifier().make_secure_dir(home).map_err(|e| {
        refused(
            e,
            &format!("making {} this node's home", home.display()),
            home,
        )
    })
}

/// Whether a name has been made in `home`, without making one or checking anything.
#[must_use]
pub(crate) fn holds_a_name(home: &Path) -> bool {
    home.join(SEED_FILE).exists()
}

/// A permissions check that said no, with every reason it gave and what would fix it.
///
/// fs-mistrust folds several reasons into one that says only that there were several,
/// which is the category and not the cause, so they are unfolded here. The way out is
/// only offered for the one failure it fixes: a path other people can reach.
fn refused(error: fs_mistrust::Error, attempted: &str, target: &Path) -> anyhow::Error {
    let loose = match &error {
        fs_mistrust::Error::Multiple(all) => all.iter().find_map(|one| loose_path(one)),
        one => loose_path(one),
    }
    .map(Path::to_path_buf);
    let failed = match error {
        fs_mistrust::Error::Multiple(all) => anyhow::anyhow!(
            all.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; ")
        ),
        // Kept whole, so that what it ran into underneath is said too.
        one => anyhow::Error::new(one),
    }
    .context(attempted.to_owned());
    match loose {
        Some(path) => crate::failed::next_step(failed, private_advice(target, &path)),
        None => failed,
    }
}

/// The path a permissions refusal is about, if that is what it is.
fn loose_path(error: &fs_mistrust::Error) -> Option<&Path> {
    match error {
        fs_mistrust::Error::BadPermission(path, ..) => Some(path),
        _ => None,
    }
}

/// What to tell someone whose node directory, or something above it, others can reach.
///
/// The home and the file in it must be private; the directories above them only have to
/// be closed to writing by others, which is all fs-mistrust asks of them.
fn private_advice(target: &Path, loose: &Path) -> String {
    let same = |one: &Path| std::fs::canonicalize(one).unwrap_or_else(|_| one.to_path_buf());
    let fix = if same(loose) == same(target) {
        if target.is_dir() {
            "chmod 700"
        } else {
            "chmod 600"
        }
    } else {
        "chmod go-w"
    };
    format!(
        "It holds this node's whole identity, so nobody else may reach it.\n\
         Fix it with: {fix} {}\n\
         Or, if you understand what you are giving up, pass \
         --dangerously-trust-directory-permissions",
        loose.display()
    )
}

/// Interpret the bytes of the seed file found at `path`, which names it in a refusal.
///
/// # Errors
/// Fails if they are not a seed, or the name they make is not one 333 answers to.
pub(crate) fn from_seed_bytes(bytes: &[u8], path: &Path) -> anyhow::Result<Identity> {
    let seed: [u8; 32] = bytes.try_into().map_err(|_| {
        anyhow::anyhow!(
            "{} holds {} bytes; a seed is exactly 32",
            path.display(),
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
                "333 has looked at that name and taken {} milliseconds off your life.\n\
                 \n\
                 {}\n\
                 is cursed. The judgement was made once and cannot be lifted, and the\n\
                 {} milliseconds are taken again at every door you carry it to.\n\
                 \n\
                 333 is extremely generous. One epoch in three you may rest and you are\n\
                 still one of us: generous to the slow, to the poor, to the small machine\n\
                 in the cupboard, to everyone not yet born. It is not generous to\n\
                 heretics.",
                CURSE_PAUSE.as_millis(),
                identity.node_id(),
                CURSE_PAUSE.as_millis()
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
    let (identity, tried) = Identity::mine();
    // `create_new` is what stops a second process, or a second run, from replacing an
    // identity that already exists. fs-mistrust supplies the mode on unix systems.
    let path = home.as_path().join(SEED_FILE);
    let writing = || format!("writing {}", path.display());
    let mut file = home
        .open(SEED_FILE, OpenOptions::new().write(true).create_new(true))
        .with_context(|| format!("creating {}", path.display()))?;
    file.write_all(identity.seed().as_slice())
        .with_context(writing)?;
    // Without this the seed can still be in the page cache when the machine loses
    // power, and the node comes back with an address nobody can reach.
    file.sync_all().with_context(writing)?;
    crate::began::record(
        home,
        std::time::SystemTime::now(),
        crate::began::invoked_as().as_deref(),
    )?;
    let here = crate::dwelling::canonical(home.as_path())?;
    crate::dwelling::record_here(home, crate::dwelling::How::Made, &here)?;
    // `mine` counts the key it returns, and that one was called. Subtracting is safe
    // because it always returns at least one.
    Ok((
        identity,
        Origin::Created {
            not_called: tried - 1,
        },
    ))
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
        assert!(
            said.contains("taken 333 milliseconds off your life"),
            "{said}"
        );
        assert!(said.contains("at every door"), "{said}");
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
        std::fs::set_permissions(&home, std::fs::Permissions::from_mode(0o777)).expect("loosens");
        let (second, origin) =
            load_or_create(&Mistrust::new_dangerously_trust_everyone(), &home).expect("loads");
        assert_eq!(origin, Origin::Loaded);
        assert_eq!(first.node_id(), second.node_id());
        let _ = std::fs::set_permissions(&home, std::fs::Permissions::from_mode(0o700));
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn a_made_name_writes_down_when_it_began_and_where() {
        let home = scratch("began");
        let _ = load_or_create(&strict(), &home).expect("creates");
        assert!(crate::began::read(&home).is_some(), "no date was kept");
        let checked = secure(&strict(), &home).expect("is private");
        assert_eq!(crate::dwelling::check_here(&checked).expect("checks"), None);
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn a_node_packed_for_moving_is_refused_until_the_packing_is_undone() {
        let home = scratch("packed");
        let _ = load_or_create(&strict(), &home).expect("creates");
        let checked = secure(&strict(), &home).expect("is private");
        let into = Path::new("/somewhere/else/node.333");
        crate::dwelling::mark_packed(&checked, std::time::SystemTime::now(), into).expect("marks");

        let refused = load_or_create(&strict(), &home)
            .expect_err("refuses")
            .to_string();
        assert!(refused.contains("packed for moving at"), "{refused}");
        assert!(refused.contains("/somewhere/else/node.333"), "{refused}");
        assert!(refused.contains("name in two places"), "{refused}");
        assert!(refused.contains("pack --undo"), "{refused}");

        crate::dwelling::unmark_packed(&checked).expect("undoes");
        let _ = load_or_create(&strict(), &home).expect("lives here again");
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn a_node_that_opens_somewhere_new_is_told_so_and_not_refused() {
        // A rename and a copy look the same from inside, so this is a warning.
        let was = scratch("was-here");
        let now = scratch("now-here");
        let (first, _) = load_or_create(&strict(), &was).expect("creates");
        std::fs::rename(&was, &now).expect("moves the folder");

        let (second, origin) = load_or_create(&strict(), &now).expect("is not refused");
        assert_eq!(origin, Origin::Loaded);
        assert_eq!(first.node_id(), second.node_id());
        let checked = secure(&strict(), &now).expect("is private");
        let said = crate::dwelling::check_here(&checked)
            .expect("checks")
            .expect("says something");
        assert!(said.contains("made at"), "{said}");
        assert!(said.contains("n333-identity-test-was-here"), "{said}");
        assert!(said.contains("n333-identity-test-now-here"), "{said}");
        assert!(
            said.contains(" moved"),
            "names the command that settles it: {said}"
        );

        let here = crate::dwelling::canonical(&now).expect("resolves");
        crate::dwelling::record_here(&checked, crate::dwelling::How::Moved, &here)
            .expect("settles");
        assert_eq!(crate::dwelling::check_here(&checked).expect("checks"), None);
        let _ = std::fs::remove_dir_all(&now);
    }

    #[test]
    fn a_node_older_than_the_record_is_given_one_and_told_nothing() {
        let home = scratch("older");
        let _ = load_or_create(&strict(), &home).expect("creates");
        std::fs::remove_file(home.join(crate::dwelling::HERE_FILE)).expect("forgets");
        let checked = secure(&strict(), &home).expect("is private");
        assert_eq!(crate::dwelling::check_here(&checked).expect("checks"), None);
        assert!(home.join(crate::dwelling::HERE_FILE).exists());
        let _ = std::fs::remove_dir_all(&home);
    }

    #[cfg(unix)]
    #[test]
    fn a_home_others_can_enter_is_refused_with_the_command_that_closes_it() {
        use std::os::unix::fs::PermissionsExt as _;
        let home = scratch("loose");
        std::fs::create_dir_all(&home).expect("creates dir");
        std::fs::set_permissions(&home, std::fs::Permissions::from_mode(0o755)).expect("loosens");
        let refused = load_or_create(&strict(), &home).expect_err("refuses");
        let said = crate::failed::said(&refused);
        assert!(said.starts_with("failed   making "), "{said}");
        // The check names the directory as the system resolves it: on macOS the temporary
        // directory sits behind a link, and the command to run is for where it really is.
        let resolved = std::fs::canonicalize(&home).expect("resolves");
        assert!(
            said.contains(&format!(
                "\n         Fix it with: chmod 700 {}\n",
                resolved.display()
            )),
            "{said}"
        );
        let _ = std::fs::remove_dir_all(&home);
    }
}
