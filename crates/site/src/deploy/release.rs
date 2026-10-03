//! The releases directory: one directory per commit, and `current` pointing at one.

use std::path::{Path, PathBuf};

use anyhow::Context as _;

/// The symlink to the running release.
const CURRENT: &str = "current";

/// The whole commit, inside each release.
const COMMIT: &str = "COMMIT";

/// The short commit, inside each release, read by `serve --version-file`.
const VERSION: &str = "VERSION";

/// The commit that last failed to deploy, and is not tried again.
const FAILED: &str = ".failed";

/// The commit whose build last failed, and how many times in a row: `<commit> <count>`.
const ATTEMPTS: &str = ".attempts";

/// The binary inside each release.
const BINARY: &str = "333-site";

/// The commit `current` runs, if there is one.
pub(crate) fn deployed(releases: &Path) -> Option<String> {
    read_trimmed(&releases.join(CURRENT).join(COMMIT))
}

/// The commit that last failed to deploy, if one did.
pub(crate) fn failed(releases: &Path) -> Option<String> {
    read_trimmed(&releases.join(FAILED))
}

/// Remember that `commit` failed to deploy, so later runs do not try it again.
///
/// # Errors
/// Fails if the file cannot be written.
pub(crate) fn mark_failed(releases: &Path, commit: &str) -> anyhow::Result<()> {
    crate::atomic::write(
        &releases.join(FAILED),
        format!("{commit}\n").as_bytes(),
        0o644,
    )
    .context("noting the failed commit")
}

/// Count one more failed build of `commit`, and say how many there have been in a row.
///
/// A build can fail for reasons the commit does not have (the network was down while
/// cargo fetched a crate), so one failure is not enough to give up on a commit.
///
/// # Errors
/// Fails if the file cannot be written.
pub(crate) fn count_failure(releases: &Path, commit: &str) -> anyhow::Result<u32> {
    let before = read_trimmed(&releases.join(ATTEMPTS))
        .and_then(|text| {
            let (counted, count) = text.split_once(' ')?;
            if counted == commit {
                count.parse::<u32>().ok()
            } else {
                None
            }
        })
        .unwrap_or(0);
    let now = before.saturating_add(1);
    crate::atomic::write(
        &releases.join(ATTEMPTS),
        format!("{commit} {now}\n").as_bytes(),
        0o644,
    )
    .context("counting the failed build")?;
    Ok(now)
}

/// A file's contents without the line break, if it can be read.
fn read_trimmed(path: &Path) -> Option<String> {
    let text = std::fs::read_to_string(path).ok()?;
    Some(text.trim().to_owned())
}

/// Put the built binary and the site into `<releases>/<commit>`.
///
/// Assembled under a hidden name and renamed into place, so a release directory either
/// holds everything or does not exist.
///
/// # Errors
/// Fails if anything cannot be copied or written.
pub(crate) fn stage(repo: &Path, releases: &Path, commit: &str, short: &str) -> anyhow::Result<()> {
    let staging = releases.join(format!(".staging-{commit}"));
    let release = releases.join(commit);
    remove_if_there(&staging)?;
    std::fs::create_dir_all(&staging).with_context(|| format!("making {}", staging.display()))?;
    let built = repo.join("target").join("release").join(BINARY);
    std::fs::copy(&built, staging.join(BINARY))
        .with_context(|| format!("copying {}", built.display()))?;
    copy_tree(&repo.join("site"), &staging.join("site"))?;
    super::lastmod::write(repo, &staging.join("site"));
    std::fs::write(staging.join(VERSION), format!("{short}\n")).context("writing VERSION")?;
    // Last, so its time is when the release was made; pruning goes by it.
    std::fs::write(staging.join(COMMIT), format!("{commit}\n")).context("writing COMMIT")?;
    remove_if_there(&release)?;
    std::fs::rename(&staging, &release)
        .with_context(|| format!("moving the release to {}", release.display()))
}

/// Point `current` at `<releases>/<commit>` in one rename.
///
/// # Errors
/// Fails if the symlink cannot be made or renamed into place.
#[cfg(unix)]
pub(crate) fn switch(releases: &Path, commit: &str) -> anyhow::Result<()> {
    let next = releases.join(".current-next");
    remove_if_there(&next)?;
    // Relative, so the releases directory can be moved as a whole.
    std::os::unix::fs::symlink(commit, &next).context("making the new symlink")?;
    std::fs::rename(&next, releases.join(CURRENT)).context("switching current")?;
    std::fs::File::open(releases)
        .and_then(|dir| dir.sync_all())
        .context("syncing the releases directory")
}

/// Symlinks that can be renamed over each other are a Unix thing; this runs on Linux.
#[cfg(not(unix))]
pub(crate) fn switch(_releases: &Path, _commit: &str) -> anyhow::Result<()> {
    anyhow::bail!("deploying needs symbolic links, which this needs a Unix system for")
}

/// Delete all but the newest `keep` releases, never the one `current` runs.
///
/// Only directories named like a commit (40 or 64 hex digits) are counted or touched.
///
/// # Errors
/// Fails if the directory cannot be listed or a release cannot be deleted.
pub(crate) fn prune(releases: &Path, keep: usize) -> anyhow::Result<Vec<PathBuf>> {
    let running = deployed(releases);
    let mut made = Vec::new();
    for entry in std::fs::read_dir(releases).context("listing the releases")? {
        let entry = entry.context("listing the releases")?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let is_commit =
            matches!(name.len(), 40 | 64) && name.bytes().all(|byte| byte.is_ascii_hexdigit());
        if !is_commit || !entry.file_type().is_ok_and(|kind| kind.is_dir()) {
            continue;
        }
        let when = std::fs::metadata(entry.path().join(COMMIT))
            .and_then(|meta| meta.modified())
            .ok();
        made.push((when, name, entry.path()));
    }
    // Newest first; a release with no COMMIT file is the oldest of all.
    made.sort_by(|one, two| two.cmp(one));
    let mut gone = Vec::new();
    for (_, name, path) in made.into_iter().skip(keep) {
        if running.as_deref() == Some(name.as_str()) {
            continue;
        }
        std::fs::remove_dir_all(&path).with_context(|| format!("deleting {}", path.display()))?;
        gone.push(path);
    }
    Ok(gone)
}

/// Copy a directory and everything in it. Symbolic links are not followed or copied:
/// a link in the site could point anywhere on the build machine.
fn copy_tree(from: &Path, to: &Path) -> anyhow::Result<()> {
    std::fs::create_dir_all(to).with_context(|| format!("making {}", to.display()))?;
    for entry in std::fs::read_dir(from).with_context(|| format!("listing {}", from.display()))? {
        let entry = entry.with_context(|| format!("listing {}", from.display()))?;
        let kind = entry.file_type().context("reading a file's type")?;
        let target = to.join(entry.file_name());
        if kind.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else if kind.is_file() {
            std::fs::copy(entry.path(), &target)
                .with_context(|| format!("copying {}", entry.path().display()))?;
        }
    }
    Ok(())
}

/// Remove a file, symlink or directory if it is there.
fn remove_if_there(path: &Path) -> anyhow::Result<()> {
    let Ok(meta) = std::fs::symlink_metadata(path) else {
        return Ok(());
    };
    let removed = if meta.is_dir() {
        std::fs::remove_dir_all(path)
    } else {
        std::fs::remove_file(path)
    };
    removed.with_context(|| format!("removing {}", path.display()))
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn pruning_keeps_the_newest_and_never_the_running_one() {
        let releases = std::env::temp_dir().join(format!("n333-site-prune-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&releases);
        let commits: Vec<String> = (1..=5)
            .map(|digit: u8| digit.to_string().repeat(40))
            .collect();
        for commit in &commits {
            std::fs::create_dir_all(releases.join(commit)).unwrap();
            std::fs::write(releases.join(commit).join(COMMIT), commit).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        switch(&releases, &commits[0]).unwrap();

        let gone = prune(&releases, 3).unwrap();
        assert_eq!(gone, vec![releases.join(&commits[1])]);
        assert!(releases.join(&commits[0]).is_dir(), "the running one stays");
        std::fs::remove_dir_all(&releases).unwrap();
    }
}
