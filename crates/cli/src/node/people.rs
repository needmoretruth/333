//! Who this node knows: the roll it has admissions for, and where they said to look.
//!
//! Both are built by reading files rather than kept as authoritative state, so a half
//! this build cannot open is still kept, still passed on, and still counted by a build
//! that can.

use std::collections::BTreeSet;

use anyhow::Context as _;
use n333_core::attestation;
use n333_core::roll::Member;
use n333_core::transfer::{self, Half};
use n333_core::whereabouts::{self};
use n333_core::{Epoch, utterance};

use super::sources::Source;
use super::{Heard, Node};
use crate::words::Arg;

impl Node {
    /// The members this node knows of, in the shape the draw takes.
    pub(crate) async fn roll(&self) -> BTreeSet<[u8; 32]> {
        self.state.lock().await.admissions.roll().keys()
    }

    /// Keep halves of admissions, and put anyone they complete on the roll.
    ///
    /// Unreadable halves are kept too. A half this build cannot open may still pair up
    /// for a build that can, and passing it on costs nothing.
    ///
    /// A half already held is not written down again. Every peer offers what it has
    /// every round, and most of what arrives is what this node handed that peer in the
    /// first place — so an admission that has travelled a thousand times has to cost a
    /// thousandth of nothing, or a network where nobody is joining still fills a disk.
    ///
    /// # Errors
    /// Fails if the file cannot be written.
    pub(crate) async fn admit(&self, halves: &[Vec<u8>]) -> anyhow::Result<usize> {
        let mut state = self.state.lock().await;
        state.admissions.keep(halves)?;
        Ok(state.admissions.roll().len())
    }

    /// Everything this node is willing to pass on to a peer.
    ///
    /// Three kinds travel, and none of them may starve the others. Addresses, because
    /// a node that does not know where the members are cannot ask them anything.
    /// Admissions, because that is the only way a roll ever grows past the one step a
    /// newcomer is handed at the door. And what was said about the epochs that have not
    /// been judged yet.
    ///
    /// WHY THE ROOM IS SHARED RATHER THAN FILLED IN ORDER. One run holds a few hundred
    /// frames. Filling it with addresses first works perfectly until the day a node
    /// knows a few hundred addresses, and then admissions stop travelling entirely,
    /// for ever, with no error and no counter — and the symptom is a network that
    /// quietly stops growing, which looks exactly like a network nobody is joining.
    /// Each kind gets its own share, gives back what it does not use, and the offset
    /// moves every round so that it is not always the same peers who get through.
    ///
    /// Utterances from the same epochs go too. They are the only statements
    /// here that are worth nothing unless they spread: a signal nobody relays is a
    /// signal one node heard, and the whole of what the count is for is the shape of
    /// what everybody said.
    ///
    /// So do statements about the epochs that have not been judged yet. Those have to
    /// travel or the two-thirds rule cannot bind on anybody: a node that was asked and
    /// did not answer is judged absent only if EVERY verifier drawn for it published a
    /// negative, and a negative that never left the verifier's disk is a negative
    /// nobody can read. They stop travelling the moment the epoch is old enough to have
    /// been judged, so this is at most four epochs of them and never a warehouse.
    ///
    /// The same run goes to a newcomer at the door, where it is the difference between
    /// a node that can take part and one that knows nobody and nowhere.
    ///
    /// # Errors
    /// Fails if the logs cannot be read.
    pub(crate) async fn tidings(&self, now: Epoch) -> anyhow::Result<Tidings> {
        let mut state = self.state.lock().await;
        let oldest = now
            .0
            .saturating_sub(n333_core::attestation::JUDGEMENT_DELAY_EPOCHS);
        let mut about_epochs = Vec::new();
        for number in oldest..=now.0 {
            let epoch = Epoch(number);
            let held = state
                .window
                .read(epoch)
                .with_context(|| words!("node-people-reading-epoch", epoch = Arg::exact(number)))?;
            about_epochs.extend(held.into_iter().filter(|frame| {
                // Everything anybody said about this epoch except the questions and the
                // answers: those two are the prover's own receipt and are its business
                // to keep, not this node's to hand around.
                utterance::open(frame).is_ok() || attestation::open(frame).is_ok()
            }));
        }
        let offsets = state.passed_on;
        let lengths = [
            state.directory.len(),
            about_epochs.len(),
            state.admissions.frames().len(),
        ];
        let (tidings, taken) = share_the_room(
            [
                state.directory.frames().collect(),
                about_epochs.iter().map(Vec::as_slice).collect(),
                state
                    .admissions
                    .frames()
                    .iter()
                    .map(Vec::as_slice)
                    .collect(),
            ],
            offsets,
        );
        // Where each kind stopped is where it starts next time. Advancing by one
        // instead — which is what this did — means a node holding more than fits sends
        // almost the same run for ever, and a genuinely new admission waits behind
        // every old one, once per round, for as many rounds as there are records.
        for (kind, ((offset, took), held)) in state
            .passed_on
            .iter_mut()
            .zip(taken)
            .zip(lengths)
            .enumerate()
        {
            *offset = onward(*offset, took, held, kind);
        }
        Ok(tidings)
    }

    /// File what a peer passed on, each statement by what it opens as.
    ///
    /// Nothing is trusted about who handed these over, which is why there is no check
    /// on that. A statement either opens under its own signature or it does not. Who
    /// it was is written down only as where the addresses came from, for this node's
    /// owner to read.
    ///
    /// # Errors
    /// Fails if a log cannot be written.
    pub(crate) async fn hear(
        &self,
        told: &[Vec<u8>],
        now: Epoch,
        from: &Source,
    ) -> anyhow::Result<Heard> {
        let mut heard = Heard::default();
        let mut admissions = Vec::new();
        let mut speakers = BTreeSet::new();
        for frame in told {
            if let Ok(signed) = whereabouts::open(frame) {
                if self.note_signed(signed, frame, from, now).await? {
                    heard.addresses += 1;
                }
            } else if transfer::open(frame, Half::Gave).is_ok()
                || transfer::open(frame, Half::Received).is_ok()
            {
                admissions.push(frame.clone());
            } else if let Ok(signed) = utterance::open(frame) {
                if self.keep_new(signed.utterance.epoch(), frame).await? {
                    speakers.insert(signed.utterance.speaker);
                }
            } else if let Ok(signed) = attestation::open(frame) {
                // Kept only while it could still change a verdict. After that the epoch
                // has been judged by everyone who was going to judge it, and holding
                // other people's statements about it is being an archive nobody asked
                // for.
                let epoch = Epoch(signed.attestation.epoch);
                if still_open(epoch, now) && self.keep_new(epoch, frame).await? {
                    heard.witnessed += 1;
                }
            } else {
                heard.unreadable += 1;
            }
        }
        heard.speakers = speakers.len();
        if !admissions.is_empty() {
            let before = self.state.lock().await.admissions.roll().len();
            heard.were = before;
            heard.members = self.admit(&admissions).await?.saturating_sub(before);
        }
        Ok(heard)
    }

    /// The hands this node's copy came through, this node's own first.
    ///
    /// Walked over admissions already on disk: each member's record names who handed
    /// it to them, and that person's record names who handed it to *them*. The walk
    /// stops at the first key this node holds no admission for — which is either
    /// whoever was given the file by nobody, or simply where this node stopped
    /// knowing. Nothing here can tell those apart, and nothing should pretend to.
    ///
    /// A roll assembled from records that disagree could name a circle. The walk
    /// refuses to go round one rather than deciding which of them is the lie.
    pub(crate) async fn lineage(&self) -> Vec<Member> {
        let state = self.state.lock().await;
        let roll = state.admissions.roll();
        let mut walked = BTreeSet::new();
        let mut hands = Vec::new();
        let mut key = self.identity.public_key();
        while walked.insert(key)
            && let Some(member) = roll.member(&key)
        {
            hands.push(member.clone());
            key = member.sponsor;
        }
        hands
    }

    /// Everybody on the roll, with the epoch each was handed the file.
    pub(crate) async fn members(&self) -> Vec<Member> {
        let state = self.state.lock().await;
        state.admissions.roll().members().cloned().collect()
    }

    /// The epoch somebody handed this node the file, if anybody has.
    ///
    /// Absent for a node nobody has admitted — which is both a node that has not
    /// joined and the one node that was never given the file by anybody.
    pub(crate) async fn joined_in(&self) -> Option<Epoch> {
        let key = self.identity.public_key();
        self.state
            .lock()
            .await
            .admissions
            .roll()
            .member(&key)
            .map(|member| member.received_in)
    }

    /// Which epoch of this line `now` is, the first being the epoch of the earliest
    /// admission this node holds.
    ///
    /// That is the epoch the file was first handed on, if this node has been handed
    /// that admission, and otherwise the earliest it has been handed: the same limit
    /// as [`Node::lineage`], and nothing here can tell the two apart. Absent for a
    /// node that holds no admission, and for a clock behind that epoch.
    pub(crate) async fn line_epoch(&self, now: Epoch) -> Option<u64> {
        let state = self.state.lock().await;
        let roll = state.admissions.roll();
        let first = roll.members().map(|member| member.received_in.0).min()?;
        now.0.checked_sub(first).map(|run| run.saturating_add(1))
    }

    /// Has the file passed between this node and `peer` in `epoch`, either way?
    ///
    /// Asked before `join` asks `peer` for it: handed back in the same epoch it would
    /// be the same handover, both sides would sign it again, and nobody would be
    /// admitted.
    pub(crate) async fn handed_with(&self, peer: &[u8; 32], epoch: Epoch) -> bool {
        let me = self.identity.public_key();
        self.state.lock().await.admissions.between(&me, peer, epoch)
    }
}

/// Could anything said about this epoch still change what anybody writes down?
///
/// A verdict is reached three epochs after the fact and then never revisited, so a
/// statement about an epoch older than that arrives too late for every reader at once.
fn still_open(epoch: Epoch, now: Epoch) -> bool {
    epoch.0 + n333_core::attestation::JUDGEMENT_DELAY_EPOCHS >= now.0 && epoch.0 <= now.0 + 1
}

/// What one node passes on to another, and what it could not fit.
#[derive(Debug, Clone, Default)]
pub(crate) struct Tidings {
    /// The frames to send.
    pub(crate) frames: Vec<Vec<u8>>,
    /// How many were left behind for want of room.
    pub(crate) left_behind: usize,
}

/// How many kinds of statement travel in one run.
pub(crate) const KINDS: usize = 3;

/// Where a kind's next run begins, after one that took `took` of the `held` it had.
///
/// On from where it stopped, so that one pass over everything takes as few runs as
/// fit it. And at the end of every pass, somewhere else: the same number of runs
/// every round — one for the peers this node dials, one for each that dials it —
/// would otherwise bring a pass back to where it began, and whoever takes the first
/// run of every round would be handed the same slice of what this node holds for as
/// long as it held it.
///
/// Scattered by a hash of where the pass ended rather than drawn from chance: nothing
/// here needs to be unpredictable, only not periodic, and a rule that gives the same
/// answer twice is a rule a test can hold to.
fn onward(offset: u64, took: usize, held: usize, kind: usize) -> u64 {
    use std::hash::{Hash as _, Hasher as _};
    let next = offset.wrapping_add(took as u64);
    let Some(held) = std::num::NonZeroU64::new(held as u64) else {
        return next;
    };
    if next / held == offset / held {
        return next;
    }
    let mut scatter = std::hash::DefaultHasher::new();
    (next, kind).hash(&mut scatter);
    next.wrapping_add(scatter.finish() % held)
}

/// Fit several kinds of statement into one run without letting any kind starve.
///
/// Each kind gets an equal share; a kind that does not use its share gives it back to
/// the others. `offsets` say where each kind starts, so a node that permanently has
/// more to say than fits does not send the same frames every single round and never
/// the rest. What comes back with the run is how many of each kind it took, which is
/// how far that kind's next run begins.
fn share_the_room<const N: usize>(
    kinds: [Vec<&[u8]>; N],
    offsets: [u64; N],
) -> (Tidings, [usize; N]) {
    let room = n333_net::frame::MAX_BATCH_FRAMES;
    let held: usize = kinds.iter().map(Vec::len).sum();
    let mut taken = [0_usize; N];
    let mut left = room;

    // Hand out the room a round at a time. A kind with nothing more to give is simply
    // skipped, which is how its share reaches the others without any arithmetic.
    while left > 0 && taken.iter().zip(&kinds).any(|(t, k)| *t < k.len()) {
        for (n, kind) in kinds.iter().enumerate() {
            if left == 0 {
                break;
            }
            if let Some(count) = taken.get_mut(n)
                && *count < kind.len()
            {
                *count += 1;
                left -= 1;
            }
        }
    }

    let mut frames = Vec::with_capacity(room.min(held));
    for (n, kind) in kinds.iter().enumerate() {
        let want = taken.get(n).copied().unwrap_or_default();
        if kind.is_empty() {
            continue;
        }
        let from = offsets.get(n).copied().unwrap_or_default();
        let start = usize::try_from(from % kind.len() as u64).unwrap_or_default();
        frames.extend(
            kind.iter()
                .cycle()
                .skip(start)
                .take(want)
                .map(|frame| frame.to_vec()),
        );
    }
    (
        Tidings {
            left_behind: held.saturating_sub(frames.len()),
            frames,
        },
        taken,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let pairs = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            [(
                words!(
                    "node-people-reading-epoch",
                    epoch = crate::words::Arg::exact(89_612)
                ),
                "reading epoch 89612",
            )]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

    /// `count` frames of one kind, each one distinguishable from the others.
    fn kind(mark: u8, count: usize) -> Vec<Vec<u8>> {
        (0..count)
            .map(|n| {
                let mut frame = vec![mark];
                frame.extend(n.to_be_bytes());
                frame
            })
            .collect()
    }

    #[test]
    fn a_node_with_more_to_say_than_fits_gets_all_of_it_out_in_a_few_rounds() {
        // The rotation used to move by one frame a round. A node holding four times
        // what fits would then need four times as many rounds as it holds records
        // before the last one had been offered to anybody once — at a few thousand
        // records that is years, and what waits at the back is every new member.
        let room = n333_net::frame::MAX_BATCH_FRAMES;
        let all = kind(b'a', room * 4);
        let borrowed: Vec<&[u8]> = all.iter().map(Vec::as_slice).collect();

        let mut offsets = [0_u64; 1];
        let mut seen = std::collections::BTreeSet::new();
        let mut rounds = 0;
        while seen.len() < all.len() {
            let (run, took) = share_the_room([borrowed.clone()], offsets);
            seen.extend(run.frames);
            offsets[0] = offsets[0].wrapping_add(took[0] as u64);
            rounds += 1;
            assert!(rounds <= 8, "a full pass is taking rounds it should not");
        }
        assert_eq!(rounds, 4, "each round carries a roomful nobody has had yet");
    }

    #[test]
    fn no_kind_is_starved_by_a_kind_that_has_more_to_say() {
        let room = n333_net::frame::MAX_BATCH_FRAMES;
        let (many, few) = (kind(b'a', room * 2), kind(b'b', 4));
        let (run, took) = share_the_room(
            [
                many.iter().map(Vec::as_slice).collect(),
                few.iter().map(Vec::as_slice).collect(),
            ],
            [0, 0],
        );
        assert_eq!(took[1], few.len(), "the small kind is sent whole");
        assert_eq!(run.frames.len(), room);
        assert_eq!(took[0], room - few.len(), "and gives the rest back");
    }

    #[tokio::test]
    async fn what_is_heard_again_is_neither_kept_again_nor_counted_again() {
        // What a restart or a join used to do: every trade handed back the same
        // statements, each was written again, and one speaker was said as many.
        let home =
            std::env::temp_dir().join(format!("n333-people-test-again-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&home);
        std::fs::create_dir_all(&home).expect("creates dir");
        let mistrust = fs_mistrust::Mistrust::new_dangerously_trust_everyone();
        let (node, _) =
            Node::open(&mistrust, &home, super::super::Keeping::TheWindow).expect("opens");
        let speaker = n333_core::Identity::from_seed(&[5; 32]);
        let now = Epoch(1_000);
        let said = |epoch| {
            let signal = n333_core::signal::Signal::new(7).expect("a signal");
            utterance::Utterance::of(&speaker, signal, epoch)
                .seal(&speaker)
                .expect("seals")
        };
        let told = vec![said(now), said(now), said(Epoch(now.0 - 1))];

        let first = node.hear(&told, now, &Source::ByHand).await.expect("hears");
        assert_eq!(first.speakers, 1, "one of us spoke, in two epochs");
        let file = home
            .join(super::super::WINDOW_DIR)
            .join(format!("{:020}.seg", now.0));
        let size = std::fs::metadata(&file).expect("kept").len();

        let again = node.hear(&told, now, &Source::ByHand).await.expect("hears");
        assert_eq!(again.speakers, 0, "nothing new was said");
        assert_eq!(std::fs::metadata(&file).expect("kept").len(), size);
        let _ = std::fs::remove_dir_all(&home);
    }

    #[tokio::test]
    async fn the_peer_that_asks_first_every_round_still_hears_everything() {
        // Twice what fits, and two runs put together every round: one for the peers this
        // node dials, one for the peer that dials it. A rotation that only ever moves on
        // by what it sent comes back to the same place every round, and whoever takes
        // the first run hears the same half of the addresses for ever.
        let home =
            std::env::temp_dir().join(format!("n333-people-test-first-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&home);
        std::fs::create_dir_all(&home).expect("creates dir");
        let room = n333_net::frame::MAX_BATCH_FRAMES;
        let mut log = Vec::new();
        for seed in 0..room * 2 {
            let mut bytes = [0_u8; 32];
            bytes[..8].copy_from_slice(&(seed as u64).to_be_bytes());
            let someone = n333_core::Identity::from_seed(&bytes);
            let frame =
                whereabouts::Whereabouts::of(&someone, format!("n{seed}.example:3333"), Epoch(1))
                    .seal(&someone)
                    .expect("seals");
            log.extend(u32::try_from(frame.len()).expect("small").to_be_bytes());
            log.extend(frame);
        }
        std::fs::write(home.join("whereabouts.log"), log).expect("writes");
        let mistrust = fs_mistrust::Mistrust::new_dangerously_trust_everyone();
        let (node, opened) =
            Node::open(&mistrust, &home, super::super::Keeping::TheWindow).expect("opens");
        assert_eq!(opened.addresses, room * 2);

        let mut heard_first = BTreeSet::new();
        for _ in 0..12 {
            heard_first.extend(node.tidings(Epoch(2)).await.expect("gathers").frames);
            let _second = node.tidings(Epoch(2)).await.expect("gathers");
        }
        assert_eq!(
            heard_first.len(),
            room * 2,
            "every address reaches the peer that happens to ask first"
        );
        let _ = std::fs::remove_dir_all(&home);
    }
}
