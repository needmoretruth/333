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
//!    Beside the pages goes `.lastmod.json`: the date each page's files last changed in
//!    git, which the sitemap gives as `lastmod`.
//! 4. Refuse it if a language the running release publishes would stop being published,
//!    or English would not load (see [`guard`]).
//! 5. Point `<releases>/current` at it by renaming a new symlink over the old one, so
//!    there is no moment with no `current`; then `systemctl --user restart <unit>`, and
//!    ask the server for its home page for about thirty seconds. If the restart fails or
//!    no page comes, `current` goes back to the release that ran before and that one is
//!    restarted.
//! 6. Tell search engines (IndexNow) which pages' dates changed since the release that
//!    ran before. That can only be logged as failing, never fail the deploy.
//! 7. Delete all but the newest three releases. The one `current` names is never deleted.
//!
//! IF ANYTHING FAILS, what ran before keeps running and this exits non-zero with the
//! reason. A commit refused in step 4 or 5 is written to `<releases>/.failed` at once, and
//! one whose build failed [`BUILD_ATTEMPTS`] runs in a row (counted in
//! `<releases>/.attempts`): a build can fail for a reason that is not the commit's, such as
//! the network while cargo fetches. Later runs refuse to build a commit in `.failed` again,
//! since a broken commit would otherwise cost the machine a full build every five minutes.
//! A newer commit is built as usual; deleting the file retries this one. Waiting too long
//! for the build lock is not a failed build and is not counted.

mod guard;
mod indexnow;
mod lastmod;
mod release;

use std::fs::{File, OpenOptions, TryLockError};
use std::net::SocketAddr;
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

/// How many failed builds in a row it takes to give up on a commit.
const BUILD_ATTEMPTS: u32 = 3;

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
    /// Where the unit's server listens, asked for the home page after a restart.
    #[arg(long, default_value = crate::serve::LISTEN)]
    listen: SocketAddr,
}

/// Deploy `origin/main` if it is not what runs.
///
/// # Errors
/// Fails if git, the build, the copy, the checks, the switch or the restart fails,
/// saying which.
pub(crate) fn run(args: &Args) -> anyhow::Result<()> {
    std::fs::create_dir_all(&args.releases)
        .with_context(|| format!("making {}", args.releases.display()))?;
    git(&args.repo, &["fetch", "--quiet", "origin", "main"])?;
    let target = git_says(
        &args.repo,
        &["rev-parse", "--verify", "origin/main^{commit}"],
    )?;
    let running = release::deployed(&args.releases);
    if running.as_deref() == Some(target.as_str()) {
        date_if_undated(args, &target);
        tracing::info!("{target} is already running");
        return Ok(());
    }
    if release::failed(&args.releases).as_deref() == Some(target.as_str()) {
        bail!("{target} failed to deploy before; waiting for a newer commit");
    }
    git(&args.repo, &["reset", "--quiet", "--hard", &target])?;
    let short = git_says(&args.repo, &["rev-parse", "--short", &target])?;
    let kept = || format!("{short} was not deployed; the running release stays");
    build(args, &target).with_context(kept)?;
    release::stage(&args.repo, &args.releases, &target, &short)?;
    let before = running
        .as_ref()
        .map(|commit| args.releases.join(commit).join("site"));
    let staged = args.releases.join(&target).join("site");
    if let Err(lost) = guard::languages_kept(before.as_deref(), &staged) {
        release::mark_failed(&args.releases, &target)?;
        return Err(lost.context(kept()));
    }
    release::switch(&args.releases, &target)?;
    if let Err(broken) = restart(&args.unit).and_then(|()| guard::serving(args.listen)) {
        return Err(roll_back(args, running.as_deref(), &target, &short, broken));
    }
    tracing::info!("{short} is running");
    indexnow::notify(before.as_deref(), &staged);
    for gone in release::prune(&args.releases, KEEP)? {
        tracing::info!("deleted {}", gone.display());
    }
    Ok(())
}

/// Date the running release's pages if it has no dates, then restart it to serve them
/// and tell search engines.
///
/// The binary that deploys is the one already running, so a release that brings a new
/// deploy step is staged by the release before it, without that step. The release that
/// first dated pages was deployed that way, and the next run fills the gap.
fn date_if_undated(args: &Args, target: &str) {
    let site_copy = args.releases.join(target).join("site");
    if site_copy.join(crate::site::LASTMOD_FILE).exists() {
        return;
    }
    lastmod::write(&args.repo, &site_copy);
    if !site_copy.join(crate::site::LASTMOD_FILE).exists() {
        return;
    }
    match restart(&args.unit).and_then(|()| guard::serving(args.listen)) {
        Ok(()) => {
            tracing::info!("dated the running release's pages");
            indexnow::notify(None, &site_copy);
        }
        Err(error) => tracing::error!("dated the running release's pages: {error:#}"),
    }
}

/// Build the site binary, holding the heavy-work lock, and give up on `commit` once it
/// has failed [`BUILD_ATTEMPTS`] times in a row.
fn build(args: &Args, commit: &str) -> anyhow::Result<()> {
    // Not counted when it fails: waiting too long for the lock says nothing of the commit.
    let _held = heavy_lock()?;
    let built = run_command(
        Command::new("cargo")
            .args(["build", "--release", "--locked", "-p", "n333-site"])
            .current_dir(&args.repo),
        "cargo build",
    );
    if built.is_err() {
        let count = release::count_failure(&args.releases, commit)?;
        if count >= BUILD_ATTEMPTS {
            release::mark_failed(&args.releases, commit)?;
        } else {
            tracing::warn!("build {count} of {BUILD_ATTEMPTS} of {commit} failed; trying again");
        }
    }
    built
}

/// Restart the unit that serves `current`.
fn restart(unit: &str) -> anyhow::Result<()> {
    run_command(
        Command::new("systemctl").args(["--user", "restart", unit]),
        "systemctl --user restart",
    )
}

/// Point `current` back at `before` and restart it, after `target` failed to serve, and
/// never try `target` again. Returns the one error that says all of it.
///
/// The first deploy has nothing to go back to (and its restart fails until the unit is
/// installed), so `current` stays on `target` and it is not marked.
fn roll_back(
    args: &Args,
    before: Option<&str>,
    target: &str,
    short: &str,
    broken: anyhow::Error,
) -> anyhow::Error {
    let Some(before) = before else {
        return broken.context(format!("{short} is current and does not serve"));
    };
    let back = release::switch(&args.releases, before).and_then(|()| restart(&args.unit));
    if let Err(error) = release::mark_failed(&args.releases, target) {
        tracing::error!("{error:#}");
    }
    match back {
        Ok(()) => broken.context(format!(
            "{short} did not serve, so {before} runs again and {short} is not tried again"
        )),
        Err(error) => error.context(format!(
            "{short} did not serve ({broken:#}), and going back to {before} failed too"
        )),
    }
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
