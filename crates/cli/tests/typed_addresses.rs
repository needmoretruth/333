//! Addresses typed by hand, with the shipped binary.
//!
//! An address somebody typed is kept, and knocked on by the vigil, only once a node
//! answered there. One that could never be an address is refused before anything is
//! kept, with what was wrong and how one is written.

#![cfg(unix)]
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A scratch node directory that is removed however the test ends.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("n333-typed-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("creates");
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o700)).expect("closes");
        Self(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Run `333 <words>` on `home`, in English, and return how it ended and what it said.
fn run(home: &Path, words: &[&str]) -> (i32, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_333"))
        .env("THE333_LANGUAGE", "en")
        .env_remove("THE333_COUNT_IN")
        .arg("--data-dir")
        .arg(home)
        .args(["--timeout", "5"])
        .args(words)
        .output()
        .expect("runs");
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    (out.status.code().unwrap_or(-1), said)
}

/// A port nothing listens on: taken from the system and let go again.
fn nobody_there() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("a port");
    let port = listener.local_addr().expect("bound").port();
    drop(listener);
    format!("127.0.0.1:{port}")
}

#[test]
fn an_address_nobody_answered_at_is_not_kept() {
    let home = Scratch::new("unanswered");
    assert_eq!(run(&home.0, &["id"]).0, 0);
    let refused = nobody_there();
    for command in ["ping", "join"] {
        let (code, said) = run(&home.0, &[command, &refused]);
        assert_ne!(code, 0, "{said}");
        let sources = std::fs::read_to_string(home.0.join("sources.json")).unwrap_or_default();
        assert!(!sources.contains(&refused), "{command} kept it:\n{sources}");
    }
}

#[test]
fn what_could_never_be_an_address_is_refused_before_anything_is_kept() {
    let home = Scratch::new("refused");
    assert_eq!(run(&home.0, &["id"]).0, 0);
    for (typed, why) in [
        ("http://127.0.0.1:43331", "http:// belongs to a web address"),
        (
            "334:127.0.0.1:43331",
            "an invitation starts with 333:, not 334:",
        ),
        ("abc.onion:3333", "abc.onion is not an onion address"),
    ] {
        let (code, said) = run(&home.0, &["ping", typed]);
        assert_eq!(code, 2, "{said}");
        assert!(said.contains(why), "{typed}:\n{said}");
        assert!(said.contains("as in node.example:3333"), "{said}");
    }
    // Everything that reaches beyond this machine is off, in case it is ever accepted.
    let serve = [
        "serve",
        "--plain",
        "--no-meet",
        "--no-mdns",
        "--no-router",
        "--bind",
        "127.0.0.1:0",
        "--announce",
        "not an address",
    ];
    let (code, said) = run(&home.0, &serve);
    assert_eq!(code, 2, "{said}");
    assert!(
        said.contains("\"not an address\" is not a host name or an IP address"),
        "{said}"
    );
    let sources = std::fs::read_to_string(home.0.join("sources.json")).unwrap_or_default();
    assert!(!sources.contains("43331"), "{sources}");
}
