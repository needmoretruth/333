//! When this node's name was made, and by which command.
//!
//! Written once, beside the seed, on the run that makes the name. It is there so that
//! a person can say afterwards exactly when they agreed to take part: the moment a key
//! was called is the moment this machine began keeping hours for somebody, and a
//! client that did not write it down would leave them to guess.
//!
//! Plain text, two lines: the moment in UTC as ISO 8601 to the second, and the command
//! as it would have been typed. It travels with the node when the node is packed,
//! because the day somebody agreed does not change when their machine does.
//!
//! A node made before this client kept the date has no such file. It is said to have
//! begun before the date was kept, and is never given a date nobody wrote down.

use std::path::Path;
use std::time::SystemTime;

use anyhow::Context as _;
use fs_mistrust::CheckedDir;

/// The name of the file inside the node's directory.
pub(crate) const BEGAN_FILE: &str = "began";

/// What was written down when this node's name was made.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Began {
    /// The moment, as it was written: UTC, ISO 8601, to the second.
    pub(crate) at: String,
    /// The command that made it, as typed (`333 id`), if the client could tell.
    pub(crate) by: Option<String>,
}

/// A moment, the way this client writes one down: `2026-09-27T10:12:00Z`.
///
/// UTC and to the second, because it is read by people in every timezone and a
/// fraction of a second is not something anybody agreed to.
#[must_use]
pub(crate) fn utc(at: SystemTime) -> String {
    humantime::format_rfc3339_seconds(at).to_string()
}

/// Write down that the name was made at `at`, by `by`.
///
/// Called only on the run that made the seed, which is what keeps it the first run's
/// word. Anything already under this name belonged to a seed that is gone, and is
/// replaced rather than allowed to stop a new name being made.
///
/// # Errors
/// Fails if the file cannot be written.
pub(crate) fn record(home: &CheckedDir, at: SystemTime, by: Option<&str>) -> anyhow::Result<()> {
    let mut text = utc(at);
    text.push('\n');
    if let Some(by) = by {
        text.push_str(by);
        text.push('\n');
    }
    crate::dwelling::replace(home, BEGAN_FILE, text.as_bytes())
        .with_context(|| words!("began-writing"))
}

/// Read what was written down, if anything was.
#[must_use]
pub(crate) fn read(home: &Path) -> Option<Began> {
    let text = std::fs::read_to_string(home.join(BEGAN_FILE)).ok()?;
    let mut lines = text.lines();
    let at = lines.next().filter(|at| !at.is_empty())?.to_owned();
    let by = lines.next().filter(|by| !by.is_empty()).map(str::to_owned);
    Some(Began { at, by })
}

/// The line `333 id` says about it.
#[must_use]
pub(crate) fn describe(began: Option<&Began>) -> String {
    match began {
        Some(Began { at, by: Some(by) }) => words!("began-at-by", at = at, by = by),
        Some(Began { at, by: None }) => words!("began-at", at = at),
        None => words!("began-before"),
    }
}

/// The command this process was started as, the way a person would type it.
///
/// Read from the command line again rather than handed down, because the place that
/// makes a name is underneath every command that can, and threading the command's own
/// name through each of them would put a word about the caller into the opening of a
/// directory. It is the same parser `main` used, so it cannot read the line
/// differently; outside a real run (a test) it finds nothing and says so.
#[must_use]
pub(crate) fn invoked_as() -> Option<String> {
    let matches = <crate::Cli as clap::CommandFactory>::command()
        .try_get_matches_from(std::env::args_os())
        .ok()?;
    matches.subcommand_name().map(|name| format!("333 {name}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, UNIX_EPOCH};

    fn scratch(name: &str) -> (std::path::PathBuf, CheckedDir) {
        let dir = std::env::temp_dir().join(format!("n333-began-test-{name}"));
        let _ = std::fs::remove_dir_all(&dir);
        let checked = fs_mistrust::Mistrust::new_dangerously_trust_everyone()
            .verifier()
            .make_secure_dir(&dir)
            .expect("makes the dir");
        (dir, checked)
    }

    #[test]
    fn a_moment_is_written_in_utc_to_the_second() {
        // Pinned against a literal: this is what a person reads back years later.
        let at = UNIX_EPOCH + Duration::from_millis(1_790_503_920_500);
        assert_eq!(utc(at), "2026-09-27T10:12:00Z");
    }

    #[test]
    fn what_is_recorded_is_read_back_and_said() {
        let (dir, home) = scratch("round");
        let at = UNIX_EPOCH + Duration::from_secs(1_790_503_920);
        record(&home, at, Some("333 id")).expect("records");
        let began = read(&dir);
        assert_eq!(
            describe(began.as_ref()),
            "began    2026-09-27T10:12:00Z, by `333 id`"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn in_korean_the_date_begins_in_the_ninth_column() {
        use crate::words::count::Base;
        use crate::words::layout::{COLUMN, where_the_words_begin};
        let began = Began {
            at: "2026-09-27T10:12:00Z".to_owned(),
            by: Some("333 id".to_owned()),
        };
        let lines =
            crate::words::speaking("ko", Base::Ten, || [describe(Some(&began)), describe(None)]);
        assert_eq!(lines[0], "시작     2026-09-27T10:12:00Z, `333 id` 실행");
        for line in &lines {
            assert_eq!(where_the_words_begin(line), COLUMN, "{line:?}");
        }
    }

    #[test]
    fn a_node_older_than_the_record_is_not_given_a_date() {
        let (dir, _home) = scratch("older");
        assert_eq!(
            describe(read(&dir).as_ref()),
            "began    before this client kept the date"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
