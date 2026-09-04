//! The statements a node holds about an epoch, kept for as long as they can still
//! change a verdict.
//!
//! One file per epoch, each an append-only [`crate::log::Log`]. Forgetting an epoch is
//! deleting a file, which is the cheapest operation a filesystem has and the only one
//! that genuinely reclaims the space.
//!
//! HOW LONG IT IS KEPT. The presence window is the last 333 completed epochs, so an
//! epoch older than that can no longer change anybody's standing and the raw
//! statements are dropped. What survives is this node's own chain, which holds the
//! verdict it reached at the time — see [`n333_core::chain`], which says plainly what
//! that costs a reader years later.
//!
//! A NODE THAT WANTS TO KEEP EVERYTHING JUST DOES NOT PRUNE. Nothing about that gives
//! it any authority: every statement carries its own signature, so a copy kept by a
//! stranger verifies exactly as well as one kept by anybody else, and there is no
//! canonical archive to be.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use n333_core::Epoch;
use n333_core::presence::WINDOW_EPOCHS;

use crate::log::{Error, Log, Opened};

/// The extension every epoch file carries.
const EXTENSION: &str = "seg";

/// Statements about an epoch — others' about this node, this node's about others,
/// and the challenges and answers behind them — one file per epoch.
pub struct Window {
    /// The directory holding the epoch files.
    root: PathBuf,
    /// How many epochs to keep, counting back from the newest written.
    keep: u64,
}

/// What was found for one epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Epochs {
    /// The oldest epoch still held, if any.
    pub oldest: Option<u64>,
    /// The newest epoch held, if any.
    pub newest: Option<u64>,
    /// How many epoch files there are.
    pub count: usize,
}

impl Window {
    /// Open the window under `root`, keeping the standard 333 epochs.
    ///
    /// # Errors
    /// Fails if the directory cannot be created.
    pub fn open(root: &Path) -> Result<Self, Error> {
        Self::keeping(root, WINDOW_EPOCHS)
    }

    /// Open the window keeping a different number of epochs.
    ///
    /// A node configured to keep everything passes [`u64::MAX`]. It gains nothing by
    /// doing so except its own copy.
    ///
    /// # Errors
    /// Fails if the directory cannot be created.
    pub fn keeping(root: &Path, keep: u64) -> Result<Self, Error> {
        std::fs::create_dir_all(root).map_err(|source| Error::Io {
            path: root.to_path_buf(),
            source,
        })?;
        Ok(Self {
            root: root.to_path_buf(),
            keep,
        })
    }

    /// The file one epoch's statements live in.
    fn path_for(&self, epoch: Epoch) -> PathBuf {
        // Zero-padded so that the directory lists in epoch order, which makes the
        // files legible to a person with nothing but `ls`.
        self.root.join(format!("{:020}.{EXTENSION}", epoch.0))
    }

    /// Add one statement to an epoch.
    ///
    /// # Errors
    /// Fails if the file cannot be opened or written.
    pub fn record(&self, epoch: Epoch, frame: &[u8]) -> Result<(), Error> {
        let (mut log, _) = Log::open(&self.path_for(epoch))?;
        log.append(frame)
    }

    /// Mark an epoch as one this node was awake for, without recording anything.
    ///
    /// The difference between "nobody spoke to me" and "I was not here" is the whole
    /// of a node's right to say the network has ended, and nothing else on disk holds
    /// it: an epoch in which nothing happened writes nothing, and an epoch the machine
    /// was switched off for also writes nothing. So an epoch this node kept is given a
    /// file whether or not anything went into it.
    ///
    /// # Errors
    /// Fails if the file cannot be created.
    pub fn touch(&self, epoch: Epoch) -> Result<(), Error> {
        Log::open(&self.path_for(epoch)).map(|_| ())
    }

    /// Was this node keeping this epoch at all?
    ///
    /// False for every epoch before this node was first run, every epoch it was
    /// switched off for, and every epoch already forgotten.
    #[must_use]
    pub fn kept(&self, epoch: Epoch) -> bool {
        self.path_for(epoch).exists()
    }

    /// Everything held for one epoch, in the order it arrived.
    ///
    /// An epoch nothing was ever recorded for reads as empty rather than missing:
    /// having heard nothing and never having asked look the same from here, and it is
    /// the reader that knows which it was.
    ///
    /// # Errors
    /// Fails if the file cannot be read.
    pub fn read(&self, epoch: Epoch) -> Result<Vec<Vec<u8>>, Error> {
        let path = self.path_for(epoch);
        if !path.exists() {
            return Ok(Vec::new());
        }
        let (mut log, _) = Log::open(&path)?;
        log.read_all()
    }

    /// Open one epoch's file directly, to see whether it was torn.
    ///
    /// # Errors
    /// Fails if the file cannot be opened.
    pub fn inspect(&self, epoch: Epoch) -> Result<Opened, Error> {
        Log::open(&self.path_for(epoch)).map(|(_, opened)| opened)
    }

    /// Which epochs are held.
    ///
    /// # Errors
    /// Fails if the directory cannot be listed.
    pub fn epochs(&self) -> Result<Epochs, Error> {
        let held = self.held()?;
        Ok(Epochs {
            oldest: held.first().copied(),
            newest: held.last().copied(),
            count: held.len(),
        })
    }

    /// Every epoch held, in order.
    fn held(&self) -> Result<Vec<u64>, Error> {
        let entries = std::fs::read_dir(&self.root).map_err(|source| Error::Io {
            path: self.root.clone(),
            source,
        })?;
        let mut epochs = BTreeSet::new();
        for entry in entries {
            let entry = entry.map_err(|source| Error::Io {
                path: self.root.clone(),
                source,
            })?;
            let name = entry.file_name();
            let name = name.to_string_lossy();
            // Anything that is not one of ours is left alone rather than guessed at.
            if let Some(number) = name.strip_suffix(&format!(".{EXTENSION}"))
                && let Ok(epoch) = number.parse::<u64>()
            {
                epochs.insert(epoch);
            }
        }
        Ok(epochs.into_iter().collect())
    }

    /// Forget epochs that can no longer change anybody's standing.
    ///
    /// Counted back from `now` rather than from the newest file held, because the
    /// newest file is whatever happened to arrive and a quiet network would stop
    /// pruning exactly when it had least reason to keep anything.
    ///
    /// Returns how many epochs were dropped.
    ///
    /// # Errors
    /// Fails if the directory cannot be listed or a file cannot be removed.
    pub fn forget_before(&self, now: Epoch) -> Result<usize, Error> {
        let Some(oldest_kept) = now.0.checked_sub(self.keep) else {
            return Ok(0);
        };
        let mut dropped = 0;
        for epoch in self.held()? {
            if epoch < oldest_kept {
                let path = self.path_for(Epoch(epoch));
                std::fs::remove_file(&path).map_err(|source| Error::Io { path, source })?;
                dropped += 1;
            }
        }
        Ok(dropped)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("n333-window-test-{name}"));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn statements_come_back_from_the_epoch_they_were_filed_under() {
        let root = scratch("roundtrip");
        let window = Window::open(&root).expect("opens");
        window.record(Epoch(10), b"first").expect("records");
        window.record(Epoch(10), b"second").expect("records");
        window.record(Epoch(11), b"elsewhere").expect("records");

        assert_eq!(
            window.read(Epoch(10)).expect("reads"),
            vec![b"first".to_vec(), b"second".to_vec()]
        );
        assert_eq!(
            window.read(Epoch(11)).expect("reads"),
            vec![b"elsewhere".to_vec()]
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn an_epoch_kept_is_told_from_an_epoch_nobody_was_here_for() {
        // Both hold nothing. One of them is evidence of silence and the other is
        // evidence of nothing at all, and a node may only claim the end from the
        // first kind.
        let root = scratch("kept");
        let window = Window::open(&root).expect("opens");
        window.touch(Epoch(7)).expect("touches");

        assert!(window.kept(Epoch(7)), "watched, and heard nothing");
        assert!(!window.kept(Epoch(6)), "not here for it");
        assert!(window.read(Epoch(7)).expect("reads").is_empty());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn an_epoch_nothing_was_filed_for_reads_as_empty() {
        let root = scratch("empty");
        let window = Window::open(&root).expect("opens");
        assert!(window.read(Epoch(99)).expect("reads").is_empty());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn the_window_reports_what_it_holds() {
        let root = scratch("range");
        let window = Window::open(&root).expect("opens");
        for epoch in [5_u64, 9, 7] {
            window.record(Epoch(epoch), b"x").expect("records");
        }
        assert_eq!(
            window.epochs().expect("lists"),
            Epochs {
                oldest: Some(5),
                newest: Some(9),
                count: 3
            }
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn epochs_that_can_no_longer_change_a_verdict_are_forgotten() {
        let root = scratch("prune");
        let window = Window::keeping(&root, 5).expect("opens");
        for epoch in 0..10_u64 {
            window.record(Epoch(epoch), b"x").expect("records");
        }
        assert_eq!(window.forget_before(Epoch(10)).expect("prunes"), 5);
        assert_eq!(
            window.epochs().expect("lists"),
            Epochs {
                oldest: Some(5),
                newest: Some(9),
                count: 5
            }
        );
        // Pruning again drops nothing: it is not a counter, it is a rule about what
        // is old, and running it twice must not eat into what is kept.
        assert_eq!(window.forget_before(Epoch(10)).expect("prunes"), 0);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn pruning_counts_back_from_now_and_not_from_the_newest_file() {
        // A network that went quiet stops writing new files. Counting back from the
        // newest one would then stop pruning for ever, which is exactly when a small
        // machine can least afford it.
        let root = scratch("quiet");
        let window = Window::keeping(&root, 5).expect("opens");
        for epoch in 0..3_u64 {
            window.record(Epoch(epoch), b"x").expect("records");
        }
        assert_eq!(window.forget_before(Epoch(100)).expect("prunes"), 3);
        assert_eq!(window.epochs().expect("lists").count, 0);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn nothing_is_forgotten_early_in_the_life_of_a_network() {
        let root = scratch("early");
        let window = Window::keeping(&root, 333).expect("opens");
        window.record(Epoch(1), b"x").expect("records");
        assert_eq!(window.forget_before(Epoch(2)).expect("prunes"), 0);
        assert_eq!(window.epochs().expect("lists").count, 1);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn the_default_window_is_the_one_standing_is_measured_over() {
        let root = scratch("default");
        let window = Window::open(&root).expect("opens");
        assert_eq!(window.keep, WINDOW_EPOCHS);
        assert_eq!(window.keep, 333);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn files_that_are_not_ours_are_left_alone() {
        let root = scratch("strangers");
        let window = Window::open(&root).expect("opens");
        window.record(Epoch(1), b"x").expect("records");
        std::fs::write(root.join("notes.txt"), b"someone put this here").expect("writes");
        std::fs::write(root.join("nonsense.seg"), b"not a number").expect("writes");

        assert_eq!(window.epochs().expect("lists").count, 1);
        assert_eq!(window.forget_before(Epoch(10_000)).expect("prunes"), 1);
        assert!(root.join("notes.txt").exists(), "left where it was");
        assert!(root.join("nonsense.seg").exists(), "left where it was");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn epoch_files_sort_the_way_a_person_would_read_them() {
        // Zero-padded, so `ls` shows them in epoch order for as long as an epoch
        // number fits in a u64.
        let root = scratch("sorted");
        let window = Window::keeping(&root, 333).expect("opens");
        let small = window.path_for(Epoch(9));
        let large = window.path_for(Epoch(10));
        assert!(small < large);
        assert!(window.path_for(Epoch(u64::MAX)) > large);
        let _ = std::fs::remove_dir_all(&root);
    }
}
