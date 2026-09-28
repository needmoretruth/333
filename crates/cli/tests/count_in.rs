//! A base nobody counts in, asked for by a person who reads Korean, with the shipped
//! binary.
//!
//! Reading a base happens before the words are chosen, and the first sentence said in a
//! process fixes its words for good. So the refusal of a base must wait to be said
//! until the words are chosen, or everything after it is said in English.

#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use std::process::Command;

/// Everything `333 <words>` printed, both streams, in Korean and with nothing chosen
/// about the base but what is given here.
fn in_korean(words: &[&str], count_in: Option<&str>) -> String {
    let mut command = Command::new(env!("CARGO_BIN_EXE_333"));
    // A directory that is never made: `languages` reads nothing of a node.
    let nowhere = std::env::temp_dir().join(format!("n333-count-in-{}", std::process::id()));
    command
        .arg("--data-dir")
        .arg(&nowhere)
        .args(words)
        .env("THE333_LANGUAGE", "ko")
        .env_remove("THE333_COUNT_IN");
    if let Some(count_in) = count_in {
        command.env("THE333_COUNT_IN", count_in);
    }
    let out = command.output().expect("runs");
    String::from_utf8_lossy(&out.stdout).into_owned() + &String::from_utf8_lossy(&out.stderr)
}

#[test]
fn a_flag_naming_no_base_is_refused_in_korean() {
    let said = in_korean(&["--count-in", "eight", "languages"], None);
    assert!(said.contains("eight은(는) "), "{said}");
    assert!(!said.contains("invalid value"), "{said}");
}

#[test]
fn a_variable_naming_no_base_leaves_everything_after_it_in_korean() {
    let said = in_korean(&["languages"], Some("eight"));
    assert!(said.contains("THE333_COUNT_IN"), "{said}");
    assert!(said.contains("언어     ko"), "{said}");
}
