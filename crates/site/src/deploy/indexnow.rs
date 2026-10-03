//! Telling search engines which pages changed, once a release is running (IndexNow).
//!
//! The pages whose date ([`crate::deploy::lastmod`]) differs from the release that ran
//! before, or that it did not have, are sent in one request to the shared IndexNow
//! endpoint, which passes them to every engine taking part. The key that proves the
//! site sent them is public by design: it is the file `site/<key>.txt`, served at the
//! site's root, whose text is its own name.
//!
//! NOTHING HERE CAN FAIL A DEPLOY. The release is already running when this is called;
//! a refused or unanswered request is one line in the journal, and crawlers still find
//! the change through the sitemap.

use std::path::Path;
use std::time::Duration;

use crate::site::{self, LANGUAGES, Lastmod, ORIGIN, PAGES};

/// The shared endpoint.
const ENDPOINT: &str = "https://api.indexnow.org/indexnow";

/// How long the request may take, all of it.
const PATIENCE: Duration = Duration::from_secs(20);

/// Send what changed between the site that ran (`before`, if there was one) and the
/// site that runs now. Logs one line either way.
pub(crate) fn notify(before: Option<&Path>, now: &Path) {
    let old = before.map(site::read_lastmod).unwrap_or_default();
    let new = site::read_lastmod(now);
    let urls = changed(&old, &new);
    if urls.is_empty() {
        tracing::info!("no page changed; nothing to tell search engines");
        return;
    }
    let Some(key) = key(now) else {
        tracing::warn!(
            "{} pages changed, and there is no IndexNow key file to send them with",
            urls.len()
        );
        return;
    };
    let host = ORIGIN.trim_start_matches("https://");
    let body = serde_json::json!({
        "host": host,
        "key": key,
        "keyLocation": format!("{ORIGIN}/{key}.txt"),
        "urlList": urls,
    });
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(PATIENCE))
        .build()
        .into();
    let sent = agent
        .post(ENDPOINT)
        .header("Content-Type", "application/json; charset=utf-8")
        .send(body.to_string());
    match sent {
        Ok(answer) => tracing::info!(
            "told IndexNow about {} changed pages ({})",
            urls.len(),
            answer.status()
        ),
        Err(error) => tracing::warn!(
            "IndexNow was not told about {} changed pages: {error}",
            urls.len()
        ),
    }
}

/// The addresses of the pages whose date is new or different.
fn changed(old: &Lastmod, new: &Lastmod) -> Vec<String> {
    let mut urls = Vec::new();
    for language in &LANGUAGES {
        let Some(dates) = new.get(language.tag) else {
            continue;
        };
        for page in &PAGES {
            let Some(date) = dates.get(page.route) else {
                continue;
            };
            let before = old
                .get(language.tag)
                .and_then(|dates| dates.get(page.route));
            if before != Some(date) {
                urls.push(language.url(page));
            }
        }
    }
    urls
}

/// The key: the one file at the site's root named 32 hexadecimal digits and `.txt`
/// whose text is that name.
fn key(site: &Path) -> Option<String> {
    std::fs::read_dir(site).ok()?.find_map(|entry| {
        let entry = entry.ok()?;
        let name = entry.file_name().into_string().ok()?;
        let stem = name.strip_suffix(".txt")?;
        let hex = stem.len() == 32 && stem.bytes().all(|byte| byte.is_ascii_hexdigit());
        let text = std::fs::read_to_string(entry.path()).ok()?;
        (hex && text.trim() == stem).then(|| stem.to_owned())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_pages_with_a_new_date_are_sent() {
        let date = |route: &str, when: &str| (route.to_owned(), when.to_owned());
        let old: Lastmod = [("en".to_owned(), [date("", "1"), date("law", "1")].into())].into();
        let new: Lastmod = [
            (
                "en".to_owned(),
                [date("", "1"), date("law", "2"), date("about", "1")].into(),
            ),
            ("ko".to_owned(), [date("", "1")].into()),
        ]
        .into();
        assert_eq!(
            changed(&old, &new),
            [
                "https://the333.dev/law",
                "https://the333.dev/about",
                "https://the333.dev/ko/"
            ]
        );
    }

    #[test]
    fn the_repository_has_its_key() {
        let site = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../site");
        assert_eq!(key(&site).map(|key| key.len()), Some(32));
    }
}
