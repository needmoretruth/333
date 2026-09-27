//! One file, one responsibility, at most four hundred lines before its tests.
//!
//! The rule is read by people, and people let a file grow by ten lines at a time
//! without anybody deciding it should be twice the size. Two files went over it that
//! way before this test existed. Counting is what a machine is for, so the count is
//! kept here rather than in a reviewer's patience.

#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing
)]

use std::path::{Path, PathBuf};

/// The most lines a file may have before its tests begin.
const MOST: usize = 400;

/// Where the tests begin. Everything below it lives beside the code it tests and is
/// not counted, so that testing a file thoroughly never pushes it over.
const TESTS_BEGIN: &str = "#[cfg(test)]";

/// Every Rust source file under `dir`, at any depth.
fn sources(dir: &Path, found: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("reads a source directory") {
        let path = entry.expect("reads an entry").path();
        if path.is_dir() {
            sources(&path, found);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            found.push(path);
        }
    }
}

/// How many lines come before the first line that starts the tests.
fn counted(text: &str) -> usize {
    text.lines()
        .take_while(|line| !line.starts_with(TESTS_BEGIN))
        .count()
}

#[test]
fn no_file_is_over_four_hundred_lines_before_its_tests() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("finds the workspace");
    let mut found = Vec::new();
    for krate in std::fs::read_dir(root.join("crates")).expect("reads crates") {
        let src = krate.expect("reads a crate").path().join("src");
        if src.is_dir() {
            sources(&src, &mut found);
        }
    }
    assert!(!found.is_empty(), "no source files found under {root:?}");
    found.sort();
    let over: Vec<String> = found
        .iter()
        .filter_map(|path| {
            let lines = counted(&std::fs::read_to_string(path).expect("reads a source"));
            let name = path.strip_prefix(&root).unwrap_or(path).display();
            (lines > MOST).then(|| format!("{name}: {lines}"))
        })
        .collect();
    assert!(
        over.is_empty(),
        "over {MOST} lines before {TESTS_BEGIN}; split by responsibility:\n{}",
        over.join("\n")
    );
}

#[test]
fn counting_stops_where_the_tests_begin() {
    assert_eq!(counted("a\nb\n#[cfg(test)]\nmod tests {}\n"), 2);
    assert_eq!(
        counted("a\nb\nc\n"),
        3,
        "a file with no tests is counted whole"
    );
    assert_eq!(
        counted("a\n    #[cfg(test)]\nb\n"),
        3,
        "only a test module at the top of the file ends the count"
    );
}
