//! `333 languages` — the languages there are words for, and how much of each.
//!
//! How much is counted against English: the share of English's messages a language
//! has its own words for. Whatever it lacks is said in English, so a language at half
//! is usable and reads as two languages; the number says which to expect.

use std::path::Path;

use crate::commands::Common;
use crate::words::catalog;

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
fn said(beside: Option<&Path>) -> Vec<String> {
    let english = catalog::keys(&catalog::built_in(catalog::ENGLISH));
    let mut lines = Vec::new();
    for tag in catalog::tags(beside) {
        // What went wrong reading them is said when they are spoken, not listed.
        let mut unsaid = Vec::new();
        let sources = catalog::sources(&tag, beside, &mut unsaid);
        let have = catalog::keys(&sources).intersection(&english).count();
        let percent = have * 100 / english.len().max(1);
        let name = own_name(&tag, sources);
        lines.push(words!(
            "languages-one",
            tag = tag.as_str(),
            name = name,
            percent = percent
        ));
    }
    let speaking = crate::words::current().tag();
    lines.push(words!("languages-speaking", tag = speaking));
    lines
}

/// What a language calls itself, in its own words, or its tag if it does not say.
fn own_name(tag: &str, sources: Vec<catalog::Source>) -> String {
    let mut unsaid = Vec::new();
    let bundle = catalog::bundle(tag, sources, &mut unsaid);
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::words::count::Base;

    #[test]
    fn every_built_in_language_is_listed_by_its_own_name() {
        let lines = crate::words::speaking("en", Base::Ten, || said(None));
        assert!(lines.contains(&"language en  English  100% of what English says".to_owned()));
        assert!(
            lines
                .iter()
                .any(|line| line.starts_with("language ko  한국어  "))
        );
        assert_eq!(
            lines.last().unwrap(),
            "speaking en. `--language <TAG>` or THE333_LANGUAGE chooses another."
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
}
