//! Reading a node's files without touching them.
//!
//! READ-ONLY, ALWAYS. Every file is opened with `File::open`, which cannot create,
//! append to or truncate anything. A torn tail (the node was writing at that instant) is
//! left where it is and the frames before it are read; the node repairs its own files
//! the next time it opens them. `identity.key` is never named here.
//!
//! The names are the ones the client gives its files (`crates/cli/src/node.rs`).

use std::path::Path;

use n333_core::Epoch;

use crate::frames;

/// The node's own record chain: its first entry names the node's key.
const CHAIN_FILE: &str = "chain.log";

/// The admission halves the roll is made of.
const ADMISSIONS_FILE: &str = "admissions.log";

/// Statements nodes signed about where they are.
const WHEREABOUTS_FILE: &str = "whereabouts.log";

/// What others signed about this node, kept past the window.
const WITNESSED_FILE: &str = "witnessed.log";

/// One file per epoch of statements: `<epoch, 20 digits>.seg`.
const WINDOW_DIR: &str = "statements";

/// How many epochs of statements are read, counting back from now.
pub(crate) const RECENT_EPOCHS: u64 = 3;

/// The frames a node holds, as far as the observation needs them.
#[derive(Debug, Default)]
pub(crate) struct Held {
    /// The node's own chain.
    pub(crate) chain: Vec<Vec<u8>>,
    /// Admission halves.
    pub(crate) admissions: Vec<Vec<u8>>,
    /// Whereabouts statements.
    pub(crate) whereabouts: Vec<Vec<u8>>,
    /// Attestations about this node.
    pub(crate) witnessed: Vec<Vec<u8>>,
    /// Statements filed under the last [`RECENT_EPOCHS`] epochs.
    pub(crate) recent: Vec<Vec<u8>>,
}

/// Read everything the observation is made of, from the node at `home`.
///
/// A file that is missing or cannot be read counts as empty: a node that has not yet
/// written one has nothing in it to show.
pub(crate) fn read(home: &Path, now: Epoch) -> Held {
    let mut recent = Vec::new();
    for back in 0..RECENT_EPOCHS {
        let Some(epoch) = now.0.checked_sub(back) else {
            break;
        };
        recent.extend(frames_of(
            &home.join(WINDOW_DIR).join(format!("{epoch:020}.seg")),
        ));
    }
    Held {
        chain: frames_of(&home.join(CHAIN_FILE)),
        admissions: frames_of(&home.join(ADMISSIONS_FILE)),
        whereabouts: frames_of(&home.join(WHEREABOUTS_FILE)),
        witnessed: frames_of(&home.join(WITNESSED_FILE)),
        recent,
    }
}

/// Every whole frame in one file.
fn frames_of(path: &Path) -> Vec<Vec<u8>> {
    use std::io::Read as _;
    let mut bytes = Vec::new();
    match std::fs::File::open(path).and_then(|mut file| file.read_to_end(&mut bytes)) {
        Ok(_) => frames::split(&bytes),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(error) => {
            tracing::warn!("{} could not be read: {error}", path.display());
            Vec::new()
        }
    }
}
