//! Who this node knows: the roll it has admissions for, and where they said to look.
//!
//! Both are built by reading files rather than kept as authoritative state, so a half
//! this build cannot open is still kept, still passed on, and still counted by a build
//! that can.

use std::collections::BTreeSet;
use std::sync::atomic::Ordering;

use anyhow::Context as _;
use n333_core::attestation;
use n333_core::roll::Roll;
use n333_core::transfer::{self, Half};
use n333_core::whereabouts::{self};
use n333_core::{Epoch, utterance};

use super::{Heard, Node};

impl Node {
    /// The members this node knows of, in the shape the draw takes.
    pub(crate) async fn roll(&self) -> BTreeSet<[u8; 32]> {
        self.state.lock().await.roll.keys()
    }

    /// Keep halves of admissions, and put anyone they complete on the roll.
    ///
    /// Unreadable halves are kept too. A half this build cannot open may still pair up
    /// for a build that can, and the roll is rebuilt from the file every time anyway.
    ///
    /// # Errors
    /// Fails if the file cannot be written or read back.
    pub(crate) async fn admit(&self, halves: &[Vec<u8>]) -> anyhow::Result<usize> {
        let mut state = self.state.lock().await;
        for half in halves {
            state
                .admissions
                .append(half)
                .context("keeping an admission")?;
        }
        let frames = state
            .admissions
            .read_all()
            .context("reading the admissions")?;
        let (roll, _) = Roll::from_halves(&frames);
        state.roll = roll;
        Ok(state.roll.len())
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
        let addresses: Vec<Vec<u8>> = state.directory.frames().map(<[u8]>::to_vec).collect();

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
        let admissions = state
            .admissions
            .read_all()
            .context("reading the admissions")?;

        Ok(share_the_room(
            [addresses, about_epochs, admissions],
            self.passed_on.fetch_add(1, Ordering::Relaxed),
        ))
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
            let before = self.state.lock().await.roll.len();
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
            .roll
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

/// Fit several kinds of statement into one run without letting any kind starve.
///
/// Each kind gets an equal share; a kind that does not use its share gives it back to
/// the others. `offset` rotates where each kind starts, so a node that permanently has
/// more to say than fits does not send the same frames every single round and never
/// the rest.
fn share_the_room<const N: usize>(kinds: [Vec<Vec<u8>>; N], offset: u64) -> Tidings {
    let room = n333_net::frame::MAX_BATCH_FRAMES;
    let held: usize = kinds.iter().map(Vec::len).sum();
    let mut taken = vec![0_usize; N];
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
    for (n, kind) in kinds.into_iter().enumerate() {
        let want = taken.get(n).copied().unwrap_or_default();
        if kind.is_empty() {
            continue;
        }
        let start = usize::try_from(offset % kind.len() as u64).unwrap_or_default();
        frames.extend(kind.iter().cycle().skip(start).take(want).cloned());
    }
    Tidings {
        left_behind: held.saturating_sub(frames.len()),
        frames,
    }
}
