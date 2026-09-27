//! The two lines along the bottom — whether anybody is here, and what the keys do —
//! kept apart because they change with the verdict and with what is being typed, not
//! with anything in the columns above them.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use n333_core::epoch;
use n333_core::extinction::{Remaining, Verdict};
use n333_core::signal::SIGNAL_COUNT;

use crate::screen::Saying;
use crate::screen::watch::Watch;

/// Whether anybody is here, and what is left if nobody is.
pub(super) fn the_silence<'a>(watch: &'a Watch, wide: bool) -> Paragraph<'a> {
    let line = match watch.vigil.verdict() {
        Verdict::NothingToSay => Line::styled(
            if wide {
                " no one has ever answered this node, which is what a node looks like before it has been anywhere"
            } else {
                " no one has ever answered this node"
            },
            Style::new().fg(Color::DarkGray),
        ),
        Verdict::Alive => Line::styled(
            if wide {
                " somebody is here. nothing further is owed to the arithmetic"
            } else {
                " somebody is here"
            },
            Style::new().fg(Color::DarkGray),
        ),
        Verdict::Waiting { silent, needed } => Line::styled(
            if wide {
                format!(
                    " nobody has answered for {silent} of the {needed} epochs it would take to say so"
                )
            } else {
                format!(" nobody for {silent} of {needed} epochs")
            },
            Style::new().fg(Color::Yellow),
        ),
        Verdict::Ended { since } => Line::styled(
            match watch.vigil.remaining_at(epoch::unix_now_seconds()) {
                Some(Remaining { years, days }) => format!(
                    " nobody has answered since epoch {}. {years} years and {days} days until it is gone",
                    since.0
                ),
                None => format!(
                    " nobody has answered since epoch {}, and the last of the years has run out",
                    since.0
                ),
            },
            Style::new().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
    };
    Paragraph::new(line)
}

/// What the keys do, or what is being typed.
pub(super) fn the_keys<'a>(watch: &'a Watch, saying: &'a Saying, wide: bool) -> Paragraph<'a> {
    if let Saying::Typing(typed) = saying {
        let mut asked = vec![
            Span::styled(" : ", Style::new().add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{typed}\u{258f}"),
                Style::new().add_modifier(Modifier::BOLD),
            ),
        ];
        if wide {
            asked.push(Span::styled(
                "   ping · join · bootstrap · say · tor on · tor off · bridge · status · quit",
                Style::new().fg(Color::DarkGray),
            ));
        }
        return Paragraph::new(Line::from(asked));
    }
    if let Saying::Which(typed) = saying {
        let mut asked = vec![
            Span::styled(
                format!(" say which of the {SIGNAL_COUNT}? "),
                Style::new().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{typed}▏"),
                Style::new().add_modifier(Modifier::BOLD),
            ),
        ];
        if wide {
            asked.push(Span::styled(
                "   enter to say it · esc to say nothing",
                Style::new().fg(Color::DarkGray),
            ));
        }
        return Paragraph::new(Line::from(asked));
    }
    let mut keys = vec![
        Span::styled(" q ", Style::new().add_modifier(Modifier::REVERSED)),
        Span::raw(if wide {
            " leave the vigil   "
        } else {
            " leave  "
        }),
        Span::styled(" s ", Style::new().add_modifier(Modifier::REVERSED)),
        Span::raw(if wide {
            format!(" say one of the {SIGNAL_COUNT}   ")
        } else {
            " say  ".to_owned()
        }),
        Span::styled(" : ", Style::new().add_modifier(Modifier::REVERSED)),
        Span::raw(if wide {
            " everything else   "
        } else {
            " more   "
        }),
    ];
    if wide {
        keys.push(Span::styled(
            if watch.has_the_file {
                "the file is here"
            } else {
                "this node has not been given the file"
            },
            Style::new().fg(Color::DarkGray),
        ));
    }
    Paragraph::new(Line::from(keys))
}
