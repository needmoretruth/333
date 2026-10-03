//! The pages the site has, the languages it can speak, and the addresses they make.
//!
//! English is at the addresses the site always had (`/`, `/about`, `/333`); every other
//! language is the same page under its own first segment (`/ko/about`, `/zh-hans/333`).
//! Which of the languages below is actually shown is not decided here: a language is
//! published only once its catalog has every message English has (see [`crate::words`]),
//! and until then its addresses answer as missing and nothing links to them.

use std::collections::BTreeMap;
use std::path::Path;

/// Where the site is, for the addresses that have to be whole: canonical links, the
/// sitemap, the alternates search engines are told about.
pub(crate) const ORIGIN: &str = "https://the333.dev";

/// The folder under the site directory that holds one folder of catalogs per language.
pub(crate) const WORDS: &str = "words";

/// The catalogs every page is made of besides its own: the header and footer, and the
/// words its scripts are handed.
pub(crate) const SHARED_CATALOGS: [&str; 2] = ["common.ftl", "js.ftl"];

/// When each page last changed, as `deploy` works it out from git and writes it into
/// the release. Hidden, so it is never served as a file.
pub(crate) const LASTMOD_FILE: &str = ".lastmod.json";

/// One language the site can be read in.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Language {
    /// The BCP 47 tag: the catalog's folder, `lang`, `hreflang`, `Content-Language`.
    pub(crate) tag: &'static str,
    /// The first segment of its addresses, in lower case; empty for English.
    pub(crate) segment: &'static str,
    /// What Open Graph calls it.
    pub(crate) og_locale: &'static str,
}

/// The language every other one falls back to, at the addresses without a segment.
pub(crate) const ENGLISH: Language = Language {
    tag: "en",
    segment: "",
    og_locale: "en_US",
};

/// Every language the site has a place for, English first.
pub(crate) const LANGUAGES: [Language; 9] = [
    ENGLISH,
    Language {
        tag: "ko",
        segment: "ko",
        og_locale: "ko_KR",
    },
    Language {
        tag: "es",
        segment: "es",
        og_locale: "es_ES",
    },
    Language {
        tag: "fr",
        segment: "fr",
        og_locale: "fr_FR",
    },
    Language {
        tag: "de",
        segment: "de",
        og_locale: "de_DE",
    },
    Language {
        tag: "ja",
        segment: "ja",
        og_locale: "ja_JP",
    },
    Language {
        tag: "zh-Hans",
        segment: "zh-hans",
        og_locale: "zh_CN",
    },
    Language {
        tag: "zh-Hant",
        segment: "zh-hant",
        og_locale: "zh_TW",
    },
    Language {
        tag: "eo",
        segment: "eo",
        og_locale: "eo",
    },
];

impl Language {
    /// Is this the language at the addresses without a segment?
    pub(crate) fn is_english(&self) -> bool {
        self.segment.is_empty()
    }

    /// What goes in front of a page's path: empty for English, `/ko` for Korean.
    pub(crate) fn base(&self) -> String {
        if self.is_english() {
            String::new()
        } else {
            format!("/{}", self.segment)
        }
    }

    /// The path of `page` in this language: `/about`, `/ko/about`, `/` or `/ko/`.
    pub(crate) fn path(&self, page: &Page) -> String {
        format!("{}/{}", self.base(), page.route)
    }

    /// The whole address of `page` in this language.
    pub(crate) fn url(&self, page: &Page) -> String {
        format!("{ORIGIN}{}", self.path(page))
    }
}

/// The language whose addresses begin with `segment`, other than English.
pub(crate) fn language_at(segment: &str) -> Option<&'static Language> {
    LANGUAGES
        .iter()
        .find(|language| !language.is_english() && language.segment == segment)
}

/// One page: where it is, and what it is made of.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Page {
    /// Its path after the language's segment, without the leading slash; empty for home.
    pub(crate) route: &'static str,
    /// Its template under the site directory.
    pub(crate) template: &'static str,
    /// Its own catalog in each language's folder.
    pub(crate) catalog: &'static str,
}

/// Every page that is indexed, in the order the sitemap lists them.
pub(crate) const PAGES: [Page; 8] = [
    Page {
        route: "",
        template: "index.html",
        catalog: "index.ftl",
    },
    Page {
        route: "network",
        template: "network.html",
        catalog: "network.ftl",
    },
    Page {
        route: "map",
        template: "map.html",
        catalog: "map.ftl",
    },
    Page {
        route: "status",
        template: "status.html",
        catalog: "status.ftl",
    },
    Page {
        route: "333",
        template: "board.html",
        catalog: "board.ftl",
    },
    Page {
        route: "law",
        template: "law.html",
        catalog: "law.ftl",
    },
    Page {
        route: "about",
        template: "about.html",
        catalog: "about.ftl",
    },
    Page {
        route: "start",
        template: "start.html",
        catalog: "start.ftl",
    },
];

/// The page shown for an address that names nothing. Never indexed.
pub(crate) const NOT_FOUND: Page = Page {
    route: "",
    template: "404.html",
    catalog: "404.ftl",
};

/// The indexed page a template file is, if it is one.
pub(crate) fn page_of(template: &str) -> Option<&'static Page> {
    PAGES.iter().find(|page| page.template == template)
}

/// When each page last changed: language tag, then route, then an ISO 8601 date.
pub(crate) type Lastmod = BTreeMap<String, BTreeMap<String, String>>;

/// The dates `deploy` wrote beside the pages, or none if there is no file.
///
/// A file that cannot be read means no dates, never a guess: a sitemap without
/// `lastmod` is correct, and one with the wrong date is not.
pub(crate) fn read_lastmod(site: &Path) -> Lastmod {
    std::fs::read(site.join(LASTMOD_FILE))
        .ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_keeps_the_old_addresses_and_the_others_get_a_segment() {
        let about = page_of("about.html").unwrap();
        let home = page_of("index.html").unwrap();
        let korean = language_at("ko").unwrap();
        assert_eq!(ENGLISH.url(about), "https://the333.dev/about");
        assert_eq!(ENGLISH.path(home), "/");
        assert_eq!(korean.path(home), "/ko/");
        assert_eq!(
            korean.url(page_of("board.html").unwrap()),
            "https://the333.dev/ko/333"
        );
        assert_eq!(language_at("zh-hans").unwrap().tag, "zh-Hans");
        assert_eq!(language_at(""), None);
        assert_eq!(language_at("en"), None);
    }
}
