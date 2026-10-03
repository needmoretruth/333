//! `GET /sitemap.xml`: every indexed page in every published language, each with its
//! alternates in the others.
//!
//! `lastmod` is the day the page's own files last changed in git (its template, its
//! catalog and the shared ones), which `deploy` writes into the release. Without that
//! file there is no `lastmod` at all: a date that is only the moment of a build tells a
//! crawler every page changed when none did.

use std::fmt::Write as _;

use super::state::State;
use crate::site::{ENGLISH, PAGES};

/// The sitemap.
pub(crate) fn xml(state: &State) -> String {
    let published = state.words.published();
    let mut out = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\" \
         xmlns:xhtml=\"http://www.w3.org/1999/xhtml\">\n",
    );
    for page in &PAGES {
        for language in published {
            let _ = write!(out, "<url><loc>{}</loc>", language.url(page));
            let changed = state
                .lastmod
                .get(language.tag)
                .and_then(|pages| pages.get(page.route));
            if let Some(changed) = changed {
                let _ = write!(out, "<lastmod>{changed}</lastmod>");
            }
            for other in published {
                let _ = write!(
                    out,
                    r#"<xhtml:link rel="alternate" hreflang="{}" href="{}"/>"#,
                    other.tag,
                    other.url(page)
                );
            }
            let _ = writeln!(
                out,
                r#"<xhtml:link rel="alternate" hreflang="x-default" href="{}"/></url>"#,
                ENGLISH.url(page)
            );
        }
    }
    out.push_str("</urlset>\n");
    out
}
