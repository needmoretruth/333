//! Epochs.
//!
//! An epoch is 333 minutes of wall-clock time, counted from the Unix epoch. There is
//! deliberately no external time source: no NTP requirement, no beacon, no chain. A
//! node that disagrees about the time simply fails to be attested by the nodes that
//! do agree, which is the only correction this protocol has.

use std::time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH};

/// 333 minutes, in seconds. Frozen by the specification.
pub const EPOCH_SECONDS: u64 = 333 * 60;

/// An epoch number: `unix_seconds / EPOCH_SECONDS`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Epoch(pub u64);

impl Epoch {
    /// The epoch containing the given Unix timestamp.
    #[must_use]
    pub const fn at_unix_seconds(unix_seconds: u64) -> Self {
        Self(unix_seconds / EPOCH_SECONDS)
    }

    /// The epoch this node believes it is in, from its own wall clock.
    ///
    /// A clock set before 1970 reads as epoch 0 rather than failing: the protocol
    /// has no authority to appeal to, so a nonsensical clock is another node's
    /// observation to report, not an error to raise here.
    #[must_use]
    pub fn now() -> Self {
        Self::at_unix_seconds(unix_now_seconds())
    }

    /// The wall-clock second this epoch begins at.
    ///
    /// Saturates rather than wrapping, so an absurd epoch produces an absurd time
    /// instead of one in the past.
    #[must_use]
    pub const fn starts_at_unix_seconds(self) -> u64 {
        self.0.saturating_mul(EPOCH_SECONDS)
    }

    /// Distance to another epoch, signed, saturating.
    ///
    /// Used to report how far a peer's clock is from ours. Reporting is all this
    /// protocol does with the answer.
    #[must_use]
    pub fn skew_from(self, other: Self) -> i64 {
        let (a, b) = (self.0, other.0);
        if a >= b {
            i64::try_from(a - b).unwrap_or(i64::MAX)
        } else {
            i64::try_from(b - a).map_or(i64::MIN, |d| -d)
        }
    }
}

impl std::fmt::Display for Epoch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Seconds since the Unix epoch, or 0 if the clock is set before 1970.
#[must_use]
pub fn unix_now_seconds() -> u64 {
    seconds_since_epoch(SystemTime::now().duration_since(UNIX_EPOCH))
}

/// Milliseconds since the Unix epoch, or 0 if the clock is set before 1970.
#[must_use]
pub fn unix_now_millis() -> u64 {
    millis_since_epoch(SystemTime::now().duration_since(UNIX_EPOCH))
}

/// Reading the clock and interpreting it are separated so that both documented
/// fallbacks — a clock set before 1970, and a duration too large to express in
/// milliseconds — are reachable from a test. A global clock a test cannot stub is
/// the one piece of I/O this crate would otherwise contain.
fn seconds_since_epoch(since_epoch: Result<Duration, SystemTimeError>) -> u64 {
    since_epoch.map_or(0, |d| d.as_secs())
}

fn millis_since_epoch(since_epoch: Result<Duration, SystemTimeError>) -> u64 {
    since_epoch.map_or(0, |d| u64::try_from(d.as_millis()).unwrap_or(u64::MAX))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_is_333_minutes() {
        assert_eq!(EPOCH_SECONDS, 19_980);
    }

    #[test]
    fn epoch_boundaries_are_exact() {
        assert_eq!(Epoch::at_unix_seconds(0), Epoch(0));
        assert_eq!(Epoch::at_unix_seconds(EPOCH_SECONDS - 1), Epoch(0));
        assert_eq!(Epoch::at_unix_seconds(EPOCH_SECONDS), Epoch(1));
        assert_eq!(Epoch::at_unix_seconds(EPOCH_SECONDS * 333), Epoch(333));
    }

    #[test]
    fn skew_is_signed_and_saturates() {
        assert_eq!(Epoch(10).skew_from(Epoch(7)), 3);
        assert_eq!(Epoch(7).skew_from(Epoch(10)), -3);
        assert_eq!(Epoch(5).skew_from(Epoch(5)), 0);
        assert_eq!(Epoch(u64::MAX).skew_from(Epoch(0)), i64::MAX);
    }

    #[test]
    fn a_known_date_lands_in_a_known_epoch() {
        // 2020-01-01T00:00:00Z is 1_577_836_800 seconds, which is 78,970 whole
        // epochs of 19,980 seconds with 16,200 left over.
        assert_eq!(Epoch::at_unix_seconds(1_577_836_800), Epoch(78_970));
    }

    #[test]
    fn now_is_the_epoch_containing_the_current_second() {
        // A whole-epoch offset in `now` would survive any test that only asserts it
        // is large. Reading the clock either side pins it without a race.
        let before = unix_now_seconds();
        let now = Epoch::now();
        let after = unix_now_seconds();
        assert!(
            now == Epoch::at_unix_seconds(before) || now == Epoch::at_unix_seconds(after),
            "Epoch::now() was {now}, not the epoch of {before}..={after}"
        );
    }

    #[test]
    fn a_clock_set_before_1970_reads_as_zero() {
        let backwards = UNIX_EPOCH.duration_since(SystemTime::now());
        assert!(
            backwards.is_err(),
            "this machine's clock is before 1970, so the error arm is untested"
        );
        assert_eq!(seconds_since_epoch(backwards), 0);
        assert_eq!(
            millis_since_epoch(UNIX_EPOCH.duration_since(SystemTime::now())),
            0
        );
    }

    #[test]
    fn milliseconds_saturate_rather_than_wrap() {
        assert_eq!(millis_since_epoch(Ok(Duration::from_secs(3))), 3_000);
        assert_eq!(millis_since_epoch(Ok(Duration::MAX)), u64::MAX);
    }

    #[test]
    fn the_millisecond_clock_is_not_a_second_clock() {
        // Returning seconds where milliseconds are promised puts a value a thousand
        // times too small on a wire field that cannot be changed later.
        let seconds = unix_now_seconds();
        let millis = unix_now_millis();
        assert!(
            millis / 1000 + 1 >= seconds && millis / 1000 <= seconds + 1,
            "milliseconds {millis} and seconds {seconds} disagree"
        );
    }
}
