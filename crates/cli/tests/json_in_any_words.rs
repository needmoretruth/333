//! `status --json` read by a program, whatever the person running it reads in.
//!
//! What the client says about the choice of words — counting in twelve, a language it
//! has no words for, a base it does not know — is a notice about how it will speak, not
//! the output of the command, so it goes to the error stream and the output stays
//! something a program can parse.

#![cfg(unix)]
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use std::os::unix::fs::PermissionsExt as _;
use std::process::Command;

/// Run `333 <words>` on `home` with `env` set, and nothing of this machine's choice.
fn run(home: &std::path::Path, env: &[(&str, &str)], words: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_333"));
    command
        .env("THE333_LANGUAGE", "en")
        .env_remove("THE333_COUNT_IN")
        .env_remove("LANG")
        .env_remove("LC_ALL")
        .env_remove("LC_MESSAGES");
    for (name, value) in env {
        command.env(name, value);
    }
    command
        .arg("--data-dir")
        .arg(home)
        .args(words)
        .output()
        .expect("runs")
}

/// What is set in the environment, what flags are given, and whether a notice is due.
type Case<'a> = (&'a [(&'a str, &'a str)], &'a [&'a str], bool);

#[test]
fn status_json_parses_in_every_language_and_base_and_the_notices_go_to_stderr() {
    let home = std::env::temp_dir().join(format!("n333-json-words-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    std::fs::create_dir_all(&home).expect("creates");
    std::fs::set_permissions(&home, std::fs::Permissions::from_mode(0o700)).expect("restricts");
    assert!(run(&home, &[], &["id"]).status.success(), "a node to ask");

    let cases: [Case<'_>; 6] = [
        (&[("THE333_COUNT_IN", "twelve")], &[], true),
        (&[("THE333_COUNT_IN", "nonsense")], &[], true),
        (&[("THE333_LANGUAGE", "xx-nowhere")], &[], true),
        (&[], &["--language", "zz"], true),
        (&[("THE333_LANGUAGE", "ko")], &[], false),
        (
            &[("THE333_LANGUAGE", "ko"), ("THE333_COUNT_IN", "twelve")],
            &[],
            true,
        ),
    ];
    for (env, flags, a_notice) in cases {
        let mut words = vec!["status", "--json"];
        words.extend_from_slice(flags);
        let out = run(&home, env, &words);
        let (stdout, stderr) = (
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        assert!(out.status.success(), "{env:?} {flags:?}: {stderr}");
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
        assert!(parsed.is_ok(), "{env:?} {flags:?} printed:\n{stdout}");
        assert_eq!(!stderr.is_empty(), a_notice, "{env:?} {flags:?}: {stderr}");
    }
    let _ = std::fs::remove_dir_all(&home);
}
