//! Presence: who is counted, and who still stands.
//!
//! FROZEN. The window and the threshold below decide standing, so they are part of
//! the protocol and not a tuning knob.
//!
//! The window MOVES. A cumulative ratio would let an old member be absent longer than
//! a new one for the same standing, and the rule would quietly stop meaning anything
//! as the years passed. Here, only the last 333 completed epochs count, for everybody,
//! for ever.
//!
//! There is no grace counter, no recovery timer, and no clean/forgiven distinction.
//! One ratio replaced all of them. What is written down is the number itself, not a
//! verdict derived from it — a reader can always recompute the verdict, and nobody can
//! recompute a number that was thrown away.

use crate::epoch::Epoch;

/// How many completed epochs the window covers. About 77 days.
pub const WINDOW_EPOCHS: u64 = 333;

/// Presence required to keep standing: two of every three.
///
/// The specification writes this as "66.7% or above" and, in the same table, as
/// "absence exceeding 33.3%". Both are the decimal rounding of one third, and at
/// exactly two thirds they disagree with the doctrine: over a 333-epoch window,
/// 66.7% demands 223 while two thirds demands 222. One epoch, for ever.
///
/// The doctrine's sentence — rest one hour in three and you are still counted — is
/// the older promise and the one a person can hold in their head, so the fraction is
/// frozen exactly and the percentage is understood as its rounding. Written as two
/// integers because a rule that rounds differently on two machines is not a rule.
pub const REQUIRED_NUMERATOR: u64 = 2;

/// The denominator of [`REQUIRED_NUMERATOR`].
pub const REQUIRED_DENOMINATOR: u64 = 3;

/// Scale used when reporting a ratio to a person. Reporting only; nothing decides on it.
pub const PER_MILLE: u64 = 1000;

/// What a record says about one epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Attendance {
    /// At least one of the three verifiers got a valid answer. One is enough: an
    /// answer carries a signature, and silence carries nothing.
    Present,
    /// All three verifiers published a challenge and none of them got an answer.
    Absent,
    /// No verifier published a challenge, so nothing was asked. The epoch leaves the
    /// denominator entirely — a node cannot be marked absent for a question that was
    /// never put to it.
    Excluded,
}

/// The tally over one window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Standing {
    /// Epochs in the window that counted — present or absent, excluding excluded.
    pub counted: u64,
    /// Of those, how many were answered.
    pub present: u64,
}

impl Standing {
    /// Presence in parts per thousand, or `None` when nothing was asked.
    ///
    /// `None` is not zero. A node nobody challenged has no ratio, and printing 0%
    /// would be a claim the record does not support.
    #[must_use]
    pub const fn per_mille(&self) -> Option<u64> {
        if self.counted == 0 {
            return None;
        }
        Some(self.present * PER_MILLE / self.counted)
    }

    /// Does this tally keep a member's standing?
    ///
    /// Compared by cross-multiplication, so the answer never depends on how a
    /// division rounded. A window in which nothing was asked keeps standing: the
    /// faith is lenient, and being unasked is not the same as being absent.
    #[must_use]
    pub const fn qualifies(&self) -> bool {
        if self.counted == 0 {
            return true;
        }
        self.present * REQUIRED_DENOMINATOR >= self.counted * REQUIRED_NUMERATOR
    }

    /// How many of the counted epochs were missed.
    #[must_use]
    pub const fn absent(&self) -> u64 {
        self.counted.saturating_sub(self.present)
    }
}

/// The epochs the window covers at `now`: the 333 completed epochs before it.
///
/// The current epoch is left out because it has not finished — its verifiers may not
/// have asked yet, and counting an unfinished epoch as absent would mark everybody
/// absent for a third of an hour, every hour.
#[must_use]
pub fn window(now: Epoch) -> std::ops::Range<u64> {
    now.0.saturating_sub(WINDOW_EPOCHS)..now.0
}

/// Tally a record over the window ending at `now`.
///
/// Entries outside the window are ignored, so a caller may pass its whole history.
/// An epoch with no entry at all is treated as [`Attendance::Excluded`]: the record
/// says nothing about it, and this protocol does not turn silence in the record into
/// an accusation.
pub fn standing_at<I>(now: Epoch, entries: I) -> Standing
where
    I: IntoIterator<Item = (Epoch, Attendance)>,
{
    let window = window(now);
    let mut standing = Standing {
        counted: 0,
        present: 0,
    };
    for (epoch, attendance) in entries {
        if !window.contains(&epoch.0) {
            continue;
        }
        match attendance {
            Attendance::Present => {
                standing.counted += 1;
                standing.present += 1;
            }
            Attendance::Absent => standing.counted += 1,
            Attendance::Excluded => {}
        }
    }
    standing
}

/// Tally a record over its whole life, for the second number a record shows.
pub fn lifetime<I>(entries: I) -> Standing
where
    I: IntoIterator<Item = (Epoch, Attendance)>,
{
    let mut standing = Standing {
        counted: 0,
        present: 0,
    };
    for (_epoch, attendance) in entries {
        match attendance {
            Attendance::Present => {
                standing.counted += 1;
                standing.present += 1;
            }
            Attendance::Absent => standing.counted += 1,
            Attendance::Excluded => {}
        }
    }
    standing
}

/// How many peers a node can see, in the shape the screen shows them.
///
/// The founder is deliberately not in any of these numbers. Counting the founder
/// would mean [`Census::is_ended`] could never become true, and the one question this
/// protocol exists to answer would have no answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Census {
    /// Peers answering now.
    pub active: u64,
    /// Of those, how many also serve the client's own source. A mark, not a rank.
    pub seeders: u64,
    /// Peers that still hold standing but are not answering.
    pub inactive: u64,
}

impl Census {
    /// Everyone on the roll: answering or not.
    #[must_use]
    pub const fn roll(&self) -> u64 {
        self.active + self.inactive
    }

    /// Has it ended?
    ///
    /// The count that decides this is how many are answering, not how many are on the
    /// roll. A roll of a thousand names with nobody awake is over.
    #[must_use]
    pub const fn is_ended(&self) -> bool {
        self.active == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(pattern: &[Attendance], from: u64) -> Vec<(Epoch, Attendance)> {
        pattern
            .iter()
            .enumerate()
            .map(|(i, a)| (Epoch(from + i as u64), *a))
            .collect()
    }

    #[test]
    fn the_frozen_numbers_are_the_agreed_ones() {
        assert_eq!(WINDOW_EPOCHS, 333);
        assert_eq!(REQUIRED_NUMERATOR, 2);
        assert_eq!(REQUIRED_DENOMINATOR, 3);
        assert_eq!(PER_MILLE, 1000);
    }

    #[test]
    fn the_window_is_the_333_completed_epochs_before_now() {
        assert_eq!(window(Epoch(1000)), 667..1000);
        assert_eq!(window(Epoch(1000)).count(), 333);
        // The current epoch is not in it: it has not finished.
        assert!(!window(Epoch(1000)).contains(&1000));
    }

    #[test]
    fn the_window_does_not_underflow_early_in_the_network() {
        assert_eq!(window(Epoch(0)), 0..0);
        assert_eq!(window(Epoch(5)), 0..5);
        assert_eq!(window(Epoch(5)).count(), 5);
    }

    #[test]
    fn excluded_epochs_leave_the_denominator() {
        let entries = record(
            &[
                Attendance::Present,
                Attendance::Excluded,
                Attendance::Excluded,
                Attendance::Absent,
            ],
            0,
        );
        let standing = standing_at(Epoch(4), entries);
        assert_eq!(
            standing,
            Standing {
                counted: 2,
                present: 1
            }
        );
        assert_eq!(standing.per_mille(), Some(500));
        assert!(!standing.qualifies());
    }

    #[test]
    fn the_threshold_is_two_thirds_and_the_boundary_is_inclusive() {
        // Two of every three, exactly, keeps standing. Written as 66.7% instead, this
        // case would fail — 2000 is not >= 2001 — and the doctrine's own sentence
        // would be false by one epoch.
        assert!(
            Standing {
                counted: 3,
                present: 2
            }
            .qualifies()
        );
        assert!(
            Standing {
                counted: 333,
                present: 222
            }
            .qualifies(),
            "222 of 333 is exactly two thirds and must keep standing"
        );
        assert!(
            !Standing {
                counted: 333,
                present: 221
            }
            .qualifies()
        );
        let at = Standing {
            counted: 1000,
            present: 667,
        };
        assert!(at.qualifies());
        let below = Standing {
            counted: 1000,
            present: 666,
        };
        assert!(!below.qualifies());
        assert!(
            !Standing {
                counted: 3,
                present: 1
            }
            .qualifies()
        );
    }

    #[test]
    fn a_window_nobody_asked_about_keeps_standing_and_has_no_ratio() {
        let standing = standing_at(Epoch(400), record(&[Attendance::Excluded; 10], 100));
        assert_eq!(standing.counted, 0);
        assert_eq!(standing.per_mille(), None);
        assert!(standing.qualifies());
    }

    #[test]
    fn entries_outside_the_window_are_ignored() {
        // Perfect attendance long ago, absent throughout the window: the window wins.
        let mut entries = record(&[Attendance::Present; 50], 0);
        entries.extend(record(&[Attendance::Absent; 10], 1000));
        let standing = standing_at(Epoch(1010), entries);
        assert_eq!(
            standing,
            Standing {
                counted: 10,
                present: 0
            }
        );
        assert!(!standing.qualifies());
    }

    #[test]
    fn the_window_moves_rather_than_accumulating() {
        // A member present for a thousand epochs and then absent for the whole window
        // loses standing. Under a cumulative ratio it would still be at 75%.
        let mut entries = record(&[Attendance::Present; 1000], 0);
        entries.extend(record(&[Attendance::Absent; 333], 1000));
        assert!(!standing_at(Epoch(1333), entries.clone()).qualifies());
        assert_eq!(lifetime(entries).per_mille(), Some(750));
    }

    #[test]
    fn a_missing_entry_is_not_an_accusation() {
        // Only two epochs of the window have any entry at all.
        let entries = vec![
            (Epoch(10), Attendance::Present),
            (Epoch(11), Attendance::Present),
        ];
        let standing = standing_at(Epoch(300), entries);
        assert_eq!(
            standing,
            Standing {
                counted: 2,
                present: 2
            }
        );
        assert!(standing.qualifies());
    }

    #[test]
    fn the_end_is_decided_by_who_is_answering_not_by_the_roll() {
        let census = Census {
            active: 0,
            seeders: 0,
            inactive: 1447,
        };
        assert!(census.is_ended());
        assert_eq!(census.roll(), 1447);

        let alive = Census {
            active: 1,
            seeders: 0,
            inactive: 1446,
        };
        assert!(!alive.is_ended());
    }

    #[test]
    fn a_census_reads_the_way_the_screen_shows_it() {
        let census = Census {
            active: 1203,
            seeders: 89,
            inactive: 244,
        };
        assert_eq!(census.roll(), 1447);
        assert!(census.seeders <= census.active, "a seeder is an active peer");
    }
}
