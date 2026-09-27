//! A running vigil told things from another terminal, with real processes.
//!
//! Two programs on one directory is the whole of what this is about, so it cannot be
//! tested inside one: each vigil here is the shipped binary in a child process, on a
//! scratch directory, answering on loopback and on nothing else.

#![cfg(unix)]
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use std::io::{BufRead as _, BufReader};
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{Receiver, channel};
use std::time::{Duration, Instant};

/// The client under test.
const CLIENT: &str = env!("CARGO_BIN_EXE_333");

/// Longer than a vigil takes to find a name and open its socket on a slow machine.
const PATIENCE: Duration = Duration::from_secs(90);

/// A directory only its owner can enter, as the client requires.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("n333-told-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("creates");
    std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700)).expect("restricts");
    dir
}

/// A vigil in a child process, stopped when this is dropped.
struct Vigil {
    child: Child,
    port: u16,
    home: PathBuf,
    /// Held so the vigil's output always has a reader. A vigil whose standard output
    /// is closed stops at the next thing it says.
    _said: Receiver<String>,
}

impl Drop for Vigil {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = std::fs::remove_dir_all(&self.home);
    }
}

/// Keep the vigil at `home`, and wait until it answers and takes orders.
fn keep(home: &Path) -> Vigil {
    let mut child = Command::new(CLIENT)
        // The lines are read in English, whatever this machine speaks.
        .env("THE333_LANGUAGE", "en")
        .env_remove("THE333_COUNT_IN")
        .arg("--data-dir")
        .arg(home)
        .args(["serve", "--plain", "--no-meet", "--no-mdns", "--no-upnp"])
        .args(["--bind", "127.0.0.1:0"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("starts");
    let said = lines(child.stdout.take().expect("piped"));
    let (mut port, mut taking) = (None, false);
    let started = Instant::now();
    while port.is_none() || !taking {
        let left = PATIENCE.saturating_sub(started.elapsed());
        let line = said.recv_timeout(left).expect("the vigil started in time");
        if let Some(at) = line.strip_prefix("answer   127.0.0.1:") {
            port = Some(at.trim().parse().expect("a port"));
        }
        taking |= line.starts_with("orders   from any terminal");
    }
    Vigil {
        child,
        port: port.expect("answering"),
        home: home.to_path_buf(),
        _said: said,
    }
}

/// Everything a child says, line by line, as it says it.
fn lines(out: impl std::io::Read + Send + 'static) -> Receiver<String> {
    let (send, receive) = channel();
    std::thread::spawn(move || {
        for line in BufReader::new(out).lines().map_while(Result::ok) {
            if send.send(line).is_err() {
                return;
            }
        }
    });
    receive
}

/// Run the client once against `home`, and say whether it succeeded and what it said.
fn client(home: &Path, words: &[&str]) -> (bool, String) {
    let out = Command::new(CLIENT)
        // The lines are read in English, whatever this machine speaks.
        .env("THE333_LANGUAGE", "en")
        .env_remove("THE333_COUNT_IN")
        .arg("--data-dir")
        .arg(home)
        .args(words)
        .output()
        .expect("runs");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
    )
}

/// The epoch in a `said     #N in epoch E` line.
fn said_in(out: &str) -> Option<u64> {
    out.lines()
        .find_map(|line| line.strip_prefix("said     #")?.split(" in epoch ").nth(1))
        .and_then(|epoch| epoch.trim().parse().ok())
}

#[test]
fn what_another_terminal_asks_for_is_done_by_the_vigil_and_refused_by_it() {
    let giver_home = scratch("giver");
    std::fs::write(giver_home.join("333.txt"), b"333").expect("writes the file");
    let giver = keep(&giver_home);
    let asked = keep(&scratch("asked"));

    // Handed to the vigil holding the directory, which asks the giver for the file.
    let invitation = format!("333:127.0.0.1:{}", giver.port);
    let (joined, out) = client(&asked.home, &["join", &invitation]);
    assert!(joined, "{out}");
    assert!(out.contains("joined   in epoch"), "{out}");
    assert!(
        out.lines().last().unwrap_or("").starts_with("done     "),
        "{out}"
    );

    let (said, out) = client(&asked.home, &["say", "7"]);
    assert!(said, "{out}");
    let epoch = said_in(&out).expect("the vigil said it");

    // The vigil's own refusal, in its own words, and a failure here.
    let (again, out) = client(&asked.home, &["say", "8"]);
    if again {
        assert_ne!(said_in(&out), Some(epoch), "said twice in one epoch: {out}");
    } else {
        assert!(
            out.contains(&format!("you already said #7 in epoch {epoch}")),
            "{out}"
        );
        assert!(
            out.lines().last().unwrap_or("").starts_with("failed   "),
            "{out}"
        );
    }

    let (read, out) = client(&asked.home, &["status"]);
    assert!(read && out.contains("ANSWERING"), "{out}");
}

#[test]
fn a_second_vigil_on_one_directory_is_refused_and_touches_nothing() {
    let vigil = keep(&scratch("twice"));
    let lock = vigil.home.join("lock");
    let before = std::fs::read_to_string(&lock).expect("reads");
    assert_eq!(before.trim(), vigil.child.id().to_string());

    let (started, out) = client(
        &vigil.home,
        &[
            "serve",
            "--plain",
            "--no-meet",
            "--no-mdns",
            "--no-upnp",
            "--bind",
            "127.0.0.1:0",
        ],
    );
    assert!(!started, "{out}");
    assert!(
        out.contains(&format!(
            "(process {}) is already keeping the vigil here",
            vigil.child.id()
        )),
        "{out}"
    );
    assert_eq!(std::fs::read_to_string(&lock).expect("reads"), before);
}

#[test]
fn telling_a_directory_nobody_is_keeping_is_refused() {
    let home = scratch("nobody");
    let (told, out) = client(&home, &["tell", "tor", "on"]);
    assert!(!told);
    assert!(
        out.starts_with("unheard  nobody is keeping the vigil"),
        "{out}"
    );
    let _ = std::fs::remove_dir_all(&home);
}

#[test]
fn status_takes_its_flags_beside_a_vigil_and_pack_is_refused_there() {
    let vigil = keep(&scratch("beside"));

    let (read, out) = client(&vigil.home, &["status", "--json"]);
    assert!(read, "{out}");
    // Nothing after the page: a program reads all of standard output as the JSON.
    assert!(out.trim_start().starts_with('{'), "{out}");
    assert!(out.trim_end().ends_with('}'), "{out}");
    assert!(out.contains("\"format\": 1"), "{out}");
    assert!(out.contains("\"unseen\": false"), "{out}");

    let (read, out) = client(&vigil.home, &["status", "--sources"]);
    assert!(read, "{out}");
    assert!(
        !out.contains("ANSWERING"),
        "the whole status, not the sources: {out}"
    );

    let file = std::env::temp_dir().join(format!("n333-told-packed-{}.333", std::process::id()));
    let (packed, out) = client(&vigil.home, &["pack", file.to_str().expect("a path")]);
    assert!(!packed, "{out}");
    assert!(out.contains("is keeping the vigil here"), "{out}");
    assert!(!file.exists(), "nothing was written");
    assert!(
        !vigil.home.join("packed").exists(),
        "the directory was not marked"
    );
}

#[test]
fn a_vigil_whose_output_nobody_reads_goes_on_answering() {
    let home = scratch("unread");
    let mut child = Command::new(CLIENT)
        // The lines are read in English, whatever this machine speaks.
        .env("THE333_LANGUAGE", "en")
        .env_remove("THE333_COUNT_IN")
        .arg("--data-dir")
        .arg(&home)
        .args(["serve", "--plain", "--no-meet", "--no-mdns", "--no-router"])
        .args(["--bind", "127.0.0.1:0"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("starts");
    let mut said = BufReader::new(child.stdout.take().expect("piped"));
    let mut line = String::new();
    while !line.starts_with("orders   from any terminal") {
        line.clear();
        assert!(said.read_line(&mut line).expect("reads") > 0, "ended");
    }
    // The reader walks away, as `333 serve --plain | head` does.
    drop(said);
    let (_, never) = channel();
    let vigil = Vigil {
        child,
        port: 0,
        home,
        _said: never,
    };

    // Each of these makes the vigil say a line into the closed pipe first.
    for _ in 0..2 {
        let (told, out) = client(&vigil.home, &["tell", "tor", "off"]);
        assert!(told, "{out}");
        assert!(out.contains("no unseen address up"), "{out}");
    }
}
