//! Two real nodes, as two processes, handing the file from one to the other.
//!
//! Everything else in the workspace tests a part: the exchange over a pipe, the exchange
//! over a socket, the roll from admissions. This is the one test that runs the binary a
//! person downloads and does what the README tells them to do with it — a node that
//! holds the file keeps the vigil, a second one knocks, asks for the file, and then asks
//! itself where it stands. A client whose parts all pass while the whole never hands
//! anything over would fail here and nowhere else.
//!
//! Nothing leaves this machine. The node keeping the vigil listens on 127.0.0.1 only and
//! is told not to use the meeting point, not to announce itself on the network, and not
//! to ask the router for anything.

#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing
)]

use std::io::{BufRead as _, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::time::{Duration, Instant};

/// The binary under test, as cargo built it for this run.
const BINARY: &str = env!("CARGO_BIN_EXE_333");

/// How long any one step may take before it counts as hung. Generous, because a
/// shared CI machine can be slow; a step that works takes well under a second.
const PATIENCE: Duration = Duration::from_secs(60);

/// A scratch node directory that is removed however the test ends.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        let path =
            std::env::temp_dir().join(format!("333-two-nodes-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("creates the scratch directory");
        // The client refuses a directory others can enter, and it is right to.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o700))
                .expect("closes the scratch directory");
        }
        Self(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// A child process whose output is read as it arrives, and which is killed if the
/// test ends before it does.
struct Running {
    child: Child,
    lines: Receiver<String>,
    said: Vec<String>,
}

impl Running {
    fn start(home: &Path, args: &[&str]) -> Self {
        let mut child = Command::new(BINARY)
            .arg("--data-dir")
            .arg(home)
            .args(["--timeout", "30"])
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("starts the binary");
        let (sender, lines) = channel();
        for stream in [
            child
                .stdout
                .take()
                .map(|s| Box::new(s) as Box<dyn Read + Send>),
            child
                .stderr
                .take()
                .map(|s| Box::new(s) as Box<dyn Read + Send>),
        ]
        .into_iter()
        .flatten()
        {
            let sender = sender.clone();
            std::thread::spawn(move || {
                for line in BufReader::new(stream).lines().map_while(Result::ok) {
                    if sender.send(line).is_err() {
                        break;
                    }
                }
            });
        }
        Self {
            child,
            lines,
            said: Vec::new(),
        }
    }

    /// Everything said so far, for the message of a failed assertion.
    fn transcript(&self) -> String {
        self.said.join("\n")
    }

    /// Read until a line containing `needle` arrives, and return that line.
    fn wait_for(&mut self, needle: &str) -> String {
        let deadline = Instant::now() + PATIENCE;
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            match self.lines.recv_timeout(left) {
                Ok(line) => {
                    self.said.push(line.clone());
                    if line.contains(needle) {
                        return line;
                    }
                }
                Err(RecvTimeoutError::Timeout) => {
                    panic!("no `{needle}` within {PATIENCE:?}:\n{}", self.transcript())
                }
                Err(RecvTimeoutError::Disconnected) => {
                    panic!("ended without `{needle}`:\n{}", self.transcript())
                }
            }
        }
    }

    /// Wait for the process to end on its own, and return how it ended with everything
    /// it said.
    fn finish(mut self) -> (ExitStatus, String) {
        let deadline = Instant::now() + PATIENCE;
        let status = loop {
            if let Some(status) = self.child.try_wait().expect("can be waited on") {
                break status;
            }
            assert!(
                Instant::now() < deadline,
                "still running after {PATIENCE:?}:\n{}",
                self.transcript()
            );
            std::thread::sleep(Duration::from_millis(50));
        };
        // Both readers end when the pipes close, which they have.
        while let Ok(line) = self.lines.recv_timeout(Duration::from_secs(5)) {
            self.said.push(line);
        }
        (status, self.transcript())
    }

    /// Ask the vigil to end the way a person at the terminal does.
    ///
    /// Ctrl-C is the only way `serve` stops, so on unix it is sent as SIGINT through
    /// `kill`, which is how it arrives from a terminal. Windows has no way to send one to
    /// a single process from outside it, so there the process is ended outright and
    /// only the unix run shows the vigil closing.
    fn interrupt(&mut self) -> bool {
        if cfg!(unix) {
            Command::new("kill")
                .args(["-INT", &self.child.id().to_string()])
                .status()
                .is_ok_and(|status| status.success())
        } else {
            let _ = self.child.kill();
            false
        }
    }
}

impl Drop for Running {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// Run one command to its end, and insist that it succeeded.
fn run(home: &Path, args: &[&str]) -> String {
    let (status, said) = Running::start(home, args).finish();
    assert!(
        status.success(),
        "`333 {}` failed ({status}):\n{said}",
        args.join(" ")
    );
    said
}

/// A port nothing is listening on right now.
fn free_port() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0")
        .and_then(|listener| listener.local_addr())
        .expect("the system hands out a port")
        .port()
}

/// The name a node prints about itself on the first line of `333 id`.
fn name_of(said: &str) -> String {
    said.lines()
        .find_map(|line| line.strip_prefix("name     "))
        .expect("`id` says the name")
        .trim()
        .to_owned()
}

#[test]
fn the_file_passes_from_a_node_keeping_the_vigil_to_one_that_asks() {
    let giver = Scratch::new("giver");
    let asker = Scratch::new("asker");
    std::fs::write(giver.0.join("333.txt"), b"333").expect("writes the file");

    let giver_name = name_of(&run(&giver.0, &["id"]));
    let asker_name = name_of(&run(&asker.0, &["id"]));

    let port = free_port().to_string();
    let bind = format!("127.0.0.1:{port}");
    let invite = format!("333:127.0.0.1:{port}");
    let mut vigil = Running::start(
        &giver.0,
        &[
            "serve",
            "--plain",
            "--no-meet",
            "--no-mdns",
            "--no-upnp",
            "--bind",
            &bind,
        ],
    );
    vigil.wait_for(&format!("answer   {bind}"));

    let pinged = run(&asker.0, &["ping", &invite]);
    assert!(
        pinged.contains(&format!("witness  {giver_name}")),
        "the ping reached the node keeping the vigil:\n{pinged}"
    );
    assert!(
        pinged.contains("(answered the challenge we chose)"),
        "{pinged}"
    );

    let joined = run(&asker.0, &["join", &invite]);
    assert!(
        joined.contains(&format!("given    by {giver_name}")),
        "{joined}"
    );
    assert!(
        joined.contains("holding  the file, and able to pass it on"),
        "{joined}"
    );
    assert!(joined.contains("roll     1 of us"), "{joined}");
    assert_eq!(
        std::fs::read(asker.0.join("333.txt")).expect("the file was written"),
        b"333",
        "what was written is the file, byte for byte"
    );
    vigil.wait_for(&format!("gave     the file to {asker_name}"));

    let status = run(&asker.0, &["status"]);
    assert!(
        status.contains("roll       1"),
        "the roll counts it:\n{status}"
    );
    assert!(status.contains("Given the file in epoch"), "{status}");
    assert!(status.contains("GIVEN BY"), "{status}");

    if vigil.interrupt() {
        let (ended, said) = vigil.finish();
        assert!(
            ended.success(),
            "the vigil ended cleanly ({ended}):\n{said}"
        );
        assert!(said.contains("vigil    ended in epoch"), "{said}");
    }
}
