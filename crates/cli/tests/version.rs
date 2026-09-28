//! What the binary says about itself when asked, as a person asks it.
//!
//! The format is tested beside the code. This is the other half: that `--version` and
//! `-V` both say the long form, which is clap's wiring and only shows in the binary.

#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing
)]

use std::process::Command;

/// Run the binary with one argument and return what it printed.
fn asked(argument: &str) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_333"))
        // The words are read in English, whatever this machine speaks.
        .env("THE333_LANGUAGE", "en")
        .env_remove("THE333_COUNT_IN")
        .arg(argument)
        .output()
        .expect("runs");
    assert!(out.status.success(), "{argument} failed");
    String::from_utf8(out.stdout).expect("prints text")
}

/// What this test run's build should call itself, written out for each feature set CI
/// builds rather than computed the way the binary computes it.
fn edition() -> &'static str {
    match (cfg!(feature = "screen"), cfg!(feature = "tor")) {
        (true, true) => "Standard, tor",
        (false, true) => "Light, tor",
        (false, false) => "Light, no tor",
        (true, false) => "Standard, no tor",
    }
}

#[test]
fn the_long_version_names_the_edition_and_the_target() {
    let target = format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS);
    assert_eq!(
        asked("--version"),
        format!(
            "333 {} ({}, {target})\n",
            env!("CARGO_PKG_VERSION"),
            edition()
        )
    );
}

#[test]
fn the_short_flag_says_the_same_as_the_long_one() {
    assert_eq!(asked("-V"), asked("--version"));
}
