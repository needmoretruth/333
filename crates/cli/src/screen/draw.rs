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

mod bottom;
mod left;
mod right;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Padding, Paragraph};

use super::Saying;
use super::watch::Watch;
use crate::commands::hours::{to_the_boundary, until};

use bottom::{the_keys, the_silence};
use left::this_node;
use right::vigil;

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
