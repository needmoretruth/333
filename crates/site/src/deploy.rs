//! `333-site deploy`: build what `main` holds and switch to it, or keep what runs.
//!
//! Run by the site's own unprivileged user from a timer every five minutes, inside a
//! clean clone it owns. One run:
//!
//! 1. `git fetch origin main`. If `origin/main` is the commit in `<releases>/current/COMMIT`,
//!    stop: nothing changed.
//! 2. `git reset --hard <that commit>`, then `cargo build --release --locked -p n333-site`
//!    while holding the machine's heavy-work lock (`/tmp/big-heavy.lock`, waiting up to an
//!    hour), so a deploy never builds at the same time as somebody else's build.
//! 3. Copy the binary and `site/` into `<releases>/<commit>/`, with two small files beside
//!    them: `VERSION` (the short commit, which `serve --version-file` reads and pages show)
//!    and `COMMIT` (the whole one, which step 1 compares).
//! 4. Point `<releases>/current` at it by renaming a new symlink over the old one, so
//!    there is no moment with no `current`; then `systemctl --user restart <unit>`.
//! 5. Delete all but the newest three releases. The one `current` names is never deleted.
//!
//! IF THE BUILD FAILS, what runs keeps running and this exits non-zero with the reason.
//! The commit is written to `<releases>/.failed`, and later runs refuse to build that
//! same commit again: a broken commit would otherwise cost the machine a full build every
//! five minutes. A newer commit is built as usual; deleting the file retries this one.

mod release;

use std::fs::{File, OpenOptions, TryLockError};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use anyhow::{Context as _, bail};

/// The lock every heavy job on this machine takes.
const HEAVY_LOCK: &str = "/tmp/big-heavy.lock";

/// How long to wait for it.
const LOCK_PATIENCE: Duration = Duration::from_secs(3600);

/// How many releases are kept, the running one among them.
const KEEP: usize = 3;

/// What `deploy` is told.
#[derive(clap::Args)]
pub(crate) struct Args {
    /// A clean clone of the repository, owned by this user.
    #[arg(long)]
    repo: PathBuf,
    /// Where releases go; `current` is the symlink to the running one.
    #[arg(long)]
    releases: PathBuf,
    /// The systemd user unit that serves, restarted after a switch.
    #[arg(long)]
    unit: String,
}

/// Deploy `origin/main` if it is not what runs.
///
/// # Errors
/// Fails if git, the build, the copy, the switch or the restart fails, saying which.
pub(crate) fn run(args: &Args) -> anyhow::Result<()> {
    std::fs::create_dir_all(&args.releases)
        .with_context(|| format!("making {}", args.releases.display()))?;
    git(&args.repo, &["fetch", "--quiet", "origin", "main"])?;
    let target = git_says(
        &args.repo,
        &["rev-parse", "--verify", "origin/main^{commit}"],
    )?;
    if release::deployed(&args.releases).as_deref() == Some(target.as_str()) {
        tracing::info!("{target} is already running");
        return Ok(());
    }
    if release::failed(&args.releases).as_deref() == Some(target.as_str()) {
        bail!("{target} failed to build before; waiting for a newer commit");
    }
    git(&args.repo, &["reset", "--quiet", "--hard", &target])?;
    let short = git_says(&args.repo, &["rev-parse", "--short", &target])?;
    if let Err(failure) = build(&args.repo) {
        release::mark_failed(&args.releases, &target)?;
        return Err(failure.context(format!(
            "{short} was not deployed; the running release stays"
        )));
    }
    release::stage(&args.repo, &args.releases, &target, &short)?;
    release::switch(&args.releases, &target)?;
    run_command(
        Command::new("systemctl").args(["--user", "restart", &args.unit]),
        "systemctl --user restart",
    )?;
    tracing::info!("{short} is running");
    for gone in release::prune(&args.releases, KEEP)? {
        tracing::info!("deleted {}", gone.display());
    }
    Ok(())
}

/// Build the site binary, holding the heavy-work lock.
fn build(repo: &Path) -> anyhow::Result<()> {
    let _held = heavy_lock()?;
    run_command(
        Command::new("cargo")
            .args(["build", "--release", "--locked", "-p", "n333-site"])
            .current_dir(repo),
        "cargo build",
    )
}

/// Take the machine's heavy-work lock, waiting up to [`LOCK_PATIENCE`].
///
/// The same `flock(2)` lock `flock -w 3600 /tmp/big-heavy.lock` takes, taken here rather
/// than through the `flock` command. That command opens the file with `O_CREAT`, and with
/// `fs.protected_regular` set (Ubuntu sets it to 2) the kernel refuses that open in `/tmp`
/// when another user owns the file — which it does whenever the operator built last.
/// Opening an existing file without `O_CREAT` is allowed, so this only creates the file
/// when nobody has yet. The lock is released when the returned file is dropped.
fn heavy_lock() -> anyhow::Result<File> {
    let file = match File::open(HEAVY_LOCK) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(HEAVY_LOCK)
            .with_context(|| format!("making {HEAVY_LOCK}"))?,
        Err(error) => return Err(error).with_context(|| format!("opening {HEAVY_LOCK}")),
    };
    let deadline = Instant::now() + LOCK_PATIENCE;
    loop {
        match file.try_lock() {
            Ok(()) => return Ok(file),
            Err(TryLockError::WouldBlock) if Instant::now() < deadline => {
                std::thread::sleep(Duration::from_secs(1));
            }
            Err(TryLockError::WouldBlock) => bail!("{HEAVY_LOCK} was held for over an hour"),
            Err(TryLockError::Error(error)) => {
                return Err(error).with_context(|| format!("locking {HEAVY_LOCK}"));
            }
        }
    }
}

/// Run git in `repo`.
fn git(repo: &Path, arguments: &[&str]) -> anyhow::Result<()> {
    run_command(
        Command::new("git").arg("-C").arg(repo).args(arguments),
        "git",
    )
}

/// Run git in `repo` and keep what it printed, trimmed.
fn git_says(repo: &Path, arguments: &[&str]) -> anyhow::Result<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(arguments)
        .output()
        .context("starting git")?;
    if !output.status.success() {
        bail!(
            "git {} failed ({}): {}",
            arguments.join(" "),
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

/// Run a command whose output goes to the journal, and fail if it fails.
fn run_command(command: &mut Command, what: &str) -> anyhow::Result<()> {
    let status = command
        .status()
        .with_context(|| format!("starting {what}"))?;
    if !status.success() {
        bail!("{what} failed ({status})");
    }
    Ok(())
}
