//! A vigil stopped the way a service manager or a closed terminal stops it.
//!
//! `systemctl stop`, `kill` and `timeout` send SIGTERM, and a terminal that closes
//! sends SIGHUP. Each is somebody asking the vigil to end, and it ends as it does on
//! Ctrl-C: the socket other terminals hand orders through is taken away, anything
//! asked of the router is given back, and the farewell is said.

#![cfg(unix)]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::io::Read as _;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Start a vigil that reaches nothing past this machine, stop it with `signal`, and
/// return how it ended, what it said and whether its order socket was left behind.
fn stopped_by(signal: &str) -> (std::process::ExitStatus, String, bool) {
    use std::os::unix::fs::PermissionsExt as _;

    let root = std::env::temp_dir().join(format!("333-stopped-{signal}-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700)).unwrap();
    let mut vigil = Command::new(env!("CARGO_BIN_EXE_333"))
        .env("THE333_LANGUAGE", "en")
        .env_remove("THE333_COUNT_IN")
        .arg("--data-dir")
        .arg(&root)
        .args(["serve", "--plain", "--no-meet", "--no-mdns", "--no-router"])
        .args(["--bind", "127.0.0.1:0"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    let socket = root.join("control.sock");
    let deadline = Instant::now() + Duration::from_secs(120);
    while !socket.exists() && vigil.try_wait().unwrap().is_none() && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(100));
    }
    assert!(socket.exists(), "the vigil never opened its order socket");
    let sent = Command::new("kill")
        .args([&format!("-{signal}"), &vigil.id().to_string()])
        .status()
        .unwrap();
    assert!(sent.success());

    let deadline = Instant::now() + Duration::from_secs(60);
    let ended = loop {
        if let Some(ended) = vigil.try_wait().unwrap() {
            break ended;
        }
        assert!(Instant::now() < deadline, "still running after SIG{signal}");
        std::thread::sleep(Duration::from_millis(50));
    };
    let mut said = String::new();
    vigil
        .stdout
        .take()
        .unwrap()
        .read_to_string(&mut said)
        .unwrap();
    let left = socket.exists();
    std::fs::remove_dir_all(&root).unwrap();
    (ended, said, left)
}

#[test]
fn sigterm_ends_the_vigil_as_ctrl_c_does() {
    let (ended, said, left) = stopped_by("TERM");
    assert!(ended.success(), "{ended}:\n{said}");
    assert!(said.contains("node     ended in epoch"), "{said}");
    assert!(!left, "control.sock was left behind");
}

#[test]
fn sighup_ends_the_vigil_as_ctrl_c_does() {
    let (ended, said, left) = stopped_by("HUP");
    assert!(ended.success(), "{ended}:\n{said}");
    assert!(said.contains("node     ended in epoch"), "{said}");
    assert!(!left, "control.sock was left behind");
}
