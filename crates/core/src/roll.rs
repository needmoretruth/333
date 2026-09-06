//! Who is on the roll, and since when.
//!
//! A node is on the roll because somebody handed it the file and both of them signed
//! for it ([`crate::transfer`]). There is no other way on. Nobody admits themselves,
//! nobody is admitted by a list, and there is no roll anywhere that is the real one:
//! each node's roll is whatever admissions it has seen, and two nodes may hold
//! different ones without either being wrong.
//!
//! MERGING IS ALWAYS A GAIN, WHICH IS WHY IT HAPPENS BY ITSELF. Two nodes that meet
//! exchange admissions and each keeps the union. Nothing has to be reconciled, because
//! an admission means the same thing to everybody who can check two signatures, and
//! nothing is lost, because a node that is already known is simply already known. A
//! merge that could cost somebody something would be a merge nobody performs.
//!
//! A NODE NOT ON MY ROLL IS NOT ABSENT, IT IS UNKNOWN. Standing is only ever read for
//! members, so a node from a part of the network this one has never met does not
//! accrue absences here while they are apart — it accrues nothing at all, and joining
//! the two rolls does not hand it a punishment for the time they were separated.
//!
//! THE FOUNDER IS NOT HERE. Whoever first held the file was given it by nobody, so
//! there is no admission for them and they are on no roll. That is deliberate: a
//! founder on the roll is a member who can never leave, and a count that can never
//! reach zero.

use std::collections::{BTreeMap, BTreeSet};

use crate::epoch::Epoch;
use crate::transfer::{self, Half, Signed, Transfer};

/// One member, and the admission that put them there.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member {
    /// The member's public key.
    pub key: [u8; 32],
    /// The epoch they received the file in.
    pub received_in: Epoch,
    /// Who handed it to them.
    pub sponsor: [u8; 32],
}

impl Member {
    /// The first epoch this member's record covers.
    #[must_use]
    pub const fn counts_from(&self) -> Epoch {
        crate::enrollment::active_from(self.received_in)
    }
}

/// What one node knows about who is a member.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Roll {
    /// Members by key. A map rather than a set so that a member's admission can be
    /// looked up, and ordered so that two nodes holding the same members hold them
    /// in the same order.
    members: BTreeMap<[u8; 32], Member>,
}

/// What reading a pile of admission records produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Read {
    /// How many admissions were assembled from two matching halves.
    pub admitted: usize,
    /// How many records could not be read at all.
    pub unreadable: usize,
    /// How many halves never found their other half.
    ///
    /// Ordinary rather than suspicious: one side's record often arrives before the
    /// other's. A half on its own says nothing and admits nobody.
    pub unpaired: usize,
}

impl Roll {
    /// An empty roll.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// How many members.
    #[must_use]
    pub fn len(&self) -> usize {
        self.members.len()
    }

    /// Is the roll empty?
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// The member behind a key, if this node knows of them.
    #[must_use]
    pub fn member(&self, key: &[u8; 32]) -> Option<&Member> {
        self.members.get(key)
    }

    /// Every member's key, in the shape the verifier draw takes.
    #[must_use]
    pub fn keys(&self) -> BTreeSet<[u8; 32]> {
        self.members.keys().copied().collect()
    }

    /// Every member, in key order.
    pub fn members(&self) -> impl Iterator<Item = &Member> {
        self.members.values()
    }

    /// Add an admission.
    ///
    /// A member already on the roll keeps the earlier admission. Somebody who
    /// received the file twice joined once, and which of the two counts must not
    /// depend on the order the records happened to arrive in.
    pub fn admit(&mut self, transfer: &Transfer) {
        self.keep(Member {
            key: transfer.received.record.author,
            received_in: transfer.epoch(),
            sponsor: transfer.gave.record.author,
        });
    }

    /// Put a member on the roll, or keep whichever admission comes first.
    ///
    /// Earlier epoch wins; on the same epoch, the lower sponsor key wins. The
    /// tie-break is not cosmetic — without it, two nodes that merge the same pair of
    /// rolls in opposite directions end up naming different sponsors for one member,
    /// and a lineage that reads differently on two machines is a lineage nobody can
    /// follow.
    fn keep(&mut self, member: Member) {
        let better_than = |existing: &Member| {
            (member.received_in.0, member.sponsor) < (existing.received_in.0, existing.sponsor)
        };
        self.members
            .entry(member.key)
            .and_modify(|existing| {
                if better_than(existing) {
                    *existing = member.clone();
                }
            })
            .or_insert(member);
    }

    /// Take everything the other roll knows and keep the union.
    ///
    /// Nothing is dropped and nothing is overwritten except by an earlier admission
    /// for the same member. This is the whole of healing a split: it cannot fail and
    /// it cannot cost anybody anything, which is why it does not need to be agreed.
    pub fn merge(&mut self, other: &Self) {
        for member in other.members.values() {
            self.keep(member.clone());
        }
    }

    /// Build a roll out of admission halves, in any order, from anywhere.
    ///
    /// Each record is one side's signed statement about one handover. A statement on
    /// its own admits nobody; only a matching pair does, and a pair needs two keys.
    /// Anything that does not open, does not pair, or does not agree with its other
    /// half is counted and left out.
    #[must_use]
    pub fn from_halves(frames: &[Vec<u8>]) -> (Self, Read) {
        let mut held = Admissions::new();
        for frame in frames {
            held.add(frame);
        }
        let read = held.read();
        (held.roll, read)
    }
}

/// Admission halves as they arrive, and the roll they have made so far.
///
/// WHY A NODE KEEPS THIS RATHER THAN READING EVERYTHING AGAIN. [`Roll::from_halves`]
/// opens and checks every half it is handed, which is right for a pile of records and
/// wrong for a node that is running: gossip hands a node the admissions it already
/// holds several times an epoch, and rebuilding from the whole file each time makes
/// the cost of one new member the cost of every member ever admitted. Kept here
/// instead, a half that pairs is paired once and a half that arrives twice costs
/// nothing.
///
/// PAIRED HALVES ARE DROPPED, THE HANDOVER THEY NAMED IS REMEMBERED. Once two halves
/// have made a member there is nothing left for them to pair with, so what is kept is
/// what they were about rather than the halves themselves: the ones still here are the
/// ones still waiting for their other side, which is a handful in a network that is
/// working rather than everything anybody was ever admitted by.
#[derive(Debug, Clone, Default)]
pub struct Admissions {
    /// Givers' halves that have not found their other side.
    gave: BTreeMap<Pairing, Signed>,
    /// Receivers' halves that have not found their other side.
    received: BTreeMap<Pairing, Signed>,
    /// The handovers already made, each named the same way from either side.
    done: BTreeSet<Pairing>,
    /// Who has been admitted by a pair.
    roll: Roll,
    /// How many frames opened as nothing.
    unreadable: usize,
}

impl Admissions {
    /// Nothing admitted and nothing waiting.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Take one half, and admit somebody if it completes a pair.
    ///
    /// The same half twice is not an error and does nothing: whoever passed it on the
    /// second time was doing the one thing that makes a record travel.
    pub fn add(&mut self, frame: &[u8]) {
        let Some((half, signed)) = open_either(frame) else {
            self.unreadable += 1;
            return;
        };
        let key = Pairing::of(&signed);
        let mirror = key.mirrored();
        let handover = key.min(mirror);
        if self.done.contains(&handover) {
            return;
        }
        let paired = {
            let (mine, theirs) = match half {
                Half::Gave => (&mut self.gave, &mut self.received),
                Half::Received => (&mut self.received, &mut self.gave),
            };
            // A pairing already held is already accounted for, whether it paired or is
            // still waiting. Keeping the first is what makes two nodes that were handed
            // the same records in different orders hold the same roll.
            if mine.insert(key, signed.clone()).is_some() {
                return;
            }
            let Some(other) = theirs.get(&mirror).cloned() else {
                return;
            };
            let (given, taken) = match half {
                Half::Gave => (signed, other),
                Half::Received => (other, signed),
            };
            match Transfer::assemble(given, taken) {
                Ok(transfer) => {
                    mine.remove(&key);
                    theirs.remove(&mirror);
                    transfer
                }
                // Two halves that name the same handover and do not agree about it
                // admit nobody, and both stay where they are. One of them is a lie and
                // nothing here can say which, so dropping either would be choosing.
                Err(_) => return,
            }
        };
        self.done.insert(handover);
        self.roll.admit(&paired);
    }

    /// Who these halves have admitted.
    #[must_use]
    pub const fn roll(&self) -> &Roll {
        &self.roll
    }

    /// What has been made of everything taken so far.
    #[must_use]
    pub fn read(&self) -> Read {
        Read {
            admitted: self.done.len(),
            unreadable: self.unreadable,
            unpaired: self.gave.len() + self.received.len(),
        }
    }
}

/// What has to match for two halves to be about the same handover.
///
/// EVERY SIGNED FIELD IS IN HERE, and that is the point. A key that names only some of
/// them lets one node overwrite its own half with a second, differently-worded frame —
/// same author, same counterparty, same epoch, a different file — and whichever arrived
/// last would occupy the slot. The good pair would then fail to assemble and be dropped
/// in silence, so one key would have retracted a record that takes two keys to make,
/// and two nodes reading the same frames in different orders would hold different rolls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pairing {
    /// Which version of the wire format said it.
    protocol: u16,
    /// Who signed.
    author: [u8; 32],
    /// Who they named.
    counterparty: [u8; 32],
    /// When.
    epoch: u64,
    /// What was handed over.
    subject: [u8; 32],
}

impl Pairing {
    fn of(signed: &Signed) -> Self {
        Self {
            protocol: signed.record.protocol,
            author: signed.record.author,
            counterparty: signed.record.counterparty,
            epoch: signed.record.epoch,
            subject: signed.record.subject,
        }
    }

    /// The same handover, seen from the other side.
    const fn mirrored(self) -> Self {
        Self {
            protocol: self.protocol,
            author: self.counterparty,
            counterparty: self.author,
            epoch: self.epoch,
            subject: self.subject,
        }
    }
}

/// Read a frame as whichever half it turns out to be.
///
/// Both are tried because a stored record does not say which it is: the domain inside
/// the signature does, and the only way to ask is to check.
fn open_either(frame: &[u8]) -> Option<(Half, Signed)> {
    for half in [Half::Gave, Half::Received] {
        if let Ok(signed) = transfer::open(frame, half) {
            return Some((half, signed));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::Identity;
    use crate::subject::DIGEST;

    fn identity(seed: u8) -> Identity {
        Identity::from_seed(&[seed; 32])
    }

    /// Both halves of one handover, as they would be stored.
    fn admission(giver: &Identity, taker: &Identity, epoch: u64) -> Vec<Vec<u8>> {
        let epoch = Epoch(epoch);
        vec![
            transfer::Record::new(giver, taker.public_key(), epoch, DIGEST)
                .seal(Half::Gave, giver)
                .expect("seals"),
            transfer::Record::new(taker, giver.public_key(), epoch, DIGEST)
                .seal(Half::Received, taker)
                .expect("seals"),
        ]
    }

    #[test]
    fn one_key_cannot_retract_a_record_that_took_two_keys_to_make() {
        // The giver signs a second, differently-worded half about the same handover:
        // same counterparty, same epoch, a file that is not the file. If the pairing
        // key ignored that field, whichever frame arrived last would occupy the slot,
        // the good pair would fail to assemble, and one key would have undone a record
        // that takes two. Fed in both orders, the roll must come out the same.
        let (giver, newcomer) = (identity(1), identity(2));
        let good = admission(&giver, &newcomer, 100);
        let poison = transfer::Record::new(&giver, newcomer.public_key(), Epoch(100), [0_u8; 32])
            .seal(Half::Gave, &giver)
            .expect("seals");

        for order in [
            vec![good[0].clone(), good[1].clone(), poison.clone()],
            vec![poison.clone(), good[0].clone(), good[1].clone()],
        ] {
            let (roll, read) = Roll::from_halves(&order);
            assert_eq!(read.admitted, 1, "the good pair still assembles");
            assert!(roll.member(&newcomer.public_key()).is_some());
        }
    }

    #[test]
    fn two_matching_halves_put_somebody_on_the_roll() {
        let (sponsor, newcomer) = (identity(1), identity(2));
        let (roll, read) = Roll::from_halves(&admission(&sponsor, &newcomer, 100));
        assert_eq!(read.admitted, 1);
        assert_eq!(roll.len(), 1);

        let member = roll.member(&newcomer.public_key()).expect("on the roll");
        assert_eq!(member.received_in, Epoch(100));
        assert_eq!(member.sponsor, sponsor.public_key());
        // The wait is what turns a handover into membership.
        assert_eq!(member.counts_from(), Epoch(102));
        // The sponsor is not admitted by giving: their own admission is elsewhere.
        assert!(roll.member(&sponsor.public_key()).is_none());
    }

    #[test]
    fn one_half_alone_admits_nobody() {
        let (sponsor, newcomer) = (identity(1), identity(2));
        let both = admission(&sponsor, &newcomer, 100);
        for half in &both {
            let (roll, read) = Roll::from_halves(std::slice::from_ref(half));
            assert!(roll.is_empty(), "one signature is one node's own claim");
            assert_eq!(read.admitted, 0);
            assert_eq!(read.unpaired, 1);
        }
    }

    #[test]
    fn the_order_records_arrive_in_does_not_matter() {
        let (sponsor, newcomer) = (identity(1), identity(2));
        let mut forwards = admission(&sponsor, &newcomer, 100);
        let backwards: Vec<_> = forwards.iter().rev().cloned().collect();
        forwards.extend(admission(&identity(3), &identity(4), 50));
        let mut shuffled: Vec<_> = forwards.iter().rev().cloned().collect();
        shuffled.extend(backwards);

        let (a, _) = Roll::from_halves(&forwards);
        let (b, _) = Roll::from_halves(&shuffled);
        assert_eq!(a, b);
        assert_eq!(a.len(), 2);
    }

    #[test]
    fn somebody_who_received_the_file_twice_joined_once_and_at_the_earlier_time() {
        let newcomer = identity(9);
        let mut frames = admission(&identity(1), &newcomer, 200);
        frames.extend(admission(&identity(2), &newcomer, 100));
        let (roll, read) = Roll::from_halves(&frames);
        assert_eq!(read.admitted, 2);
        assert_eq!(roll.len(), 1);
        assert_eq!(
            roll.member(&newcomer.public_key())
                .expect("on the roll")
                .received_in,
            Epoch(100),
            "the earlier admission is the one that counts, whatever order they arrived"
        );
    }

    #[test]
    fn rubbish_is_counted_and_left_out() {
        let (sponsor, newcomer) = (identity(1), identity(2));
        let mut frames = admission(&sponsor, &newcomer, 100);
        frames.push(b"not a record at all".to_vec());
        frames.push(Vec::new());
        let (roll, read) = Roll::from_halves(&frames);
        assert_eq!(roll.len(), 1);
        assert_eq!(read.unreadable, 2);
    }

    #[test]
    fn merging_two_rolls_loses_nothing_and_costs_nobody_anything() {
        // The property the whole design leans on: healing a split has to be strictly
        // a gain, or nobody performs it.
        let shared = identity(9);
        let (mut here, _) = Roll::from_halves(&{
            let mut f = admission(&identity(1), &identity(2), 10);
            f.extend(admission(&identity(1), &shared, 20));
            f
        });
        let (there, _) = Roll::from_halves(&{
            let mut f = admission(&identity(3), &identity(4), 30);
            f.extend(admission(&identity(3), &shared, 20));
            f
        });

        let before = here.clone();
        here.merge(&there);
        // Four admissions, three members: one of them is on both rolls already.
        assert_eq!(here.len(), 3);
        for member in before.members() {
            assert_eq!(
                here.member(&member.key),
                Some(member),
                "nobody already known may change"
            );
        }
        for member in there.members() {
            assert!(here.member(&member.key).is_some(), "nobody may be dropped");
        }
    }

    #[test]
    fn merging_is_the_same_whichever_side_does_it() {
        let (a, _) = Roll::from_halves(&admission(&identity(1), &identity(2), 10));
        let (b, _) = Roll::from_halves(&admission(&identity(3), &identity(4), 30));
        let (mut left, mut right) = (a.clone(), b.clone());
        left.merge(&b);
        right.merge(&a);
        assert_eq!(left, right);
    }

    #[test]
    fn two_sponsors_in_one_epoch_are_settled_the_same_way_on_both_sides() {
        // The case the tie-break exists for. Without it the merge is not commutative
        // and one member has two different sponsors depending on who merged whom.
        let shared = identity(9);
        let (a, _) = Roll::from_halves(&admission(&identity(1), &shared, 50));
        let (b, _) = Roll::from_halves(&admission(&identity(2), &shared, 50));
        let (mut left, mut right) = (a.clone(), b.clone());
        left.merge(&b);
        right.merge(&a);
        assert_eq!(left, right);
        assert_eq!(
            left.member(&shared.public_key())
                .expect("on the roll")
                .sponsor,
            right
                .member(&shared.public_key())
                .expect("on the roll")
                .sponsor
        );
    }

    #[test]
    fn merging_keeps_the_earlier_admission_for_a_member_both_sides_know() {
        let shared = identity(9);
        let (mut late, _) = Roll::from_halves(&admission(&identity(1), &shared, 500));
        let (early, _) = Roll::from_halves(&admission(&identity(2), &shared, 100));
        late.merge(&early);
        assert_eq!(
            late.member(&shared.public_key())
                .expect("on the roll")
                .received_in,
            Epoch(100)
        );
    }

    #[test]
    fn the_roll_is_the_shape_the_draw_wants() {
        let (roll, _) = Roll::from_halves(&{
            let mut f = admission(&identity(1), &identity(2), 10);
            f.extend(admission(&identity(1), &identity(3), 10));
            f
        });
        let keys = roll.keys();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&identity(2).public_key()));
        assert_eq!(
            crate::draw::verifier_count(&identity(2).public_key(), &keys),
            1
        );
    }

    #[test]
    fn the_same_admission_told_twice_costs_nothing_and_changes_nothing() {
        // Gossip hands a node what it already holds every round, which is the mechanism
        // working. Taking it again must not admit anybody twice, must not leave a half
        // waiting, and must not depend on how many times anybody passed it on.
        let both = admission(&identity(1), &identity(2), 100);
        let mut held = Admissions::new();
        for frame in &both {
            held.add(frame);
        }
        let once = held.read();
        for _ in 0..3 {
            for frame in &both {
                held.add(frame);
            }
        }
        assert_eq!(held.read(), once);
        assert_eq!(once.admitted, 1);
        assert_eq!(
            once.unpaired, 0,
            "a pair that completed is not still waiting"
        );
        let (whole, read) = Roll::from_halves(&both);
        assert_eq!(held.roll(), &whole);
        assert_eq!(once, read);
    }

    #[test]
    fn a_half_that_arrives_long_after_its_other_side_still_pairs() {
        let (sponsor, newcomer) = (identity(1), identity(2));
        let both = admission(&sponsor, &newcomer, 100);
        let mut held = Admissions::new();
        held.add(&both[0]);
        for other in [
            admission(&identity(3), &identity(4), 50),
            admission(&identity(5), &identity(6), 60),
        ] {
            for frame in &other {
                held.add(frame);
            }
        }
        assert!(held.roll().member(&newcomer.public_key()).is_none());
        held.add(&both[1]);
        assert!(held.roll().member(&newcomer.public_key()).is_some());
        assert_eq!(held.read().admitted, 3);
        assert_eq!(held.read().unpaired, 0);
    }
}
