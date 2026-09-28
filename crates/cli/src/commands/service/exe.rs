//! Which program a service would run, and whether it would still be there tomorrow.
//!
//! A service names one file and runs it at every boot. If that file is in a build
//! directory or somewhere the system empties, the service works today and fails on
//! the morning after the next `cargo clean` or reboot, silently and for ever. So the
//! program asked to install itself refuses to when it is running from somewhere like
//! that, and says where to put it first.

use std::path::{Path, PathBuf};

use anyhow::{Context as _, bail};

/// This program's own path, if it is somewhere a service can rely on.
///
/// # Errors
/// Fails if the system cannot say where this program is, or if it is somewhere that
/// does not last.
pub(crate) fn lasting() -> anyhow::Result<PathBuf> {
    let exe = std::env::current_exe().with_context(|| words!("service-exe-asking"))?;
    let exe = exe.canonicalize().unwrap_or(exe);
    if let Some(why) = temporary(&exe, &temporary_places()) {
        bail!(words!(
            "service-exe-fleeting",
            exe = exe.display().to_string(),
            why = why,
            home = suggested_home()
        ));
    }
    Ok(exe)
}

/// Whether a node directory is somewhere the system empties.
///
/// Not refused: a scratch node kept as a service is a reasonable thing to try. It is
/// said out loud, because a reboot that empties it takes the node's name with it.
#[must_use]
pub(crate) fn fleeting(directory: &Path) -> bool {
    let directory = std::path::absolute(directory).unwrap_or_else(|_| directory.to_path_buf());
    temporary_places()
        .iter()
        .any(|place| directory.starts_with(place))
}

/// The places this system empties on its own.
fn temporary_places() -> Vec<PathBuf> {
    let mut places = vec![std::env::temp_dir()];
    if let Ok(canonical) = std::env::temp_dir().canonicalize() {
        places.push(canonical);
    }
    if cfg!(unix) {
        places.extend(["/tmp", "/var/tmp", "/dev/shm"].map(PathBuf::from));
    }
    places
}

/// Why this path is no place for a service's program, if it is not.
///
/// A build directory is recognised by what cargo writes at the top of every target
/// directory, rather than by its name: a target directory can be called anything, and
/// a directory called `target` can be somebody's real one. `.rustc_info.json` is
/// always there; `CACHEDIR.TAG` only in some versions.
fn temporary(exe: &Path, places: &[PathBuf]) -> Option<String> {
    if places.iter().any(|place| exe.starts_with(place)) {
        return Some(words!("service-exe-emptied"));
    }
    let built = exe
        .ancestors()
        .skip(1)
        .any(|dir| dir.join(".rustc_info.json").is_file() || dir.join("CACHEDIR.TAG").is_file());
    built.then(|| words!("service-exe-build-directory"))
}

/// Where a person would put this program on this system, said as they would type it.
const fn suggested_home() -> &'static str {
    if cfg!(windows) {
        r"%LOCALAPPDATA%\Programs\333\333.exe"
    } else if cfg!(target_os = "macos") {
        "~/bin/333"
    } else {
        "~/.local/bin/333"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let pairs = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            [
                (
                    words!("service-exe-asking"),
                    "asking the system where this program is",
                ),
                (
                    words!(
                        "service-exe-fleeting",
                        exe = "/tmp/x/333",
                        why = words!("service-exe-emptied"),
                        home = "~/.local/bin/333"
                    ),
                    "this program is running from /tmp/x/333, which is a directory the system \
                     empties. A service pointed at it would stop the day that goes. Copy it to \
                     ~/.local/bin/333 and run `service install` from there.",
                ),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

    #[test]
    fn a_program_under_a_temporary_directory_is_refused() {
        let places = [PathBuf::from("/tmp")];
        assert!(temporary(Path::new("/tmp/x/333"), &places).is_some());
        assert!(temporary(Path::new("/usr/bin/333"), &places).is_none());
    }

    #[test]
    fn a_program_inside_a_cargo_target_directory_is_refused_whatever_it_is_called() {
        let target = std::env::temp_dir().join(format!("333-exe-{}", std::process::id()));
        let release = target.join("release");
        std::fs::create_dir_all(&release).unwrap();
        // Only the one file every cargo writes: the target directory this was first
        // tried from had no CACHEDIR.TAG, and a check that wanted both let it through.
        std::fs::write(target.join(".rustc_info.json"), "").unwrap();
        let why = temporary(&release.join("333"), &[]);
        std::fs::remove_dir_all(&target).unwrap();
        assert_eq!(
            why,
            Some("a build directory, which the next build or clean replaces".to_owned())
        );
    }

    #[test]
    fn a_node_under_the_temporary_directory_is_said_to_be_fleeting() {
        assert!(fleeting(&std::env::temp_dir().join("333-a-node")));
    }
}
