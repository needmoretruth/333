//! `333 languages` — the languages there are words for, and how much of each.
//!
//! How much is counted against English: the share of English's messages a language
//! can say in its own words. A message counts only when the language has it in the
//! shape English has it — a line where English has a line — because anything else is
//! said in English when it comes up, and the number is there to say how much of what
//! a person reads will be in the language they chose. Whatever it lacks is said in
//! English, so a language at half is usable and reads as two languages; the number
//! says which to expect.

use std::path::Path;

use unicode_width::UnicodeWidthStr as _;

use crate::commands::Common;
use crate::words::Arg;
use crate::words::catalog::{self, Bundle};

/// Print one line per language, then which one is being spoken.
///
/// Through one locked handle rather than `println!`, so that a reader who walked away
/// (`333 languages | head -1`) ends this quietly instead of panicking inside the macro.
///
/// # Errors
/// Fails if standard output is closed.
pub(crate) fn run(common: &Common) -> anyhow::Result<()> {
    use std::io::Write as _;
    let beside = catalog::beside(common.paths.root());
    let mut out = std::io::stdout().lock();
    for line in said(Some(&beside)) {
        writeln!(out, "{line}")?;
    }
    Ok(())
}

/// The lines, for the catalogs built in and the ones in `beside`.
///
/// The tags and the names are padded to the widest of each as a terminal counts
/// columns, so the share after them lines up whatever script a name is written in.
fn said(beside: Option<&Path>) -> Vec<String> {
    // What went wrong reading them is said when they are spoken, not listed.
    let mut unsaid = Vec::new();
    let english_files = catalog::built_in(catalog::ENGLISH);
    let keys = catalog::keys(&english_files);
    let english = catalog::bundle(catalog::ENGLISH, english_files, &mut unsaid);
    let listed: Vec<(String, String, usize)> = catalog::tags(beside)
        .into_iter()
        .map(|tag| {
            let sources = catalog::sources(&tag, beside, &mut unsaid);
            let bundle = catalog::bundle(&tag, sources, &mut unsaid);
            let own = keys
                .iter()
                .filter(|key| its_own(&bundle, &english, key))
                .count();
            let name = own_name(&tag, &bundle);
            (tag, name, own * 100 / keys.len().max(1))
        })
        .collect();
    let widest = |column: fn(&(String, String, usize)) -> &str| {
        listed
            .iter()
            .map(|one| column(one).width())
            .max()
            .unwrap_or(0)
    };
    let (tags, names) = (widest(|one| one.0.as_str()), widest(|one| one.1.as_str()));
    let mut lines: Vec<String> = listed
        .iter()
        .map(|(tag, name, percent)| {
            words!(
                "languages-one",
                tag = padded(tag, tags),
                name = padded(name, names),
                percent = Arg::exact(percent)
            )
        })
        .collect();
    let speaking = crate::words::current().tag();
    lines.push(words!("languages-speaking", tag = speaking));
    lines
}

/// Whether a language says this message itself: it has it, with the same attributes
/// English gives it, so it is not handed back to English when it is said.
fn its_own(bundle: &Bundle, english: &Bundle, key: &str) -> bool {
    let line = |bundle: &Bundle| {
        bundle
            .get_message(key)
            .map(|message| message.get_attribute("keyword").is_some())
    };
    bundle
        .get_message(key)
        .is_some_and(|message| message.value().is_some())
        && line(bundle) == line(english)
}

/// What a language calls itself, in its own words, or its tag if it does not say.
fn own_name(tag: &str, bundle: &Bundle) -> String {
    bundle
        .get_message("languages-name")
        .and_then(|message| message.value())
        .map_or_else(
            || tag.to_owned(),
            |name| {
                bundle
                    .format_pattern(name, None, &mut Vec::new())
                    .into_owned()
            },
        )
}

/// Text followed by spaces to fill `columns` terminal columns.
fn padded(text: &str, columns: usize) -> String {
    format!("{text}{}", " ".repeat(columns.saturating_sub(text.width())))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::words::count::Base;

    #[test]
    fn every_built_in_language_is_listed_by_its_own_name() {
        let lines = crate::words::speaking("en", Base::Ten, || said(None));
        assert!(
            lines.contains(&"language en  English  100% of the messages English has".to_owned()),
            "{lines:?}"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.starts_with("language ko  한국어   "))
        );
        assert_eq!(
            lines.last().unwrap(),
            "speaking en. `--language <TAG>` or THE333_LANGUAGE chooses another."
        );
    }

    #[test]
    fn the_share_lines_up_whatever_script_a_name_is_written_in() {
        let lines = crate::words::speaking("ko", Base::Twelve, || said(None));
        let columns: Vec<usize> = lines
            .iter()
            .filter_map(|line| line.find("영어에").map(|at| line[..at].width()))
            .collect();
        assert_eq!(columns.len(), 2, "{lines:?}");
        assert_eq!(columns[0], columns[1], "{lines:?}");
        assert!(
            lines[0].contains("100%"),
            "a share is per hundred: {lines:?}"
        );
    }

    #[test]
    fn a_folder_beside_the_node_is_a_language_and_counts_what_it_has() {
        let beside = std::env::temp_dir().join(format!("333-words-{}", std::process::id()));
        std::fs::create_dir_all(beside.join("eo")).unwrap();
        std::fs::write(beside.join("eo/id.ftl"), "languages-name = Esperanto\n").unwrap();
        let lines = crate::words::speaking("en", Base::Ten, || said(Some(&beside)));
        std::fs::remove_dir_all(&beside).unwrap();
        let esperanto = lines.iter().find(|line| line.contains(" eo ")).unwrap();
        assert!(
            esperanto.starts_with("language eo  Esperanto  "),
            "{esperanto}"
        );
        assert!(
            !esperanto.contains("100%"),
            "one message of many is not all of them"
        );
    }

    #[test]
    fn a_message_that_would_be_said_in_english_is_not_counted_as_the_language_s() {
        let beside = std::env::temp_dir().join(format!("333-shape-{}", std::process::id()));
        std::fs::create_dir_all(beside.join("eo")).unwrap();
        // A line in English, a phrase here: said in English whenever it comes up.
        std::fs::write(beside.join("eo/id.ftl"), "id-name = { $name }\n").unwrap();
        let (english, keys) = (
            catalog::bundle("en", catalog::built_in("en"), &mut Vec::new()),
            catalog::keys(&catalog::built_in("en")),
        );
        let sources = catalog::sources("eo", Some(&beside), &mut Vec::new());
        let bundle = catalog::bundle("eo", sources, &mut Vec::new());
        std::fs::remove_dir_all(&beside).unwrap();
        assert!(keys.contains("id-name"));
        assert!(!its_own(&bundle, &english, "id-name"));
    }
}
