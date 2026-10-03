//! `333 language` — the languages there are words for and how much of each, and the
//! one every command speaks from now on, saved with `333 language <TAG>`.
//!
//! How much is counted against English: the share of English's messages a language
//! can say in its own words. A message counts only when the language has it in the
//! shape English has it — a line where English has a line — because anything else is
//! said in English when it comes up, and the number is there to say how much of what
//! a person reads will be in the language they chose. Whatever it lacks is said in
//! English, so a language at half is usable and reads as two languages; the number
//! says which to expect.

use std::path::Path;

use anyhow::Context as _;
use unicode_width::UnicodeWidthStr as _;

use crate::commands::Common;
use crate::words::Arg;
use crate::words::catalog::{self, Bundle};

/// Save `tag` for every later command, or without one list the languages.
///
/// # Errors
/// Fails if there are no words for `tag`, the choice cannot be written, or standard
/// output is closed.
pub(crate) fn run(common: &Common, tag: Option<&str>) -> anyhow::Result<()> {
    match tag {
        Some(tag) => save(common, tag),
        None => list(common),
    }
}

/// Save the language every command at this node speaks from now on.
///
/// The folder that answers it is what is saved, so `ko-KR` is saved as `ko`, and
/// English is saved as nothing at all: it is what is spoken when nothing is asked for.
/// The directory is made private first, as it is for the node's name, because it is
/// the node's directory.
fn save(common: &Common, tag: &str) -> anyhow::Result<()> {
    let root = common.paths.root();
    let beside = catalog::beside(root);
    let Some(found) = crate::words::choose::matching(tag, &catalog::tags(Some(&beside))) else {
        anyhow::bail!(words!("languages-unknown", tag = tag));
    };
    let home = crate::identity_file::secure(&common.mistrust(), root)?;
    let file = crate::words::saved::FILE;
    let path = root.join(file).display().to_string();
    if found.eq_ignore_ascii_case(catalog::ENGLISH) {
        match home.remove_file(file) {
            Ok(()) | Err(fs_mistrust::Error::NotFound(_)) => {}
            Err(e) => return Err(e).with_context(|| words!("languages-saving", path = path)),
        }
        aloud_in!("languages-english");
        return Ok(());
    }
    home.write_and_replace(file, format!("{found}\n"))
        .with_context(|| words!("languages-saving", path = path))?;
    aloud_in!("languages-saved", tag = found);
    Ok(())
}

/// Print one line per language, then which one is being spoken.
///
/// Through one locked handle rather than `println!`, so that a reader who walked away
/// (`333 language | head -1`) ends this quietly instead of panicking inside the macro.
///
/// # Errors
/// Fails if standard output is closed.
fn list(common: &Common) -> anyhow::Result<()> {
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
        let named = |tag: &str, name: &str| {
            lines.iter().any(|line| {
                let words: Vec<&str> = line.split_whitespace().collect();
                words.get(1) == Some(&tag) && words.get(2) == Some(&name)
            })
        };
        assert!(named("en", "English"), "{lines:?}");
        assert!(named("ko", "한국어"), "{lines:?}");
        assert!(named("zh-Hant", "繁體中文"), "{lines:?}");
        assert_eq!(
            lines.last().unwrap(),
            "speaking en. `333 language <TAG>` saves another for every command."
        );
    }

    #[test]
    fn a_saved_language_is_the_folder_that_answers_it_and_english_is_saved_as_none() {
        let root = std::env::temp_dir().join(format!("333-language-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let common = crate::commands::Common {
            paths: crate::paths::NodePaths::at(root.clone()),
            timeout: std::time::Duration::from_secs(1),
            keeping: crate::node::Keeping::TheWindow,
            bridges: std::sync::Arc::new(std::sync::Mutex::new(n333_net::bridges::Bridges::none())),
            trust_directory_permissions: true,
        };
        crate::words::speaking("en", Base::Ten, || {
            run(&common, Some("ko-KR")).unwrap();
            assert_eq!(crate::words::saved::read(&root).as_deref(), Some("ko"));
            assert!(run(&common, Some("xx-nowhere")).is_err());
            assert_eq!(crate::words::saved::read(&root).as_deref(), Some("ko"));
            run(&common, Some("en")).unwrap();
            assert_eq!(crate::words::saved::read(&root), None);
        });
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn the_share_lines_up_whatever_script_a_name_is_written_in() {
        let lines = crate::words::speaking("ko", Base::Twelve, || said(None));
        let columns: Vec<usize> = lines
            .iter()
            .filter_map(|line| line.find("번역").map(|at| line[..at].width()))
            .collect();
        let languages = crate::words::catalog::every_built_in().len();
        assert_eq!(columns.len(), languages, "{lines:?}");
        assert!(columns.iter().all(|at| *at == columns[0]), "{lines:?}");
        assert!(
            lines[0].contains("100%"),
            "a share is per hundred: {lines:?}"
        );
    }

    #[test]
    fn a_folder_beside_the_node_is_a_language_and_counts_what_it_has() {
        let beside = std::env::temp_dir().join(format!("333-words-{}", std::process::id()));
        std::fs::create_dir_all(beside.join("la")).unwrap();
        std::fs::write(beside.join("la/id.ftl"), "languages-name = Latina\n").unwrap();
        let lines = crate::words::speaking("en", Base::Ten, || said(Some(&beside)));
        std::fs::remove_dir_all(&beside).unwrap();
        let latin = lines.iter().find(|line| line.contains(" la ")).unwrap();
        assert!(
            latin
                .split_whitespace()
                .take(3)
                .eq(["language", "la", "Latina"]),
            "{latin}"
        );
        assert!(
            !latin.contains("100%"),
            "one message of many is not all of them"
        );
    }

    #[test]
    fn a_message_that_would_be_said_in_english_is_not_counted_as_the_language_s() {
        let beside = std::env::temp_dir().join(format!("333-shape-{}", std::process::id()));
        std::fs::create_dir_all(beside.join("la")).unwrap();
        // A line in English, a phrase here: said in English whenever it comes up.
        std::fs::write(beside.join("la/id.ftl"), "id-name = { $name }\n").unwrap();
        let (english, keys) = (
            catalog::bundle("en", catalog::built_in("en"), &mut Vec::new()),
            catalog::keys(&catalog::built_in("en")),
        );
        let sources = catalog::sources("la", Some(&beside), &mut Vec::new());
        let bundle = catalog::bundle("la", sources, &mut Vec::new());
        std::fs::remove_dir_all(&beside).unwrap();
        assert!(keys.contains("id-name"));
        assert!(!its_own(&bundle, &english, "id-name"));
    }
}
