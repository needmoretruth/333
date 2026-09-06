//! An append-only log that keeps each record once.
//!
//! WHY A NODE NEEDS ONE. What arrives from a peer is mostly what this node handed that
//! peer in the first place: passing a record on is the only way it travels, so the same
//! bytes come back every round from everybody. Written down each time, a network where
//! nothing at all is happening still fills a disk, and reading the file back costs more
//! every day. Here a record that has travelled a thousand times costs one hash.
//!
//! IT IS THE BYTES THAT ARE COMPARED, NOT WHAT THEY MEAN. This does not open a record,
//! check a signature or know what one is — that belongs to whoever reads them. Two
//! records are the same record when they are the same bytes, which is a rule that
//! cannot be wrong, and the cost of it is that a second signature over the same
//! statement is a second record. That is honest: it is a thing somebody signed twice.

use std::collections::HashSet;
use std::path::Path;

use n333_core::subject::digest_of;

use crate::log::{Error, Log};

/// An append-only log that keeps each record once.
#[derive(Debug)]
pub struct Once {
    /// The file itself.
    log: Log,
    /// What it holds, by digest. Kept in memory because the answer is needed on every
    /// record that arrives, and reading the file to find out is the cost this exists
    /// to remove.
    held: HashSet<[u8; 32]>,
}

impl Once {
    /// Open the file, and read back what is in it with any repeats left out.
    ///
    /// A file written by an earlier build may hold the same record many times. It is
    /// left on disk — rewriting a node's own history to save space is not a trade this
    /// makes — and read once.
    ///
    /// # Errors
    /// Fails if the file cannot be opened or read.
    pub fn open(path: &Path) -> Result<(Self, Vec<Vec<u8>>), Error> {
        let (mut log, _) = Log::open(path)?;
        let stored = log.read_all()?;
        let mut held = HashSet::with_capacity(stored.len());
        let records: Vec<Vec<u8>> = stored
            .into_iter()
            .filter(|record| held.insert(digest_of(record)))
            .collect();
        Ok((Self { log, held }, records))
    }

    /// Keep a record, unless this log already holds it. True if it was written.
    ///
    /// # Errors
    /// Fails if the file cannot be written.
    pub fn keep(&mut self, record: &[u8]) -> Result<bool, Error> {
        let digest = digest_of(record);
        if self.held.contains(&digest) {
            return Ok(false);
        }
        // Written before it is remembered, so that a node that dies here comes back
        // holding what it wrote rather than believing in what it did not.
        self.log.append(record)?;
        self.held.insert(digest);
        Ok(true)
    }

    /// How many distinct records are held.
    #[must_use]
    pub fn len(&self) -> usize {
        self.held.len()
    }

    /// Is there nothing here?
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.held.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("n333-once-test-{name}"));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("creates dir");
        dir.join("kept.log")
    }

    #[test]
    fn the_same_record_handed_over_and_over_is_written_once() {
        let path = scratch("repeats");
        let (mut kept, empty) = Once::open(&path).expect("opens");
        assert!(empty.is_empty());

        assert!(kept.keep(b"a statement").expect("keeps"), "new");
        let size = std::fs::metadata(&path).expect("written").len();
        for _ in 0..10 {
            assert!(!kept.keep(b"a statement").expect("keeps"), "already held");
        }
        assert_eq!(std::fs::metadata(&path).expect("there").len(), size);
        assert_eq!(kept.len(), 1);

        assert!(kept.keep(b"another statement").expect("keeps"));
        let (reopened, records) = Once::open(&path).expect("reopens");
        assert_eq!(records.len(), 2, "both, each once, in the order written");
        assert_eq!(records[0], b"a statement");
        assert_eq!(reopened.len(), 2);
    }

    #[test]
    fn a_file_that_already_holds_repeats_is_read_as_what_it_says_once() {
        // What an earlier build left behind. It is not rewritten and not an error.
        let path = scratch("old-file");
        let (mut log, _) = Log::open(&path).expect("opens");
        for _ in 0..5 {
            log.append(b"the same thing").expect("appends");
        }
        drop(log);

        let (kept, records) = Once::open(&path).expect("opens");
        assert_eq!(records.len(), 1);
        assert_eq!(kept.len(), 1);
    }
}
