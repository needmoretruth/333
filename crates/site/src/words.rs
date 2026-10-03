//! The words on the pages, one catalog per language, kept apart from the templates.
//!
//! Each language is a folder under `<site>/words/` named by its tag (`en`, `ko`,
//! `zh-Hans`), holding Project Fluent files: one per page, `common.ftl` for what every
//! page shares, and `js.ftl` for what the pages' scripts say. They are the same format
//! the client's own catalogs are in, read by the same crate.
//!
//! A LANGUAGE IS PUBLISHED ONLY WHEN IT IS WHOLE. Every message English has must be in
//! it, and every file must read. Until then nothing links to it and its addresses are
//! missing, so a half-translated page is never what a visitor or a search engine finds.
//! A message a published language cannot say (an argument it lacks, say) is still said
//! in English, message by message, so a page never shows a key.
//!
//! The values are HTML written by us: the server puts them into pages as they are. What
//! is handed to them as an argument is escaped by whoever hands it over.

mod script;

use std::collections::BTreeSet;
use std::path::Path;

use fluent_bundle::{FluentArgs, FluentResource, FluentValue};
use fluent_syntax::ast;

use crate::site::{self, LANGUAGES, Language};

/// One language's messages, ready to be said.
type Bundle = fluent_bundle::concurrent::FluentBundle<FluentResource>;

/// The prefix of every message a script is handed.
pub(crate) const SCRIPT_PREFIX: &str = "js-";

/// One language's catalog.
pub(crate) struct Catalog {
    /// Which language.
    pub(crate) language: &'static Language,
    /// Its messages.
    bundle: Bundle,
    /// Every message it has, by name.
    pub(crate) keys: BTreeSet<String>,
    /// Whether every file in it could be read whole.
    readable: bool,
    /// What its scripts are handed, already JSON, made once.
    script: String,
}

/// Every language there is a catalog for, read once when the server starts.
pub(crate) struct Words {
    /// English first, then the others in [`LANGUAGES`] order, each only if its folder exists.
    catalogs: Vec<Catalog>,
    /// The languages a visitor can be shown, English first.
    published: Vec<&'static Language>,
}

/// Something handed to a message.
#[derive(Debug, Clone)]
pub(crate) enum Arg {
    /// Text, said as it is. HTML-escape it first where the message goes into HTML.
    Text(String),
    /// A count, which can choose a plural.
    Count(u64),
}

impl Words {
    /// Read every language's folder under `site`. A problem is logged and never fatal:
    /// the language it is in is simply not published.
    pub(crate) fn load(site: &Path) -> Self {
        let folder = site.join(site::WORDS);
        let mut catalogs = Vec::new();
        for language in &LANGUAGES {
            let dir = folder.join(language.tag);
            if dir.is_dir() {
                catalogs.push(read(language, &dir));
            }
        }
        let scripts: Vec<String> = {
            let english = catalogs
                .iter()
                .find(|catalog| catalog.language.is_english());
            catalogs
                .iter()
                .map(|catalog| script::json(catalog, english))
                .collect()
        };
        let english = catalogs
            .iter()
            .find(|catalog| catalog.language.is_english())
            .map(|catalog| catalog.keys.clone())
            .unwrap_or_default();
        if english.is_empty() {
            tracing::error!("there is no English catalog in {}", folder.display());
        }
        let mut published = vec![&site::ENGLISH];
        for (catalog, script) in catalogs.iter_mut().zip(scripts) {
            catalog.script = script;
            if catalog.language.is_english() {
                continue;
            }
            let missing = english.difference(&catalog.keys).count();
            if catalog.readable && missing == 0 {
                published.push(catalog.language);
            } else if missing > 0 {
                tracing::info!(
                    "{} is not published: {missing} of English's {} messages are missing",
                    catalog.language.tag,
                    english.len()
                );
            }
        }
        Self {
            catalogs,
            published,
        }
    }

    /// The languages a visitor can be shown, English first.
    pub(crate) fn published(&self) -> &[&'static Language] {
        &self.published
    }

    /// Can `language` be shown?
    pub(crate) fn is_published(&self, language: &Language) -> bool {
        self.published.contains(&language)
    }

    /// Whether English read whole. It is listed as published even when it did not, since
    /// every page falls back to it; `deploy` must not switch to a site where it is broken.
    pub(crate) fn english_is_whole(&self) -> bool {
        self.catalog(&site::ENGLISH)
            .is_some_and(|catalog| catalog.readable && !catalog.keys.is_empty())
    }

    /// Why `language` is not published, in a few words for a log line.
    pub(crate) fn why_not_published(&self, language: &Language) -> String {
        let Some(catalog) = self.catalog(language) else {
            return format!("there is no {} catalog", language.tag);
        };
        if !catalog.readable {
            return "one of its files cannot be read whole".to_owned();
        }
        let missing = self
            .catalog(&site::ENGLISH)
            .map(|english| english.keys.difference(&catalog.keys).count())
            .unwrap_or_default();
        format!("{missing} of English's messages are missing from it")
    }

    /// One language's catalog, if there is one.
    pub(crate) fn catalog(&self, language: &Language) -> Option<&Catalog> {
        self.catalogs
            .iter()
            .find(|catalog| catalog.language == language)
    }

    /// The repository's own catalogs, for tests.
    #[cfg(test)]
    pub(crate) fn repository() -> Self {
        Self::load(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../site"))
    }

    /// The words for a page in `language`, with English behind them.
    pub(crate) fn speaker(&self, language: &'static Language) -> Speaker<'_> {
        Speaker {
            language,
            spoken: self.catalog(language),
            english: self.catalog(&site::ENGLISH),
        }
    }
}

/// Read one language's folder.
fn read(language: &'static Language, dir: &Path) -> Catalog {
    let tag = language.tag.parse().unwrap_or_default();
    let mut bundle = Bundle::new_concurrent(vec![tag]);
    // Fluent wraps every argument in invisible direction marks by default. Inside HTML
    // they are characters in an attribute, a link's text and a copied command.
    bundle.set_use_isolating(false);
    let mut readable = bundle.add_builtins().is_ok();
    let mut keys = BTreeSet::new();
    for (name, text) in files(dir, &mut readable) {
        let resource = match FluentResource::try_new(text) {
            Ok(resource) => resource,
            Err((resource, errors)) => {
                let at = errors.first().map(|error| error.pos.start).unwrap_or(0);
                let line = resource
                    .source()
                    .get(..at)
                    .unwrap_or_default()
                    .lines()
                    .count();
                tracing::warn!("{name} cannot all be read; the first problem is near line {line}");
                readable = false;
                resource
            }
        };
        keys.extend(messages(&resource).map(|message| message.id.name.to_owned()));
        if let Err(errors) = bundle.add_resource(resource) {
            tracing::warn!("{name} says some messages twice ({} times)", errors.len());
            readable = false;
        }
    }
    Catalog {
        language,
        bundle,
        keys,
        readable,
        script: String::from("{}"),
    }
}

/// The `.ftl` files in a folder, in name order, with their text.
fn files(dir: &Path, readable: &mut bool) -> Vec<(String, String)> {
    let mut paths: Vec<_> = std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok().map(|entry| entry.path()))
                .filter(|path| path.extension().is_some_and(|extension| extension == "ftl"))
                .collect()
        })
        .unwrap_or_default();
    paths.sort();
    let mut found = Vec::new();
    for path in paths {
        let name = path.display().to_string();
        match std::fs::read_to_string(&path) {
            Ok(text) => found.push((name, text)),
            Err(error) => {
                tracing::warn!("{name} could not be read: {error}");
                *readable = false;
            }
        }
    }
    found
}

/// The messages in one file.
fn messages(resource: &FluentResource) -> impl Iterator<Item = &ast::Message<&str>> {
    resource.entries().filter_map(|entry| match entry {
        ast::Entry::Message(message) => Some(message),
        _ => None,
    })
}

/// The words for one page: one language, and English for what it cannot say.
#[derive(Clone, Copy)]
pub(crate) struct Speaker<'a> {
    /// The language the page is in.
    pub(crate) language: &'static Language,
    /// Its catalog, if it has one.
    spoken: Option<&'a Catalog>,
    /// English's.
    english: Option<&'a Catalog>,
}

impl Speaker<'_> {
    /// One message, with `$base` and these arguments, or `None` if nobody has it.
    pub(crate) fn say(&self, key: &str, args: &[(&str, Arg)]) -> Option<String> {
        let mut fluent = FluentArgs::new();
        fluent.set("base", FluentValue::from(self.language.base()));
        for (name, value) in args {
            match value {
                Arg::Text(text) => fluent.set(*name, FluentValue::from(text.clone())),
                Arg::Count(count) => fluent.set(*name, FluentValue::from(*count)),
            }
        }
        said(self.spoken, key, &fluent, true).or_else(|| said(self.english, key, &fluent, false))
    }

    /// [`Self::say`] with nothing handed over, or the key itself, which is a bug the
    /// tests catch and a page shows rather than hides.
    pub(crate) fn word(&self, key: &str) -> String {
        self.say(key, &[]).unwrap_or_else(|| key.to_owned())
    }

    /// Every script message in this language, as the JSON object the scripts read.
    pub(crate) fn script(&self) -> &str {
        self.spoken
            .or(self.english)
            .map_or("{}", |catalog| catalog.script.as_str())
    }
}

/// One message from one catalog, if it can say it. `strict` refuses one that could only
/// be said with errors, so English says it instead.
fn said(
    catalog: Option<&Catalog>,
    key: &str,
    args: &FluentArgs<'_>,
    strict: bool,
) -> Option<String> {
    let bundle = &catalog?.bundle;
    let pattern = bundle.get_message(key)?.value()?;
    let mut errors = Vec::new();
    let words = bundle.format_pattern(pattern, Some(args), &mut errors);
    if !errors.is_empty() {
        if strict {
            return None;
        }
        tracing::warn!("{key} was said with errors: {errors:?}");
    }
    Some(words.into_owned())
}

#[cfg(test)]
mod check;
