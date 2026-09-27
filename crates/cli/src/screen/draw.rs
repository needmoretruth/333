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
mod keys;
mod left;
mod right;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Padding, Paragraph};

use super::Saying;
use super::watch::Watch;
use unicode_width::UnicodeWidthStr as _;

use crate::commands::hours::to_the_boundary;
use crate::words::Arg;

use bottom::{the_keys, the_silence};
use left::this_node;
use right::vigil;

/// How wide the left column is, when there is room for one.
const APART: u16 = 34;

/// The narrowest terminal that still gets two columns.
const TOO_NARROW: u16 = 62;

/// Draw everything.
pub(super) fn everything(frame: &mut Frame<'_>, watch: &Watch, log: &[String], saying: &Saying) {
    let twelve = counting_in_twelve();
    let [top, counting, middle, bottom] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(u16::from(twelve.is_some())),
        Constraint::Min(3),
        Constraint::Length(2),
    ])
    .areas(frame.area());

    let wide = frame.area().width >= TOO_NARROW;
    frame.render_widget(header(watch, wide), top);
    if let Some(twelve) = twelve {
        frame.render_widget(Paragraph::new(format!(" {twelve}")), counting);
    }
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
    frame.render_widget(the_silence(watch, silence.width), silence);
    frame.render_widget(the_keys(watch, saying, keys.width), keys);
    if matches!(saying, Saying::Keys) {
        keys::every_key(frame, frame.area());
    }
}

/// The line the client says at the start when it counts in twelve, which the screen
/// covers up as soon as it is drawn. Said again here, once, where it is seen without
/// looking for it: every number below it is in twelve, and read in ten each of them
/// is wrong.
fn counting_in_twelve() -> Option<String> {
    let (ten, eleven) = crate::words::current().base().past_nine()?;
    Some(words!(
        "words-counting-in-twelve",
        ten = ten.to_string(),
        eleven = eleven.to_string()
    ))
}

/// The one line that is always true: who this is, when it is, and how long is left.
fn header<'a>(watch: &'a Watch, wide: bool) -> Paragraph<'a> {
    let left = remaining(to_the_boundary(watch.epoch));
    let left = if wide {
        words!("screen-draw-to-the-boundary", left = left)
    } else {
        words!("screen-draw-time-left", left = left)
    };
    Paragraph::new(Line::from(vec![
        Span::styled(" 333 ", Style::new().add_modifier(Modifier::REVERSED)),
        Span::raw("  "),
        Span::styled(
            crate::commands::shorten(&watch.name),
            Style::new().add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!("   {} ", words!("screen-draw-epoch"))),
        Span::styled(
            words!("screen-draw-number", number = watch.epoch.0),
            Style::new().add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!("   {left}")),
    ]))
}

/// How long is left of this epoch, the way a person waiting would say it, in the base
/// they count in: hours and minutes, or minutes and seconds in the last hour.
fn remaining(seconds: u64) -> String {
    let (hours, minutes) = (seconds / 3600, (seconds % 3600) / 60);
    if hours == 0 {
        return words!(
            "commands-minutes-and-seconds",
            minutes = minutes,
            seconds = seconds % 60
        );
    }
    words!(
        "commands-hours-and-minutes",
        hours = hours,
        minutes = Arg::padded(minutes, 2)
    )
}

/// A pane's frame, with its name on it.
fn titled(name: String) -> Block<'static> {
    Block::bordered()
        .border_style(Style::new().fg(Color::DarkGray))
        .padding(Padding::horizontal(1))
        .title(Span::styled(
            format!(" {name} "),
            Style::new().fg(Color::DarkGray),
        ))
}

/// The mark colour is allowed to be: a dot, with the words beside it carrying the
/// meaning. A whole line in red is unreadable on a cheap screen, in sunlight and to
/// somebody who cannot tell red from green, and on a terminal it sets the mood of
/// everything around it.
const MARK: &str = "\u{25cf} ";

/// Words folded to `width`, the first line after a dot of `colour` and every further
/// line lined up under the words rather than under the dot.
fn marked(text: &str, width: usize, colour: Color, style: Style) -> Vec<Line<'static>> {
    let room = width.saturating_sub(MARK.width()).max(1);
    let mut lines = Vec::new();
    for (at, line) in text
        .split('\n')
        .flat_map(|line| right::fold(line, room))
        .enumerate()
    {
        let lead = if at == 0 {
            Span::styled(MARK, Style::new().fg(colour))
        } else {
            Span::raw(" ".repeat(MARK.width()))
        };
        lines.push(Line::from(vec![
            lead,
            Span::styled(line.trim_start().to_owned(), style),
        ]));
    }
    lines
}

/// Text followed by spaces to fill `columns` terminal columns, counted as a terminal
/// counts them: a Korean label padded by its letters would push the number after it
/// out of line by the width of the label.
fn padded(text: &str, columns: usize) -> String {
    format!("{text}{}", " ".repeat(columns.saturating_sub(text.width())))
}

/// Spaces, then text, to fill `columns` terminal columns: a number lined up on the right.
fn ahead(text: &str, columns: usize) -> String {
    format!("{}{text}", " ".repeat(columns.saturating_sub(text.width())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    use crate::words::count::Base;

    /// Every row of the screen drawn at this size, as text, without trailing spaces.
    fn drawn(width: u16, height: u16, saying: &Saying) -> Vec<String> {
        drawn_of(&Watch::quiet(Vec::new()), width, height, saying)
    }

    /// The same, of a node given here.
    fn drawn_of(watch: &Watch, width: u16, height: u16, saying: &Saying) -> Vec<String> {
        let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
        terminal
            .draw(|frame| everything(frame, watch, &[], saying))
            .unwrap();
        let buffer = terminal.backend().buffer();
        (0..height)
            .map(|y| {
                let row: String = (0..width).map(|x| buffer[(x, y)].symbol()).collect();
                row.trim_end().to_owned()
            })
            .collect()
    }

    #[test]
    fn in_english_the_header_and_the_bottom_say_what_they_said_before() {
        let rows = |saying| crate::words::speaking("en", Base::Ten, || drawn(130, 20, &saying));
        let watching = rows(Saying::Nothing);
        assert!(
            watching[0].starts_with(" 333   333   epoch 9   "),
            "{}",
            watching[0]
        );
        assert!(watching[0].ends_with(" to the boundary"), "{}", watching[0]);
        assert_eq!(
            watching[18],
            " no one has ever answered this node, which is what a node looks like before it has been anywhere"
        );
        assert_eq!(
            watching[19],
            " q  leave the vigil    s  say one of the 333    :  everything else    ?  \
             all the keys   this node has not been given the file"
        );
        assert_eq!(
            rows(Saying::Which("4".into()))[19],
            " say which of the 333? 4▏   enter to say it · esc to say nothing"
        );
        assert_eq!(
            rows(Saying::Typing("pi".into()))[19],
            " : pi\u{258f}   ping · join · bootstrap · say · tor on · tor off · bridge · status · quit"
        );
    }

    #[test]
    fn counting_in_twelve_is_said_under_the_header_and_in_ten_nothing_is() {
        let twelve =
            crate::words::speaking("en", Base::Twelve, || drawn(100, 20, &Saying::Nothing));
        assert_eq!(
            twelve[1],
            " counting in twelve. \u{218A} is ten, \u{218B} is eleven, and 10 is twelve."
        );
        assert!(
            twelve[0].contains("epoch 9 "),
            "nine is nine: {}",
            twelve[0]
        );
        let ten = crate::words::speaking("en", Base::Ten, || drawn(100, 20, &Saying::Nothing));
        assert!(!ten.concat().contains("twelve"));
    }

    #[test]
    fn the_keys_are_the_last_thing_the_bottom_line_gives_up() {
        let at = |width| {
            crate::words::speaking("en", Base::Ten, || drawn(width, 20, &Saying::Nothing))[19]
                .clone()
        };
        assert!(at(130).ends_with("given the file"), "{}", at(130));
        assert_eq!(
            at(100),
            " q  leave the vigil    s  say one of the 333    :  everything else    ?  all the keys"
        );
        assert_eq!(at(48), " q  leave   s  say   :  more   ?  keys");
        assert_eq!(at(20), " q   s   :   ?");
    }

    #[test]
    fn question_mark_shows_every_key_and_every_order_word() {
        let shown =
            crate::words::speaking("en", Base::Ten, || drawn(48, 20, &Saying::Keys)).join("\n");
        for typed in [
            "q  esc",
            "ping ADDRESS",
            "join INVITATION",
            "bootstrap",
            "say N",
            "tor on",
            "bridge LINE",
            "helper PROGRAM",
            "status",
            "quit",
            "any key closes this",
        ] {
            assert!(shown.contains(typed), "{typed} is missing from\n{shown}");
        }
    }

    #[test]
    fn in_english_the_countdown_is_said_as_it_was() {
        let said = crate::words::speaking("en", Base::Ten, || {
            [remaining(2 * 3600 + 5 * 60 + 9), remaining(4 * 60 + 5)]
        });
        assert_eq!(said, ["2h 05m".to_owned(), "4m 5s".to_owned()]);
    }
}
