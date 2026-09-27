//! Where the words are kept: inside the binary, and beside the node for trying.
//!
//! Inside, one folder per language under `crates/cli/words/`, put into the binary
//! whole at compile time. Adding a folder there adds a language; no Rust names the
//! languages. Beside the node, `<data-dir>/words/<tag>/` is read at every start if it
//! is there, so somebody translating can see their words in the client without
//! building it. What is read from the node's folder wins over what was built in,
//! message by message.
//!
//! A catalog that cannot all be read is still used for what it could say. What it
//! could not say falls back to English, and the problem is said once, naming the file.
//! Nothing here stops a node.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use fluent_bundle::FluentResource;
use fluent_syntax::ast::Entry;

/// Every catalog this client was built with.
static BUILT_IN: include_dir::Dir<'static> = include_dir::include_dir!("$CARGO_MANIFEST_DIR/words");

/// The language every other one falls back to, message by message.
pub(crate) const ENGLISH: &str = "en";

/// One language's messages, ready to be said.
pub(crate) type Bundle = fluent_bundle::concurrent::FluentBundle<FluentResource>;

/// One catalog file: what it is called, so a problem can name it, and what it says.
#[derive(Debug)]
pub(crate) struct Source {
    /// `ko/id.ftl` for a built-in one; the whole path for one read from disk.
    pub(crate) name: String,
    /// The file.
    pub(crate) text: String,
}

/// Something that went wrong reading a catalog. Said, and never fatal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Problem {
    /// The file was read and part of it is not Fluent.
    Broken {
        /// Which file.
        file: String,
        /// The first line that could not be read, counted from one.
        line: usize,
    },
    /// The file could not be read at all.
    Unreadable {
        /// Which file.
        file: String,
        /// What the system said.
        why: String,
    },
}

/// The folder beside a node where catalogs are read at run time.
#[must_use]
pub(crate) fn beside(root: &Path) -> PathBuf {
    root.join("words")
}

/// The built-in catalog files for one language, in a fixed order.
#[must_use]
pub(crate) fn built_in(tag: &str) -> Vec<Source> {
    let mut found: Vec<Source> = BUILT_IN
        .dirs()
        .filter(|dir| named(dir.path()).eq_ignore_ascii_case(tag))
        .flat_map(include_dir::Dir::files)
        .filter(|file| is_catalog(file.path()))
        .map(|file| Source {
            name: file.path().display().to_string(),
            text: file.contents_utf8().unwrap_or_default().to_owned(),
        })
        .collect();
    found.sort_by(|a, b| a.name.cmp(&b.name));
    found
}

/// The catalog files for one language in the folder beside a node, if there are any.
fn on_disk(beside: &Path, tag: &str, problems: &mut Vec<Problem>) -> Vec<Source> {
    let Some(folder) = folders(beside)
        .into_iter()
        .find(|folder| named(folder).eq_ignore_ascii_case(tag))
    else {
        return Vec::new();
    };
    let Ok(entries) = std::fs::read_dir(&folder) else {
        return Vec::new();
    };
    let mut paths: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| is_catalog(path))
        .collect();
    paths.sort();
    let mut found = Vec::new();
    for path in paths {
        let name = path.display().to_string();
        match std::fs::read_to_string(&path) {
            Ok(text) => found.push(Source { name, text }),
            Err(e) => problems.push(Problem::Unreadable {
                file: name,
                why: e.to_string(),
            }),
        }
    }
    found
}

/// Everything one language has to say: beside the node first, so that it wins, then
/// what was built in.
#[must_use]
pub(crate) fn sources(
    tag: &str,
    beside: Option<&Path>,
    problems: &mut Vec<Problem>,
) -> Vec<Source> {
    let mut found = beside.map_or_else(Vec::new, |beside| on_disk(beside, tag, problems));
    found.extend(built_in(tag));
    found
}

/// Every language there are words for, built in or beside the node.
#[must_use]
pub(crate) fn tags(beside: Option<&Path>) -> Vec<String> {
    let mut tags: BTreeSet<String> = BUILT_IN
        .dirs()
        .map(|dir| named(dir.path()).to_owned())
        .collect();
    for folder in beside.map(folders).unwrap_or_default() {
        if !tags
            .iter()
            .any(|tag| tag.eq_ignore_ascii_case(named(&folder)))
        {
            tags.insert(named(&folder).to_owned());
        }
    }
    tags.into_iter().collect()
}

/// Read one language's files into something that can be said.
#[must_use]
pub(crate) fn bundle(tag: &str, sources: Vec<Source>, problems: &mut Vec<Problem>) -> Bundle {
    let language = tag.parse().unwrap_or_default();
    let mut bundle = Bundle::new_concurrent(vec![language]);
    // Fluent wraps every argument in invisible direction marks by default, for text
    // that mixes left-to-right and right-to-left. A terminal prints them as nothing
    // that still takes up a place, and every column after them is off by two.
    bundle.set_use_isolating(false);
    bundle.set_formatter(Some(super::count::fluent));
    for source in sources {
        let resource = parsed(source, problems);
        // The only thing adding can refuse is a message already there, and one
        // already there came from beside the node on purpose.
        let _ = bundle.add_resource(resource);
    }
    bundle
}

/// Parse one file, keeping what could be read and saying where it stopped making sense.
pub(crate) fn parsed(source: Source, problems: &mut Vec<Problem>) -> FluentResource {
    let Source { name, text } = source;
    FluentResource::try_new(text).unwrap_or_else(|(resource, errors)| {
        let at = errors.first().map_or(0, |error| error.pos.start);
        let before = resource.source().get(..at).unwrap_or_default();
        let line = before.matches('\n').count() + 1;
        problems.push(Problem::Broken { file: name, line });
        resource
    })
}

/// The names of the messages in some catalog files.
#[must_use]
pub(crate) fn keys(sources: &[Source]) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    for source in sources {
        let resource = fluent_syntax::parser::parse_runtime(source.text.as_str())
            .unwrap_or_else(|(resource, _)| resource);
        for entry in resource.body {
            if let Entry::Message(message) = entry {
                keys.insert(message.id.name.to_owned());
            }
        }
    }
    keys
}

/// The folders inside one folder.
fn folders(beside: &Path) -> Vec<PathBuf> {
    std::fs::read_dir(beside)
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok().map(|entry| entry.path()))
                .filter(|path| path.is_dir())
                .collect()
        })
        .unwrap_or_default()
}

/// The last part of a path, which for a language's folder is its tag.
fn named(path: &Path) -> &str {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
}

/// Whether a file is a catalog: a Fluent file, and nothing else in the folder.
fn is_catalog(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "ftl")
}

/// Every built-in catalog file, for the checks that read them all.
#[cfg(test)]
pub(crate) fn every_built_in() -> Vec<(String, Vec<Source>)> {
    tags(None)
        .into_iter()
        .map(|tag| {
            let files = built_in(&tag);
            (tag, files)
        })
        .collect()
}
