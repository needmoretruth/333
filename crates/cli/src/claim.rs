//! One directory, one program: the claim a running 333 lays on its node's directory.
//!
//! Everything a node is lives in files that are only ever appended to, and opening one
//! repairs it: a record that was never finished is cut off the end. That is right when
//! the process that was writing it has died, and it is destruction when that process is
//! alive and in the middle of the write. Two programs with the same directory open can
//! do that to each other's only copy of a node's record, and nothing on the disk would
//! say which one did it.
//!
//! So the first thing any command does, before it reads a byte, is take a lock on a
//! file in that directory, and it holds it until it exits. The lock is the operating
//! system's, through the standard library: it goes when the process goes, however it
//! goes, so a crash leaves nothing behind that has to be cleared by hand. The process
//! number written inside is only for saying which program it is. It decides nothing.
//!
//! WHAT A SECOND ONE DOES INSTEAD. It does not wait, and it writes nothing. What can be
//! handed to a vigil that is running is handed to it; everything else is refused, and
//! the refusal says who has the directory.

use std::fs::{File, OpenOptions, TryLockError};
use std::io::{Read as _, Write as _};
use std::path::Path;

use anyhow::Context as _;
use fs_mistrust::Mistrust;

/// The file the lock is taken on, inside the node's directory.
pub(crate) const LOCK_FILE: &str = "lock";

/// This process holds the directory. Dropping it lets go.
#[derive(Debug)]
pub(crate) struct Claim {
    /// Held open for the lock on it, and never read again.
    _file: File,
}

/// Somebody else holds the directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Holder {
    /// The process number it wrote down, when that can be read.
    ///
    /// Absent on a system whose locks keep other programs from reading the file at
    /// all, and absent for a moment after the other one takes the lock and before it
    /// has written its number.
    pub(crate) pid: Option<u32>,
}

/// Who has the directory.
#[derive(Debug)]
pub(crate) enum Taken {
    /// This process does, until it exits.
    Ours(Claim),
    /// Another one does.
    Theirs(Holder),
}

/// Take the directory at `home` for this process, or say who has it.
///
/// The directory is made, or checked, first: the lock file is inside it, and a
/// directory other people can enter is refused before anything is written there.
///
/// # Errors
/// Fails if the directory is not private or the lock file cannot be opened or locked.
pub(crate) fn take(mistrust: &Mistrust, home: &Path) -> anyhow::Result<Taken> {
    let checked = crate::identity_file::secure(mistrust, home)?;
    // Not truncated on opening: the number in it is the other process's until the lock
    // says the file is this one's.
    let file = checked
        .open(
            LOCK_FILE,
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false),
        )
        .context("opening the lock on this node's directory")?;
    match file.try_lock() {
        Ok(()) => {
            write_our_number(&file).context("writing this process's number into the lock")?;
            Ok(Taken::Ours(Claim { _file: file }))
        }
        Err(TryLockError::WouldBlock) => Ok(Taken::Theirs(Holder {
            pid: their_number(&file),
        })),
        Err(TryLockError::Error(e)) => {
            Err(e).context("locking this node's directory for this process")
        }
    }
}

/// Replace whatever number is in the lock file with this process's.
fn write_our_number(mut file: &File) -> std::io::Result<()> {
    file.set_len(0)?;
    file.write_all(format!("{}\n", std::process::id()).as_bytes())
}

/// The number the holder wrote, if it can be read and is a number.
fn their_number(mut file: &File) -> Option<u32> {
    let mut written = String::new();
    file.read_to_string(&mut written).ok()?;
    written.trim().parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("n333-claim-test-{name}"));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    fn trusting() -> Mistrust {
        Mistrust::new_dangerously_trust_everyone()
    }

    #[test]
    fn a_second_claim_on_one_directory_is_refused_and_names_the_first() {
        // Two opens of the file in one process are two separate locks on every system
        // this is built for, which is what lets this be tested without a second
        // program. The end-to-end test does it with a second program as well.
        let home = scratch("second");
        let first = take(&trusting(), &home).expect("takes");
        assert!(matches!(first, Taken::Ours(_)));
        let second = take(&trusting(), &home).expect("looks");
        let Taken::Theirs(holder) = second else {
            panic!("two claims on one directory were both granted");
        };
        // A system whose locks keep the file from being read says no number at all.
        if cfg!(unix) {
            assert_eq!(holder.pid, Some(std::process::id()));
        }
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn a_claim_let_go_can_be_taken_again() {
        let home = scratch("again");
        let first = take(&trusting(), &home).expect("takes");
        drop(first);
        assert!(matches!(
            take(&trusting(), &home).expect("takes"),
            Taken::Ours(_)
        ));
        let _ = std::fs::remove_dir_all(&home);
    }

    #[cfg(unix)]
    #[test]
    fn a_refused_claim_writes_nothing() {
        let home = scratch("untouched");
        let _first = take(&trusting(), &home).expect("takes");
        let before = std::fs::read(home.join(LOCK_FILE)).expect("reads");
        let _second = take(&trusting(), &home).expect("looks");
        assert_eq!(std::fs::read(home.join(LOCK_FILE)).expect("reads"), before);
        let _ = std::fs::remove_dir_all(&home);
    }
}
