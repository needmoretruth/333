//! What a page says about itself to search engines and link previews, and the list of
//! the other languages it can be read in.
//!
//! Only published languages are named anywhere: in `hreflang` alternates, in
//! `og:locale:alternate`, and in the list at the foot of the page. While English is the
//! only one, the list is not drawn at all and each page names only itself.

use std::fmt::Write as _;

use super::template::html_escaped;
use crate::site::{ENGLISH, Language, PAGES, Page};
use crate::words::{Speaker, Words};

/// The links and Open Graph lines of one page's head, for `{{html:head}}`.
pub(crate) fn links(words: &Words, language: &Language, page: &Page) -> String {
    let mut out = String::new();
    let own = language.url(page);
    let _ = writeln!(out, r#"<link rel="canonical" href="{own}">"#);
    for other in words.published() {
        let _ = writeln!(
            out,
            r#"<link rel="alternate" hreflang="{}" href="{}">"#,
            other.tag,
            other.url(page)
        );
    }
    let _ = writeln!(
        out,
        r#"<link rel="alternate" hreflang="x-default" href="{}">"#,
        ENGLISH.url(page)
    );
    let _ = writeln!(out, r#"<meta property="og:url" content="{own}">"#);
    let _ = writeln!(out, r#"<meta property="og:site_name" content="333">"#);
    let _ = writeln!(
        out,
        r#"<meta property="og:locale" content="{}">"#,
        language.og_locale
    );
    for other in words
        .published()
        .iter()
        .filter(|other| other.tag != language.tag)
    {
        let _ = writeln!(
            out,
            r#"<meta property="og:locale:alternate" content="{}">"#,
            other.og_locale
        );
    }
    out.trim_end().to_owned()
}

/// The list of languages at the foot of a page, each named in itself and linking to the
/// same page in it; home for a page that is not one of [`PAGES`]. Nothing while there
/// is only one language to read.
pub(crate) fn languages(words: &Words, here: &Speaker<'_>, page: Option<&Page>) -> String {
    let published = words.published();
    if published.len() < 2 {
        return String::new();
    }
    let home = PAGES.first();
    let mut out = format!(
        r#"<nav class="langs" aria-label="{}">"#,
        html_escaped(&here.word("languages"))
    );
    for language in published {
        let Some(target) = page.or(home) else {
            continue;
        };
        let name = words.speaker(language).word("language-name");
        let current = if *language == here.language {
            r#" aria-current="page""#
        } else {
            ""
        };
        let _ = write!(
            out,
            r#"<a href="{}" hreflang="{tag}" lang="{tag}"{current}>{}</a>"#,
            language.path(target),
            html_escaped(&name),
            tag = language.tag,
        );
    }
    out.push_str("</nav>");
    out
}
