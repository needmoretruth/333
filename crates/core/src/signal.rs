//! Signal: the one thing a node may say each epoch, and what a node makes of what
//! it hears.
//!
//! FROZEN. The count of signals and the threshold are part of the protocol.
//!
//! A signal is an index and nothing else. There are 333 of them, numbered 0 to 332,
//! and what travels between nodes is the number. The words those numbers stand for
//! are not decided yet and are not in this crate; when they are, they will be one
//! table that is never translated. Translating it would split the count — a hundred
//! nodes agreeing on one word would be counted as several different words, one per
//! language, and the number this protocol exists to show would be wrong in a way
//! nobody could see.
//!
//! Every node decides on its own. There is no consensus round, no quorum and no
//! authority: a node tallies what it saw, and two nodes that saw different things
//! report different things. That is not a fault to be corrected.

use crate::ratio::{Fraction, per_mille};

/// How many signals there are. Numbered 0 to 332.
pub const SIGNAL_COUNT: u16 = 333;

/// The share of observed peers that makes a signal worth marking on the screen.
///
/// The design writes this as "33.3% or above", which is the decimal rounding of one
/// third and disagrees with it at exactly one third. Frozen as the fraction, for the
/// same reason the presence threshold is: see [`crate::ratio`].
///
/// It decides nothing but what a screen shows. No message carries it and no node has
/// to agree with another about it.
pub const THRESHOLD: Fraction = Fraction {
    numerator: 1,
    denominator: 3,
};

/// One of the 333 signals.
///
/// A number, never a word. The type exists so that an index out of range cannot be
/// carried around waiting to be looked up.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Signal(u16);

impl Signal {
    /// The signal with this index, or `None` if there is no such signal.
    #[must_use]
    pub const fn new(index: u16) -> Option<Self> {
        if index < SIGNAL_COUNT {
            Some(Self(index))
        } else {
            None
        }
    }

    /// This signal's index: what goes on the wire.
    #[must_use]
    pub const fn index(self) -> u16 {
        self.0
    }

    /// Every signal, in index order.
    pub fn all() -> impl Iterator<Item = Self> {
        (0..SIGNAL_COUNT).map(Self)
    }
}

impl std::fmt::Display for Signal {
    /// Shown the way the design writes it: `#187`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

/// What one node heard during one epoch.
///
/// The denominator is every active peer that was observed, not just the ones that
/// said something. A signal chosen by a third of those who spoke, when only a
/// handful spoke, has not been chosen by a third of the network, and the screen must
/// not say it has.
///
/// One observation per peer per epoch is the caller's promise to keep: only the
/// caller knows which peer said what. Calling [`Tally::observe`] twice for one peer
/// counts that peer twice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tally {
    /// Active peers observed this epoch, silent ones included.
    observed: u64,
    /// How many chose each signal, indexed by signal number.
    counts: [u64; SIGNAL_COUNT as usize],
}

impl Default for Tally {
    fn default() -> Self {
        Self::new()
    }
}

impl Tally {
    /// An epoch nobody has been heard from yet.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            observed: 0,
            counts: [0; SIGNAL_COUNT as usize],
        }
    }

    /// Record one observed peer, and the signal it chose if it chose one.
    ///
    /// A peer that said nothing still counts towards [`Tally::observed`]: silence is
    /// part of what the epoch looked like.
    pub fn observe(&mut self, chosen: Option<Signal>) {
        self.observed = self.observed.saturating_add(1);
        if let Some(signal) = chosen
            && let Some(count) = self.counts.get_mut(usize::from(signal.index()))
        {
            *count = count.saturating_add(1);
        }
    }

    /// Tally a whole epoch's observations at once, one entry per observed peer.
    pub fn of<I>(observations: I) -> Self
    where
        I: IntoIterator<Item = Option<Signal>>,
    {
        let mut tally = Self::new();
        for chosen in observations {
            tally.observe(chosen);
        }
        tally
    }

    /// How many active peers were observed, whether or not they said anything.
    #[must_use]
    pub const fn observed(&self) -> u64 {
        self.observed
    }

    /// How many chose this signal.
    #[must_use]
    pub fn count(&self, signal: Signal) -> u64 {
        self.counts
            .get(usize::from(signal.index()))
            .copied()
            .unwrap_or(0)
    }

    /// How many observed peers said nothing at all.
    #[must_use]
    pub fn silent(&self) -> u64 {
        self.observed.saturating_sub(self.spoken())
    }

    /// How many observed peers chose some signal.
    #[must_use]
    pub fn spoken(&self) -> u64 {
        self.counts.iter().copied().fold(0, u64::saturating_add)
    }

    /// This signal's share of the observed peers, in parts per thousand.
    ///
    /// `None` when no peers were observed. For showing, never for deciding.
    #[must_use]
    pub fn share(&self, signal: Signal) -> Option<u64> {
        per_mille(self.count(signal), self.observed)
    }

    /// Has this signal been chosen by at least [`THRESHOLD`] of the observed peers?
    ///
    /// An epoch in which nothing was observed reaches nothing. Without that line the
    /// cross-multiplication would say every signal had reached the threshold, since
    /// zero is a third of zero.
    #[must_use]
    pub fn reached(&self, signal: Signal) -> bool {
        self.observed != 0 && THRESHOLD.is_met(self.count(signal), self.observed)
    }

    /// Every signal that reached the threshold, in index order.
    ///
    /// More than one can, and no winner is picked from among them. Up to three
    /// signals can hold a third each, and a screen that showed only the largest
    /// would be inventing a contest the protocol does not hold.
    pub fn at_threshold(&self) -> impl Iterator<Item = Signal> + '_ {
        Signal::all().filter(|s| self.reached(*s))
    }

    /// All 333 signals with their counts, in index order.
    ///
    /// The whole distribution, including the zeroes. The design asks for the shape
    /// of what the network said, not for its loudest word.
    pub fn distribution(&self) -> impl Iterator<Item = (Signal, u64)> + '_ {
        Signal::all().map(|s| (s, self.count(s)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn signal(index: u16) -> Signal {
        Signal::new(index).expect("a signal in range")
    }

    fn many(signal: Signal, times: usize) -> Vec<Option<Signal>> {
        vec![Some(signal); times]
    }

    #[test]
    fn the_frozen_numbers_are_the_agreed_ones() {
        assert_eq!(SIGNAL_COUNT, 333);
        assert_eq!(THRESHOLD.numerator, 1);
        assert_eq!(THRESHOLD.denominator, 3);
    }

    #[test]
    fn there_are_exactly_333_signals_numbered_from_zero() {
        assert_eq!(Signal::all().count(), 333);
        assert_eq!(signal(0).index(), 0);
        assert_eq!(signal(332).index(), 332);
        assert!(Signal::new(333).is_none());
        assert!(Signal::new(u16::MAX).is_none());
    }

    #[test]
    fn a_signal_is_written_the_way_the_design_writes_it() {
        assert_eq!(signal(187).to_string(), "#187");
        assert_eq!(signal(0).to_string(), "#0");
    }

    #[test]
    fn silence_stays_in_the_denominator() {
        // A third of those who spoke is not a third of the network. Ten peers, three
        // of them saying #7 and seven saying nothing: 3/10, not 3/3.
        let mut tally = Tally::of(many(signal(7), 3));
        for _ in 0..7 {
            tally.observe(None);
        }
        assert_eq!(tally.observed(), 10);
        assert_eq!(tally.spoken(), 3);
        assert_eq!(tally.silent(), 7);
        assert_eq!(tally.share(signal(7)), Some(300));
        assert!(!tally.reached(signal(7)));
    }

    #[test]
    fn the_threshold_is_one_third_and_the_boundary_is_inclusive() {
        // Exactly one third reaches it. Written as 33.3%, 111 of 333 would not.
        let mut tally = Tally::of(many(signal(1), 111));
        for _ in 0..222 {
            tally.observe(None);
        }
        assert_eq!(tally.observed(), 333);
        assert!(tally.reached(signal(1)));
        assert_eq!(tally.share(signal(1)), Some(333));

        let mut short = Tally::of(many(signal(1), 110));
        for _ in 0..223 {
            short.observe(None);
        }
        assert!(!short.reached(signal(1)));
    }

    #[test]
    fn an_epoch_nobody_was_heard_in_reaches_nothing() {
        let tally = Tally::new();
        assert_eq!(tally.observed(), 0);
        assert_eq!(tally.at_threshold().count(), 0);
        assert_eq!(tally.share(signal(0)), None);
        // Zero is a third of zero, so without the guard every signal would read as
        // having reached the threshold.
        assert!(!tally.reached(signal(0)));
    }

    #[test]
    fn three_signals_can_hold_a_third_each_and_none_of_them_wins() {
        let mut observations = many(signal(1), 111);
        observations.extend(many(signal(2), 111));
        observations.extend(many(signal(3), 111));
        let tally = Tally::of(observations);

        let reached: Vec<_> = tally.at_threshold().collect();
        assert_eq!(reached, vec![signal(1), signal(2), signal(3)]);
        assert_eq!(tally.silent(), 0);
    }

    #[test]
    fn the_whole_distribution_is_reported_including_the_zeroes() {
        let tally = Tally::of(many(signal(200), 5));
        let distribution: Vec<_> = tally.distribution().collect();
        assert_eq!(distribution.len(), 333);
        assert_eq!(distribution.first(), Some(&(signal(0), 0)));
        assert_eq!(distribution.get(200), Some(&(signal(200), 5)));
        let total: u64 = distribution.iter().map(|(_, n)| n).sum();
        assert_eq!(total, tally.spoken());
    }

    #[test]
    fn observing_one_at_a_time_and_all_at_once_agree() {
        let observations = vec![Some(signal(9)), None, Some(signal(9)), Some(signal(4))];
        let at_once = Tally::of(observations.clone());
        let mut one_by_one = Tally::new();
        for chosen in observations {
            one_by_one.observe(chosen);
        }
        assert_eq!(at_once, one_by_one);
        assert_eq!(at_once.count(signal(9)), 2);
        assert_eq!(at_once.count(signal(4)), 1);
        assert_eq!(at_once.observed(), 4);
    }
}
