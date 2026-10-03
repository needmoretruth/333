//! What a new release must pass: before the switch, its words; after it, its answer.
//!
//! Both catch a release that builds and still breaks the site. A typo in one `.ftl` file
//! unpublishes that language, and every one of its pages turns into a 404 without an
//! error anywhere; a binary that starts and dies is restarted by systemd for ever while
//! Caddy answers 502. Nobody watches the site, so `deploy` has to notice for them.

use std::net::SocketAddr;
use std::path::Path;
use std::time::Duration;

use anyhow::bail;

use crate::words::Words;

/// How many times the new release is asked whether it serves.
const TRIES: u32 = 10;

/// How long to wait before each ask; with [`TRIES`], about thirty seconds in all.
const PAUSE: Duration = Duration::from_secs(3);

/// How long one ask may take.
const PATIENCE: Duration = Duration::from_secs(5);

/// Refuse a site that would stop publishing a language `before` publishes, or whose
/// English does not read whole. `before` is `None` on the first deploy.
///
/// # Errors
/// Names the first language lost and why.
pub(crate) fn languages_kept(before: Option<&Path>, after: &Path) -> anyhow::Result<()> {
    let next = Words::load(after);
    if !next.english_is_whole() {
        bail!(
            "English would not load: {}",
            next.why_not_published(&crate::site::ENGLISH)
        );
    }
    let Some(before) = before else {
        return Ok(());
    };
    let now = Words::load(before);
    for language in now.published() {
        if !next.is_published(language) {
            bail!(
                "{} would stop being published: {}",
                language.tag,
                next.why_not_published(language)
            );
        }
    }
    Ok(())
}

/// Ask the server at `listen` for its home page until it answers with one, for about
/// thirty seconds. A page is HTTP 200 with `<html` in it: a server that answers with an
/// error, or with nothing, is not serving the site.
///
/// # Errors
/// Says what the last ask got.
pub(crate) fn serving(listen: SocketAddr) -> anyhow::Result<()> {
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(PATIENCE))
        .build()
        .into();
    let url = format!("http://{listen}/");
    let mut last = String::new();
    for _ in 0..TRIES {
        std::thread::sleep(PAUSE);
        match ask(&agent, &url) {
            Ok(()) => return Ok(()),
            Err(got) => last = got,
        }
    }
    bail!("{url} did not serve the home page in {TRIES} tries; the last got {last}")
}

/// Ask once; what came back instead of a page, if it was not one.
fn ask(agent: &ureq::Agent, url: &str) -> Result<(), String> {
    let mut answer = agent.get(url).call().map_err(|error| error.to_string())?;
    if answer.status() != ureq::http::StatusCode::OK {
        return Err(format!("HTTP {}", answer.status()));
    }
    let page = answer
        .body_mut()
        .read_to_string()
        .map_err(|error| format!("an unreadable answer ({error})"))?;
    if page.contains("<html") {
        Ok(())
    } else {
        Err("an answer that is not a page".to_owned())
    }
}
