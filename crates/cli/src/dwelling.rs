//! Where this node lives, and whether it still lives here.
//!
//! Two nodes answering to one key are one node contradicting itself: each keeps a
//! record, each signs answers naming where its record stands, and the two records
//! cannot both be true. The ordinary way to arrive there is not malice but a copy —
//! a directory carried to a new machine with the old one left running. Two small
//! files inside the node's directory are how this client notices.
//!
//! `packed` is written by `333 pack` after the node has been written into a file for
//! moving. From then on nothing acts as this node in this directory, and that is a
//! REFUSAL, because it is certain: the person said, in that command, that the node
//! was leaving. The one way back is `333 pack --undo`, for a move that was abandoned.
//!
//! `here` holds the canonical path the directory had when the name was made, when it
//! was unpacked, or when somebody last said it had moved. If the node opens somewhere
//! else, that is a WARNING and not a refusal. A different path is certain; what it
//! means is not. A folder renamed, a disk remounted, a home directory moved and a
//! directory copied with the original still running all look exactly the same from
//! inside it, and only the last is two copies of one name. Refusing would be the
//! client claiming to know which one happened, and it does not. So it says what it
//! found, what that would mean if it was a copy, and the one command that settles it
//! (`333 moved`), and carries on. Nothing about the machine is looked at to decide:
//! a fingerprint of the hardware identifies a machine rather than a key, which is a
//! thing this client does not do.

use std::path::{Path, PathBuf};
use std::time::SystemTime;

use anyhow::{Context as _, bail};
use fs_mistrust::CheckedDir;

/// The file holding where this directory was, and how it came to be there.
pub(crate) const HERE_FILE: &str = "here";

/// The file saying this node was packed for moving.
pub(crate) const PACKED_FILE: &str = "packed";

/// How a directory came to be where `here` says it was.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum How {
    /// The name was made in it.
    Made,
    /// A packed node was unpacked into it.
    Unpacked,
    /// Somebody said, with `333 moved`, that it was moved rather than copied.
    Moved,
    /// A node older than this record was first opened by a client that keeps it.
    Found,
}

impl How {
    /// The word written in the file. FROZEN in effect: an older word must still read.
    const fn word(self) -> &'static str {
        match self {
            Self::Made => "made",
            Self::Unpacked => "unpacked",
            Self::Moved => "moved",
            Self::Found => "found",
        }
    }

    /// Read a word back.
    fn from_word(word: &str) -> Option<Self> {
        [Self::Made, Self::Unpacked, Self::Moved, Self::Found]
            .into_iter()
            .find(|how| how.word() == word)
    }

    /// How it is said in a sentence ending in the path.
    const fn said(self) -> &'static str {
        match self {
            Self::Made => "made at",
            Self::Unpacked => "unpacked at",
            Self::Moved => "said to have moved to",
            Self::Found => "first opened by this client at",
        }
    }
}

/// Write `bytes` to `name` inside `home`, replacing what was there in one step.
///
/// Written beside it and renamed over it, so that a machine losing power halfway
/// leaves the old file or the new one and never half of either.
///
/// # Errors
/// Fails if the file cannot be written or renamed.
pub(crate) fn replace(home: &CheckedDir, name: &str, bytes: &[u8]) -> anyhow::Result<()> {
    use std::io::Write as _;
    let partial = format!("{name}.partial");
    let mut file = home
        .open(
            &partial,
            std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true),
        )
        .with_context(|| format!("writing {name}"))?;
    file.write_all(bytes)?;
    file.sync_all()?;
    drop(file);
    std::fs::rename(home.as_path().join(&partial), home.as_path().join(name))
        .with_context(|| format!("putting {name} in place"))
}

/// This directory's canonical path, as it is written down and compared.
///
/// # Errors
/// Fails if the path cannot be resolved.
pub(crate) fn canonical(dir: &Path) -> anyhow::Result<PathBuf> {
    std::fs::canonicalize(dir).with_context(|| format!("resolving {}", dir.display()))
}

/// Write down that the node inside `home` lives at `at`, and how it came to.
///
/// `at` is passed rather than read from `home`, because an unpacked node is written
/// in one place and renamed into another, and what is recorded is the second.
///
/// # Errors
/// Fails if the file cannot be written.
pub(crate) fn record_here(home: &CheckedDir, how: How, at: &Path) -> anyhow::Result<()> {
    let text = format!("{}\n{}\n", how.word(), at.to_string_lossy());
    replace(home, HERE_FILE, text.as_bytes())
}

/// Read back how and where, if it was written down and reads as something.
fn read_here(home: &Path) -> Option<(How, PathBuf)> {
    let text = std::fs::read_to_string(home.join(HERE_FILE)).ok()?;
    let (word, path) = text.split_once('\n')?;
    let path = path.strip_suffix('\n').unwrap_or(path);
    Some((How::from_word(word)?, PathBuf::from(path)))
}

/// What to say if this node is not where it was; and write it down if nothing was.
///
/// Only called on a directory that already holds a name. A node older than this
/// record has nothing to compare against, so where it is now is written down and
/// nothing is said: there is no evidence of anything.
///
/// # Errors
/// Fails if the directory's path cannot be resolved or the record cannot be written.
pub(crate) fn check_here(home: &CheckedDir) -> anyhow::Result<Option<String>> {
    let now = canonical(home.as_path())?;
    match read_here(home.as_path()) {
        None => record_here(home, How::Found, &now).map(|()| None),
        Some((_, was)) if was == now => Ok(None),
        Some((how, was)) => Ok(Some(elsewhere(how, &was, &now, &command(&now, "moved")))),
    }
}

/// What is said when a node opens somewhere other than where it was.
fn elsewhere(how: How, was: &Path, now: &Path, settle: &str) -> String {
    format!(
        "home     this node was {} {},\n\
         \x20        and it is now at {}.\n\
         \x20        If the directory was moved or renamed, that is all this is. If it was\n\
         \x20        copied and the one it came from still runs, this is one name in two\n\
         \x20        places, and each will contradict the other's record. Once only one of\n\
         \x20        them is left, say so here: {settle}",
        how.said(),
        was.display(),
        now.display(),
    )
}

/// What `packed` says: when, and into which file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Packed {
    /// The moment, as written: UTC, ISO 8601, to the second.
    pub(crate) at: String,
    /// The file it was written into, as its path was when it was written.
    pub(crate) into: String,
}

/// Mark this directory as packed for moving.
///
/// # Errors
/// Fails if the marker cannot be written.
pub(crate) fn mark_packed(home: &CheckedDir, at: SystemTime, into: &Path) -> anyhow::Result<()> {
    let text = format!("{}\n{}\n", crate::began::utc(at), into.to_string_lossy());
    replace(home, PACKED_FILE, text.as_bytes())
}

/// Read the marker, if there is one. A marker that does not read is still a marker.
#[must_use]
pub(crate) fn packed(home: &Path) -> Option<Packed> {
    let path = home.join(PACKED_FILE);
    if !path.exists() {
        return None;
    }
    let text = std::fs::read_to_string(&path).unwrap_or_default();
    let mut lines = text.lines();
    let unread = "a moment that could not be read";
    Some(Packed {
        at: lines.next().unwrap_or(unread).to_owned(),
        into: lines
            .next()
            .unwrap_or("a file whose name could not be read")
            .to_owned(),
    })
}

/// Refuse to act as this node if it was packed for moving.
///
/// # Errors
/// Fails, saying why and how to undo it, if the directory is marked.
pub(crate) fn refuse_if_packed(home: &Path) -> anyhow::Result<()> {
    let Some(packed) = packed(home) else {
        return Ok(());
    };
    bail!(
        "this node was packed for moving at {}, into {}.\n\
         It lives wherever that file was unpacked. Running it here too would be one\n\
         name in two places, so nothing in {} will act as it.\n\
         \n\
         If the move was abandoned and that file was never unpacked anywhere, this\n\
         puts it back: {}",
        packed.at,
        packed.into,
        home.display(),
        command(home, "pack --undo")
    )
}

/// Take the marker away, so the node lives here again.
///
/// # Errors
/// Fails if the marker cannot be removed.
pub(crate) fn unmark_packed(home: &CheckedDir) -> anyhow::Result<()> {
    home.remove_file(PACKED_FILE)
        .context("removing the mark that this node was packed")
}

/// The command a person types to do `rest` to the node in `home`.
///
/// The default directory needs no `--data-dir`, and a command with one in it that was
/// not needed reads as if it were.
#[must_use]
pub(crate) fn command(home: &Path, rest: &str) -> String {
    let default = crate::paths::NodePaths::default_home();
    let is_default =
        default.root() == home || canonical(default.root()).is_ok_and(|default| default == home);
    if is_default {
        format!("333 {rest}")
    } else {
        format!("333 --data-dir {} {rest}", quoted(home))
    }
}

/// A path as it can be pasted into a shell.
fn quoted(path: &Path) -> String {
    let text = path.to_string_lossy();
    let plain = text
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || "/._-~:\\+,@".contains(c));
    if plain {
        text.into_owned()
    } else {
        format!("'{}'", text.replace('\'', r"'\''"))
    }
}
