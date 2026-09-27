//! A command whose reader walked away before it finished, with the shipped binary.
//!
//! `333 id | head -1` closes the pipe while `id` still has lines to say. What is left
//! to say goes nowhere, and the command ends as it would have: a reader leaving is not
//! a failure of the thing they stopped reading.

#![cfg(unix)]
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use std::os::unix::fs::PermissionsExt as _;
use std::process::Command;

/// Run `333 <words>` on a scratch node with a standard output nobody reads.
fn unread(name: &str, words: &[&str]) -> std::process::Output {
    let home = std::env::temp_dir().join(format!("n333-unread-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    std::fs::create_dir_all(&home).expect("creates");
    std::fs::set_permissions(&home, std::fs::Permissions::from_mode(0o700)).expect("restricts");
    let (reader, writer) = std::io::pipe().expect("a pipe");
    // Closed before anything is written: every line the command says is a broken pipe.
    drop(reader);
    let out = Command::new(env!("CARGO_BIN_EXE_333"))
        .arg("--data-dir")
        .arg(&home)
        .args(words)
        .stdout(writer)
        .output()
        .expect("runs");
    let _ = std::fs::remove_dir_all(&home);
    out
}

#[test]
fn id_ends_quietly_when_nobody_reads_it() {
    let out = unread("id", &["id"]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        out.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn languages_ends_quietly_when_nobody_reads_it() {
    let out = unread("languages", &["languages"]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        out.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}
