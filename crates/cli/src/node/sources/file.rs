//! The record of where things came from, on disk.
//!
//! WHY A SMALL FILE REWRITTEN WHOLE, AND NOT A LOG. What is worth keeping is the
//! latest of everything — first heard, last heard, per address — and never the
//! history of how it got there. A log in the style of the others would have to be
//! folded back down to that on every start, and it would grow by a record every time
//! a peer handed over an address this node already had, which is most of every trade.
//! Written whole, the file is exactly as large as what the window holds.
//!
//! It is written through a temporary file and a rename, so a crash leaves either the
//! old record or the new one and never half of each. It is JSON because the only
//! reader it has besides this client is the person whose disk it is on.
//!
//! LOSING IT COSTS NOTHING ANYBODY DECIDES ON. Nothing about standing, the roll or the
//! draw reads it. A node that finds it unreadable says so and starts a new one.

use std::io::Write as _;
use std::path::Path;

use anyhow::Context as _;
use n333_core::{Epoch, NodeId};
use serde::{Deserialize, Serialize};

use super::Sources;

/// The file, inside the node's directory. It travels in `333 pack`: where an address
/// was heard of is part of what the node knows, not of where it is kept.
pub(crate) const FILE: &str = "sources.json";

/// Which shape of record this is. Written into the file so that a later client can
/// tell a record it must convert from one it can read.
const FORMAT: u32 = 1;

/// The file as it is written.
#[derive(Serialize, Deserialize)]
struct OnDisk {
    /// [`FORMAT`].
    format: u32,
    /// The record itself.
    #[serde(flatten)]
    sources: Sources,
}

/// What opening the record found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Loaded {
    /// There was none; this is the first run that keeps one.
    Fresh,
    /// It was read.
    Read,
    /// It was there and could not be read, so a new one was begun.
    Unreadable,
}

/// Read the record in `home`, or begin one counting from `now`.
pub(crate) fn load(home: &Path, now: Epoch) -> (Sources, Loaded) {
    match read(home) {
        Ok(Some(sources)) => (sources, Loaded::Read),
        Ok(None) => (Sources::from(now), Loaded::Fresh),
        Err(_) => (Sources::from(now), Loaded::Unreadable),
    }
}

/// The record on disk, if there is one.
fn read(home: &Path) -> anyhow::Result<Option<Sources>> {
    let bytes = match std::fs::read(home.join(FILE)) {
        Ok(bytes) => bytes,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e.into()),
    };
    let on_disk: OnDisk = serde_json::from_slice(&bytes)?;
    anyhow::ensure!(on_disk.format == FORMAT, "a record of another format");
    Ok(Some(on_disk.sources))
}

/// Fold in whatever is on disk, forget what the window has passed, and write it back.
///
/// # Errors
/// Fails if the file cannot be written.
pub(crate) fn save(home: &Path, sources: &mut Sources, now: Epoch) -> anyhow::Result<()> {
    if let Ok(Some(theirs)) = read(home) {
        sources.merge(theirs);
    }
    sources.prune(now);
    let on_disk = OnDisk {
        format: FORMAT,
        sources: sources.clone(),
    };
    let bytes = serde_json::to_vec_pretty(&on_disk)
        .with_context(|| words!("node-sources-file-writing-down"))?;
    // Named for this process and this write, so that two writers at once — two
    // processes, or the vigil and an order typed into its screen — never write into
    // each other's half-finished file.
    static WRITES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let write = WRITES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let temporary = home.join(format!("{FILE}.{}.{write}.tmp", std::process::id()));
    let mut file = std::fs::File::create(&temporary).with_context(|| {
        words!(
            "node-sources-file-creating",
            file = temporary.display().to_string()
        )
    })?;
    file.write_all(&bytes)
        .and_then(|()| file.sync_data())
        .with_context(|| {
            words!(
                "node-sources-file-writing",
                file = temporary.display().to_string()
            )
        })?;
    std::fs::rename(&temporary, home.join(FILE))
        .with_context(|| words!("node-sources-file-putting", file = FILE))?;
    Ok(())
}

/// Note an address somebody typed, from a command that has not opened the node.
///
/// `ping` knocks with nothing but the name in hand, and a running vigil may have the
/// node open in another process. This touches only the record, and the vigil folds
/// it in the next time it writes its own.
///
/// # Errors
/// Fails if the record cannot be written.
pub(crate) fn typed(home: &Path, address: &str, name: Option<NodeId>) -> anyhow::Result<()> {
    let now = Epoch::now();
    let (mut sources, _) = load(home, now);
    let heard = super::Heard {
        from: super::Source::ByHand,
        epoch: now.0,
    };
    sources.heard(address, name.map(|name| name.to_string()), heard);
    save(home, &mut sources, now)
}

#[cfg(test)]
mod tests {
    use super::super::{Heard, Mine, Sighting, Source};
    use super::*;

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let pairs = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            [
                (
                    words!("node-sources-file-writing-down"),
                    "writing down where things came from",
                ),
                (
                    words!(
                        "node-sources-file-creating",
                        file = "/n/sources.json.1.0.tmp"
                    ),
                    "creating /n/sources.json.1.0.tmp",
                ),
                (
                    words!(
                        "node-sources-file-writing",
                        file = "/n/sources.json.1.0.tmp"
                    ),
                    "writing /n/sources.json.1.0.tmp",
                ),
                (
                    words!("node-sources-file-putting", file = FILE),
                    "putting sources.json in place",
                ),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

    /// A directory for one test, gone when the test is.
    struct Scratch(std::path::PathBuf);

    impl std::ops::Deref for Scratch {
        type Target = Path;
        fn deref(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn scratch(name: &str) -> Scratch {
        let dir =
            std::env::temp_dir().join(format!("n333-sources-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("creates dir");
        Scratch(dir)
    }

    fn by_hand(epoch: u64) -> Heard {
        Heard {
            from: Source::ByHand,
            epoch,
        }
    }

    #[test]
    fn what_was_written_is_what_is_read_after_a_restart() {
        let home = scratch("restart");
        let mut sources = Sources::from(Epoch(10));
        sources.heard("a.example:3333", None, by_hand(11));
        sources.said("me.example:3333", Epoch(11));
        save(&home, &mut sources, Epoch(11)).expect("saves");

        let (read, loaded) = load(&home, Epoch(12));
        assert_eq!(loaded, Loaded::Read);
        assert_eq!(read, sources);
        assert_eq!(read.is_mine("me.example:3333", Epoch(11)), Mine::Yes);
    }

    #[test]
    fn what_another_process_wrote_is_kept_when_this_one_writes() {
        // `333 ping` in one terminal while the vigil runs in another: both write the
        // same file, and the vigil writing last must not throw away what was typed.
        let home = scratch("merge");
        let mut typed = Sources::from(Epoch(10));
        typed.heard("typed.example:3333", None, by_hand(12));
        save(&home, &mut typed, Epoch(12)).expect("saves");

        let mut vigil = Sources::from(Epoch(10));
        let from_peer = Heard {
            from: Source::Peer {
                name: "333b".into(),
            },
            epoch: 12,
        };
        vigil.heard("peer.example:3333", Some("333c".into()), from_peer);
        save(&home, &mut vigil, Epoch(12)).expect("saves");

        let (read, _) = load(&home, Epoch(12));
        assert!(read.of("typed.example:3333").is_some());
        assert!(read.of("peer.example:3333").is_some());
    }

    #[test]
    fn what_the_window_has_passed_is_forgotten() {
        let home = scratch("prune");
        let mut sources = Sources::from(Epoch(1));
        sources.heard("old.example:3333", None, by_hand(1));
        sources.heard("new.example:3333", None, by_hand(400));
        sources.sighted(Sighting {
            address: "copy.example:3333".into(),
            said_in: 2,
            heard: by_hand(2),
        });
        save(&home, &mut sources, Epoch(400)).expect("saves");
        let (read, _) = load(&home, Epoch(400));
        assert!(read.of("old.example:3333").is_none());
        assert!(read.of("new.example:3333").is_some());
        assert!(read.sightings().is_empty());
    }

    #[test]
    fn a_record_that_cannot_be_read_is_begun_again_and_said() {
        let home = scratch("unreadable");
        std::fs::write(home.join(FILE), b"not json").expect("writes");
        let (read, loaded) = load(&home, Epoch(7));
        assert_eq!(loaded, Loaded::Unreadable);
        assert_eq!(read, Sources::from(Epoch(7)));
    }

    #[test]
    fn the_first_hearing_stays_first_and_the_last_moves_on() {
        let mut sources = Sources::from(Epoch(1));
        assert!(sources.heard("x.example:3333", None, by_hand(5)));
        let later = Heard {
            from: Source::Peer {
                name: "333d".into(),
            },
            epoch: 9,
        };
        assert!(!sources.heard("x.example:3333", Some("333e".into()), later.clone()));
        let learned = sources.of("x.example:3333").expect("noted");
        assert_eq!(learned.first, by_hand(5));
        assert_eq!(learned.last, later);
        assert_eq!(learned.name.as_deref(), Some("333e"));
    }

    #[test]
    fn a_statement_from_before_the_count_began_cannot_be_called_a_copy() {
        // A node that upgrades has signed statements nobody wrote down, and they will
        // come back to it from its peers. Calling those a second copy would be the
        // loudest false alarm this client could raise.
        let sources = Sources::from(Epoch(100));
        assert_eq!(sources.is_mine("a:1", Epoch(100)), Mine::CannotTell);
        assert_eq!(sources.is_mine("a:1", Epoch(101)), Mine::No);
    }
}
