//! The map's dots, its table and the line saying when it was read, written into the page
//! the way `map.js` draws them, so the page says everything before any script runs.
//!
//! A country is written as its ISO code (`data-c` holds it too). The page's script names
//! it in the reader's language from the browser's own list (`Intl.DisplayNames`); the
//! server keeps no table of country names, which would be two hundred and fifty names
//! in every language for a column whose codes already say it.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::time::{Duration, UNIX_EPOCH};

use super::counted::Where;
use super::template::{Values, html_escaped};
use crate::words::{Arg, Speaker};

/// How wide the map is in its own units: 360 degrees at four units each.
const UNITS_PER_DEGREE: i32 = 4;

/// `{{html:map_dots}}`, `{{html:map_tor_dots}}`, `{{html:map_rows}}` and `{{read_at}}`.
pub(crate) fn fill(values: &mut Values, found: &Where, words: &Speaker<'_>) {
    values.html("map_dots", dots(found, words));
    values.html("map_tor_dots", tor_dots(found, words));
    values.html("map_rows", rows(found, words));
    let read = UNIX_EPOCH + Duration::from_millis(found.as_of);
    let read = humantime::format_rfc3339_seconds(read).to_string();
    values.text(
        "read_at",
        read.replace('T', " ").trim_end_matches('Z').to_owned(),
    );
}

/// One ring and one dot per place, larger where more nodes share it.
fn dots(found: &Where, words: &Speaker<'_>) -> String {
    let mut seen: BTreeMap<[i32; 2], u64> = BTreeMap::new();
    for dot in &found.dots {
        *seen.entry(*dot).or_default() += 1;
    }
    let mut out = String::new();
    for ([lon, lat], many) in seen {
        let x = (lon + 180) * UNITS_PER_DEGREE;
        let y = (90 - lat) * UNITS_PER_DEGREE;
        out.push_str(&marked(x, y, many, words));
    }
    out
}

/// One ring and one dot per onion statement, in the onion's corner.
fn tor_dots(found: &Where, words: &Speaker<'_>) -> String {
    found
        .tor_dots
        .iter()
        .map(|[x, y]| {
            let (x, y) = (
                i32::try_from(*x).unwrap_or(0),
                i32::try_from(*y).unwrap_or(0),
            );
            marked(x, y, 1, words)
        })
        .collect()
}

/// A ring and a dot at `(x, y)` for `many` nodes, titled with how many.
fn marked(x: i32, y: i32, many: u64, words: &Speaker<'_>) -> String {
    let size = i32::try_from(many.min(9)).unwrap_or(9);
    let title = words
        .say("js-map-dot", &[("count", Arg::Count(many))])
        .unwrap_or_default();
    format!(
        r#"<circle class="ring" cx="{x}" cy="{y}" r="{}"></circle><circle class="dot" cx="{x}" cy="{y}" r="{}"><title>{}</title></circle>"#,
        9 + size * 3,
        5 + size,
        html_escaped(&title)
    )
}

/// The table: each country, Tor, the unplaced, the sum, and the nodes that said nothing.
fn rows(found: &Where, words: &Speaker<'_>) -> String {
    let mut out = String::new();
    let row = |out: &mut String, cell: String, count: usize, class: &str| {
        let _ = write!(
            out,
            r#"<tr{class}><td>{cell}</td><td class="n">{count}</td></tr>"#
        );
    };
    for country in &found.countries {
        let code = html_escaped(&country.c);
        row(
            &mut out,
            format!(r#"<span data-c="{code}">{code}</span>"#),
            country.n,
            "",
        );
    }
    let word = |key: &str| html_escaped(&words.word(key));
    if found.tor > 0 {
        row(&mut out, word("js-map-tor"), found.tor, "");
    }
    if found.unplaced > 0 {
        row(&mut out, word("js-map-nowhere"), found.unplaced, "");
    }
    if found.saying == 0 {
        let _ = write!(
            out,
            r#"<tr><td class="dim" colspan="2">{}</td></tr>"#,
            word("js-map-nobody")
        );
    } else {
        row(
            &mut out,
            word("js-map-all"),
            found.saying,
            r#" class="sum""#,
        );
    }
    if let Some(unsaid) = found.unsaid {
        row(
            &mut out,
            word("js-map-unsaid"),
            unsaid,
            r#" class="unsaid""#,
        );
    }
    out
}
