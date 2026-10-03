//! What the status page says about the machine the site runs on: which release, since
//! when, and how long the machine has been up. Nothing that names a path, a user or an
//! address.
//!
//! The release is read once, when the server starts: `deploy` restarts the server after
//! every switch, so a release cannot change under a running server.

use std::path::Path;
use std::time::UNIX_EPOCH;

/// The file `deploy` writes beside `VERSION` with the whole commit.
const COMMIT_FILE: &str = "COMMIT";

/// The release this server runs.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct Release {
    /// The whole commit, 40 lower-case hex digits, if `deploy` wrote one.
    pub(crate) commit: Option<String>,
    /// When the release was staged, in milliseconds.
    pub(crate) deployed_at: Option<u64>,
}

impl Release {
    /// The release whose `VERSION` file is `version_file`.
    ///
    /// When it was deployed is the version file's modification time: `deploy` writes
    /// that file into the release just before switching to it and never touches it
    /// again. The directory's own time would move whenever anything inside it changed.
    pub(crate) fn beside(version_file: &Path) -> Self {
        let commit = std::fs::read_to_string(version_file.with_file_name(COMMIT_FILE))
            .ok()
            .map(|text| text.trim().to_owned())
            .filter(|commit| is_commit(commit));
        let deployed_at = std::fs::metadata(version_file)
            .and_then(|metadata| metadata.modified())
            .ok()
            .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
            .and_then(|since| u64::try_from(since.as_millis()).ok());
        Self {
            commit,
            deployed_at,
        }
    }
}

/// Is this a whole git commit, safe to put into a link as it is?
fn is_commit(text: &str) -> bool {
    text.len() == 40
        && text
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

/// How long the machine has been up, in whole seconds, from `/proc/uptime`.
#[cfg(target_os = "linux")]
pub(crate) fn uptime() -> Option<u64> {
    let text = std::fs::read_to_string("/proc/uptime").ok()?;
    let seconds = text.split_whitespace().next()?;
    seconds.split('.').next()?.parse().ok()
}

/// Only Linux says how long it has been up in a file anybody can read.
#[cfg(not(target_os = "linux"))]
pub(crate) fn uptime() -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_release_is_its_commit_and_the_time_its_version_was_written() {
        let dir = std::env::temp_dir().join(format!("n333-site-machine-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let version = dir.join("VERSION");
        std::fs::write(&version, "abc1234\n").unwrap();
        let commit = "a".repeat(40);
        std::fs::write(dir.join(COMMIT_FILE), format!("{commit}\n")).unwrap();
        let release = Release::beside(&version);
        assert_eq!(release.commit.as_deref(), Some(commit.as_str()));
        assert!(release.deployed_at.is_some_and(|at| at > 1_700_000_000_000));

        std::fs::write(dir.join(COMMIT_FILE), "<a>\n").unwrap();
        assert_eq!(Release::beside(&version).commit, None);
        assert_eq!(Release::beside(&dir.join("nothing")), Release::default());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn linux_says_how_long_it_has_been_up() {
        assert!(uptime().is_some());
    }
}
