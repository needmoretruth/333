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

use std::collections::BTreeSet;
use std::path::Path;

use anyhow::Context as _;
use fs_mistrust::Mistrust;
use n333_core::chain::{self, Head};
use n333_core::roll::{Read, Roll};
use n333_core::subject::{self, Subject};
use n333_core::transfer::{self, Half};
use n333_core::whereabouts::{self, Directory};
use n333_core::{Epoch, Identity};
use n333_store::{Log, Window};
use tokio::sync::Mutex;

use crate::identity_file::{self, Origin};

/// The file holding this node's own record chain.
const CHAIN_FILE: &str = "chain.log";

/// The file holding the admissions this node knows about.
const ADMISSIONS_FILE: &str = "admissions.log";

/// The directory holding one file per epoch of statements.
const WINDOW_DIR: &str = "statements";

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
    /// Who this node knows to be a member.
    roll: Roll,
    /// Statements held, one file per epoch.
    window: Window,
    /// The admissions the roll was built from, append-only.
    admissions: Log,
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
    /// What reading the admissions produced.
    pub(crate) read: Read,
}

impl Node {
    /// Open everything under `home`, creating what is not there.
    ///
    /// # Errors
    /// Fails if the identity cannot be read or written, a log cannot be opened, or
    /// this node's own chain does not verify.
    pub(crate) fn open(mistrust: &Mistrust, home: &Path) -> anyhow::Result<(Self, Opened)> {
        let (identity, origin) = identity_file::load_or_create(mistrust, home)?;

        let (mut chain, chain_opened) =
            Log::open(&home.join(CHAIN_FILE)).context("opening this node's record")?;
        let entries = chain.read_all().context("reading this node's record")?;
        let head = chain::verify(&entries).context("this node's own record does not verify")?;

        let (mut admissions, _) =
            Log::open(&home.join(ADMISSIONS_FILE)).context("opening the admissions")?;
        let (roll, read) = Roll::from_halves(&admissions.read_all().context("reading them")?);

        let window = Window::open(&home.join(WINDOW_DIR)).context("opening the statements")?;

        let (mut whereabouts, _) =
            Log::open(&home.join(WHEREABOUTS_FILE)).context("opening the addresses")?;
        let (directory, _) =
            Directory::from_frames(&whereabouts.read_all().context("reading them")?);

        let subject = read_the_file(home);
        let opened = Opened {
            origin,
            chain_length: head.length,
            chain_truncated: chain_opened.truncated,
            members: roll.len(),
            addresses: directory.len(),
            has_the_file: subject.is_some(),
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
                    roll,
                    window,
                    admissions,
                    directory,
                    whereabouts,
                }),
            },
            opened,
        ))
    }

    /// The key everything is signed with.
    pub(crate) const fn identity(&self) -> &Identity {
        &self.identity
    }

    /// Where this node's record ends right now.
    pub(crate) async fn head(&self) -> Head {
        self.state.lock().await.head
    }

    /// The members this node knows of, in the shape the draw takes.
    pub(crate) async fn roll(&self) -> BTreeSet<[u8; 32]> {
        self.state.lock().await.roll.keys()
    }

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

    /// The file, if this node has it.
    pub(crate) async fn subject(&self) -> Option<Subject> {
        *self.subject.lock().await
    }

    /// Everything this node is willing to pass on to a peer.
    ///
    /// Addresses first, then admissions. Addresses because a node that does not know
    /// where the members are cannot ask them anything, which makes every other kind of
    /// statement moot; admissions because that is the only way a roll ever grows past
    /// the one step a newcomer is handed at the door.
    ///
    /// Statements about epochs are not passed on. A node keeps those about itself,
    /// because they are what it judges its own record from, and being a warehouse for
    /// everybody else's is a job nobody asked for and nothing here needs done.
    ///
    /// The same run goes to a newcomer at the door, where it is the difference between
    /// a node that can take part and one that knows nobody and nowhere.
    ///
    /// # Errors
    /// Fails if the logs cannot be read.
    pub(crate) async fn tidings(&self) -> anyhow::Result<Vec<Vec<u8>>> {
        let mut state = self.state.lock().await;
        let mut passed: Vec<Vec<u8>> = state
            .directory
            .frames()
            .map(<[u8]>::to_vec)
            .collect();
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

    /// Write one epoch's verdict into this node's own record.
    ///
    /// # Errors
    /// Fails if the entry cannot be sealed or written.
    pub(crate) async fn record(
        &self,
        epoch: Epoch,
        attendance: n333_core::presence::Attendance,
        evidence: [u8; 32],
    ) -> anyhow::Result<Head> {
        let mut state = self.state.lock().await;
        let entry = chain::Entry::following(
            Some(&state.head),
            &self.identity,
            epoch,
            attendance,
            evidence,
        );
        let frame = entry.seal(&self.identity).context("sealing the entry")?;
        // The head moves only after the bytes are on the disk. A head advanced first
        // and written second is a head that answers can commit to and nothing holds.
        state.chain.append(&frame).context("writing the entry")?;
        state.head = Head {
            digest: n333_core::subject::digest_of(&frame),
            length: state.head.length + 1,
        };
        Ok(state.head)
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

    /// Is this node on its own roll — has anybody admitted it?
    pub(crate) async fn is_admitted(&self) -> bool {
        let key = self.identity.public_key();
        self.state.lock().await.roll.member(&key).is_some()
    }

    /// The newest epoch this node's own record judges, if it has judged any.
    pub(crate) async fn last_judged(&self) -> anyhow::Result<Option<Epoch>> {
        let mut state = self.state.lock().await;
        let frames = state.chain.read_all().context("reading this node's record")?;
        let Some(last) = frames.last() else {
            return Ok(None);
        };
        Ok(Some(chain::open(last).context("reading the last entry")?.entry.epoch()))
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

/// What filing a peer's statements changed.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Heard {
    /// Addresses that were newer than what was held.
    pub(crate) addresses: usize,
    /// Members the admissions completed.
    pub(crate) members: usize,
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
