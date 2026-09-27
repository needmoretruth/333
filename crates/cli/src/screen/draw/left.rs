//! The left column — the count, this node's standing, and what was said — kept apart
//! because it is the part of the screen that turns this node's numbers into words.

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use n333_core::presence::WINDOW_EPOCHS;
use n333_core::signal::SIGNAL_COUNT;

use super::titled;
use crate::screen::watch::{Said, Watch, Where};

/// The left column: the count, then this node, then what was said.
pub(super) fn this_node<'a>(watch: &'a Watch, area: Rect) -> Paragraph<'a> {
    let mut lines = vec![
        counted("ANSWERING", watch.answering, true),
        counted("silent", watch.roll.saturating_sub(watch.answering), false),
        Line::from(Span::styled("─────────", Style::new().fg(Color::DarkGray))),
        counted("roll", watch.roll, false),
        counted("known where", watch.addresses, false),
        counted("witnessed", watch.witnessed, false),
        Line::raw(""),
        Line::from(Span::styled(
            "YOU",
            Style::new().add_modifier(Modifier::BOLD),
        )),
    ];
    lines.extend(standing(&watch.standing));
    lines.push(Line::raw(""));
    lines.extend(said(&watch.said, area.height));
    Paragraph::new(lines).block(titled("this node"))
}

/// One number with its name, in a column.
fn counted<'a>(name: &'a str, count: usize, loud: bool) -> Line<'a> {
    let style = if loud {
        Style::new().add_modifier(Modifier::BOLD)
    } else {
        Style::new()
    };
    Line::from(vec![
        Span::styled(format!("{name:<13}"), style),
        Span::styled(count.to_string(), style),
    ])
}

/// What this node's own record says about it, in the words that are true of it.
fn standing(standing: &Where) -> Vec<Line<'static>> {
    match standing {
        Where::OnNobodysRoll => vec![
            Line::raw("on nobody's roll."),
            Line::raw("nobody has handed you"),
            Line::raw("the file yet. it takes"),
            Line::raw("an invitation."),
            Line::styled(crate::commands::THE_PLACE, Style::new().fg(Color::DarkGray)),
        ],
        Where::Waiting {
            joined,
            counted_from,
        } => vec![
            Line::raw(format!("given the file in {}", joined.0)),
            Line::raw(format!("counted from {}", counted_from.0)),
            Line::styled("answer everything until", Style::new().fg(Color::DarkGray)),
            Line::styled(
                "then. none of it is banked.",
                Style::new().fg(Color::DarkGray),
            ),
        ],
        Where::Counted {
            standing,
            silent_on,
        } => {
            let share = standing.per_mille().map_or_else(
                || "—".to_owned(),
                |per_mille| format!("{}.{}%", per_mille / 10, per_mille % 10),
            );
            let mut lines = vec![Line::from(vec![
                Span::raw(format!(
                    "present in {} of {} — ",
                    standing.present, standing.counted
                )),
                Span::styled(share, Style::new().add_modifier(Modifier::BOLD)),
            ])];
            if standing.qualifies() {
                lines.push(Line::styled(
                    "by your own record, counted",
                    Style::new().fg(Color::DarkGray),
                ));
            } else {
                lines.push(Line::styled(
                    "not counted. two of every",
                    Style::new().fg(Color::Red),
                ));
                lines.push(Line::styled(
                    "three is all that is asked",
                    Style::new().fg(Color::Red),
                ));
            }
            if *silent_on != 0 {
                lines.push(Line::styled(
                    format!("silent on {silent_on} of {WINDOW_EPOCHS}"),
                    Style::new().fg(Color::DarkGray),
                ));
            }
            lines
        }
    }
}

/// The shape of what everybody said this epoch, as much of it as there is room for.
fn said(said: &Said, height: u16) -> Vec<Line<'static>> {
    if said.spoken == 0 {
        return vec![
            Line::from(Span::styled(
                "SAID",
                Style::new().add_modifier(Modifier::BOLD),
            )),
            Line::styled(
                format!("nothing yet. {SIGNAL_COUNT} things"),
                Style::new().fg(Color::DarkGray),
            ),
            Line::styled("can be said.", Style::new().fg(Color::DarkGray)),
        ];
    }
    let mut lines = vec![Line::from(vec![
        Span::styled("SAID  ", Style::new().add_modifier(Modifier::BOLD)),
        Span::raw(format!("{} of {} spoke", said.spoken, said.observed)),
    ])];
    // However many rows are left on this column, and a line saying what did not fit.
    // Cutting the tail off in silence would make a distribution look like the whole of
    // one, which is the one thing this screen must never do.
    let room = usize::from(height).saturating_sub(lines.len() + 14).max(1);
    for (index, count, share, reached) in said.rows.iter().take(room) {
        let share = share.map_or_else(
            || "—".to_owned(),
            |per_mille| format!("{}.{}%", per_mille / 10, per_mille % 10),
        );
        let mark = if *reached { "  a third" } else { "" };
        let style = if *reached {
            Style::new().add_modifier(Modifier::BOLD)
        } else {
            Style::new()
        };
        lines.push(Line::styled(
            format!(" #{index:<5}{count:>3} {share:>6}{mark}"),
            style,
        ));
    }
    if said.rows.len() > room {
        lines.push(Line::styled(
            format!(" and {} more said", said.rows.len() - room),
            Style::new().fg(Color::DarkGray),
        ));
    }
    if let Some(mine) = said.mine {
        lines.push(Line::styled(
            format!(" you said #{mine}"),
            Style::new().fg(Color::DarkGray),
        ));
    }
    lines
}
