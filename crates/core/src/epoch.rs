//! Epochs.
//!
//! An epoch is 333 minutes of wall-clock time, counted from the Unix epoch. There is
//! deliberately no external time source: no NTP requirement, no beacon, no chain. A
//! node that disagrees about the time simply fails to be attested by the nodes that
//! do agree, which is the only correction this protocol has.

use std::time::{SystemTime, UNIX_EPOCH};

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
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

/// Milliseconds since the Unix epoch, or 0 if the clock is set before 1970.
#[must_use]
pub fn unix_now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| u64::try_from(d.as_millis()).unwrap_or(u64::MAX))
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
    fn now_is_after_2020() {
        // 2020-01-01 is epoch 78949 at 333-minute steps; any sane clock is past it.
        assert!(Epoch::now() > Epoch::at_unix_seconds(1_577_836_800));
    }
}
