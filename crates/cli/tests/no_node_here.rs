//! A command pointed at a directory that holds no node, with the shipped binary.
//!
//! `status` on a mistyped `--data-dir` used to make a name there and report standing 0
//! for it. Only the commands a name is for make one; the others say there is no node
//! there and make nothing, not even the directory.

#![cfg(unix)]
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use std::process::Command;

/// Run `333 <words>` with `--data-dir` at `home`, in English.
fn run(home: &std::path::Path, words: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_333"))
        .env("THE333_LANGUAGE", "en")
        .env_remove("THE333_COUNT_IN")
        .arg("--data-dir")
        .arg(home)
        .args(words)
        .output()
        .expect("runs")
}

#[test]
fn a_command_that_reads_a_node_makes_nothing_where_there_is_none() {
    let parent = std::env::temp_dir().join(format!("n333-no-node-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&parent);
    let typo = parent.join("nodde");
    for words in [
        &["status"][..],
        &["status", "--json"],
        &["say", "7"],
        &["pack", "/nonexistent/out.333"],
        &["moved"],
    ] {
        let out = run(&typo, words);
        let said = String::from_utf8_lossy(&out.stderr);
        assert_eq!(out.status.code(), Some(1), "{words:?}: {said}");
        assert!(
            said.starts_with(&format!("failed   there is no node in {}", typo.display())),
            "{words:?}: {said}"
        );
        assert!(said.contains("`333 name` makes one there."), "{said}");
        assert!(out.stdout.is_empty(), "{words:?}");
        assert!(!parent.exists(), "{words:?} made {}", parent.display());
    }
    // Telling a vigil that is not there is refused as that, and makes nothing either.
    let told = run(&typo, &["tell", "tor", "on"]);
    assert!(!told.status.success());
    assert!(
        String::from_utf8_lossy(&told.stdout).starts_with("unheard  no node is running"),
        "{told:?}"
    );
    assert!(!parent.exists(), "tell made {}", parent.display());
    // And the command a name is for still makes one.
    std::fs::create_dir_all(&parent).expect("creates");
    {
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(&parent, std::fs::Permissions::from_mode(0o700)).expect("closes");
    }
    assert!(run(&typo, &["id"]).status.success());
    assert!(typo.join("identity.key").exists());
    assert!(run(&typo, &["status"]).status.success());
    let _ = std::fs::remove_dir_all(&parent);
}
