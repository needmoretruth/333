//! The two lines along the bottom — whether anybody is here, and what the keys do —
//! kept apart because they change with the verdict and with what is being typed, not
//! with anything in the columns above them.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use n333_core::epoch;
use n333_core::extinction::{Remaining, Verdict};
use n333_core::signal::SIGNAL_COUNT;

use unicode_width::UnicodeWidthStr as _;

use super::MARK;
use crate::screen::Saying;
use crate::screen::watch::Watch;
use crate::words::Arg;

/// Whether anybody is here, and what is left if nobody is.
pub(super) fn the_silence<'a>(watch: &'a Watch, width: u16) -> Paragraph<'a> {
    // The long form of each sentence when all of it fits, since a sentence cut off at
    // the edge says less than the short form does.
    let wide = usize::from(width) >= 3 + wide_verdict(watch).width();
    let (said, mark, style) = match watch.vigil.verdict() {
        Verdict::NothingToSay => (
            if wide {
                words!("screen-draw-bottom-never-answered-wide")
            } else {
                words!("screen-draw-bottom-never-answered")
            },
            None,
            Style::new().fg(Color::DarkGray),
        ),
        Verdict::Alive => (
            if wide {
                words!("screen-draw-bottom-alive-wide")
            } else {
                words!("screen-draw-bottom-alive")
            },
            Some(Color::Green),
            Style::new().fg(Color::DarkGray),
        ),
        Verdict::Waiting { silent, needed } => (
            if wide {
                words!(
                    "screen-draw-bottom-waiting-wide",
                    silent = silent,
                    needed = needed
                )
            } else {
                words!(
                    "screen-draw-bottom-waiting",
                    silent = silent,
                    needed = needed
                )
            },
            Some(Color::Yellow),
            Style::new(),
        ),
        Verdict::Ended { since } => (
            match watch.vigil.remaining_at(epoch::unix_now_seconds()) {
                Some(Remaining { years, days }) => words!(
                    "screen-draw-bottom-ended",
                    since = since.0,
                    years = Arg::grouped(years),
                    days = days
                ),
                None => words!("screen-draw-bottom-ended-and-gone", since = since.0),
            },
            Some(Color::Red),
            Style::new().add_modifier(Modifier::BOLD),
        ),
    };
    // A line broken over several in its catalog, to keep the file readable, is one
    // line here, after the one space the bottom lines start with. The dot, when there
    // is one, only says that this line changed colour; the words say what it means.
    let mut line = vec![Span::raw(" ")];
    if let Some(colour) = mark {
        line.push(Span::styled(MARK, Style::new().fg(colour)));
    }
    line.push(Span::styled(said.replace('\n', " "), style));
    Paragraph::new(Line::from(line))
}

/// The order words, as they are typed. The command line's own, in every language.
const ORDER_WORDS: &str =
    "ping · join · bootstrap · say · tor on · tor off · bridge · status · quit";

/// How wide the long form of the verdict is, to know whether it fits.
fn wide_verdict(watch: &Watch) -> String {
    match watch.vigil.verdict() {
        Verdict::NothingToSay => words!("screen-draw-bottom-never-answered-wide"),
        Verdict::Alive => words!("screen-draw-bottom-alive-wide"),
        Verdict::Waiting { silent, needed } => words!(
            "screen-draw-bottom-waiting-wide",
            silent = silent,
            needed = needed
        ),
        Verdict::Ended { .. } => String::new(),
    }
    .replace('\n', " ")
}

/// What the keys do, or what is being typed.
pub(super) fn the_keys<'a>(watch: &'a Watch, saying: &'a Saying, width: u16) -> Paragraph<'a> {
    let width = usize::from(width);
    if let Saying::Typing(entry) = saying {
        let words = format!("   {ORDER_WORDS}");
        return Paragraph::new(typing(" : ".to_owned(), &entry.typed, width, &words));
    }
    if let Saying::Which(entry) = saying {
        let asked = format!(
            " {} ",
            words!("screen-draw-bottom-say-which", signals = SIGNAL_COUNT)
        );
        let keys = format!("   {}", words!("screen-draw-bottom-say-keys"));
        return Paragraph::new(typing(asked, &entry.typed, width, &keys));
    }
    Paragraph::new(what_the_keys_do(watch, width))
}

/// What is being typed, after what it is being typed for, with the hint after it when
/// the hint fits.
///
/// The end of what was typed is the part being typed, so on a line too narrow for all
/// of it the beginning is what goes, and the cursor stays in sight.
fn typing(asked: String, typed: &str, width: usize, hint: &str) -> Line<'static> {
    let bold = Style::new().add_modifier(Modifier::BOLD);
    let room = width.saturating_sub(asked.width() + 1);
    let (mut shown, cut) = (typed, typed.width() > room);
    while cut && !shown.is_empty() && shown.width() + 1 > room {
        let mut letters = shown.chars();
        letters.next();
        shown = letters.as_str();
    }
    let mut spans = vec![
        Span::styled(asked, bold),
        Span::styled(
            format!("{}{shown}\u{258f}", if cut { "\u{2026}" } else { "" }),
            bold,
        ),
    ];
    let used: usize = spans.iter().map(|span| span.content.width()).sum();
    if used + hint.width() <= width {
        spans.push(Span::styled(
            hint.to_owned(),
            Style::new().fg(Color::DarkGray),
        ));
    }
    Line::from(spans)
}

/// Why the last line typed was refused, folded to fit, under the verdict and over the
/// line being typed. Nothing when nothing was refused.
pub(super) fn refused(saying: &Saying, width: u16) -> Vec<Line<'static>> {
    let (Saying::Typing(entry) | Saying::Which(entry)) = saying else {
        return Vec::new();
    };
    let Some(why) = &entry.refused else {
        return Vec::new();
    };
    let mut lines = super::marked(
        why,
        usize::from(width).saturating_sub(1),
        Color::Red,
        Style::new(),
    );
    for line in &mut lines {
        line.spans.insert(0, Span::raw(" "));
    }
    lines
}

/// The four keys, with as much said about each as fits, so that the keys themselves are
/// the last thing to go and never the first: labels in full, then labels short, then
/// the keys alone.
fn what_the_keys_do(watch: &Watch, width: usize) -> Line<'static> {
    let file = if watch.has_the_file {
        words!("screen-draw-bottom-has-the-file")
    } else {
        words!("screen-draw-bottom-not-given")
    };
    let full = [
        words!("screen-draw-bottom-leave-wide"),
        words!("screen-draw-bottom-say-wide", signals = SIGNAL_COUNT),
        words!("screen-draw-bottom-more-wide"),
        words!("screen-draw-bottom-keys-wide"),
    ];
    let short = [
        words!("screen-draw-bottom-leave"),
        words!("screen-draw-bottom-say"),
        words!("screen-draw-bottom-more"),
        words!("screen-draw-bottom-keys"),
    ];
    let bare = [String::new(), String::new(), String::new(), String::new()];
    let tried = [
        (&full, Some(file), "   "),
        (&full, None, "   "),
        (&short, None, "  "),
        (&bare, None, ""),
    ];
    let mut chosen = Vec::new();
    for (labels, note, gap) in tried {
        chosen = keys_line(labels, note, gap);
        let covered: usize = chosen.iter().map(|span| span.content.width()).sum();
        if covered <= width {
            break;
        }
    }
    Line::from(chosen)
}

/// The keys with these labels, and a note after them if there is one.
fn keys_line(labels: &[String; 4], note: Option<String>, gap: &str) -> Vec<Span<'static>> {
    let key = Style::new().add_modifier(Modifier::REVERSED);
    let mut spans = Vec::new();
    for (pressed, label) in ["q", "s", ":", "?"].into_iter().zip(labels) {
        spans.push(Span::styled(format!(" {pressed} "), key));
        spans.push(Span::raw(if label.is_empty() {
            " ".to_owned()
        } else {
            format!(" {label}{gap}")
        }));
    }
    if let Some(note) = note {
        spans.push(Span::styled(note, Style::new().fg(Color::DarkGray)));
    }
    spans
}
