//! Finding the licence text a package ships, and keeping one copy of each.
//!
//! Cargo records no such thing. The `license` field is an SPDX expression, not a file,
//! so the text itself has to be looked for beside the manifest under the names the
//! ecosystem happens to use. Nothing here searches subdirectories: a package that
//! keeps its text somewhere else is reported as having none, which puts its name in
//! the generated file where a person will read it, rather than leaving the gap to be
//! found by whoever was owed the attribution.
//!
//! Two packages that ship byte-identical text get one copy of it between them. That is
//! most of the file: the Apache-2.0 text alone would otherwise be repeated hundreds of
//! times, and a reader looking for what a licence says is not helped by the repetition.

use crate::graph::Shipped;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// What a licence file's name starts with, lowercased. Matched as a prefix because the
/// same licence arrives as `LICENSE`, `LICENSE-MIT`, `LICENSE.txt` and `license-mit`.
const NAMES: [&str; 4] = ["license", "licence", "copying", "notice"];

/// How much of the digest names a text. Long enough that no two of them collide, short
/// enough to read in a heading.
pub(crate) const DIGEST_SHOWN: usize = 16;

/// One licence text and every package that ships exactly these bytes.
pub(crate) struct Text {
    /// Names the text by its content, so that two different files both called
    /// `LICENSE-MIT` — which is most of them, they carry different names in them — are
    /// told apart in the headings and can be linked to.
    pub(crate) digest: String,
    /// The file name it was first found under.
    pub(crate) file: String,
    /// The text, with line endings normalised so the generated file is the same
    /// whichever system generated it.
    pub(crate) body: String,
    /// Every package carrying it, in the order the packages were offered.
    pub(crate) covers: Vec<String>,
}

/// A package that contributed no licence text, and why not.
pub(crate) struct Gap {
    /// The name as crates.io knows it.
    pub(crate) name: String,
    /// The version, as text, because nothing sorts by it here.
    pub(crate) version: String,
    /// What the manifest declared, which is all the attribution there is for it.
    pub(crate) licence: String,
    /// What a person has to go and do something about.
    pub(crate) reason: String,
}

/// Everything the generated file needs to say about licence text.
pub(crate) struct Corpus {
    /// The distinct texts, in the order they were first met.
    pub(crate) texts: Vec<Text>,
    /// The packages that had none.
    pub(crate) gaps: Vec<Gap>,
}

/// A licence file as it was read.
#[derive(Debug)]
struct Found {
    /// The name it had on disk.
    name: String,
    /// Its text, line endings normalised.
    body: String,
}

/// Read the licence text of every package, keeping one copy of each distinct text.
pub(crate) fn gather(shipped: &[Shipped]) -> Corpus {
    let mut texts: Vec<Text> = Vec::new();
    let mut placed: HashMap<String, usize> = HashMap::new();
    let mut gaps: Vec<Gap> = Vec::new();
    for package in shipped {
        let label = package.label();
        let missing = |reason: String| Gap {
            name: package.name.clone(),
            version: package.version.to_string(),
            licence: package.expression(),
            reason,
        };
        match beside(&package.directory, package.licence_file.as_deref()) {
            Err(reason) => gaps.push(missing(reason)),
            Ok(files) if files.is_empty() => gaps.push(missing(
                "ships no file named LICENSE*, LICENCE*, COPYING* or NOTICE*".to_owned(),
            )),
            Ok(files) => {
                for file in files {
                    let digest = hex(&Sha256::digest(file.body.as_bytes()));
                    match placed.get(&digest) {
                        Some(&at) => {
                            if let Some(text) = texts.get_mut(at) {
                                text.covers.push(label.clone());
                            }
                        }
                        None => {
                            placed.insert(digest.clone(), texts.len());
                            texts.push(Text {
                                digest: digest.chars().take(DIGEST_SHOWN).collect(),
                                file: file.name,
                                body: file.body,
                                covers: vec![label.clone()],
                            });
                        }
                    }
                }
            }
        }
    }
    Corpus { texts, gaps }
}

/// The licence files sitting beside a package's manifest, in a fixed order, or the
/// reason there are none to be had.
fn beside(directory: &Path, named: Option<&Path>) -> Result<Vec<Found>, String> {
    let listing = std::fs::read_dir(directory).map_err(|why| format!("{directory:?}: {why}"))?;
    let mut candidates: Vec<(String, PathBuf)> = Vec::new();
    for entry in listing {
        let entry = entry.map_err(|why| format!("{directory:?}: {why}"))?;
        let Ok(name) = entry.file_name().into_string() else {
            continue;
        };
        let lowercased = name.to_lowercase();
        if !NAMES.iter().any(|prefix| lowercased.starts_with(prefix)) {
            continue;
        }
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        candidates.push((name, path));
    }
    // A package may name its licence file itself, and name one this list would miss.
    if let Some(named) = named {
        let shown = named.file_name().map_or_else(
            || named.display().to_string(),
            |name| name.to_string_lossy().into_owned(),
        );
        if named.is_file() && !candidates.iter().any(|(_, path)| path == named) {
            candidates.push((shown, named.to_path_buf()));
        }
    }
    candidates.sort();

    let mut found = Vec::new();
    for (name, path) in candidates {
        let raw = std::fs::read(&path).map_err(|why| format!("{name}: {why}"))?;
        let body = String::from_utf8(raw).map_err(|_| format!("{name} is not UTF-8"))?;
        found.push(Found {
            name,
            body: body.replace("\r\n", "\n"),
        });
    }
    Ok(found)
}

/// Bytes as lowercase hex, without pulling in a crate to do it.
fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        // Writing to a String cannot fail, and a hex digest that lost a byte would be
        // worse than a run that stopped, so the result is deliberately not discarded.
        if write!(out, "{byte:02x}").is_err() {
            return String::new();
        }
    }
    out
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::expect_used,
        clippy::unwrap_used,
        clippy::panic,
        clippy::indexing_slicing
    )]

    use super::*;
    use cargo_metadata::semver::Version;

    fn scratch(name: &str) -> PathBuf {
        let directory = std::env::temp_dir().join(format!("n333-notices-test-{name}"));
        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir_all(&directory).expect("creates the directory");
        directory
    }

    fn write(directory: &Path, name: &str, body: &str) {
        std::fs::write(directory.join(name), body).expect("writes the file");
    }

    fn package(directory: &Path, name: &str, version: &str) -> Shipped {
        Shipped {
            name: name.to_owned(),
            version: Version::parse(version).expect("parses"),
            licence: Some("MIT".to_owned()),
            licence_file: None,
            directory: directory.to_path_buf(),
        }
    }

    #[test]
    fn every_name_the_ecosystem_uses_is_found_and_nothing_else_is() {
        let directory = scratch("names");
        for name in [
            "LICENSE",
            "LICENSE-MIT",
            "license-apache-2.0",
            "LICENCE",
            "COPYING",
            "NOTICE.md",
        ] {
            write(&directory, name, name);
        }
        write(&directory, "README.md", "not a licence");
        write(&directory, "Cargo.toml", "not a licence");
        write(
            &directory,
            "Licensing.txt",
            "a note about licensing, not a licence",
        );
        std::fs::create_dir_all(directory.join("LICENSES")).expect("creates a directory");

        let found = beside(&directory, None).expect("reads the directory");
        let names: Vec<&str> = found.iter().map(|file| file.name.as_str()).collect();
        assert_eq!(
            names,
            [
                "COPYING",
                "LICENCE",
                "LICENSE",
                "LICENSE-MIT",
                "NOTICE.md",
                "license-apache-2.0",
            ],
            "matched case-insensitively as a prefix, sorted, directories left out"
        );
        assert!(
            !names.contains(&"Licensing.txt"),
            "the prefix is a whole word: `licensing` is not `license`"
        );
    }

    #[test]
    fn a_directory_that_is_not_there_says_so_instead_of_looking_empty() {
        let missing = scratch("missing").join("gone");
        let why = beside(&missing, None).expect_err("cannot be read");
        assert!(!why.is_empty(), "the reason is carried, not swallowed");
    }

    #[test]
    fn a_package_with_no_licence_file_is_named_rather_than_skipped() {
        let directory = scratch("bare");
        write(&directory, "README.md", "nothing to see");
        let corpus = gather(&[package(&directory, "bare", "1.0.0")]);
        assert!(corpus.texts.is_empty());
        assert_eq!(corpus.gaps.len(), 1);
        assert_eq!(corpus.gaps[0].name, "bare");
        assert_eq!(corpus.gaps[0].version, "1.0.0");
        assert_eq!(corpus.gaps[0].licence, "MIT");
    }

    #[test]
    fn windows_line_endings_do_not_reach_the_generated_file() {
        let directory = scratch("crlf");
        write(&directory, "LICENSE", "one\r\ntwo\r\n");
        let found = beside(&directory, None).expect("reads");
        assert_eq!(found[0].body, "one\ntwo\n");
    }

    #[test]
    fn a_licence_file_the_manifest_names_is_taken_even_under_an_odd_name() {
        let directory = scratch("named");
        write(&directory, "UNLICENSE", "public domain");
        let named = directory.join("UNLICENSE");
        let found = beside(&directory, Some(&named)).expect("reads");
        assert_eq!(found.len(), 1, "found only because the manifest named it");
        assert_eq!(found[0].name, "UNLICENSE");
    }

    #[test]
    fn the_same_text_from_two_packages_is_kept_once_and_credits_both() {
        let shared = "Permission is hereby granted";
        let first = scratch("shared-a");
        let second = scratch("shared-b");
        let third = scratch("shared-c");
        write(&first, "LICENSE", shared);
        write(&second, "LICENSE-MIT", "a different licence entirely");
        write(&third, "LICENSE.txt", shared);

        let corpus = gather(&[
            package(&first, "alpha", "1.0.0"),
            package(&second, "beta", "2.0.0"),
            package(&third, "gamma", "3.0.0"),
        ]);

        assert_eq!(
            corpus.texts.len(),
            2,
            "identical bytes collapse to one text"
        );
        assert_eq!(
            corpus.texts[0].covers,
            ["alpha 1.0.0", "gamma 3.0.0"],
            "blocks appear in the order packages first offer them"
        );
        assert_eq!(corpus.texts[0].file, "LICENSE", "named as first found");
        assert_eq!(corpus.texts[1].covers, ["beta 2.0.0"]);
        assert_ne!(corpus.texts[0].digest, corpus.texts[1].digest);
        assert_eq!(corpus.texts[0].digest.len(), DIGEST_SHOWN);
    }

    #[test]
    fn a_package_shipping_two_licences_offers_both() {
        let directory = scratch("dual");
        write(&directory, "LICENSE-APACHE", "Apache");
        write(&directory, "LICENSE-MIT", "MIT");
        let corpus = gather(&[package(&directory, "dual", "0.9.0")]);
        assert_eq!(corpus.texts.len(), 2);
        assert!(corpus.gaps.is_empty());
    }

    #[test]
    fn the_digest_is_the_hex_of_the_bytes() {
        assert_eq!(hex(&[0x00, 0x0f, 0xff]), "000fff");
        assert_eq!(hex(&[]), "");
    }
}
