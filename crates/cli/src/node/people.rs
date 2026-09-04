//! Who this node knows: the roll it has admissions for, and where they said to look.
//!
//! Both are built by reading files rather than kept as authoritative state, so a half
//! this build cannot open is still kept, still passed on, and still counted by a build
//! that can.

use std::collections::BTreeSet;

use anyhow::Context as _;
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
    /// Addresses first, then admissions. Addresses because a node that does not know
    /// where the members are cannot ask them anything, which makes every other kind of
    /// statement moot; admissions because that is the only way a roll ever grows past
    /// the one step a newcomer is handed at the door.
    ///
    /// Utterances from this epoch and the last go too. They are the only statements
    /// here that are worth nothing unless they spread: a signal nobody relays is a
    /// signal one node heard, and the whole of what the count is for is the shape of
    /// what everybody said.
    ///
    /// Attestations are not passed on. A node keeps those about itself, because they
    /// are what it judges its own record from, and being a warehouse for everybody
    /// else's is a job nobody asked for and nothing here needs done.
    ///
    /// The same run goes to a newcomer at the door, where it is the difference between
    /// a node that can take part and one that knows nobody and nowhere.
    ///
    /// # Errors
    /// Fails if the logs cannot be read.
    pub(crate) async fn tidings(&self, now: Epoch) -> anyhow::Result<Vec<Vec<u8>>> {
        let mut state = self.state.lock().await;
        // Addresses first: everything else a peer could be told is useless to it while
        // it cannot reach anybody.
        let mut passed: Vec<Vec<u8>> = state.directory.frames().map(<[u8]>::to_vec).collect();
        for number in [now.0.saturating_sub(1), now.0] {
            let epoch = Epoch(number);
            let said = state
                .window
                .read(epoch)
                .with_context(|| format!("reading epoch {number}"))?;
            passed.extend(
                said.into_iter()
                    .filter(|frame| utterance::open(frame).is_ok()),
            );
        }
        passed.extend(
            state
                .admissions
                .read_all()
                .context("reading the admissions")?,
        );
        passed.truncate(n333_net::frame::MAX_BATCH_FRAMES);
        Ok(passed)
    }

    /// File what a peer passed on, each statement by what it opens as.
    ///
    /// Nothing is trusted about who handed these over, which is why there is no check
    /// on that. A statement either opens under its own signature or it does not.
    ///
    /// # Errors
    /// Fails if a log cannot be written.
    pub(crate) async fn hear(&self, told: &[Vec<u8>]) -> anyhow::Result<Heard> {
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
