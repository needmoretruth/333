//! Thresholds, written as fractions rather than as percentages.
//!
//! FROZEN in shape, not in value: the thresholds themselves live where they are
//! used, but every one of them is compared through here.
//!
//! Each threshold in this protocol has a memorable name — two thirds present, one
//! third agreeing — and the design writes each of them a second time as a decimal:
//! 66.7%, 33.3%. The two spellings are not the same number. Over a window of 333,
//! two thirds is 222 and 66.7% is 223, so the doctrine's own sentence would be false
//! by one epoch, for ever, depending only on which spelling a programmer read first.
//!
//! So a threshold here is two integers, and deciding whether it is met is a
//! cross-multiplication. Nothing in this module divides. Division is for the number
//! shown to a person, which is what [`per_mille`] is for, and nothing decides on it.

/// The scale used when showing a ratio to a person: parts per thousand.
///
/// Reporting only. A tenth of a percent is as fine as any of these numbers ever
/// needs to be read, and it keeps the arithmetic in integers.
pub const PER_MILLE: u64 = 1000;

/// A threshold, as the fraction it is named by.
///
/// The fields are open because every value this protocol uses is a constant declared
/// beside the rule it belongs to, where a reader can see it. A denominator of zero
/// is meaningless rather than dangerous: it makes [`Fraction::is_met`] answer `true`
/// only when nothing was counted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fraction {
    /// The top of the fraction. Two, in "two thirds".
    pub numerator: u64,
    /// The bottom of the fraction. Three, in "two thirds".
    pub denominator: u64,
}

impl Fraction {
    /// Is `part` out of `whole` at least this fraction?
    ///
    /// The boundary is inclusive: exactly two thirds meets a threshold of two
    /// thirds. That is what "rest one hour in three and you are still counted"
    /// says, and it is the case both decimal spellings get wrong.
    ///
    /// Widened to 128 bits before multiplying, so that no count a real network
    /// could produce can turn the comparison around by overflowing it.
    #[must_use]
    pub const fn is_met(self, part: u64, whole: u64) -> bool {
        (part as u128) * (self.denominator as u128) >= (whole as u128) * (self.numerator as u128)
    }
}

/// `part` out of `whole` in parts per thousand, or `None` when `whole` is zero.
///
/// `None` is not zero. Nothing observed has no ratio, and printing 0‰ would be a
/// claim the record does not support.
#[must_use]
pub const fn per_mille(part: u64, whole: u64) -> Option<u64> {
    if whole == 0 {
        return None;
    }
    Some(part * PER_MILLE / whole)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TWO_THIRDS: Fraction = Fraction {
        numerator: 2,
        denominator: 3,
    };
    const ONE_THIRD: Fraction = Fraction {
        numerator: 1,
        denominator: 3,
    };

    #[test]
    fn the_boundary_is_inclusive_where_the_decimal_spelling_is_not() {
        // 222 of 333 is exactly two thirds. Compared as 667 per mille it fails, and
        // the doctrine's sentence would be false by one epoch out of 333.
        assert!(TWO_THIRDS.is_met(222, 333));
        assert!(!TWO_THIRDS.is_met(221, 333));
        assert!(TWO_THIRDS.is_met(2, 3));
        assert!(ONE_THIRD.is_met(111, 333));
        assert!(!ONE_THIRD.is_met(110, 333));
    }

    #[test]
    fn a_count_no_network_could_reach_does_not_turn_the_answer_around() {
        // In 64 bits, part * denominator wraps here and the comparison inverts.
        assert!(TWO_THIRDS.is_met(u64::MAX, u64::MAX));
        assert!(!TWO_THIRDS.is_met(1, u64::MAX));
        assert!(ONE_THIRD.is_met(u64::MAX / 2, u64::MAX));
    }

    #[test]
    fn nothing_counted_has_no_ratio_rather_than_a_ratio_of_zero() {
        assert_eq!(per_mille(0, 0), None);
        assert_eq!(per_mille(0, 7), Some(0));
        assert_eq!(per_mille(1, 3), Some(333));
        assert_eq!(per_mille(2, 3), Some(666));
        assert_eq!(per_mille(1, 1), Some(1000));
    }

    #[test]
    fn the_shown_ratio_and_the_decided_one_can_disagree_and_the_decision_wins() {
        // 2/3 shows as 666‰, which reads as below a 667‰ threshold, while the
        // decision correctly says the threshold is met. This is the whole reason
        // the two are separate functions.
        assert_eq!(per_mille(222, 333), Some(666));
        assert!(TWO_THIRDS.is_met(222, 333));
    }
}
