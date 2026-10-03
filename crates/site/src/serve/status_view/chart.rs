//! One chart of the history: nodes on the roll and nodes answering, per epoch, as an
//! inline SVG the server draws, with the same numbers written out beneath it.
//!
//! No script and no `style` attribute: the content security policy allows neither
//! inline, so the lines take their look from classes in `status.css`. The picture is
//! for the eye; the summary under it is what a reader without it, or a crawler, gets,
//! and the two say the same.
//!
//! An epoch the observer did not see is a gap in the line, not a line drawn across it:
//! a straight segment would claim numbers nobody wrote down.

use std::fmt::Write as _;
use std::time::{Duration, UNIX_EPOCH};

use n333_core::Epoch;

use crate::history::Sample;
use crate::serve::template::html_escaped;
use crate::words::{Arg, Speaker};

/// The drawing's size in its own units.
const WIDTH: u64 = 560;
/// The drawing's height in its own units.
const HEIGHT: u64 = 252;
/// The plot's left edge; the numbers of the vertical axis stand left of it.
const LEFT: u64 = 44;
/// The plot's right edge.
const RIGHT: u64 = 548;
/// The plot's top edge.
const TOP: u64 = 14;
/// The plot's bottom edge, the zero line; epochs and dates stand under it.
const BOTTOM: u64 = 192;

/// One series: its class, and its number in a sample.
type Series = (&'static str, fn(&Sample) -> Option<u64>);

/// The two series, the roll first.
const SERIES: [Series; 2] = [
    ("roll", |sample| Some(sample.roll)),
    ("answering", |sample| sample.answering),
];

/// The chart, its key and its summary for `samples` (oldest first, one per epoch), or a
/// sentence saying there is not enough to draw. `id` keeps two charts' labels apart;
/// `title` is the message naming the chart.
pub(super) fn html(id: &str, title: &str, samples: &[Sample], words: &Speaker<'_>) -> String {
    let (Some(first), Some(last)) = (samples.first(), samples.last()) else {
        return too_few(words);
    };
    if samples.len() < 2 || first.epoch >= last.epoch {
        return too_few(words);
    }
    let high = samples
        .iter()
        .flat_map(|sample| SERIES.iter().filter_map(|(_, value)| value(sample)))
        .max()
        .unwrap_or(0)
        .max(1);
    let scale = Scale {
        first: first.epoch,
        span: last.epoch - first.epoch,
        high,
    };
    let mut svg = format!(
        r#"<svg viewBox="0 0 {WIDTH} {HEIGHT}" role="img" aria-labelledby="{id}-title {id}-sum"><title id="{id}-title">{}</title>"#,
        html_escaped(&words.word(title))
    );
    axes(&mut svg, &scale, last.epoch);
    for (class, value) in SERIES {
        line(&mut svg, &scale, samples, class, value);
    }
    svg.push_str("</svg>");
    format!(
        r#"<figure class="chart">{}{svg}<figcaption id="{id}-sum">{}</figcaption></figure>"#,
        key(words),
        html_escaped(&summary(samples, words))
    )
}

/// Said instead of a chart that would be a dot or nothing.
fn too_few(words: &Speaker<'_>) -> String {
    format!(
        r#"<p class="dim">{}</p>"#,
        html_escaped(&words.word("status-chart-too-few"))
    )
}

/// Which line is which, in words beside a sample of each line.
fn key(words: &Speaker<'_>) -> String {
    format!(
        r#"<ul class="chart-key"><li><i class="k roll"></i>{}</li><li><i class="k answering"></i>{}</li></ul>"#,
        html_escaped(&words.word("status-roll")),
        html_escaped(&words.word("figure-answering"))
    )
}

/// From epochs and counts to the drawing's units.
struct Scale {
    /// The first epoch drawn, at the left edge.
    first: u64,
    /// How many epochs after it the last one is, at the right edge; at least 1.
    span: u64,
    /// The count at the top edge; at least 1.
    high: u64,
}

impl Scale {
    /// Across, for an epoch.
    fn x(&self, epoch: u64) -> u64 {
        LEFT + (epoch - self.first) * (RIGHT - LEFT) / self.span
    }

    /// Down, for a count.
    fn y(&self, count: u64) -> u64 {
        BOTTOM - count.min(self.high) * (BOTTOM - TOP) / self.high
    }
}

/// The zero line and the top line with their counts, and the first, middle and last
/// epochs with the day each began.
fn axes(svg: &mut String, scale: &Scale, last: u64) {
    for count in [0, scale.high] {
        let y = scale.y(count);
        let _ = write!(
            svg,
            r#"<line class="grid" x1="{LEFT}" y1="{y}" x2="{RIGHT}" y2="{y}"></line><text class="tick" x="{}" y="{}" text-anchor="end">{count}</text>"#,
            LEFT - 8,
            y + 4
        );
    }
    let mut ticks = vec![(scale.first, "start")];
    if scale.span >= 2 {
        ticks.push((scale.first + scale.span / 2, "middle"));
    }
    ticks.push((last, "end"));
    for (epoch, anchor) in ticks {
        let x = scale.x(epoch);
        let _ = write!(
            svg,
            r#"<text class="tick" x="{x}" y="{}" text-anchor="{anchor}">{epoch}</text><text class="tick" x="{x}" y="{}" text-anchor="{anchor}">{}</text>"#,
            BOTTOM + 24,
            BOTTOM + 50,
            day(epoch)
        );
    }
}

/// One series: a path through each run of consecutive epochs, a dot for a run of one.
fn line(
    svg: &mut String,
    scale: &Scale,
    samples: &[Sample],
    class: &str,
    value: fn(&Sample) -> Option<u64>,
) {
    let mut runs: Vec<Vec<(u64, u64)>> = Vec::new();
    let mut previous: Option<u64> = None;
    for sample in samples {
        let Some(count) = value(sample) else {
            previous = None;
            continue;
        };
        let point = (scale.x(sample.epoch), scale.y(count));
        match runs.last_mut() {
            Some(run) if previous.is_some_and(|epoch| epoch + 1 == sample.epoch) => {
                run.push(point);
            }
            _ => runs.push(vec![point]),
        }
        previous = Some(sample.epoch);
    }
    let mut path = String::new();
    for run in &runs {
        match run.as_slice() {
            [(x, y)] => {
                let _ = write!(
                    svg,
                    r#"<circle class="point {class}" cx="{x}" cy="{y}" r="3"></circle>"#
                );
            }
            points => {
                for (at, (x, y)) in points.iter().enumerate() {
                    let _ = write!(path, "{}{x} {y}", if at == 0 { "M" } else { "L" });
                }
            }
        }
    }
    if !path.is_empty() {
        let _ = write!(svg, r#"<path class="series {class}" d="{path}"></path>"#);
    }
}

/// The day an epoch began, `YYYY-MM-DD`, UTC.
pub(super) fn day(epoch: u64) -> String {
    let began = UNIX_EPOCH + Duration::from_secs(Epoch(epoch).starts_at_unix_seconds());
    let mut at = humantime::format_rfc3339_seconds(began).to_string();
    at.truncate(10);
    at
}

/// The chart's numbers in one sentence: the epochs and days it covers, and each series'
/// lowest, highest and latest count.
fn summary(samples: &[Sample], words: &Speaker<'_>) -> String {
    let first = samples.first().map_or(0, |sample| sample.epoch);
    let last = samples.last().map_or(0, |sample| sample.epoch);
    let mut args = vec![
        ("first", Arg::Text(first.to_string())),
        ("last", Arg::Text(last.to_string())),
        ("from", Arg::Text(day(first))),
        ("to", Arg::Text(day(last))),
    ];
    for ((low, high, latest), (_, value)) in [
        ("roll_low", "roll_high", "roll_latest"),
        ("answering_low", "answering_high", "answering_latest"),
    ]
    .into_iter()
    .zip(SERIES)
    {
        let counts: Vec<u64> = samples.iter().filter_map(value).collect();
        let shown = |count: Option<&u64>| {
            Arg::Text(count.map_or_else(|| "\u{2014}".to_owned(), u64::to_string))
        };
        args.push((low, shown(counts.iter().min())));
        args.push((high, shown(counts.iter().max())));
        args.push((latest, shown(counts.last())));
    }
    words.say("status-chart-summary", &args).unwrap_or_default()
}
