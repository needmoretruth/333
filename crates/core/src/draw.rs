//! Who asks whom, each epoch.
//!
//! FROZEN. The domain, the count and the ordering below decide which nodes are
//! entitled to challenge a given node in a given epoch, so two nodes that disagree
//! about them cannot read each other's records.
//!
//! Three verifiers are drawn for each prover, each epoch, by giving every candidate a
//! hash and taking the smallest three. Nothing is random: every node computes the
//! same three from public values, and a node can work out for itself whether a
//! challenge it received was one it was entitled to send.
//!
//! WHAT IS DELIBERATELY NOT IN THE HASH. Not a snapshot of the membership roll, not a
//! root over it, not the previous epoch's aggregate. Folding the roll in would make
//! every ticket depend on the whole roll, so two nodes whose rolls differ by one
//! newcomer would compute three entirely unrelated verifiers for everybody — a
//! disagreement about one member becomes a disagreement about all of them. Here a
//! ticket depends only on the epoch, the prover and the one candidate, so two nodes
//! agree about every member they both know, and disagreement is bounded to the
//! members one of them has not heard of. It also removes a canonical-ordering rule
//! for hashing, a snapshot object, two more domains, and the question of who gets to
//! choose the last input to a shared seed.
//!
//! THE DRAW IS NOT A DEFENCE. Somebody who runs many nodes gets many tickets, and
//! this hands them more of the draws. The protocol does not try to prevent that — see
//! the project's own statement of what it does not claim. What the draw buys is that
//! nobody chooses when it is their turn.

use std::collections::BTreeSet;

use crate::epoch::Epoch;
use crate::subject::digest_of;
use crate::wire::DOMAIN_LEN;

/// How many verifiers are drawn for one prover in one epoch.
///
/// FROZEN. Three is the whole of it: the count is not raised when the network grows,
/// because the cost of being asked has to stay flat for the weakest machine.
pub const VERIFIERS_PER_EPOCH: usize = 3;

/// The domain a ticket is hashed under. FROZEN, and exactly [`DOMAIN_LEN`] bytes.
pub const DOMAIN_TICKET: &[u8; DOMAIN_LEN] = b"333.v1.draw.tick";

/// One candidate's ticket for one prover in one epoch.
///
/// The epoch is written big-endian and fixed width, and every field has a fixed
/// length, so no two different inputs can produce the same byte string to hash.
#[must_use]
pub fn ticket(epoch: Epoch, prover: &[u8; 32], candidate: &[u8; 32]) -> [u8; 32] {
    let mut input = Vec::with_capacity(DOMAIN_LEN + 8 + 32 + 32);
    input.extend_from_slice(DOMAIN_TICKET);
    input.extend_from_slice(&epoch.0.to_be_bytes());
    input.extend_from_slice(prover);
    input.extend_from_slice(candidate);
    digest_of(&input)
}

/// The verifiers entitled to challenge `prover` in `epoch`.
///
/// Returns fewer than [`VERIFIERS_PER_EPOCH`] when the roll holds fewer candidates,
/// and an empty list when the prover is the only member. **The list is never padded
/// back to three** by repeating a candidate or wrapping around: with two members, two
/// verifiers is the truth, and a rule that reads "all three said nothing" would
/// otherwise be satisfiable by a roll that never had three.
///
/// The prover is excluded from its own draw. Nobody witnesses themselves.
#[must_use]
pub fn verifiers_for(
    epoch: Epoch,
    prover: &[u8; 32],
    roll: &BTreeSet<[u8; 32]>,
) -> Vec<[u8; 32]> {
    let mut tickets: Vec<([u8; 32], [u8; 32])> = roll
        .iter()
        .filter(|candidate| *candidate != prover)
        .map(|candidate| (ticket(epoch, prover, candidate), *candidate))
        .collect();
    // Ordered by ticket, then by key: two candidates whose tickets collide must still
    // land in the same order on every machine.
    tickets.sort_unstable();
    tickets
        .into_iter()
        .take(VERIFIERS_PER_EPOCH)
        .map(|(_, candidate)| candidate)
        .collect()
}

/// How many verifiers a prover has this epoch: `min(3, roll without the prover)`.
///
/// The denominator every judgement about an epoch is read against. An epoch in which
/// this is zero cannot produce an absence, because nobody was entitled to ask.
#[must_use]
pub fn verifier_count(prover: &[u8; 32], roll: &BTreeSet<[u8; 32]>) -> usize {
    let others = roll.len() - usize::from(roll.contains(prover));
    others.min(VERIFIERS_PER_EPOCH)
}

/// May `verifier` challenge `prover` in `epoch`?
///
/// Asked by the prover of every challenge that arrives, and by anyone reading a
/// record later. Both get the same answer from public values.
#[must_use]
pub fn is_entitled(
    epoch: Epoch,
    prover: &[u8; 32],
    verifier: &[u8; 32],
    roll: &BTreeSet<[u8; 32]>,
) -> bool {
    verifiers_for(epoch, prover, roll).contains(verifier)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A key that is nothing but its number, so failures name the member.
    fn key(n: u8) -> [u8; 32] {
        let mut k = [0_u8; 32];
        k[0] = n;
        k
    }

    fn roll_of(n: u8) -> BTreeSet<[u8; 32]> {
        (0..n).map(key).collect()
    }

    #[test]
    fn the_frozen_values_are_the_agreed_ones() {
        assert_eq!(VERIFIERS_PER_EPOCH, 3);
        assert_eq!(DOMAIN_TICKET, b"333.v1.draw.tick");
        assert_eq!(DOMAIN_TICKET.len(), DOMAIN_LEN);
    }

    #[test]
    fn a_ticket_is_a_known_answer_and_not_a_recomputation_of_itself() {
        // Written as a literal so that a change to the domain, the field order or the
        // integer encoding fails here rather than silently splitting the network into
        // builds that draw different verifiers. The value was computed by a separate
        // implementation over the documented byte string
        // `"333.v1.draw.tick" || epoch:u64be || prover || candidate`, not read back
        // out of this code — a digest compared only against itself is not a test.
        let t = ticket(Epoch(89_516), &key(1), &key(2));
        assert_eq!(
            t.iter().map(|b| format!("{b:02x}")).collect::<String>(),
            "f63684702f67111d13202ee0d7a5617d4237d756298beb74a0453a41702f6444"
        );
    }

    #[test]
    fn the_draw_moves_with_the_epoch_and_with_the_prover() {
        let roll = roll_of(30);
        let a = verifiers_for(Epoch(1), &key(0), &roll);
        assert_ne!(a, verifiers_for(Epoch(2), &key(0), &roll));
        assert_ne!(a, verifiers_for(Epoch(1), &key(1), &roll));
    }

    #[test]
    fn nobody_witnesses_themselves() {
        let roll = roll_of(30);
        for prover in 0..30_u8 {
            assert!(!verifiers_for(Epoch(7), &key(prover), &roll).contains(&key(prover)));
        }
    }

    #[test]
    fn the_same_inputs_give_the_same_three_however_the_roll_was_built() {
        // The roll is a set, so insertion order cannot reach the result. Asserted
        // rather than assumed, because it is the property that lets two nodes agree.
        let forwards: BTreeSet<_> = (0..40_u8).map(key).collect();
        let backwards: BTreeSet<_> = (0..40_u8).rev().map(key).collect();
        assert_eq!(
            verifiers_for(Epoch(500), &key(3), &forwards),
            verifiers_for(Epoch(500), &key(3), &backwards)
        );
    }

    #[test]
    fn a_roll_that_differs_by_one_member_still_agrees_about_everyone_shared() {
        // The reason the roll is not hashed into the ticket. One node has heard of a
        // newcomer and the other has not; every ticket they both compute is equal, so
        // their three can differ only by that newcomer.
        let small = roll_of(20);
        let mut large = small.clone();
        large.insert(key(99));

        for prover in 0..20_u8 {
            let a = verifiers_for(Epoch(11), &key(prover), &small);
            let b = verifiers_for(Epoch(11), &key(prover), &large);
            let differing: Vec<_> = b.iter().filter(|v| !a.contains(v)).collect();
            assert!(
                differing.is_empty() || differing == vec![&key(99)],
                "prover {prover}: {a:?} vs {b:?}"
            );
        }
    }

    #[test]
    fn a_small_roll_gives_a_small_draw_and_is_never_padded_back_to_three() {
        // Two members means one verifier, not one repeated three times. A rule that
        // read "all three said nothing" must not be satisfiable by a roll of two.
        let two = roll_of(2);
        assert_eq!(verifiers_for(Epoch(1), &key(0), &two), vec![key(1)]);
        assert_eq!(verifier_count(&key(0), &two), 1);

        let three = roll_of(3);
        assert_eq!(verifiers_for(Epoch(1), &key(0), &three).len(), 2);
        assert_eq!(verifier_count(&key(0), &three), 2);

        let four = roll_of(4);
        assert_eq!(verifiers_for(Epoch(1), &key(0), &four).len(), 3);
        assert_eq!(verifier_count(&key(0), &four), 3);
    }

    #[test]
    fn a_lone_member_has_no_verifiers_and_therefore_can_never_be_absent() {
        let alone: BTreeSet<_> = [key(0)].into_iter().collect();
        assert!(verifiers_for(Epoch(1), &key(0), &alone).is_empty());
        assert_eq!(verifier_count(&key(0), &alone), 0);

        let empty = BTreeSet::new();
        assert!(verifiers_for(Epoch(1), &key(0), &empty).is_empty());
        assert_eq!(verifier_count(&key(0), &empty), 0);
    }

    #[test]
    fn a_node_can_tell_whether_a_challenge_was_one_the_sender_was_entitled_to_send() {
        let roll = roll_of(30);
        let drawn = verifiers_for(Epoch(42), &key(0), &roll);
        for verifier in &drawn {
            assert!(is_entitled(Epoch(42), &key(0), verifier, &roll));
        }
        let uninvited = (1..30_u8)
            .map(key)
            .find(|k| !drawn.contains(k))
            .expect("some member is not drawn");
        assert!(!is_entitled(Epoch(42), &key(0), &uninvited, &roll));
        // ...and the same key is entitled in some other epoch, so the refusal is
        // about this epoch rather than about that node.
        let ever = (0..100_u64).any(|e| is_entitled(Epoch(e), &key(0), &uninvited, &roll));
        assert!(ever, "a member should get a turn eventually");
    }

    #[test]
    fn the_turns_spread_over_the_roll_rather_than_settling_on_a_few() {
        // Not a claim that the draw is uniform — only that it is not obviously stuck.
        // A draw that always picked the same three would pass every test above.
        let roll = roll_of(30);
        let mut seen = BTreeSet::new();
        for e in 0..200_u64 {
            seen.extend(verifiers_for(Epoch(e), &key(0), &roll));
        }
        assert_eq!(seen.len(), 29, "every other member should get a turn");
    }
}
