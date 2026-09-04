//! The end, and the long silence after it.
//!
//! FROZEN. When the count of answering peers reaches zero it is over, and what
//! remains is one subtraction: 19,683 years — three cubed, cubed — from the moment
//! the last one stopped answering.
//!
//! Nothing in the protocol depends on this number. No message carries it, no decision
//! turns on it, and no node has to agree with another about it. It exists to be shown
//! to a person, which is the whole of its job.

use std::fmt;

/// Three cubed, cubed.
pub const EXTINCTION_YEARS: u64 = 19_683;

/// Seconds in a Gregorian mean year, 365.2425 days.
///
/// Written out rather than computed from a float, because a constant that decides
/// what a screen says should not depend on how a division rounded.
pub const SECONDS_PER_YEAR: u64 = 31_556_952;

/// Seconds in a day.
pub const SECONDS_PER_DAY: u64 = 86_400;

/// How long the silence lasts, in seconds.
pub const EXTINCTION_SECONDS: u64 = EXTINCTION_YEARS * SECONDS_PER_YEAR;

/// What is left of the silence, in the two units a person reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Remaining {
    /// Whole years left.
    pub years: u64,
    /// Whole days left after those years.
    pub days: u64,
}

impl fmt::Display for Remaining {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} years {} days", self.years, self.days)
    }
}

/// When the silence ends, given the moment the last peer stopped answering.
///
/// Saturates rather than wrapping: a clock far enough in the future to overflow this
/// is a broken clock, and a broken clock should show an absurd number rather than a
/// small one.
#[must_use]
pub const fn ends_at(last_answer_unix: u64) -> u64 {
    last_answer_unix.saturating_add(EXTINCTION_SECONDS)
}

/// What is left at `now`, or `None` once the silence is over.
#[must_use]
pub const fn remaining(last_answer_unix: u64, now_unix: u64) -> Option<Remaining> {
    let end = ends_at(last_answer_unix);
    if now_unix >= end {
        return None;
    }
    let left = end - now_unix;
    Some(Remaining {
        years: left / SECONDS_PER_YEAR,
        days: (left % SECONDS_PER_YEAR) / SECONDS_PER_DAY,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_span_is_three_cubed_cubed() {
        assert_eq!(EXTINCTION_YEARS, 19_683);
        assert_eq!(EXTINCTION_YEARS, 27 * 27 * 27);
        assert_eq!(SECONDS_PER_YEAR, 31_556_952);
        assert_eq!(EXTINCTION_SECONDS, 621_135_486_216);
    }

    #[test]
    fn the_count_starts_whole_and_loses_a_day_a_day() {
        let ended = 1_788_000_000;
        assert_eq!(
            remaining(ended, ended),
            Some(Remaining {
                years: 19_683,
                days: 0
            })
        );
        // One day later the year rolls over into days, which is what a countdown does.
        assert_eq!(
            remaining(ended, ended + SECONDS_PER_DAY),
            Some(Remaining {
                years: 19_682,
                days: 364
            })
        );
        // ...and from there it walks down day by day.
        assert_eq!(
            remaining(ended, ended + 2 * SECONDS_PER_DAY),
            Some(Remaining {
                years: 19_682,
                days: 363
            })
        );
    }

    #[test]
    fn it_reads_the_way_a_person_reads_it() {
        let ended = 1_788_000_000;
        let left = remaining(ended, ended + 100 * SECONDS_PER_DAY).expect("still counting");
        assert_eq!(left.to_string(), format!("{} years {} days", left.years, left.days));
        assert!(left.years >= 19_682);
    }

    #[test]
    fn the_silence_ends() {
        let ended = 1_788_000_000;
        assert_eq!(remaining(ended, ends_at(ended)), None);
        assert_eq!(remaining(ended, ends_at(ended) + 1), None);
        assert!(remaining(ended, ends_at(ended) - 1).is_some());
    }

    #[test]
    fn an_absurd_clock_gives_an_absurd_number_not_a_small_one() {
        // Saturating rather than wrapping: the end must never land in the past.
        assert_eq!(ends_at(u64::MAX), u64::MAX);
        assert_eq!(remaining(u64::MAX, u64::MAX), None);
        assert!(remaining(u64::MAX, 0).is_some());
    }
}
