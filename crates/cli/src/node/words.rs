//! What was said in an epoch, and what this node makes of having heard it.
//!
//! Everything filed under an epoch lives together — questions put, answers given,
//! statements published, signals spoken — because they are all the same kind of thing:
//! bytes somebody signed with this epoch written inside the signature. The rules for
//! reading them differ; the rule for keeping them does not.

use std::collections::BTreeSet;

use anyhow::Context as _;
use n333_core::attestation;
use n333_core::challenge;
use n333_core::extinction::{Vigil, Watched};
use n333_core::presence;
use n333_core::{Epoch, utterance};

use super::Node;

impl Node {
    /// Keep a statement about some epoch.
    ///
    /// Nothing is checked here. A frame is kept as it arrived and judged when it is
    /// read, which is what lets a statement this build does not understand still be
    /// passed on by a build that does.
    ///
    /// # Errors
    /// Fails if the file cannot be written.
    pub(crate) async fn keep(&self, epoch: Epoch, frame: &[u8]) -> anyhow::Result<()> {
        self.state
            .lock()
            .await
            .window
            .record(epoch, frame)
            .with_context(|| format!("keeping a statement about epoch {}", epoch.0))
    }

    /// Everything held about one epoch.
    ///
    /// # Errors
    /// Fails if the file cannot be read.
    pub(crate) async fn statements(&self, epoch: Epoch) -> anyhow::Result<Vec<Vec<u8>>> {
        self.state
            .lock()
            .await
            .window
            .read(epoch)
            .with_context(|| format!("reading the statements about epoch {}", epoch.0))
    }

    /// Keep an utterance, whoever it came from, filed under the epoch it was said in.
    ///
    /// # Errors
    /// Fails if the file cannot be written.
    pub(crate) async fn keep_utterance(&self, frame: &[u8]) -> anyhow::Result<Epoch> {
        let signed = utterance::open(frame).context("reading an utterance")?;
        let epoch = signed.utterance.epoch();
        self.keep(epoch, frame).await?;
        Ok(epoch)
    }

    /// What this node heard said in an epoch, and by whom.
    ///
    /// # Errors
    /// Fails if the epoch cannot be read.
    pub(crate) async fn overheard(&self, epoch: Epoch) -> anyhow::Result<utterance::Heard> {
        let mut heard = utterance::Heard::new();
        for frame in self.statements(epoch).await? {
            if let Ok(signed) = utterance::open(&frame) {
                heard.take(&signed);
            }
        }
        Ok(heard)
    }

    /// Mark this epoch as one this node was awake for.
    ///
    /// Written whether or not anything happens in it, because the difference between
    /// an epoch nobody spoke in and an epoch this node was switched off for is the
    /// whole of its right to say the network has ended.
    ///
    /// # Errors
    /// Fails if the file cannot be created.
    pub(crate) async fn keeping(&self, epoch: Epoch) -> anyhow::Result<()> {
        self.state
            .lock()
            .await
            .window
            .touch(epoch)
            .with_context(|| format!("marking epoch {} as kept", epoch.0))
    }

    /// What this node watched, epoch by epoch, over everything it still holds.
    ///
    /// An epoch it was not running for is skipped, which breaks the run: a node cannot
    /// vouch for what happened while it was not there.
    ///
    /// # Errors
    /// Fails if the window cannot be listed or read.
    pub(crate) async fn watched(&self, now: Epoch) -> anyhow::Result<Vigil> {
        let state = self.state.lock().await;
        let mut vigil = Vigil::new();
        for number in presence::window(now).chain(std::iter::once(now.0)) {
            let epoch = Epoch(number);
            if !state.window.kept(epoch) {
                continue;
            }
            let statements = state
                .window
                .read(epoch)
                .with_context(|| format!("reading epoch {number}"))?;
            vigil.watch(
                epoch,
                if statements.is_empty() {
                    Watched::Nobody
                } else {
                    Watched::Someone
                },
            );
        }
        Ok(vigil)
    }

    /// Everyone this node holds a signed word from, stamped with this epoch or the last.
    ///
    /// One rule rather than several: whatever the message was — a question put, an
    /// answer given, a statement published, a signal spoken — somebody signed it and
    /// wrote this epoch inside the signature, and it reached this node. That is the
    /// whole of what "here" can mean from one machine.
    ///
    /// Its own observation and nobody else's. Two nodes will not agree on this number
    /// and are not meant to. This node is never in it: a count that included the
    /// counter could never reach zero, and reaching zero is the one thing the number
    /// is for.
    ///
    /// # Errors
    /// Fails if the window cannot be read.
    pub(crate) async fn answering(&self, now: Epoch) -> anyhow::Result<BTreeSet<[u8; 32]>> {
        let me = self.identity.public_key();
        let state = self.state.lock().await;
        let mut here = BTreeSet::new();
        for number in [now.0.saturating_sub(1), now.0] {
            let epoch = Epoch(number);
            for frame in state
                .window
                .read(epoch)
                .with_context(|| format!("reading epoch {number}"))?
            {
                for who in signers_of(&frame, epoch) {
                    if who != me {
                        here.insert(who);
                    }
                }
            }
        }
        Ok(here)
    }

    /// Forget statements about epochs that can no longer change anybody's standing.
    ///
    /// # Errors
    /// Fails if a file cannot be removed.
    pub(crate) async fn forget_old(&self, now: Epoch) -> anyhow::Result<usize> {
        self.state
            .lock()
            .await
            .window
            .forget_before(now)
            .context("forgetting old statements")
    }
}

/// Whose signatures a statement carries, if it is stamped with `epoch`.
///
/// A positive attestation carries two: the verifier who published it and the prover
/// whose answer is inside it, and the prover's is the stronger of the two — it could
/// not have been made without them.
fn signers_of(frame: &[u8], epoch: Epoch) -> Vec<[u8; 32]> {
    if let Ok(signed) = attestation::open(frame) {
        if signed.attestation.epoch != epoch.0 {
            return Vec::new();
        }
        let mut both = vec![signed.attestation.verifier];
        if signed.is_positive() {
            both.push(signed.attestation.prover);
        }
        return both;
    }
    if let Ok(signed) = challenge::open_challenge(frame) {
        return if signed.challenge.epoch == epoch.0 {
            vec![signed.challenge.verifier]
        } else {
            Vec::new()
        };
    }
    if let Ok(signed) = challenge::open_answer(frame) {
        return if signed.answer.epoch == epoch.0 {
            vec![signed.answer.prover]
        } else {
            Vec::new()
        };
    }
    if let Ok(signed) = utterance::open(frame) {
        return if signed.utterance.epoch == epoch.0 {
            vec![signed.utterance.speaker]
        } else {
            Vec::new()
        };
    }
    Vec::new()
}

