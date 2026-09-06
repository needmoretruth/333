//! Who this node knows: the roll it has admissions for, and where they said to look.
//!
//! Both are built by reading files rather than kept as authoritative state, so a half
//! this build cannot open is still kept, still passed on, and still counted by a build
//! that can.

use std::collections::BTreeSet;

use anyhow::Context as _;
use n333_core::attestation;
use n333_core::transfer::{self, Half};
use n333_core::whereabouts::{self};
use n333_core::{Epoch, utterance};

use super::{Heard, Node};

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
        let oldest = now.0.saturating_sub(n333_core::attestation::JUDGEMENT_DELAY_EPOCHS);
        let mut about_epochs = Vec::new();
        for number in oldest..=now.0 {
            let epoch = Epoch(number);
            let held = state
                .window
                .read(epoch)
                .with_context(|| format!("reading epoch {number}"))?;
            about_epochs.extend(held.into_iter().filter(|frame| {
                // Everything anybody said about this epoch except the questions and the
                // answers: those two are the prover's own receipt and are its business
                // to keep, not this node's to hand around.
                utterance::open(frame).is_ok() || attestation::open(frame).is_ok()
            }));
        }
        let offsets = state.passed_on;
        let (tidings, taken) = share_the_room(
            [
                state.directory.frames().collect(),
                about_epochs.iter().map(Vec::as_slice).collect(),
                state.admissions.frames().iter().map(Vec::as_slice).collect(),
            ],
            offsets,
        );
        // Where each kind stopped is where it starts next time. Advancing by one
        // instead — which is what this did — means a node holding more than fits sends
        // almost the same run for ever, and a genuinely new admission waits behind
        // every old one, once per round, for as many rounds as there are records.
        for (offset, took) in state.passed_on.iter_mut().zip(taken) {
            *offset = offset.wrapping_add(took as u64);
        }
        Ok(tidings)
    }

    /// File what a peer passed on, each statement by what it opens as.
    ///
    /// Nothing is trusted about who handed these over, which is why there is no check
    /// on that. A statement either opens under its own signature or it does not.
    ///
    /// # Errors
    /// Fails if a log cannot be written.
    pub(crate) async fn hear(&self, told: &[Vec<u8>], now: Epoch) -> anyhow::Result<Heard> {
        let mut heard = Heard::default();
        let mut admissions = Vec::new();
        for frame in told {
            if whereabouts::open(frame).is_ok() {
                if self.note_address(frame).await? {
                    heard.addresses += 1;
                }
            } else if transfer::open(frame, Half::Gave).is_ok()
                || transfer::open(frame, Half::Received).is_ok()
            {
                admissions.push(frame.clone());
            } else if utterance::open(frame).is_ok() {
                self.keep_utterance(frame).await?;
                heard.said += 1;
            } else if let Ok(signed) = attestation::open(frame) {
                // Kept only while it could still change a verdict. After that the epoch
                // has been judged by everyone who was going to judge it, and holding
                // other people's statements about it is being an archive nobody asked
                // for.
                let epoch = Epoch(signed.attestation.epoch);
                if still_open(epoch, now) {
                    self.keep(epoch, frame).await?;
                    heard.witnessed += 1;
                }
            } else {
                heard.unreadable += 1;
            }
        }
        if !admissions.is_empty() {
            let before = self.state.lock().await.admissions.roll().len();
            heard.were = before;
            heard.members = self.admit(&admissions).await?.saturating_sub(before);
        }
        Ok(heard)
    }

    /// Keep a node's statement about where it is, if it is newer than what is held.
    ///
    /// # Errors
    /// Fails if the file cannot be written.
    pub(crate) async fn note_address(&self, frame: &[u8]) -> anyhow::Result<bool> {
        let signed = whereabouts::open(frame).context("reading an address")?;
        let mut state = self.state.lock().await;
        if !state.directory.note(signed, frame.to_vec()) {
            return Ok(false);
        }
        state
            .whereabouts
            .append(frame)
            .context("keeping an address")?;
        Ok(true)
    }

    /// Where a node last said it could be found.
    pub(crate) async fn address_of(&self, node: &[u8; 32]) -> Option<String> {
        self.state
            .lock()
            .await
            .directory
            .address_of(node)
            .map(ToOwned::to_owned)
    }

    /// Where every node other than this one last said it could be found.
    pub(crate) async fn where_others_are(&self) -> Vec<String> {
        let me = self.identity.public_key();
        self.state
            .lock()
            .await
            .directory
            .entries()
            .filter(|(key, _)| **key != me)
            .map(|(_, address)| address.to_owned())
            .collect()
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
}
