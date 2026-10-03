//! The live values a page is rendered with.
//!
//! | token | value |
//! |---|---|
//! | `version` | what `--version` or `--version-file` said (`dev` without either) |
//! | `epoch` | the current epoch number |
//! | `epoch_ends_utc` | when it ends, ISO 8601 UTC to the second |
//! | `saying` | how many statements the board holds now |
//! | `answering`, `roll` | the site node's `status.answering` and `status.roll` |
//! | `site_node`, `site_node_short` | the site node's name, whole and its first 12 digits |
//! | `invitation` | `333:host:port` from the site node's own statement, empty if unknown |
//! | `node_running` | `true` or `false` |
//! | `site_dot` | `ok` when the node is running, otherwise `bad` |
//! | `site_state` | `This site's node is awake`, or `This site's node is not running` |
//! | `json:network` | the observation as written by `observe` |
//! | `json:board` | the same list `/api/board` gives |
//!
//! AN OBSERVATION THAT CANNOT BE READ DOES NOT BREAK A PAGE. Every value taken from it
//! becomes an em dash and `json:network` becomes `null`, except `site_dot` and
//! `site_state`, which then say the node is not running: nothing shows that it is.
//!
//! AN OBSERVATION OLDER THAN 90 SECONDS SAYS NOTHING ABOUT NOW. The observer writes every
//! 15 seconds; one that has stopped would leave `running: true` up for ever. So an
//! observation whose `as_of` is older than that (or missing) is read with `running` set
//! to false and `"stale": true` added, here and in what `/api/network` returns.

use std::path::Path;
use std::time::{Duration, UNIX_EPOCH};

use n333_core::Epoch;
use serde::Serialize;
use serde_json::Value;

use super::template::Values;
use crate::board::Board;
use crate::place::Place;

/// What stands in for a value nobody can produce.
const UNKNOWN: &str = "\u{2014}";

/// How many digits of a name are enough to tell nodes apart on a page.
const SHORT: usize = 12;

/// How old an observation may be, in milliseconds, before it no longer shows the node
/// running: six of the observer's 15-second runs.
pub(crate) const STALE_AFTER_MS: u64 = 90_000;

/// The observation `observe` wrote, if it can be read as JSON, read as of `now_ms`.
pub(crate) fn observation(path: &Path, now_ms: u64) -> Option<Value> {
    let text = std::fs::read(path).ok()?;
    let mut observed = serde_json::from_slice(&text).ok()?;
    mark_if_stale(&mut observed, now_ms);
    Some(observed)
}

/// If the observation is older than [`STALE_AFTER_MS`] at `now_ms`, or does not say
/// when it was made, set `running` to false and add `"stale": true`. True if it did.
pub(crate) fn mark_if_stale(observed: &mut Value, now_ms: u64) -> bool {
    let fresh = observed
        .get("as_of")
        .and_then(Value::as_u64)
        .is_some_and(|as_of| now_ms.saturating_sub(as_of) <= STALE_AFTER_MS);
    let Some(fields) = observed.as_object_mut() else {
        return false;
    };
    if fresh {
        return false;
    }
    fields.insert("running".to_owned(), Value::Bool(false));
    fields.insert("stale".to_owned(), Value::Bool(true));
    true
}

/// One verified statement, for a browser.
#[derive(Serialize)]
struct Shown<'a> {
    /// The node's name.
    node: &'a str,
    /// Where it said to look.
    address: &'a str,
    /// The epoch it said it in.
    epoch: u64,
    /// The country the edge placed it in, if it did.
    country: Option<&'a str>,
    /// Whether the statement names an onion address.
    tor: bool,
}

/// The live statements whose signatures check out, as `/api/board` gives them.
pub(crate) fn board_json(board: &Board, now_ms: u64) -> String {
    let shown: Vec<Shown<'_>> = board
        .alive(now_ms)
        .filter_map(|line| {
            let said = line.said()?;
            Some(Shown {
                node: &said.node,
                address: &said.address,
                epoch: said.epoch,
                country: match line.place() {
                    Some(Place::At { country, .. }) => Some(country),
                    _ => None,
                },
                tor: line.place() == Some(&Place::Tor),
            })
        })
        .collect();
    serde_json::to_string(&shown).unwrap_or_else(|_| "[]".to_owned())
}

/// Everything a page can ask for.
pub(crate) fn for_page(
    version: &str,
    observed: Option<&Value>,
    board: &Board,
    now_ms: u64,
) -> Values {
    let mut values = Values::default();
    let epoch = Epoch::at_unix_seconds(now_ms / 1000);
    let ends = UNIX_EPOCH + Duration::from_secs(Epoch(epoch.0 + 1).starts_at_unix_seconds());
    values.text("version", version);
    values.text("epoch", epoch.0.to_string());
    values.text(
        "epoch_ends_utc",
        humantime::format_rfc3339_seconds(ends).to_string(),
    );
    values.text("saying", board.alive(now_ms).count().to_string());
    values.json("board", board_json(board, now_ms));
    from_observation(&mut values, observed);
    values
}

/// The values that come from the observation.
fn from_observation(values: &mut Values, observed: Option<&Value>) {
    let Some(observed) = observed else {
        for name in [
            "answering",
            "roll",
            "site_node",
            "site_node_short",
            "invitation",
            "node_running",
        ] {
            values.text(name, UNKNOWN);
        }
        running(values, false);
        values.json("network", "null".to_owned());
        return;
    };
    let number = |pointer: &str| {
        observed
            .pointer(pointer)
            .and_then(Value::as_u64)
            .map_or_else(|| UNKNOWN.to_owned(), |count| count.to_string())
    };
    values.text("answering", number("/status/answering"));
    values.text("roll", number("/status/roll"));
    let node = observed.get("site_node").and_then(Value::as_str);
    values.text("site_node", node.unwrap_or(UNKNOWN));
    values.text(
        "site_node_short",
        node.map_or(UNKNOWN, |name| name.get(..SHORT).unwrap_or(name)),
    );
    let invitation = observed.get("invitation").and_then(Value::as_str);
    values.text("invitation", invitation.unwrap_or_default());
    let is_running = observed.get("running").and_then(Value::as_bool);
    values.text(
        "node_running",
        is_running.map_or(UNKNOWN, |yes| if yes { "true" } else { "false" }),
    );
    running(values, is_running == Some(true));
    values.json("network", observed.to_string());
}

/// The two values a page shows the node's state with.
fn running(values: &mut Values, awake: bool) {
    values.text("site_dot", if awake { "ok" } else { "bad" });
    values.text(
        "site_state",
        if awake {
            "This site's node is awake"
        } else {
            "This site's node is not running"
        },
    );
}

#[cfg(test)]
mod tests {
    use super::super::template::render;
    use super::*;

    #[test]
    fn an_observation_that_cannot_be_read_leaves_dashes_and_null() {
        let values = for_page("abc1234", None, &Board::default(), 0);
        let page = render(
            "{{site_node}}|{{node_running}}|{{site_dot}}|{{json:network}}|{{epoch_ends_utc}}",
            &values,
        );
        assert_eq!(page, "\u{2014}|\u{2014}|bad|null|1970-01-01T05:33:00Z");
    }

    #[test]
    fn an_observation_older_than_ninety_seconds_does_not_show_the_node_running() {
        let written = serde_json::json!({ "as_of": 1_000_000, "running": true });
        let read = |now_ms: u64| {
            let mut observed = written.clone();
            mark_if_stale(&mut observed, now_ms);
            let values = for_page("v", Some(&observed), &Board::default(), now_ms);
            render("{{node_running}} {{site_dot}} {{site_state}}", &values)
        };
        assert_eq!(
            read(1_000_000 + STALE_AFTER_MS),
            "true ok This site&#39;s node is awake"
        );
        assert_eq!(
            read(1_000_001 + STALE_AFTER_MS),
            "false bad This site&#39;s node is not running"
        );
    }
}
