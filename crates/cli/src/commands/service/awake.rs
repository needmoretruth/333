//! The line a running vigil writes to say it is still awake, and what reading it says.
//!
//! One small file in the node's directory holding the time, rewritten about once a
//! minute while `serve` runs. It is how every other command on this machine can tell
//! whether the vigil is being kept without asking the service manager, which differs
//! on every system, and without asking the network, which would be asking somebody
//! else to say where this node stands.
//!
//! IT NEVER LEAVES THE MACHINE. Nothing reads it but this client on this machine; it is
//! not in any statement, not on the wire and not at the meeting point. What it says
//! about this node is said to the person who owns it and to nobody else.

use std::path::{Path, PathBuf};
use std::time::Duration;

use n333_core::epoch::{EPOCH_SECONDS, unix_now_seconds};

/// The file, inside the node's directory.
pub(crate) const STAMP: &str = "awake";

/// How often a running vigil says it is awake.
const EVERY: Duration = Duration::from_secs(60);

/// How old the stamp may be before a vigil with a service behind it counts as stopped.
///
/// Three of the minute it is written every, so that one late write — a machine busy
/// compiling, a slow disk — is not reported as a vigil that stopped.
const INSTALLED_PATIENCE: u64 = 3 * 60;

/// Keep saying the vigil is awake, for as long as it is.
///
/// A failure to write is said once and not every minute after it: a disk that is full
/// is full for a while, and the vigil goes on answering whatever this says.
pub(crate) async fn keep_saying(root: PathBuf) {
    let mut every = tokio::time::interval(EVERY);
    let mut said_it_failed = false;
    loop {
        every.tick().await;
        match write(&root, unix_now_seconds()) {
            Ok(()) => said_it_failed = false,
            Err(e) if !said_it_failed => {
                said_it_failed = true;
                aloud!(
                    "failed   writing that the vigil is awake, in {}: {e}. Nothing on this\n\
                     \x20        machine can tell that it is being kept until that works again.",
                    root.display()
                );
            }
            Err(_) => {}
        }
    }
}

/// Write the stamp, whole or not at all.
///
/// Written beside it and renamed over it, because a reader that opened it halfway
/// through a write would read a time that never happened.
///
/// # Errors
/// Fails if the directory cannot be written to.
pub(crate) fn write(root: &Path, unix: u64) -> std::io::Result<()> {
    let fresh = root.join(format!("{STAMP}.new"));
    std::fs::write(&fresh, format!("{}\n", iso(unix)))?;
    std::fs::rename(&fresh, root.join(STAMP))
}

/// When the vigil in this directory last said it was awake, if it ever has.
#[must_use]
pub(crate) fn read(root: &Path) -> Option<u64> {
    std::fs::read_to_string(root.join(STAMP))
        .ok()
        .and_then(|text| parse_iso(text.trim()))
}

/// The one line that says the vigil has not been kept, when it has not.
///
/// `installed` is whether a service was set up for this directory. With one, a few
/// minutes of silence already means it stopped; without one, a person running `serve`
/// by hand is told only once it has been gone for an epoch, because stopping a vigil
/// kept by hand is usually something they did on purpose.
#[must_use]
pub(crate) fn not_kept(last: Option<u64>, installed: bool, now: u64) -> Option<String> {
    let Some(last) = last else {
        return installed.then(|| {
            "vigil    not kept. The service is installed and has never said it was awake.\n\
             \x20        `333 service status` says why."
                .to_owned()
        });
    };
    let gone = now.saturating_sub(last);
    let late = if installed {
        gone > INSTALLED_PATIENCE
    } else {
        gone > EPOCH_SECONDS
    };
    late.then(|| {
        format!(
            "vigil    not kept since {}, {} ago. `333 service status` says why.",
            iso(last),
            how_long(gone)
        )
    })
}

/// How long ago, in epochs once there has been one and in minutes before that.
#[must_use]
pub(crate) fn how_long(seconds: u64) -> String {
    let epochs = seconds / EPOCH_SECONDS;
    let minutes = seconds / 60;
    match (epochs, minutes) {
        (0, 0) => "under a minute".to_owned(),
        (0, 1) => "1 minute".to_owned(),
        (0, _) => format!("{minutes} minutes"),
        (1, _) => "1 epoch".to_owned(),
        (_, _) => format!("{epochs} epochs"),
    }
}

/// Seconds since 1970 as `2026-09-24T03:10:00Z`, in UTC.
///
/// Worked out here rather than taken from a date library because the smallest edition
/// carries none, and a date library for one line of output is most of a megabyte on
/// the machine that can least spare it. The arithmetic is Howard Hinnant's
/// `civil_from_days`, which is exact for every day this could ever be run on.
#[must_use]
pub(crate) fn iso(unix: u64) -> String {
    let (days, of_day) = (unix / 86_400, unix % 86_400);
    let shifted = days + 719_468;
    let era = shifted / 146_097;
    let day_of_era = shifted % 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_from_march = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_from_march + 2) / 5 + 1;
    let month = if month_from_march < 10 {
        month_from_march + 3
    } else {
        month_from_march - 9
    };
    let year = year_of_era + era * 400 + u64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        of_day / 3_600,
        of_day % 3_600 / 60,
        of_day % 60
    )
}

/// The reverse of [`iso`], for exactly the form it writes and nothing looser.
#[must_use]
pub(crate) fn parse_iso(text: &str) -> Option<u64> {
    let field = |from: usize, to: usize| text.get(from..to)?.parse::<u64>().ok();
    let separators = [
        (4, b'-'),
        (7, b'-'),
        (10, b'T'),
        (13, b':'),
        (16, b':'),
        (19, b'Z'),
    ];
    if text.len() != 20
        || separators
            .iter()
            .any(|&(at, byte)| text.as_bytes().get(at) != Some(&byte))
    {
        return None;
    }
    let (year, month, day) = (field(0, 4)?, field(5, 7)?, field(8, 10)?);
    let (hour, minute, second) = (field(11, 13)?, field(14, 16)?, field(17, 19)?);
    if year < 1970 || !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    if hour > 23 || minute > 59 || second > 59 {
        return None;
    }
    let year = if month <= 2 { year - 1 } else { year };
    let era = year / 400;
    let year_of_era = year - era * 400;
    let month_from_march = if month > 2 { month - 3 } else { month + 9 };
    let day_of_year = (153 * month_from_march + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    let days = (era * 146_097 + day_of_era).checked_sub(719_468)?;
    Some(days * 86_400 + hour * 3_600 + minute * 60 + second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_time_is_written_the_way_the_rest_of_the_world_writes_it() {
        assert_eq!(iso(0), "1970-01-01T00:00:00Z");
        assert_eq!(iso(951_782_400), "2000-02-29T00:00:00Z");
        assert_eq!(iso(1_790_046_600), "2026-09-22T03:10:00Z");
        assert_eq!(iso(4_107_542_399), "2100-02-28T23:59:59Z");
    }

    #[test]
    fn what_is_written_reads_back_as_the_same_second() {
        for unix in [
            0,
            59,
            951_782_400,
            1_790_046_600,
            4_107_542_399,
            4_107_542_400,
        ] {
            assert_eq!(parse_iso(&iso(unix)), Some(unix), "{}", iso(unix));
        }
    }

    #[test]
    fn anything_but_the_form_it_writes_is_not_read_as_a_time() {
        for text in [
            "",
            "2026-09-22 03:10:00Z",
            "2026-09-22T03:10:00",
            "2026-13-22T03:10:00Z",
            "2026-09-22T24:10:00Z",
            "1969-12-31T23:59:59Z",
            "2026-09-22T03:10:00Z\n",
        ] {
            assert_eq!(parse_iso(text), None, "{text:?}");
        }
    }

    #[test]
    fn the_stamp_reads_back_after_it_is_written_and_leaves_nothing_half_done() {
        let root = std::env::temp_dir().join(format!("333-awake-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        write(&root, 1_790_046_600).unwrap();
        assert_eq!(read(&root), Some(1_790_046_600));
        assert!(!root.join("awake.new").exists());
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn a_vigil_with_a_service_behind_it_is_missed_within_minutes() {
        let last = 1_790_046_600;
        assert_eq!(not_kept(Some(last), true, last + 120), None);
        let line = not_kept(Some(last), true, last + 12 * EPOCH_SECONDS + 5).unwrap();
        assert_eq!(
            line,
            "vigil    not kept since 2026-09-22T03:10:00Z, 12 epochs ago. \
             `333 service status` says why."
        );
    }

    #[test]
    fn a_vigil_kept_by_hand_is_missed_only_after_an_epoch() {
        let last = 1_790_046_600;
        assert_eq!(not_kept(Some(last), false, last + EPOCH_SECONDS), None);
        assert!(not_kept(Some(last), false, last + EPOCH_SECONDS + 1).is_some());
    }

    #[test]
    fn a_node_that_never_kept_a_vigil_is_told_nothing_unless_a_service_was_set_up() {
        assert_eq!(not_kept(None, false, 1_790_046_600), None);
        assert!(not_kept(None, true, 1_790_046_600).is_some());
    }

    #[test]
    fn how_long_is_said_in_minutes_until_an_epoch_has_gone() {
        assert_eq!(how_long(59), "under a minute");
        assert_eq!(how_long(61), "1 minute");
        assert_eq!(how_long(7 * 60), "7 minutes");
        assert_eq!(how_long(EPOCH_SECONDS), "1 epoch");
        assert_eq!(how_long(12 * EPOCH_SECONDS + 5), "12 epochs");
    }
}
