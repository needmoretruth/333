//! The checks that keep the catalogs, the templates and the scripts saying the same
//! keys. They read the repository's own `site/`, so they fail on the commit that breaks
//! one of these, not on a visitor's screen.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use fluent_bundle::FluentResource;
use fluent_syntax::ast;

use super::{SCRIPT_PREFIX, Words, script};
use crate::site::{ENGLISH, LANGUAGES, WORDS};

/// The repository's `site/`.
fn site() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../site")
}

/// Every message in one language's folder, with the variables it names, parsed without
/// forgiveness: a file that does not all read fails here.
fn messages(tag: &str) -> BTreeMap<String, BTreeSet<String>> {
    let dir = site().join(WORDS).join(tag);
    let mut found = BTreeMap::new();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "ftl"))
        .collect();
    paths.sort();
    for path in paths {
        let text = std::fs::read_to_string(&path).unwrap();
        let resource = FluentResource::try_new(text)
            .unwrap_or_else(|(_, errors)| panic!("{}: {errors:?}", path.display()));
        for entry in resource.entries() {
            if let ast::Entry::Message(message) = entry {
                let value = message
                    .value
                    .as_ref()
                    .unwrap_or_else(|| panic!("{} has no value", message.id.name));
                let named = script::variables(value);
                let twice = found.insert(message.id.name.to_owned(), named);
                assert!(twice.is_none(), "{} is in {tag} twice", message.id.name);
            }
        }
    }
    found
}

/// The text of every template and script under `site/`.
fn sources(extension: &str) -> Vec<(String, String)> {
    let mut found = Vec::new();
    let mut stack = vec![site()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() && !path.ends_with("vendor") && !path.ends_with(WORDS) {
                stack.push(path);
            } else if path.extension().is_some_and(|found| found == extension) {
                let text = std::fs::read_to_string(&path).unwrap();
                found.push((path.display().to_string(), text));
            }
        }
    }
    found
}

/// The keys templates ask for: `{{t:key}}`, `{{a:key}}`, with or without arguments.
fn template_keys() -> BTreeMap<String, String> {
    let mut used = BTreeMap::new();
    for (name, text) in sources("html") {
        for token in text.split("{{").skip(1) {
            let Some(token) = token.split("}}").next() else {
                continue;
            };
            let key = token
                .strip_prefix("t:")
                .or_else(|| token.strip_prefix("a:"))
                .map(|rest| rest.split('|').next().unwrap_or(rest));
            if let Some(key) = key {
                used.insert(key.to_owned(), name.clone());
            }
        }
    }
    used
}

/// The keys scripts ask for: `say("key"`.
fn script_keys() -> BTreeSet<String> {
    let mut used = BTreeSet::new();
    for (_, text) in sources("js") {
        for call in text.split("say(\"").skip(1) {
            if let Some(key) = call.split('"').next() {
                used.insert(key.to_owned());
            }
        }
    }
    used
}

/// The keys the server says itself, and the ones messages say inside others.
fn other_keys(english: &BTreeMap<String, BTreeSet<String>>) -> BTreeSet<String> {
    let mut used: BTreeSet<String> = crate::serve::SAID_BY_SERVER
        .iter()
        .map(|key| (*key).to_owned())
        .collect();
    for (_, text) in std::fs::read_dir(site().join(WORDS).join(ENGLISH.tag))
        .unwrap()
        .map(|entry| ((), std::fs::read_to_string(entry.unwrap().path()).unwrap()))
    {
        for key in english.keys() {
            if text.contains(&format!("{{ {key} }}")) {
                used.insert(key.clone());
            }
        }
    }
    used
}

#[test]
fn every_english_message_is_used_and_every_key_used_is_in_english() {
    let english = messages(ENGLISH.tag);
    let templates = template_keys();
    let scripts = script_keys();
    let others = other_keys(&english);
    for (key, page) in &templates {
        assert!(
            english.contains_key(key),
            "{page} asks for {key}, which English lacks"
        );
    }
    for key in scripts.iter().chain(&others) {
        assert!(
            english.contains_key(key),
            "{key} is asked for and English lacks it"
        );
    }
    for key in english.keys() {
        let used = templates.contains_key(key) || scripts.contains(key) || others.contains(key);
        assert!(used, "{key} is in English and nothing uses it");
    }
}

#[test]
fn a_translation_has_only_english_keys_with_english_variables() {
    let english = messages(ENGLISH.tag);
    for language in LANGUAGES.iter().filter(|language| !language.is_english()) {
        if !site().join(WORDS).join(language.tag).is_dir() {
            continue;
        }
        for (key, named) in messages(language.tag) {
            let Some(wanted) = english.get(&key) else {
                panic!("{} has {key}, which English does not", language.tag);
            };
            assert_eq!(
                &named, wanted,
                "{} names other variables in {key}",
                language.tag
            );
        }
    }
}

#[test]
fn every_script_message_can_be_said_by_a_script() {
    let words = Words::load(&site());
    let english = words.catalog(&ENGLISH).unwrap();
    for key in english
        .keys
        .iter()
        .filter(|key| key.starts_with(SCRIPT_PREFIX))
    {
        let pattern = english.bundle.get_message(key).unwrap().value().unwrap();
        assert!(
            script::shape(pattern).is_some(),
            "{key} cannot be said by a script"
        );
    }
    let json: serde_json::Value = serde_json::from_str(words.speaker(&ENGLISH).script()).unwrap();
    assert_eq!(json["js-line-epoch"]["two"], "this line's {$n}nd epoch");
    assert_eq!(json["js-line-epoch"]["type"], "ordinal");
    assert_eq!(json["js-copy"], "Copy");
}

#[test]
fn a_language_is_published_only_when_it_has_every_message() {
    let root = std::env::temp_dir().join(format!("n333-site-words-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    for tag in ["en", "ko", "eo"] {
        std::fs::create_dir_all(root.join(WORDS).join(tag)).unwrap();
    }
    std::fs::write(root.join("words/en/a.ftl"), "one = One\ntwo = Two { $n }\n").unwrap();
    std::fs::write(root.join("words/ko/a.ftl"), "one = 하나\ntwo = 둘 { $n }\n").unwrap();
    std::fs::write(root.join("words/eo/a.ftl"), "one = Unu\n").unwrap();
    let words = Words::load(&root);
    let published: Vec<&str> = words
        .published()
        .iter()
        .map(|language| language.tag)
        .collect();
    assert_eq!(published, ["en", "ko"]);
    let esperanto = crate::site::language_at("eo").unwrap();
    let speaker = words.speaker(esperanto);
    assert_eq!(speaker.word("one"), "Unu");
    assert_eq!(
        speaker.say("two", &[("n", super::Arg::Count(3))]).unwrap(),
        "Two 3",
        "a missing message falls back to English"
    );
    std::fs::remove_dir_all(&root).unwrap();
}
