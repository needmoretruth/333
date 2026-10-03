//! The observer's short memory: one line per epoch, so a page can show how the network
//! has been and not only how it is now.
//!
//! `history.jsonl` sits beside the observation, readable by everybody like it. Each line
//! is one JSON object. When the observer first sees a new epoch, it writes one line about
//! the epoch that just ended, from its last observation of that epoch:
//!
//! ```text
//! {
//!   "epoch": <the epoch that ended>,
//!   "as_of": <ms since 1970 of the last observation in it>,
//!   "roll": <nodes on the roll, its founder included>,
//!   "answering": <the site node's `status.answering`>,     absent if it never said
//!   "saying": <statements on the board>,                   absent: the observer cannot see it
//!   "tor": <of those, the onion ones>,                     absent, likewise
//!   "site_node_running": <did the site node answer>
//! }
//! ```
//!
//! NUMBERS ONLY. A line holds counts, an epoch, a time and a yes or no: never a name and
//! never an address, so nothing in it is more than the observation already made public.
//!
//! A FILE THAT CANNOT BE READ IS STARTED AGAIN, NEVER FATAL. Lines that do not read are
//! dropped the next time a line is written, and the file is then rewritten whole; so is
//! a file past [`KEEP`] lines, which keeps the newest. Otherwise a line is appended. The
//! observer logs a failure here and carries on: this file is worth less than the
//! observation it sits beside.

use std::io::Write as _;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::atomic;

/// The file's name, in the observation's directory.
pub(crate) const FILE: &str = "history.jsonl";

/// How many lines are kept: about 1.9 years of epochs.
pub(crate) const KEEP: usize = 3_000;

/// Public, like the observation.
const MODE: u32 = 0o644;

/// One epoch, as its last observation saw it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Sample {
    /// The epoch.
    pub(crate) epoch: u64,
    /// When the observation this was taken from was written, in milliseconds.
    pub(crate) as_of: u64,
    /// Nodes on the roll, its founder included.
    pub(crate) roll: u64,
    /// The site node's count of nodes answering, if it gave one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) answering: Option<u64>,
    /// Statements on the board, if whoever wrote this could see it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) saying: Option<u64>,
    /// Of those, the ones naming an onion address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) tor: Option<u64>,
    /// Whether the site node answered on its socket.
    pub(crate) site_node_running: bool,
}

/// Where the history of the observation at `observation` is kept.
pub(crate) fn beside(observation: &Path) -> PathBuf {
    observation.with_file_name(FILE)
}

/// Nodes on the roll in an observation, its founder included: every node that is a
/// founder or was admitted. `None` if the observation lists no nodes.
pub(crate) fn on_roll(observed: &Value) -> Option<u64> {
    let nodes = observed.get("nodes")?.as_array()?;
    let counted = nodes.iter().filter(|node| {
        let founder = node.get("founder").and_then(Value::as_bool) == Some(true);
        let admitted = node.get("admitted").is_some_and(|epoch| !epoch.is_null());
        founder || admitted
    });
    u64::try_from(counted.count()).ok()
}

/// The line for the epoch an observation was written in, if it says enough.
pub(crate) fn sample_of(observed: &Value) -> Option<Sample> {
    Some(Sample {
        epoch: observed.get("epoch")?.as_u64()?,
        as_of: observed.get("as_of")?.as_u64()?,
        roll: on_roll(observed)?,
        answering: observed
            .pointer("/status/answering")
            .and_then(Value::as_u64),
        saying: None,
        tor: None,
        site_node_running: observed.get("running").and_then(Value::as_bool) == Some(true),
    })
}

/// The line to write when the observation `previous` (as written) is replaced by one in
/// `epoch`: its own epoch's, if that has ended.
pub(crate) fn ended(previous: &[u8], epoch: u64) -> Option<Sample> {
    let observed: Value = serde_json::from_slice(previous).ok()?;
    sample_of(&observed).filter(|sample| sample.epoch < epoch)
}

/// Every line that reads, one per epoch, oldest first. A missing or unreadable file is
/// an empty history.
pub(crate) fn read(path: &Path) -> Vec<Sample> {
    let bytes = std::fs::read(path).unwrap_or_default();
    let mut samples = parse(&bytes).0;
    samples.sort_by_key(|sample| sample.epoch);
    // The first line written for an epoch is the one kept; a second cannot be written
    // by `record`, but a file can be copied together by hand.
    samples.dedup_by_key(|sample| sample.epoch);
    samples
}

/// The lines that read, and whether every line did and the file ends where a line does.
fn parse(bytes: &[u8]) -> (Vec<Sample>, bool) {
    let mut whole = bytes.is_empty() || bytes.ends_with(b"\n");
    let mut samples = Vec::new();
    for line in bytes.split(|byte| *byte == b'\n') {
        if line.is_empty() {
            continue;
        }
        match serde_json::from_slice(line) {
            Ok(sample) => samples.push(sample),
            Err(_) => whole = false,
        }
    }
    (samples, whole)
}

/// Write `sample` down at `path` unless its epoch, or the one after it, already is.
/// Lines for epochs later still are dropped. True if it was written.
///
/// # Errors
/// Fails if the file can be neither appended to nor replaced.
pub(crate) fn record(path: &Path, sample: &Sample) -> std::io::Result<bool> {
    let bytes = read_to_record(path);
    let (mut samples, whole) = parse(&bytes);
    let lines = samples.len();
    // A line more than one epoch past this one was written while the clock ran ahead,
    // as it can at boot before it is set. Kept, it would refuse every epoch until the
    // clock reached it again; it is dropped instead.
    samples.retain(|kept| kept.epoch <= sample.epoch.saturating_add(1));
    let ahead = lines - samples.len();
    if samples.iter().any(|kept| kept.epoch >= sample.epoch) {
        return Ok(false);
    }
    let mut line = serde_json::to_vec(sample).map_err(std::io::Error::other)?;
    line.push(b'\n');
    if whole && ahead == 0 && !bytes.is_empty() && samples.len() < KEEP {
        let mut file = std::fs::OpenOptions::new().append(true).open(path)?;
        file.write_all(&line)?;
        file.sync_data()?;
        return Ok(true);
    }
    if !whole {
        tracing::warn!(
            "{} had lines that do not read; they are dropped",
            path.display()
        );
    }
    if ahead > 0 {
        tracing::warn!(
            "{} had {ahead} lines for epochs not yet reached; they are dropped",
            path.display()
        );
    }
    samples.sort_by_key(|kept| kept.epoch);
    let keep_from = samples.len().saturating_sub(KEEP - 1);
    let mut out = Vec::with_capacity(bytes.len() + line.len());
    for kept in samples.iter().skip(keep_from) {
        out.extend(serde_json::to_vec(kept).map_err(std::io::Error::other)?);
        out.push(b'\n');
    }
    out.extend(line);
    atomic::write(path, &out, MODE)?;
    Ok(true)
}

/// The file `record` adds to, or nothing if it is not there or cannot be read.
fn read_to_record(path: &Path) -> Vec<u8> {
    match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) => {
            if error.kind() != std::io::ErrorKind::NotFound {
                tracing::warn!(
                    "{} could not be read, so it starts again: {error}",
                    path.display()
                );
            }
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("n333-site-history-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn sample(epoch: u64) -> Sample {
        Sample {
            epoch,
            as_of: epoch * 19_980_000,
            roll: epoch % 7,
            answering: Some(epoch % 5),
            saying: None,
            tor: None,
            site_node_running: true,
        }
    }

    #[test]
    fn an_observation_gives_numbers_only_and_only_once_its_epoch_has_ended() {
        let previous = serde_json::json!({
            "format": 1, "as_of": 5_000, "epoch": 41, "running": true,
            "invitation": "333:192.0.2.7:3333",
            "status": { "answering": 3, "roll": 2 },
            "nodes": [
                { "id": "f", "founder": true, "admitted": null },
                { "id": "a", "admitted": 30 },
                { "id": "b", "admitted": 31 },
                { "id": "s", "admitted": null },
            ],
        })
        .to_string();
        assert_eq!(ended(previous.as_bytes(), 41), None);
        let sample = ended(previous.as_bytes(), 42).unwrap();
        assert_eq!(
            sample,
            Sample {
                epoch: 41,
                as_of: 5_000,
                roll: 3,
                answering: Some(3),
                saying: None,
                tor: None,
                site_node_running: true,
            }
        );
        let line = serde_json::to_string(&sample).unwrap();
        assert_eq!(
            line,
            r#"{"epoch":41,"as_of":5000,"roll":3,"answering":3,"site_node_running":true}"#
        );
        assert_eq!(ended(b"not json", 42), None);
    }

    #[test]
    fn one_line_per_epoch_is_appended() {
        let path = dir("append").join(FILE);
        assert!(record(&path, &sample(10)).unwrap());
        assert!(record(&path, &sample(11)).unwrap());
        assert!(
            !record(&path, &sample(11)).unwrap(),
            "an epoch is written once"
        );
        assert!(
            !record(&path, &sample(10)).unwrap(),
            "never one older than the newest"
        );
        let text = std::fs::read_to_string(&path).unwrap();
        assert_eq!(text.lines().count(), 2);
        assert_eq!(read(&path), vec![sample(10), sample(11)]);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
            assert_eq!(mode, MODE);
        }
    }

    #[test]
    fn lines_from_a_clock_that_ran_ahead_do_not_block_later_epochs() {
        let path = dir("ahead").join(FILE);
        assert!(record(&path, &sample(10)).unwrap());
        assert!(record(&path, &sample(90)).unwrap());
        assert!(record(&path, &sample(11)).unwrap());
        assert_eq!(read(&path), vec![sample(10), sample(11)]);
    }

    #[test]
    fn past_the_limit_the_newest_lines_are_kept() {
        let path = dir("trim").join(FILE);
        let full: String = (1..=KEEP as u64)
            .map(|epoch| serde_json::to_string(&sample(epoch)).unwrap() + "\n")
            .collect();
        std::fs::write(&path, full).unwrap();
        assert!(record(&path, &sample(KEEP as u64 + 1)).unwrap());
        let kept = read(&path);
        assert_eq!(kept.len(), KEEP);
        assert_eq!(kept.first().unwrap().epoch, 2);
        assert_eq!(kept.last().unwrap().epoch, KEEP as u64 + 1);
    }

    #[test]
    fn a_broken_file_is_read_around_and_rewritten() {
        let path = dir("broken").join(FILE);
        let good = serde_json::to_string(&sample(5)).unwrap();
        std::fs::write(&path, format!("{good}\n\u{0}garbage\n{{\"epoch\":6,\"as_o")).unwrap();
        assert_eq!(read(&path), vec![sample(5)]);
        assert!(record(&path, &sample(7)).unwrap());
        assert_eq!(read(&path), vec![sample(5), sample(7)]);
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.ends_with('\n') && !text.contains("garbage"), "{text}");

        let missing = dir("missing").join(FILE);
        assert!(read(&missing).is_empty());
        assert!(record(&missing, &sample(1)).unwrap());
        assert_eq!(read(&missing), vec![sample(1)]);
    }
}
