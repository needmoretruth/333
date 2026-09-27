//! The two lines along the bottom — whether anybody is here, and what the keys do —
//! kept apart because they change with the verdict and with what is being typed, not
//! with anything in the columns above them.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use n333_core::epoch;
use n333_core::extinction::{Remaining, Verdict};
use n333_core::signal::SIGNAL_COUNT;

use super::MARK;
use crate::screen::Saying;
use crate::screen::watch::Watch;
use crate::words::Arg;

/// Whether anybody is here, and what is left if nobody is.
pub(super) fn the_silence<'a>(watch: &'a Watch, wide: bool) -> Paragraph<'a> {
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

/// What the keys do, or what is being typed.
pub(super) fn the_keys<'a>(watch: &'a Watch, saying: &'a Saying, wide: bool) -> Paragraph<'a> {
    let bold = Style::new().add_modifier(Modifier::BOLD);
    let grey = Style::new().fg(Color::DarkGray);
    if let Saying::Typing(typed) = saying {
        let mut asked = vec![
            Span::styled(" : ", bold),
            Span::styled(format!("{typed}\u{258f}"), bold),
        ];
        if wide {
            asked.push(Span::styled(format!("   {ORDER_WORDS}"), grey));
        }
        return Paragraph::new(Line::from(asked));
    }
    if let Saying::Which(typed) = saying {
        let mut asked = vec![
            Span::styled(
                format!(
                    " {} ",
                    words!("screen-draw-bottom-say-which", signals = SIGNAL_COUNT)
                ),
                bold,
            ),
            Span::styled(format!("{typed}▏"), bold),
        ];
        if wide {
            asked.push(Span::styled(
                format!("   {}", words!("screen-draw-bottom-say-keys")),
                grey,
            ));
        }
        return Paragraph::new(Line::from(asked));
    }
    let key = Style::new().add_modifier(Modifier::REVERSED);
    let (leave, say, more) = if wide {
        (
            format!(" {}   ", words!("screen-draw-bottom-leave-wide")),
            format!(
                " {}   ",
                words!("screen-draw-bottom-say-wide", signals = SIGNAL_COUNT)
            ),
            format!(" {}   ", words!("screen-draw-bottom-more-wide")),
        )
    } else {
        (
            format!(" {}  ", words!("screen-draw-bottom-leave")),
            format!(" {}  ", words!("screen-draw-bottom-say")),
            format!(" {}   ", words!("screen-draw-bottom-more")),
        )
    };
    let mut keys = vec![
        Span::styled(" q ", key),
        Span::raw(leave),
        Span::styled(" s ", key),
        Span::raw(say),
        Span::styled(" : ", key),
        Span::raw(more),
    ];
    if wide {
        keys.push(Span::styled(
            if watch.has_the_file {
                words!("screen-draw-bottom-has-the-file")
            } else {
                words!("screen-draw-bottom-not-given")
            },
            grey,
        ));
    }
    Paragraph::new(Line::from(keys))
}
