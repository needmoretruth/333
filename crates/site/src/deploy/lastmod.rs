//! When each page last changed, from git, written into the release for the sitemap.
//!
//! A page in one language is its template, its own catalog in that language and the
//! two shared ones. Its date is the newest commit that touched any of those files
//! (`git log -1`, in UTC), so a release that changes nothing on a page leaves its
//! date alone, and a crawler is not sent back to a page that is the same.
//!
//! Only published languages are dated. A failure is logged and leaves the page undated
//! rather than failing the deploy: the sitemap is right without a date and wrong with
//! the time of a build.

use std::path::Path;
use std::process::Command;

use crate::site::{self, Lastmod, PAGES, SHARED_CATALOGS};
use crate::words::Words;

/// Date every page in every published language and write the dates beside the pages in
/// `site` (the release's copy), as [`site::LASTMOD_FILE`].
pub(crate) fn write(repo: &Path, site_copy: &Path) {
    let words = Words::load(site_copy);
    let mut dates = Lastmod::new();
    for language in words.published() {
        let catalogs = Path::new("site").join(site::WORDS).join(language.tag);
        for page in &PAGES {
            let mut files = vec![
                Path::new("site").join(page.template),
                catalogs.join(page.catalog),
            ];
            files.extend(SHARED_CATALOGS.iter().map(|shared| catalogs.join(shared)));
            match last_change(repo, &files) {
                Some(date) => {
                    dates
                        .entry(language.tag.to_owned())
                        .or_default()
                        .insert(page.route.to_owned(), date);
                }
                None => {
                    tracing::warn!("{} in {} has no date from git", page.template, language.tag)
                }
            }
        }
    }
    let written = serde_json::to_vec(&dates)
        .map_err(anyhow::Error::from)
        .and_then(|bytes| {
            crate::atomic::write(&site_copy.join(site::LASTMOD_FILE), &bytes, 0o644)
                .map_err(anyhow::Error::from)
        });
    if let Err(error) = written {
        tracing::warn!("the pages' dates could not be written: {error:#}");
    }
}

/// The commit date of the newest commit that touched any of `files`, ISO 8601 in UTC:
/// `%cI` would keep the committer's own offset, and the sitemap says every date in UTC.
fn last_change(repo: &Path, files: &[std::path::PathBuf]) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .env("TZ", "UTC")
        .args([
            "log",
            "-1",
            "--format=%cd",
            "--date=format-local:%Y-%m-%dT%H:%M:%SZ",
            "--",
        ])
        .args(files)
        .output()
        .ok()?;
    let date = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    (output.status.success() && !date.is_empty()).then_some(date)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_page_is_dated_by_the_newest_commit_to_its_files() {
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let files = [
            Path::new("site").join("about.html"),
            Path::new("site/words/en/about.ftl").to_path_buf(),
        ];
        // In a checkout with history the date is a date; in an export without git
        // there is none, and that is what the sitemap then says.
        if let Some(date) = last_change(&repo, &files) {
            assert!(humantime::parse_rfc3339_weak(&date[..19]).is_ok(), "{date}");
        }
        assert_eq!(
            last_change(&repo, &[Path::new("no/such/file").to_path_buf()]),
            None
        );
    }

    #[test]
    fn the_dates_are_written_beside_the_pages_for_published_languages_only() {
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let copy = std::env::temp_dir().join(format!("n333-site-lastmod-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&copy);
        for tag in ["en", "ko"] {
            std::fs::create_dir_all(copy.join("words").join(tag)).unwrap();
        }
        std::fs::copy(
            repo.join("site/words/en/common.ftl"),
            copy.join("words/en/common.ftl"),
        )
        .unwrap();
        std::fs::write(copy.join("words/ko/common.ftl"), "skip = x\n").unwrap();
        write(&repo, &copy);
        let dates = site::read_lastmod(&copy);
        assert!(dates.keys().all(|tag| tag == "en"), "{dates:?}");
        if git_works(&repo) {
            assert_eq!(dates["en"].len(), PAGES.len(), "{dates:?}");
        }
        std::fs::remove_dir_all(&copy).unwrap();
    }

    fn git_works(repo: &Path) -> bool {
        last_change(repo, &[Path::new("site/index.html").to_path_buf()]).is_some()
    }
}
