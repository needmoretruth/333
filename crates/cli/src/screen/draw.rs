//! The screen itself: where everything goes on it, and what it says.
//!
//! One screen, not a set of pages. Everything a node knows about itself fits in a
//! terminal at once, and a person who leaves this running wants the same things in
//! the same places every time they look up — not a thing to navigate.
//!
//! THE COUNT IS THE LARGEST THING ON IT. It is the only number that can reach zero,
//! and reaching zero is what the whole design is arranged around. The roll below it is
//! deliberately quieter: a roll only ever grows, so a big roll says nothing about
//! whether anybody is still here.
//!
//! NOTHING ON THIS SCREEN IS ANYBODY ELSE'S NUMBER. It is one machine's observation,
//! drawn from its own disk, and the machine beside it is showing something else.

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Padding, Paragraph};

use n333_core::epoch;
use n333_core::extinction::{Remaining, Verdict};
use n333_core::presence::WINDOW_EPOCHS;
use n333_core::signal::SIGNAL_COUNT;

use super::Saying;
use super::watch::{Said, Watch, Where, to_the_boundary, until};

/// How wide the left column is, when there is room for one.
const APART: u16 = 34;

/// The narrowest terminal that still gets two columns.
const TOO_NARROW: u16 = 62;

/// Draw everything.
pub(super) fn everything(frame: &mut Frame<'_>, watch: &Watch, log: &[String], saying: &Saying) {
    let [top, middle, bottom] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(3),
        Constraint::Length(2),
    ])
    .areas(frame.area());

    let wide = frame.area().width >= TOO_NARROW;
    frame.render_widget(header(watch, wide), top);
    if middle.width < TOO_NARROW {
        // Too narrow for two columns. The vigil is what is left, because a person on a
        // small terminal is watching for something to happen, and the rest of it is a
        // command away.
        frame.render_widget(vigil(log, middle), middle);
    } else {
        let [left, right] =
            Layout::horizontal([Constraint::Length(APART), Constraint::Min(0)]).areas(middle);
        frame.render_widget(this_node(watch, left), left);
        frame.render_widget(vigil(log, right), right);
    }
    let [silence, keys] =
        Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(bottom);
    frame.render_widget(the_silence(watch, wide), silence);
    frame.render_widget(the_keys(watch, saying, wide), keys);
}

/// The one line that is always true: who this is, when it is, and how long is left.
fn header<'a>(watch: &'a Watch, wide: bool) -> Paragraph<'a> {
    let left = if wide { " to the boundary" } else { " left" };
    Paragraph::new(Line::from(vec![
        Span::styled(" 333 ", Style::new().add_modifier(Modifier::REVERSED)),
        Span::raw("  "),
        Span::styled(
            crate::commands::shorten(&watch.name),
            Style::new().add_modifier(Modifier::BOLD),
        ),
        Span::raw("   epoch "),
        Span::styled(
            watch.epoch.0.to_string(),
            Style::new().add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!("   {}{left}", until(to_the_boundary(watch.epoch)))),
    ]))
}

/// The left column: the count, then this node, then what was said.
fn this_node<'a>(watch: &'a Watch, area: Rect) -> Paragraph<'a> {
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

/// The right column: what this node has done and been told, newest at the bottom.
fn vigil(log: &[String], area: Rect) -> Paragraph<'static> {
    let width = usize::from(area.width).saturating_sub(4);
    let room = usize::from(area.height).saturating_sub(2);
    // Built from the newest backwards, because the newest line is the one that must be
    // on the screen. A pane filled from the top loses whatever happened last.
    let mut upward: Vec<String> = Vec::new();
    for entry in log.iter().rev() {
        let mut folded = fold(entry, width);
        folded.reverse();
        upward.append(&mut folded);
        if upward.len() >= room {
            break;
        }
    }
    upward.truncate(room);
    upward.reverse();
    Paragraph::new(upward.into_iter().map(Line::raw).collect::<Vec<Line<'_>>>())
        .block(titled("the vigil"))
}

/// What a line is lined up under when it does not fit: the text, not the hour.
const UNDER: &str = "          ";

/// Break one line to fit the pane, on spaces where there are any.
///
/// Cutting it off at the edge instead would lose the end of every sentence this client
/// has to say, and the ends are where the meaning is.
fn fold(entry: &str, width: usize) -> Vec<String> {
    let mut folded = Vec::new();
    let mut rest = entry.trim_end();
    // The hanging indent helps on a wide pane and shreds a narrow one: ten columns of
    // it out of fifteen leaves five for the words, and a node's name would come out
    // two letters at a time.
    let under = if width >= UNDER.len() * 2 { UNDER } else { "" };
    let mut indent = "";
    while !rest.is_empty() {
        let room = width.saturating_sub(indent.len());
        if room == 0 {
            break;
        }
        let (mut counted, mut ends_at, mut space) = (0_usize, rest.len(), None);
        for (at, letter) in rest.char_indices() {
            if counted == room {
                ends_at = at;
                break;
            }
            if letter == ' ' && counted > 0 {
                space = Some(at);
            }
            counted += 1;
        }
        if counted < room {
            folded.push(format!("{indent}{rest}"));
            break;
        }
        let (head, tail) = rest.split_at(space.unwrap_or(ends_at));
        folded.push(format!("{indent}{}", head.trim_end()));
        rest = tail.trim_start();
        indent = under;
    }
    folded
}

/// A pane's frame, with its name on it.
fn titled(name: &str) -> Block<'_> {
    Block::bordered()
        .border_style(Style::new().fg(Color::DarkGray))
        .padding(Padding::horizontal(1))
        .title(Span::styled(
            format!(" {name} "),
            Style::new().fg(Color::DarkGray),
        ))
}

/// Whether anybody is here, and what is left if nobody is.
fn the_silence<'a>(watch: &'a Watch, wide: bool) -> Paragraph<'a> {
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
fn the_keys<'a>(watch: &'a Watch, saying: &'a Saying, wide: bool) -> Paragraph<'a> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_line_too_long_for_the_pane_breaks_on_a_space_and_lines_up_underneath() {
        let folded = fold("12:00:00  gave the file to somebody", 20);
        assert_eq!(
            folded,
            vec![
                "12:00:00  gave the".to_owned(),
                "          file to".to_owned(),
                "          somebody".to_owned(),
            ]
        );
    }

    #[test]
    fn a_word_longer_than_the_pane_is_cut_rather_than_lost() {
        // A node's name is sixty-four characters with nothing to break on. Waiting for
        // a space that never comes would drop the line, and on a pane this narrow the
        // hanging indent would leave two columns for the letters.
        let folded = fold("name 333abcdefghijklmnop", 12);
        assert!(
            folded.iter().all(|line| line.chars().count() <= 12),
            "{folded:?}"
        );
        assert_eq!(folded.concat().replace(' ', ""), "name333abcdefghijklmnop");
    }

    #[test]
    fn a_line_that_fits_is_left_exactly_as_it_was() {
        assert_eq!(fold("short enough", 40), vec!["short enough".to_owned()]);
        assert!(fold("anything", 0).is_empty(), "no pane, nothing to draw");
    }
}
