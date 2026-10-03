//! What the server holds while it runs: the board, the gate, and where things are.

use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, PoisonError};

use anyhow::Context as _;

use crate::atomic;
use crate::board::Board;
use crate::gate::Gate;

/// The board's file inside `--state`.
const BOARD_FILE: &str = "board.json";

/// Only this server's own user reads the board file.
const BOARD_MODE: u32 = 0o600;

/// Everything a request may need.
pub(crate) struct State {
    /// The site directory, canonical, so a path inside it can be told from one outside.
    pub(crate) site: PathBuf,
    /// Where the board is kept.
    board_file: PathBuf,
    /// Where `observe` writes the observation.
    pub(crate) observation: PathBuf,
    /// The version pages show and name their assets by.
    pub(crate) version: String,
    /// The board. A plain lock, never held across an await.
    board: Mutex<Board>,
    /// Who wrote in the last minute. Taken only while the board's lock is held.
    gate: Mutex<Gate>,
    /// The newest board change on disk. Writes queue on this, never on the board.
    written: tokio::sync::Mutex<u64>,
}

impl State {
    /// Load the board from `state_dir` (made, private, if it is not there).
    ///
    /// # Errors
    /// Fails if the site directory is not there or the state directory cannot be made.
    pub(crate) fn open(
        site: &Path,
        state_dir: &Path,
        observation: PathBuf,
        version: String,
    ) -> anyhow::Result<Self> {
        let site = site
            .canonicalize()
            .with_context(|| format!("the site directory {}", site.display()))?;
        make_private_dir(state_dir)?;
        let board_file = state_dir.join(BOARD_FILE);
        let board = match std::fs::read(&board_file) {
            Ok(bytes) => Board::load(&bytes),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Board::default(),
            Err(error) => {
                return Err(error).with_context(|| format!("reading {}", board_file.display()));
            }
        };
        Ok(Self {
            site,
            board_file,
            observation,
            version,
            board: Mutex::new(board),
            gate: Mutex::new(Gate::new()),
            written: tokio::sync::Mutex::new(0),
        })
    }

    /// The board, for as long as the guard lives. Keep that short.
    pub(crate) fn board(&self) -> MutexGuard<'_, Board> {
        self.board.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// The gate. Take it only while holding the board, so the two are always taken in
    /// one order.
    pub(crate) fn gate(&self) -> MutexGuard<'_, Gate> {
        self.gate.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Write a snapshot of the board, unless a newer one is already on disk.
    ///
    /// A failure is logged and not returned: the statement is held in memory and handed
    /// out either way, and the next write carries it to disk.
    pub(crate) async fn persist(&self, generation: u64, bytes: Vec<u8>) {
        let mut written = self.written.lock().await;
        if generation <= *written {
            return;
        }
        let path = self.board_file.clone();
        let done =
            tokio::task::spawn_blocking(move || atomic::write(&path, &bytes, BOARD_MODE)).await;
        match done {
            Ok(Ok(())) => *written = generation,
            Ok(Err(error)) => tracing::error!("the board could not be written: {error}"),
            Err(error) => tracing::error!("the board write stopped: {error}"),
        }
    }
}

/// Make a directory only its owner can enter, if it is not there.
fn make_private_dir(dir: &Path) -> anyhow::Result<()> {
    let mut builder = std::fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt as _;
        builder.mode(0o700);
    }
    builder
        .create(dir)
        .with_context(|| format!("making {}", dir.display()))
}
