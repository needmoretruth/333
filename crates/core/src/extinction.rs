//! Deciding that it has ended, and counting the long silence after it.
//!
//! FROZEN. When the count of answering peers reaches zero it is over, and what
//! remains is one subtraction: 19,683 years — three cubed, cubed — from the moment
//! the last one stopped answering.
//!
//! Nothing in the protocol depends on any of this. No message carries it, no decision
//! turns on it, and no node has to agree with another about it. It exists to be shown
//! to a person, which is the whole of its job — and that is exactly why the bar for
//! saying it is set where it is.
//!
//! WHY IT IS NOT SIMPLY `active == 0`. A node counts as present only the peers it
//! exchanged heartbeats with itself, because any count sourced from other nodes'
//! attestations can be inflated by one person holding both keys of a pair, and an
//! inflatable count means the end could never be declared at all. But own observation
//! has the opposite failure: a node whose own network is broken sees nobody, and
//! would otherwise print the most consequential sentence this software has. So the
//! end is announced only after an unbroken watch — [`SILENT_EPOCHS_BEFORE_THE_END`]
//! consecutive epochs, every one of them actually watched, with this node answerable
//! throughout — and only by a node that saw somebody at least once.

use std::fmt;

use crate::epoch::Epoch;

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

/// How many consecutive silent epochs before a node will say it has ended.
///
/// 333 epochs is about 77 days. Long enough that an outage, a move, a provider
/// failure or a very quiet stretch does not produce the sentence; short enough that
/// somebody still running will see it within a season.
pub const SILENT_EPOCHS_BEFORE_THE_END: u64 = 333;

/// What one epoch looked like from here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Watched {
    /// At least one peer completed an exchange with this node.
    Someone,
    /// Nobody did, and this node was answerable the whole time.
    Nobody,
    /// Nobody did, and this node could not be reached either.
    ///
    /// The silence says nothing about anyone else, so it is not evidence. Its own
    /// listener was down, or its address unpublished, or the machine was off the
    /// network.
    NobodyWhileDeaf,
}

/// The unbroken watch a node keeps, and the only thing that entitles it to say the
/// network has ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Vigil {
    /// Consecutive watched epochs in which this node was answerable and saw nobody.
    run: u64,
    /// The last epoch folded in, so a gap in watching can be told from a run.
    watched_through: Option<Epoch>,
    /// The last epoch a peer answered this node.
    last_seen: Option<Epoch>,
}

/// What a node is entitled to say.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// This node has never seen anybody, so it has nothing to say either way.
    ///
    /// A node with the wrong address, or one that has only just started, is in this
    /// state. It must never announce the end: it has no evidence that there was ever
    /// anything to end.
    NothingToSay,
    /// Somebody is here.
    Alive,
    /// Nobody has answered for a while, but not for long enough to say it.
    Waiting {
        /// Consecutive silent epochs so far.
        silent: u64,
        /// How many are needed.
        needed: u64,
    },
    /// Nobody has answered through an unbroken watch of the full length.
    Ended {
        /// The last epoch anybody answered this node. The countdown runs from here,
        /// not from the moment the verdict was reached — the silence began when the
        /// last peer stopped, and 77 days of watching it does not move that.
        since: Epoch,
    },
}

impl Vigil {
    /// A node that has watched nothing yet.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            run: 0,
            watched_through: None,
            last_seen: None,
        }
    }

    /// Fold in one epoch.
    ///
    /// Epochs must arrive in order. An epoch already folded in is ignored rather than
    /// counted twice, and an epoch that skips over one breaks the run — a node that
    /// was not watching cannot vouch for what happened while it was not.
    pub fn watch(&mut self, epoch: Epoch, seen: Watched) {
        if let Some(through) = self.watched_through
            && epoch.0 <= through.0
        {
            return;
        }
        let continues = self.watched_through.is_some_and(|t| epoch.0 == t.0 + 1);
        match seen {
            Watched::Someone => {
                self.run = 0;
                self.last_seen = Some(epoch);
            }
            Watched::Nobody => self.run = if continues { self.run + 1 } else { 1 },
            Watched::NobodyWhileDeaf => self.run = 0,
        }
        self.watched_through = Some(epoch);
    }

    /// Consecutive silent epochs behind an unbroken watch.
    #[must_use]
    pub const fn silent(&self) -> u64 {
        self.run
    }

    /// The last epoch a peer answered this node, if one ever did.
    #[must_use]
    pub const fn last_seen(&self) -> Option<Epoch> {
        self.last_seen
    }

    /// What this node is entitled to say right now.
    #[must_use]
    pub fn verdict(&self) -> Verdict {
        let Some(since) = self.last_seen else {
            return Verdict::NothingToSay;
        };
        if self.run == 0 {
            return Verdict::Alive;
        }
        if self.run >= SILENT_EPOCHS_BEFORE_THE_END {
            Verdict::Ended { since }
        } else {
            Verdict::Waiting {
                silent: self.run,
                needed: SILENT_EPOCHS_BEFORE_THE_END,
            }
        }
    }

    /// What is left of the silence at `now`, once this node may say it ended.
    ///
    /// `None` while the node has nothing to say, while somebody is here, while the
    /// watch is still short, and after the silence itself is over.
    #[must_use]
    pub fn remaining_at(&self, now_unix: u64) -> Option<Remaining> {
        match self.verdict() {
            Verdict::Ended { since } => remaining(since.starts_at_unix_seconds(), now_unix),
            _ => None,
        }
    }
}

#[cfg(test)]
mod vigil_tests {
    use super::*;

    /// Watch `count` epochs in a row, starting at `from`, all the same way.
    fn watch_run(vigil: &mut Vigil, from: u64, count: u64, seen: Watched) {
        for e in from..from + count {
            vigil.watch(Epoch(e), seen);
        }
    }

    #[test]
    fn the_length_of_the_watch_is_the_agreed_one() {
        assert_eq!(SILENT_EPOCHS_BEFORE_THE_END, 333);
    }

    #[test]
    fn a_node_that_has_never_seen_anybody_never_announces_the_end() {
        // The case that matters most: a fresh node, or one given a wrong address,
        // watching an empty socket for a year. It has no evidence that there was
        // ever anything to end.
        let mut vigil = Vigil::new();
        watch_run(&mut vigil, 0, 1000, Watched::Nobody);
        assert_eq!(vigil.silent(), 1000);
        assert_eq!(vigil.verdict(), Verdict::NothingToSay);
        assert_eq!(vigil.remaining_at(0), None);
    }

    #[test]
    fn the_end_is_announced_only_after_a_full_unbroken_watch() {
        let mut vigil = Vigil::new();
        vigil.watch(Epoch(100), Watched::Someone);
        assert_eq!(vigil.verdict(), Verdict::Alive);

        watch_run(&mut vigil, 101, 332, Watched::Nobody);
        assert_eq!(
            vigil.verdict(),
            Verdict::Waiting {
                silent: 332,
                needed: 333
            }
        );

        vigil.watch(Epoch(433), Watched::Nobody);
        assert_eq!(vigil.verdict(), Verdict::Ended { since: Epoch(100) });
    }

    #[test]
    fn one_peer_answering_starts_the_watch_over() {
        let mut vigil = Vigil::new();
        vigil.watch(Epoch(0), Watched::Someone);
        watch_run(&mut vigil, 1, 332, Watched::Nobody);
        vigil.watch(Epoch(333), Watched::Someone);
        assert_eq!(vigil.silent(), 0);
        assert_eq!(vigil.verdict(), Verdict::Alive);

        watch_run(&mut vigil, 334, 332, Watched::Nobody);
        assert!(matches!(vigil.verdict(), Verdict::Waiting { .. }));
    }

    #[test]
    fn an_epoch_this_node_could_not_be_reached_in_is_not_evidence() {
        // The whole reason this type exists. A node with a broken listener sees
        // nobody and must not read that as nobody being there.
        let mut vigil = Vigil::new();
        vigil.watch(Epoch(0), Watched::Someone);
        watch_run(&mut vigil, 1, 332, Watched::Nobody);
        vigil.watch(Epoch(333), Watched::NobodyWhileDeaf);
        assert_eq!(vigil.silent(), 0);
        assert!(matches!(vigil.verdict(), Verdict::Alive));
    }

    #[test]
    fn a_gap_in_watching_breaks_the_run() {
        // A node that was switched off for a week cannot vouch for the week. This
        // means a node restarted often will never announce the end, and that is the
        // intended trade: the sentence belongs to whoever actually kept watching.
        let mut vigil = Vigil::new();
        vigil.watch(Epoch(0), Watched::Someone);
        watch_run(&mut vigil, 1, 300, Watched::Nobody);
        assert_eq!(vigil.silent(), 300);

        vigil.watch(Epoch(400), Watched::Nobody);
        assert_eq!(vigil.silent(), 1, "the run restarts at the epoch after the gap");
    }

    #[test]
    fn an_epoch_folded_twice_is_counted_once() {
        let mut vigil = Vigil::new();
        vigil.watch(Epoch(0), Watched::Someone);
        vigil.watch(Epoch(1), Watched::Nobody);
        vigil.watch(Epoch(1), Watched::Nobody);
        vigil.watch(Epoch(0), Watched::Nobody);
        assert_eq!(vigil.silent(), 1);
        assert_eq!(vigil.last_seen(), Some(Epoch(0)));
    }

    #[test]
    fn the_countdown_runs_from_the_last_peer_not_from_the_verdict() {
        // 333 epochs of watching pass before the sentence can be said, and none of
        // them are added to the silence. The silence began when the last peer did.
        let mut vigil = Vigil::new();
        vigil.watch(Epoch(100), Watched::Someone);
        watch_run(&mut vigil, 101, 333, Watched::Nobody);

        let began = Epoch(100).starts_at_unix_seconds();
        assert_eq!(
            vigil.remaining_at(began),
            Some(Remaining {
                years: EXTINCTION_YEARS,
                days: 0
            })
        );
        // And it is already 77 days shorter by the time it can first be shown.
        let announced = Epoch(434).starts_at_unix_seconds();
        let left = vigil.remaining_at(announced).expect("still counting");
        assert!(left.years == EXTINCTION_YEARS - 1 || left.days < 365 - 77);
    }

    #[test]
    fn nothing_is_counted_down_while_anybody_is_here() {
        let mut vigil = Vigil::new();
        vigil.watch(Epoch(1), Watched::Someone);
        assert_eq!(vigil.remaining_at(u64::MAX), None);
    }
}
