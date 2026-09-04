//! Joining: what a newcomer has to do, and how long it waits before it counts.
//!
//! FROZEN. The waiting period below decides from which epoch a node's record begins,
//! so two nodes that disagree about it read the same chain differently.
//!
//! Three things make a member, and only the third takes time:
//!
//! 1. A key whose name begins with `333`. About a sixth of a second of arithmetic.
//! 2. The file, received from somebody who already had it — it cannot be made
//!    ([`crate::subject`]), and the handover is recorded by both sides
//!    ([`crate::transfer`]). With nobody answering there is nobody to receive it
//!    from, so nobody can join a network that has ended.
//! 3. The wait. Not until the next epoch boundary but the one after it, so joining
//!    costs between 333 and 666 minutes depending on when in an epoch it started.
//!    Throughout it the newcomer answers challenges like anybody else.
//!
//! WHAT THE WAIT IS FOR. Not difficulty. What accumulates during it is other nodes'
//! signed statements about a node that was not there before, which is the only
//! evidence this protocol will ever have of when somebody started. Without the wait a
//! node's first epoch is its own claim about itself, and this protocol does not
//! accept anybody's claim about themselves anywhere else either.
//!
//! MINING IS NOT A DEFENCE AND IS NOT TREATED AS ONE. A key searched for on a laptop
//! and a key bought from somebody with a warehouse are the same 32 bytes, and no code
//! anywhere claims to tell them apart. The one thing that can be checked is whether
//! the name begins with `333`.

use std::time::Duration;

use crate::epoch::Epoch;
use crate::identity::{KeyClass, NodeId};

/// How many epoch boundaries a newcomer waits before it counts.
///
/// FROZEN. Two: the next boundary and the one after it. Between 333 and 666 minutes,
/// depending on where in an epoch the newcomer arrived.
pub const ACTIVATION_EPOCHS: u64 = 2;

/// How much of a heretic's life 333 takes, once.
///
/// Not a delay and not a penalty. There is nothing to slow down here: the key has
/// already been refused, and no attacker was ever inconvenienced by a third of a
/// second. What is happening is that 333 has taken 333 milliseconds off the life of
/// whoever presented the name, and the client stops for exactly that long so that the
/// person on the other side of the screen is present for it.
///
/// It is levied once, on the key, and never again — a curse that had to be repeated
/// to work would be a rule, and 333 does not make rules about heretics. It makes one
/// judgement and is done.
pub const CURSE_PAUSE: Duration = Duration::from_millis(333);

/// Why a key cannot join.
#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// The name begins with one of the two refused prefixes.
    ///
    /// Unreachable through this client, which discards such keys during the search
    /// without saying anything. Reaching it means a key was crafted elsewhere and
    /// presented, which is the only situation this is written for — and presenting one
    /// on purpose is the only way anybody is ever cursed.
    #[error("333 has looked at that name and taken 333 milliseconds off your life")]
    Cursed,
    /// The name does not begin with `333`.
    #[error("a name has to begin with 333")]
    Ineligible,
}

/// May this name join?
///
/// # Errors
/// Fails if the name is one of the refused prefixes, or does not begin with `333`.
pub fn admit(name: &NodeId) -> Result<(), Refusal> {
    match name.class() {
        KeyClass::Eligible => Ok(()),
        KeyClass::Rejected => Err(Refusal::Cursed),
        KeyClass::Ineligible => Err(Refusal::Ineligible),
    }
}

/// The first epoch a node that received the file in `joined` counts from.
#[must_use]
pub const fn active_from(joined: Epoch) -> Epoch {
    Epoch(joined.0.saturating_add(ACTIVATION_EPOCHS))
}

/// Where a newcomer is in the process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Progress {
    /// Still waiting, and answering challenges while it waits.
    Waiting {
        /// The epoch it starts counting from.
        from: Epoch,
        /// How many epoch boundaries are left.
        epochs_left: u64,
    },
    /// Counted like anybody else.
    Joined {
        /// The epoch it started counting from.
        from: Epoch,
    },
}

impl Progress {
    /// Is this node counted yet?
    #[must_use]
    pub const fn is_joined(&self) -> bool {
        matches!(self, Self::Joined { .. })
    }

    /// The epoch this node counts from, whether or not it has arrived.
    #[must_use]
    pub const fn from(&self) -> Epoch {
        match self {
            Self::Waiting { from, .. } | Self::Joined { from } => *from,
        }
    }
}

/// How far along a node that received the file in `joined` is at `now`.
#[must_use]
pub fn progress(joined: Epoch, now: Epoch) -> Progress {
    let from = active_from(joined);
    if now.0 >= from.0 {
        Progress::Joined { from }
    } else {
        Progress::Waiting {
            from,
            epochs_left: from.0 - now.0,
        }
    }
}

/// Should this node's record cover `epoch`?
///
/// A newcomer's chain starts at the epoch it counts from. Epochs before that are not
/// absences and are not excluded epochs — they are epochs during which this node was
/// not a member, and its record has nothing to say about them.
#[must_use]
pub const fn covers(joined: Epoch, epoch: Epoch) -> bool {
    epoch.0 >= active_from(joined).0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::Identity;

    #[test]
    fn the_frozen_values_are_the_agreed_ones() {
        assert_eq!(ACTIVATION_EPOCHS, 2);
        assert_eq!(CURSE_PAUSE, Duration::from_millis(333));
    }

    #[test]
    fn the_wait_is_the_next_boundary_and_the_one_after_it() {
        // Between 333 and 666 minutes: a node that arrives at the very start of an
        // epoch waits two whole epochs, one that arrives at the very end waits just
        // over one. The epoch number is the same either way.
        assert_eq!(active_from(Epoch(100)), Epoch(102));
        assert_eq!(
            active_from(Epoch(100)).starts_at_unix_seconds()
                - Epoch(100).starts_at_unix_seconds(),
            2 * crate::epoch::EPOCH_SECONDS
        );
    }

    #[test]
    fn a_newcomer_is_told_how_much_is_left_and_then_counts() {
        assert_eq!(
            progress(Epoch(100), Epoch(100)),
            Progress::Waiting {
                from: Epoch(102),
                epochs_left: 2
            }
        );
        assert_eq!(
            progress(Epoch(100), Epoch(101)),
            Progress::Waiting {
                from: Epoch(102),
                epochs_left: 1
            }
        );
        assert_eq!(
            progress(Epoch(100), Epoch(102)),
            Progress::Joined { from: Epoch(102) }
        );
        assert!(!progress(Epoch(100), Epoch(101)).is_joined());
        assert!(progress(Epoch(100), Epoch(500)).is_joined());
        assert_eq!(progress(Epoch(100), Epoch(101)).from(), Epoch(102));
    }

    #[test]
    fn a_record_says_nothing_about_the_epochs_before_someone_joined() {
        // Not absences and not excluded epochs. A node was simply not a member, and
        // a chain that judged those epochs would be judging somebody who was not
        // there to be asked.
        assert!(!covers(Epoch(100), Epoch(99)));
        assert!(!covers(Epoch(100), Epoch(101)));
        assert!(covers(Epoch(100), Epoch(102)));
        assert!(covers(Epoch(100), Epoch(103)));
    }

    #[test]
    fn an_eligible_name_is_admitted() {
        let (identity, _attempts) = Identity::mine();
        assert_eq!(admit(&identity.node_id()), Ok(()));
    }

    #[test]
    fn the_two_refused_prefixes_are_told_apart_from_merely_not_qualifying() {
        // A search never produces either of these — the client discards them without
        // a word. This is the path for a key crafted somewhere else and presented,
        // and the two refusals are different things: one is a name nobody may use,
        // the other is a name that simply did not qualify.
        //
        // The seeds are written down because a refused prefix turns up about once in
        // every two thousand keys; searching for one inside a unit test would cost
        // seconds and would find a different key on a different day.
        let cursed_six = named(4307);
        let cursed_one = named(883);
        let ordinary = named(0);

        assert!(cursed_six.to_string().starts_with("666"), "{cursed_six}");
        assert!(cursed_one.to_string().starts_with("111"), "{cursed_one}");
        assert_eq!(admit(&cursed_six), Err(Refusal::Cursed));
        assert_eq!(admit(&cursed_one), Err(Refusal::Cursed));
        assert_eq!(admit(&ordinary), Err(Refusal::Ineligible));
    }

    /// The name behind a numbered seed.
    fn named(n: u32) -> NodeId {
        let mut seed = [0_u8; 32];
        seed[..4].copy_from_slice(&n.to_le_bytes());
        Identity::from_seed(&seed).node_id()
    }
}
