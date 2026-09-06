//! Everything one node is, opened from one directory.
//!
//! What lives here: the identity, the file itself if this node has been given it, the
//! roll of members this node has admissions for, the addresses nodes have said they
//! are at, its own record chain, and the window of statements it is holding. They are
//! opened together, because a node missing one of them is not a node that can take
//! part.
//!
//! THE FILE IS READ, NEVER WRITTEN FROM NOTHING. A client carries the hash and can
//! recognise `333.txt`, and it has no way to produce one: the bytes are only ever
//! written here after somebody handed them over. A directory holding a `333.txt` that
//! is not the file is treated as a directory with no file in it, and it is left where
//! it is rather than overwritten.
//!
//! ITS OWN CHAIN IS VERIFIED ON EVERY START. Not because somebody else might have
//! altered it — the file is inside a directory only its owner can enter — but because
//! the alternative is discovering it is broken at the moment it has to be answered
//! from, which is the moment it cannot be repaired.

mod admissions;
mod people;
pub(crate) use people::Tidings;
mod record;
mod words;

use std::path::Path;

use anyhow::Context as _;
use fs_mistrust::Mistrust;
use n333_core::chain::{self, Head};
use n333_core::roll::Read;
use n333_core::subject::{self, Subject};
use n333_core::whereabouts::Directory;
use n333_core::Identity;
use n333_store::{Log, Window};

use admissions::Admitted;
use tokio::sync::Mutex;

use crate::identity_file::{self, Origin};

/// The file holding this node's own record chain.
const CHAIN_FILE: &str = "chain.log";

/// The file holding the admissions this node knows about.
const ADMISSIONS_FILE: &str = "admissions.log";

/// The directory holding one file per epoch of statements.
const WINDOW_DIR: &str = "statements";

/// How much of the past a node holds on to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Keeping {
    /// The window standing is measured over, and nothing older. The ordinary case.
    TheWindow,
    /// Everything, for ever.
    ///
    /// It buys the node no authority whatsoever. Every statement carries its own
    /// signature, so a copy kept by a stranger verifies exactly as well as one kept
    /// here, and there is no canonical archive to be. What it buys is that somebody,
    /// somewhere, still has the bytes — which nothing in this protocol requires and
    /// nobody can be made to do.
    Everything,
}

impl Keeping {
    /// How many epochs that is.
    const fn epochs(self) -> u64 {
        match self {
            Self::TheWindow => n333_core::presence::WINDOW_EPOCHS,
            Self::Everything => u64::MAX,
        }
    }
}

/// The file holding what nodes have said about where they are.
const WHEREABOUTS_FILE: &str = "whereabouts.log";

/// One node.
pub(crate) struct Node {
    /// The directory everything of this node's lives in.
    home: std::path::PathBuf,
    /// The key everything is signed with.
    identity: Identity,
    /// The file, if this node has been given it.
    ///
    /// Outside the lock: it is written once, in the round that makes this node a
    /// member, and never changes afterwards.
    subject: Mutex<Option<Subject>>,
    /// Everything that changes, behind one lock.
    ///
    /// One lock rather than four: the things under it are written together — a
    /// verdict goes into the chain in the same breath the statements behind it are
    /// pruned — and four locks would be four chances to take them in a different
    /// order.
    state: Mutex<State>,
}

/// The parts of a node that change while it runs.
struct State {
    /// This node's own record, append-only.
    chain: Log,
    /// Where that chain currently ends: what every answer commits to.
    head: Head,
    /// Statements held, one file per epoch.
    window: Window,
    /// The admissions this node holds, and the roll they make.
    admissions: Admitted,
    /// How far through each kind of statement the last run of tidings got, so that a
    /// node with more to say than fits carries on rather than starting over.
    passed_on: [u64; people::KINDS],
    /// Where nodes have said they can be found.
    directory: Directory,
    /// The statements that directory was built from, append-only.
    whereabouts: Log,
}

/// What opening a node found, for the operator to see once.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Opened {
    /// How this node came to have a name.
    pub(crate) origin: Origin,
    /// How long its own record is.
    pub(crate) chain_length: u64,
    /// How many bytes of an unfinished record were dropped from the chain.
    pub(crate) chain_truncated: u64,
    /// How many members it knows of.
    pub(crate) members: usize,
    /// How many nodes it knows an address for.
    pub(crate) addresses: usize,
    /// Whether it has the file.
    pub(crate) has_the_file: bool,
    /// How much of the past this node holds on to.
    pub(crate) keeping: Keeping,
    /// What reading the admissions produced.
    pub(crate) read: Read,
}

impl Node {
    /// Open everything under `home`, creating what is not there.
    ///
    /// # Errors
    /// Fails if the identity cannot be read or written, a log cannot be opened, or
    /// this node's own chain does not verify.
    pub(crate) fn open(
        mistrust: &Mistrust,
        home: &Path,
        keeping: Keeping,
    ) -> anyhow::Result<(Self, Opened)> {
        let (identity, origin) = identity_file::load_or_create(mistrust, home)?;

        let (mut chain, chain_opened) =
            Log::open(&home.join(CHAIN_FILE)).context("opening this node's record")?;
        let entries = chain.read_all().context("reading this node's record")?;
        let head = chain::verify(&entries).context("this node's own record does not verify")?;

        let (admissions, read) = Admitted::open(&home.join(ADMISSIONS_FILE))?;

        let window = Window::keeping(&home.join(WINDOW_DIR), keeping.epochs())
            .context("opening the statements")?;

        let (mut whereabouts, _) =
            Log::open(&home.join(WHEREABOUTS_FILE)).context("opening the addresses")?;
        let (directory, _) =
            Directory::from_frames(&whereabouts.read_all().context("reading them")?);

        let subject = read_the_file(home);
        let opened = Opened {
            origin,
            chain_length: head.length,
            chain_truncated: chain_opened.truncated,
            members: admissions.roll().len(),
            addresses: directory.len(),
            has_the_file: subject.is_some(),
            keeping,
            read,
        };
        Ok((
            Self {
                home: home.to_path_buf(),
                identity,
                subject: Mutex::new(subject),
                state: Mutex::new(State {
                    chain,
                    head,
                    window,
                    admissions,
                    directory,
                    whereabouts,
                    passed_on: [0; people::KINDS],
                }),
            },
            opened,
        ))
    }

    /// The key everything is signed with.
    pub(crate) const fn identity(&self) -> &Identity {
        &self.identity
    }

    /// The file, if this node has it.
    pub(crate) async fn subject(&self) -> Option<Subject> {
        *self.subject.lock().await
    }

    /// Write the file down, now that somebody has handed it over.
    ///
    /// # Errors
    /// Fails if the file cannot be written.
    pub(crate) async fn receive(&self, subject: Subject) -> anyhow::Result<()> {
        let path = self.home.join(subject::FILENAME);
        std::fs::write(&path, subject.content())
            .with_context(|| format!("writing {}", path.display()))?;
        *self.subject.lock().await = Some(subject);
        Ok(())
    }

}

/// What filing a peer's statements changed.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Heard {
    /// Addresses that were newer than what was held.
    pub(crate) addresses: usize,
    /// Members the admissions completed.
    pub(crate) members: usize,
    /// How many were on the roll before those admissions were filed.
    ///
    /// Kept so that one trade bringing more of us than this node had ever held can be
    /// said as the thing it is, rather than as another routine line.
    pub(crate) were: usize,
    /// Statements about epochs still open to judgement.
    pub(crate) witnessed: usize,
    /// Utterances kept, including ones already held: what a node said travels by
    /// being repeated, so the same one arrives many times and that is the mechanism
    /// working rather than a duplicate.
    pub(crate) said: usize,
    /// Frames that opened as nothing this build knows.
    pub(crate) unreadable: usize,
}

/// Read `333.txt` out of a node's directory, if what is there is the file.
///
/// Anything else — missing, the wrong length, the wrong bytes — is no file. It is not
/// reported as an error and not replaced: a node that has not been given the file is
/// the ordinary state of a node that has just been started for the first time, and a
/// directory holding something else under that name is holding somebody's own file.
fn read_the_file(home: &Path) -> Option<Subject> {
    let bytes = std::fs::read(home.join(subject::FILENAME)).ok()?;
    Subject::recognise(&bytes).ok()
}
