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
    /// A statement somebody signed ABOUT THIS NODE is kept twice: once with the epoch
    /// it belongs to, which is forgotten when the window moves past it, and once in a
    /// file that is never forgotten. It is the only part of this node's record that
    /// somebody else's key is on, and after the window there is nowhere else it
    /// survives. The negative ones are kept with the positive ones — a node that kept
    /// only what suited it would be keeping an advertisement, and it would know that
    /// about itself.
    ///
    /// # Errors
    /// Fails if the file cannot be written.
    pub(crate) async fn keep(&self, epoch: Epoch, frame: &[u8]) -> anyhow::Result<()> {
        let mut state = self.state.lock().await;
        state
            .window
            .record(epoch, frame)
            .with_context(|| format!("keeping a statement about epoch {}", epoch.0))?;
        if attestation::open(frame)
            .is_ok_and(|signed| signed.attestation.prover == self.identity.public_key())
        {
            state
                .witnessed
                .keep(frame)
                .context("keeping what was witnessed of this node")?;
        }
        Ok(())
    }

    /// How many statements other nodes signed about this one are held.
    pub(crate) async fn witnessed(&self) -> usize {
        self.state.lock().await.witnessed.len()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::Keeping;
    use n333_core::Identity;
    use n333_core::attestation::Attestation;

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("n333-words-test-{name}"));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("creates dir");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700))
                .expect("restricts dir");
        }
        dir
    }

    fn mistrust() -> fs_mistrust::Mistrust {
        fs_mistrust::Mistrust::builder()
            .ignore_prefix(std::env::temp_dir())
            .ignore_environment()
            .build()
            .expect("a buildable Mistrust")
    }

    /// A verifier's statement that somebody said nothing. The negative is used because
    /// it needs no answer from the prover, and it is the one a node has least reason to
    /// keep — which is the point.
    fn said_of(prover: [u8; 32], epoch: Epoch) -> Vec<u8> {
        let verifier = Identity::from_seed(&[7; 32]);
        Attestation::silent(&verifier, prover, epoch, [3; 32])
            .seal(&verifier)
            .expect("seals")
    }

    #[tokio::test]
    async fn what_others_signed_about_this_node_outlives_the_window() {
        // Everything else about an epoch is forgotten when the window moves past it.
        // These are the only part of a node's own record that somebody else's key is
        // on, so after the window they exist nowhere, and the record stops meaning
        // anything to a stranger.
        let home = scratch("witnessed");
        let (node, opened) =
            Node::open(&mistrust(), &home, Keeping::TheWindow).expect("opens a node");
        assert_eq!(opened.witnessed, 0);

        let epoch = Epoch(1000);
        let about_me = said_of(node.identity().public_key(), epoch);
        let about_somebody_else = said_of([9; 32], epoch);
        for frame in [&about_me, &about_me, &about_somebody_else] {
            node.keep(epoch, frame).await.expect("keeps");
        }
        assert_eq!(
            node.witnessed().await,
            1,
            "the one that names this node, and the same one twice is once"
        );

        let long_after = Epoch(epoch.0 + presence::WINDOW_EPOCHS + 4);
        assert_eq!(node.forget_old(long_after).await.expect("forgets"), 1);
        assert!(node.statements(epoch).await.expect("reads").is_empty());
        assert_eq!(node.witnessed().await, 1, "and this is still here");

        drop(node);
        let (_, reopened) =
            Node::open(&mistrust(), &home, Keeping::TheWindow).expect("opens again");
        assert_eq!(reopened.witnessed, 1);
    }
}
