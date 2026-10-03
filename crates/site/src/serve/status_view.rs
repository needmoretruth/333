//! The status page: how the network looks now, how it has been epoch by epoch, and
//! whether this machine has kept running, all in the page before any script runs.
//! `/api/status` gives the same [`Status`] as JSON.
//!
//! "On the roll" here counts the founder too (see [`history::on_roll`]), so that the
//! number now and the numbers in the history are counted the same way. The other pages
//! show the site node's own `status.roll`, which leaves the founder out.

mod chart;

use std::fmt::Write as _;
use std::time::{Duration, UNIX_EPOCH};

use n333_core::Epoch;
use serde::Serialize;
use serde_json::Value;

use super::counted::Where;
use super::machine;
use super::state::State;
use super::template::{Values, html_escaped};
use super::values;
use crate::history::{self, Sample};
use crate::words::{Arg, Speaker};

/// What stands in for a value nobody can produce.
const UNKNOWN: &str = "\u{2014}";

/// How many epochs the first chart shows: the evaluation window.
const RECENT: u64 = 333;

/// Where a commit is shown.
const COMMITS: &str = "https://github.com/needmoretruth/333/commit/";

/// Everything the page shows.
#[derive(Debug, Serialize)]
pub(crate) struct Status {
    /// When this was put together, in milliseconds.
    as_of: u64,
    /// The network now.
    now: Now,
    /// This site's machine.
    machine: Machine,
    /// One line per epoch, oldest first.
    history: Vec<Sample>,
}

/// The network now, as the newest observation and the board say.
#[derive(Debug, Serialize)]
struct Now {
    /// The epoch.
    epoch: u64,
    /// Which epoch of this line it is, counting its first as 1.
    line_epoch: Option<u64>,
    /// When it ends, in milliseconds.
    epoch_ends: u64,
    /// Nodes on the roll, its founder included.
    roll: Option<u64>,
    /// The site node's count of nodes answering.
    answering: Option<u64>,
    /// Statements on the board.
    saying: usize,
    /// Of those, the ones naming an onion address.
    tor: usize,
    /// Whether the site node is running, as a fresh observation says; false when the
    /// observation is missing or older than [`values::STALE_AFTER_MS`].
    site_node_running: bool,
}

/// This site's machine.
#[derive(Debug, Serialize)]
struct Machine {
    /// The version pages show.
    version: String,
    /// The whole commit it was built from.
    commit: Option<String>,
    /// When it was deployed, in milliseconds.
    deployed_at: Option<u64>,
    /// How old the newest observation is, in seconds.
    observation_age: Option<u64>,
    /// Whether that observation says the site node was running, however old it is.
    observed_running: Option<bool>,
    /// How long the machine has been up, in seconds, where the system says.
    uptime: Option<u64>,
}

impl Status {
    /// Put it together from the observation as written, the board's counts and the
    /// history file, as of `now_ms`.
    pub(crate) fn gather(state: &State, written: &[u8], board: &Where, now_ms: u64) -> Self {
        let raw: Option<Value> = serde_json::from_slice(written).ok();
        let fresh = raw.clone().map(|mut observed| {
            values::mark_if_stale(&mut observed, now_ms);
            observed
        });
        let epoch = Epoch::at_unix_seconds(now_ms / 1000).0;
        let flag = |observed: Option<&Value>| {
            observed
                .and_then(|observed| observed.get("running"))
                .and_then(Value::as_bool)
        };
        Self {
            as_of: now_ms,
            now: Now {
                epoch,
                line_epoch: values::line_number(raw.as_ref(), epoch),
                epoch_ends: Epoch(epoch + 1)
                    .starts_at_unix_seconds()
                    .saturating_mul(1000),
                roll: raw.as_ref().and_then(history::on_roll),
                answering: raw
                    .as_ref()
                    .and_then(|observed| observed.pointer("/status/answering"))
                    .and_then(Value::as_u64),
                saying: board.saying,
                tor: board.tor,
                site_node_running: flag(fresh.as_ref()) == Some(true),
            },
            machine: Machine {
                version: state.version.clone(),
                commit: state.release.commit.clone(),
                deployed_at: state.release.deployed_at,
                observation_age: raw
                    .as_ref()
                    .and_then(|observed| observed.get("as_of"))
                    .and_then(Value::as_u64)
                    .map(|as_of| now_ms.saturating_sub(as_of) / 1000),
                observed_running: flag(raw.as_ref()),
                uptime: machine::uptime(),
            },
            history: history::read(&state.history),
        }
    }

    /// As `/api/status` gives it.
    pub(crate) fn json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "null".to_owned())
    }
}

/// `{{status_roll}}`, `{{status_tor}}` and every `{{html:status_…}}` the page asks for.
pub(crate) fn fill(values: &mut Values, status: &Status, words: &Speaker<'_>) {
    let now = &status.now;
    values.text(
        "status_roll",
        now.roll
            .map_or_else(|| UNKNOWN.to_owned(), |roll| roll.to_string()),
    );
    values.text("status_tor", now.tor.to_string());
    let recent: Vec<Sample> = status
        .history
        .iter()
        .filter(|sample| sample.epoch + RECENT > now.epoch)
        .cloned()
        .collect();
    values.html(
        "status_chart_recent",
        chart::html("recent", "status-chart-recent-title", &recent, words),
    );
    values.html(
        "status_chart_all",
        chart::html("all", "status-chart-all-title", &status.history, words),
    );
    machine_rows(values, &status.machine, words);
}

/// The machine's rows.
fn machine_rows(values: &mut Values, machine: &Machine, words: &Speaker<'_>) {
    let version = html_escaped(&machine.version);
    values.html(
        "status_release",
        machine.commit.as_ref().map_or_else(
            || format!("<code>{version}</code>"),
            |commit| format!(r#"<a href="{COMMITS}{commit}"><code>{version}</code></a>"#),
        ),
    );
    values.html("status_deployed", time(machine.deployed_at));
    let observed = machine.observation_age.map_or_else(
        || UNKNOWN.to_owned(),
        |seconds| {
            let age = words
                .say("status-age", &[("seconds", Arg::Count(seconds))])
                .unwrap_or_default();
            let said = match machine.observed_running {
                Some(true) => words.word("status-observed-running"),
                _ => words.word("status-observed-not-running"),
            };
            format!("{} {}", html_escaped(&age), html_escaped(&said))
        },
    );
    values.html("status_observed", observed);
    let mut uptime = String::new();
    if let Some(seconds) = machine.uptime {
        let up = words
            .say(
                "status-uptime-value",
                &[
                    ("days", Arg::Count(seconds / 86_400)),
                    ("hours", Arg::Count(seconds % 86_400 / 3_600)),
                ],
            )
            .unwrap_or_default();
        let _ = write!(
            uptime,
            "<div><dt>{}</dt><dd>{}</dd></div>",
            html_escaped(&words.word("status-uptime")),
            html_escaped(&up)
        );
    }
    values.html("status_uptime", uptime);
}

/// A moment as `<time>`, ISO 8601 UTC to the second, or a dash.
fn time(ms: Option<u64>) -> String {
    ms.map_or_else(
        || UNKNOWN.to_owned(),
        |ms| {
            let at = humantime::format_rfc3339_seconds(UNIX_EPOCH + Duration::from_millis(ms));
            format!(r#"<time datetime="{at}">{at}</time>"#)
        },
    )
}

#[cfg(test)]
mod tests;
