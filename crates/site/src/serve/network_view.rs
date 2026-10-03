//! The network page's table of every node, written into the page the way `network.js`
//! draws it, so the page says everything before any script runs. The graph itself is a
//! canvas and stays the script's; everything it shows is in this table and the report.
//!
//! One thing only a browser knows is left out: which node is "yours", which a visitor
//! marks on their own device.

mod report;

use std::time::{Duration, UNIX_EPOCH};

use serde_json::Value;

use super::template::{Values, html_escaped};
use crate::words::{Arg, Speaker};

/// How many digits of a name the table shows.
const SHORT: usize = 12;

/// What stands in for a value nobody can produce.
const UNKNOWN: &str = "\u{2014}";

/// `{{html:network_rows}}`, `{{html:network_report}}`, `{{html:network_empty}}` and
/// `{{html:asof}}`, from the observation and its file as written.
pub(crate) fn fill(
    values: &mut Values,
    observed: Option<&Value>,
    written: &[u8],
    words: &Speaker<'_>,
) {
    let nodes: Vec<&Value> = observed
        .and_then(|observed| observed.get("nodes"))
        .and_then(Value::as_array)
        .map(|nodes| nodes.iter().collect())
        .unwrap_or_default();
    let epoch = observed
        .and_then(|observed| observed.get("epoch"))
        .and_then(Value::as_u64);
    let running = observed
        .and_then(|observed| observed.get("running"))
        .and_then(Value::as_bool)
        == Some(true);
    values.html("network_rows", rows(&nodes, epoch, running, words));
    values.html("network_report", report::html(written, words));
    values.html(
        "network_empty",
        if nodes.len() <= 1 {
            format!(
                r#"<p class="sky-empty">{}</p>"#,
                html_escaped(&words.word("js-network-empty"))
            )
        } else {
            r#"<p class="sky-empty" hidden></p>"#.to_owned()
        },
    );
    let as_of = observed
        .and_then(|observed| observed.get("as_of"))
        .and_then(Value::as_u64)
        .map(|ms| humantime::format_rfc3339_seconds(UNIX_EPOCH + Duration::from_millis(ms)));
    values.html(
        "asof",
        as_of.map_or_else(
            || format!("<time data-asof>{UNKNOWN}</time>"),
            |at| {
                let at = at.to_string();
                let shown = at.replace('T', " ").replace('Z', " UTC");
                format!(r#"<time data-asof datetime="{at}">{shown}</time>"#)
            },
        ),
    );
}

/// What one node is this epoch, as `stateOf` in `network.js` says it.
fn state_of(node: &Value, epoch: Option<u64>) -> &'static str {
    let flag = |name: &str| node.get(name).and_then(Value::as_bool) == Some(true);
    if flag("founder") {
        return "founder";
    }
    if node.get("admitted").is_none_or(Value::is_null) {
        return "seen";
    }
    let counts_from = node.get("counts_from").and_then(Value::as_u64);
    if counts_from.zip(epoch).is_some_and(|(from, now)| from > now) {
        return "later";
    }
    if flag("answered_now") { "ok" } else { "quiet" }
}

/// The state cell: a dot and its words. The site node's own is whether it runs.
fn state_line(node: &Value, epoch: Option<u64>, running: bool, words: &Speaker<'_>) -> String {
    let (dot, key) = if node.get("site").and_then(Value::as_bool) == Some(true) {
        if running {
            ("ok", "js-network-awake")
        } else {
            ("bad", "js-network-not-running")
        }
    } else {
        let state = state_of(node, epoch);
        (state, state_key(state))
    };
    format!(
        r#"<span class="state"><span class="dot {dot}"></span>{}</span>"#,
        html_escaped(&words.word(key))
    )
}

/// The message for a state.
fn state_key(state: &str) -> &'static str {
    match state {
        "founder" => "js-network-state-founder",
        "ok" => "js-network-state-ok",
        "quiet" => "js-network-state-quiet",
        "later" => "js-network-state-later",
        _ => "js-network-state-seen",
    }
}

/// The table's rows: founders first, then by the epoch each was admitted in.
fn rows(nodes: &[&Value], epoch: Option<u64>, running: bool, words: &Speaker<'_>) -> String {
    let mut order = nodes.to_vec();
    order.sort_by_key(|node| {
        let founder = node.get("founder").and_then(Value::as_bool) == Some(true);
        (!founder, number(node, "admitted").unwrap_or(u64::MAX))
    });
    order
        .into_iter()
        .map(|node| row(node, epoch, running, words))
        .collect()
}

/// One of a node's numbers, if it has it.
fn number(node: &Value, name: &str) -> Option<u64> {
    node.get(name).and_then(Value::as_u64)
}

/// `epoch N`, or a dash.
fn in_epoch(value: Option<u64>, words: &Speaker<'_>) -> String {
    value.map_or_else(
        || UNKNOWN.to_owned(),
        |epoch| {
            let epoch = Arg::Text(epoch.to_string());
            let said = words.say("js-network-epoch", &[("epoch", epoch)]);
            html_escaped(&said.unwrap_or_default())
        },
    )
}

/// One node's row.
fn row(node: &Value, epoch: Option<u64>, running: bool, words: &Speaker<'_>) -> String {
    let id = html_escaped(node.get("id").and_then(Value::as_str).unwrap_or_default());
    let short = id.get(..SHORT).unwrap_or(&id);
    let flag = |name: &str| node.get(name).and_then(Value::as_bool) == Some(true);
    let tags: Vec<String> = [
        ("founder", "js-network-tag-founder"),
        ("site", "js-network-tag-site"),
    ]
    .into_iter()
    .filter(|(name, _)| flag(name))
    .map(|(_, key)| html_escaped(&words.word(key)))
    .collect();
    let tags = if tags.is_empty() {
        String::new()
    } else {
        format!(r#"<span class="tag">{}</span>"#, tags.join(" · "))
    };
    let sponsored = node
        .get("sponsor")
        .is_some_and(|sponsor| !sponsor.is_null());
    let given = in_epoch(number(node, "admitted").filter(|_| sponsored), words);
    let last = [
        number(node, "last_answered"),
        number(node, "last_heartbeat"),
    ]
    .into_iter()
    .flatten()
    .max();
    let signal = number(node, "signal").map_or_else(|| UNKNOWN.to_owned(), |s| s.to_string());
    let reached = match node.get("reach").and_then(Value::as_str) {
        Some("direct") => html_escaped(&words.word("js-network-reach-direct")),
        Some("onion") => html_escaped(&words.word("js-network-reach-tor-short")),
        _ => UNKNOWN.to_owned(),
    };
    format!(
        "<tr>\n      <td><button class=\"name\" type=\"button\" data-pick=\"{id}\">{short}</button>{tags}</td>\n      \
         <td>{}</td>\n      <td>{given}</td>\n      <td>{}</td>\n      <td>{}</td>\n      \
         <td>{signal}</td>\n      <td>{reached}</td>\n    </tr>",
        state_line(node, epoch, running, words),
        in_epoch(number(node, "counts_from"), words),
        in_epoch(last, words),
    )
}
