//! The admissions a node holds: the file they are kept in, and who they make members.
//!
//! WHY THE ROLL IS NOT REBUILT FROM THE FILE. A roll used to be whatever reading the
//! whole admissions file produced, which is honest and, for a node that is running,
//! ruinous: gossip hands a node the records it already holds several times an epoch,
//! so every round wrote them all down again and then checked every signature it had
//! ever checked. One new member cost the file, and the file grew for ever. Here the
//! file is read once, at the start, and what arrives afterwards is measured against
//! what is already held — so a record that has travelled a thousand times costs a
//! hash, and a record nobody has seen costs one append.
//!
//! NOTHING IS THROWN AWAY, ONLY NOT KEPT TWICE. A half this build cannot open is
//! still kept, still passed on, and still counted by a build that can.

use std::path::Path;

use anyhow::Context as _;
use n333_core::roll::{Admissions, Read, Roll};
use n333_store::Once;

/// Every admission this node has kept.
pub(crate) struct Admitted {
    /// The append-only file they live in, which keeps each of them once.
    log: Once,
    /// Each of them, in the order this node first saw it, for passing on.
    frames: Vec<Vec<u8>>,
    /// The halves paired up, and the roll they have made.
    paired: Admissions,
}

impl Admitted {
    /// Open the file and read what is in it.
    ///
    /// # Errors
    /// Fails if the file cannot be opened or read.
    pub(crate) fn open(path: &Path) -> anyhow::Result<(Self, Read)> {
        let (log, stored) = Once::open(path).context("opening the admissions")?;
        let mut paired = Admissions::new();
        for frame in &stored {
            paired.add(frame);
        }
        let read = paired.read();
        Ok((
            Self {
                log,
                frames: stored,
                paired,
            },
            read,
        ))
    }

    /// Keep the halves that are new, and say how many that was.
    ///
    /// # Errors
    /// Fails if the file cannot be written.
    pub(crate) fn keep(&mut self, halves: &[Vec<u8>]) -> anyhow::Result<usize> {
        let mut fresh = 0;
        for half in halves {
            if !self.log.keep(half).context("keeping an admission")? {
                continue;
            }
            self.paired.add(half);
            self.frames.push(half.clone());
            fresh += 1;
        }
        Ok(fresh)
    }

    /// Who these admissions have made members.
    pub(crate) const fn roll(&self) -> &Roll {
        self.paired.roll()
    }

    /// Every admission held, each once, for passing on.
    pub(crate) fn frames(&self) -> &[Vec<u8>] {
        &self.frames
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use n333_core::transfer::{Half, Record};
    use n333_core::{Epoch, Identity};

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("n333-admissions-test-{name}"));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("creates dir");
        dir.join("admissions.log")
    }

    /// Both halves of one handover, as they travel.
    fn admission(giver: u8, taker: u8) -> Vec<Vec<u8>> {
        let (giver, taker) = (
            Identity::from_seed(&[giver; 32]),
            Identity::from_seed(&[taker; 32]),
        );
        let epoch = Epoch(100);
        vec![
            Record::new(
                &giver,
                taker.public_key(),
                epoch,
                n333_core::subject::DIGEST,
            )
            .seal(Half::Gave, &giver)
            .expect("seals"),
            Record::new(
                &taker,
                giver.public_key(),
                epoch,
                n333_core::subject::DIGEST,
            )
            .seal(Half::Received, &taker)
            .expect("seals"),
        ]
    }

    #[test]
    fn an_admission_that_is_already_held_is_not_written_down_again() {
        // This is what gossip does all day: every peer offers what it has, and most of
        // it is what this node handed the peer in the first place. If that costs an
        // append, the file grows without bound on a network where nothing is happening.
        let path = scratch("repeats");
        let both = admission(1, 2);

        let (mut held, _) = Admitted::open(&path).expect("opens");
        assert_eq!(held.keep(&both).expect("keeps"), 2);
        let after_first = std::fs::metadata(&path).expect("written").len();

        for _ in 0..10 {
            assert_eq!(held.keep(&both).expect("keeps"), 0, "nothing of it is new");
        }
        assert_eq!(
            std::fs::metadata(&path).expect("still there").len(),
            after_first
        );
        assert_eq!(held.frames().len(), 2);
        assert_eq!(held.roll().len(), 1);

        // And a node that is restarted holds exactly what it wrote.
        let (again, read) = Admitted::open(&path).expect("reopens");
        assert_eq!(again.frames().len(), 2);
        assert_eq!(read.admitted, 1);
        assert_eq!(again.roll(), held.roll());
    }
}
