//! A vigil with nobody at a terminal keeps going, and says that it is awake.
//!
//! This is the vigil a service manager runs: `serve --plain`, no terminal, nothing on
//! standard input. Run as the real program, because what failed here was how the parts
//! of `serve` end together, which no test of one part can see.

// The directory permissions the vigil insists on are set here the Unix way.
#![cfg(unix)]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[test]
fn a_plain_vigil_with_nobody_watching_keeps_going_and_says_it_is_awake() {
    use std::os::unix::fs::PermissionsExt as _;

    let root = std::env::temp_dir().join(format!("333-plain-vigil-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700)).unwrap();
    // Nothing that reaches past this machine: no meeting point, no neighbours, no router.
    let mut vigil = Command::new(env!("CARGO_BIN_EXE_333"))
        .arg("--data-dir")
        .arg(&root)
        .args(["serve", "--plain", "--no-meet", "--no-mdns", "--no-upnp"])
        .args(["--bind", "127.0.0.1:0"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    let deadline = Instant::now() + Duration::from_secs(120);
    let stamp = root.join("awake");
    while !stamp.exists() && vigil.try_wait().unwrap().is_none() && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(100));
    }
    // Long enough after the stamp for anything that was going to end it to have ended.
    std::thread::sleep(Duration::from_secs(2));
    let ended = vigil.try_wait().unwrap();
    let _ = vigil.kill();
    let _ = vigil.wait();
    let said = std::fs::read_to_string(&stamp);
    std::fs::remove_dir_all(&root).unwrap();

    assert_eq!(ended, None, "the vigil ended by itself");
    let said = said.expect("the vigil never said it was awake");
    assert!(said.trim_end().ends_with('Z'), "{said:?} is not a UTC time");
}
