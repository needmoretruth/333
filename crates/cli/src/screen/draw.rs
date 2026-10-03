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

use super::watch::Watch;
use super::{Heard, Saying};
use unicode_width::UnicodeWidthStr as _;

use crate::commands::hours::to_the_boundary;
use crate::words::Arg;

use bottom::{the_keys, the_silence};

use left::{at_a_glance, this_node};
use right::vigil;

/// How wide the left column is, when there is room for one.
const APART: u16 = 34;

/// The narrowest terminal that still gets two columns.
const TOO_NARROW: u16 = 62;

/// Draw everything.
pub(super) fn everything(frame: &mut Frame<'_>, watch: &Watch, log: &[Heard], saying: &Saying) {
    let twelve = counting_in_twelve(usize::from(frame.area().width).saturating_sub(1));
    let refused = bottom::refused(saying, frame.area().width);
    let [top, counting, middle, bottom] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(u16::try_from(twelve.len()).unwrap_or(0)),
        Constraint::Min(3),
        Constraint::Length(2 + u16::try_from(refused.len()).unwrap_or(0)),
    ])
    .areas(frame.area());

    frame.render_widget(header(watch, usize::from(top.width)), top);
    let twelve: Vec<Line<'_>> = twelve
        .into_iter()
        .map(|line| Line::raw(format!(" {line}")))
        .collect();
    frame.render_widget(Paragraph::new(twelve), counting);
    if middle.width < TOO_NARROW {
        // Too narrow for two columns. The count and this node's standing, in a line
        // each, then the vigil in what is left: a person on a small terminal is
        // watching for something to happen, and the rest of it is a command away.
        let glance = at_a_glance(watch, usize::from(middle.width).saturating_sub(2));
        let rows = u16::try_from(glance.len()).unwrap_or(0);
        let [glanced, lines] =
            Layout::vertical([Constraint::Length(rows), Constraint::Min(0)]).areas(middle);
        frame.render_widget(
            Paragraph::new(glance).block(Block::new().padding(Padding::horizontal(1))),
            glanced,
        );
        frame.render_widget(vigil(log, lines), lines);
    } else {
        let [left, right] =
            Layout::horizontal([Constraint::Length(APART), Constraint::Min(0)]).areas(middle);
        frame.render_widget(this_node(watch, left), left);
        frame.render_widget(vigil(log, right), right);
    }
    let [silence, why, keys] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(bottom);
    frame.render_widget(the_silence(watch, silence.width), silence);
    frame.render_widget(Paragraph::new(refused), why);
    frame.render_widget(the_keys(watch, saying, keys.width), keys);
    if matches!(saying, Saying::Keys) {
        keys::every_key(frame, frame.area());
    }
}

/// The line the client says at the start when it counts in twelve, which the screen
/// covers up as soon as it is drawn. Said again here, once, where it is seen without
/// looking for it: every number below it is in twelve, and read in ten each of them
/// is wrong. Laid out for `width`, so a narrow screen shows all of it. Nothing in ten.
fn counting_in_twelve(width: usize) -> Vec<String> {
    let Some((ten, eleven)) = crate::words::current().base().past_nine() else {
        return Vec::new();
    };
    let said = words!(
        "words-counting-in-twelve",
        ten = ten.to_string(),
        eleven = eleven.to_string()
    );
    crate::words::layout::fold(&said, width)
}

/// The one line that is always true: who this is, when it is, and how long is left.
///
/// Given up from its least needed part when the terminal is narrow: the name first,
/// which the vigil's lines and `333 id` both say, then which epoch of this line it is,
/// which `333 status` says, then the long way of saying how much is left. The epoch
/// and the time left are what a person looks up to see.
fn header(watch: &Watch, width: usize) -> Paragraph<'static> {
    let left = remaining(to_the_boundary(watch.epoch));
    let long = words!("screen-draw-to-the-boundary", left = left.as_str());
    let short = words!("screen-draw-time-left", left = left.as_str());
    let name = crate::commands::shorten(&watch.name);
    let line = watch.line.map(|nth| {
        let kind = crate::words::count::ordinal(nth);
        words!("screen-draw-line", nth = nth, kind = kind)
    });
    let mut chosen = Vec::new();
    for (name, line, left) in [
        (Some(&name), line.as_ref(), &long),
        (None, line.as_ref(), &long),
        (None, None, &long),
        (None, None, &short),
    ] {
        chosen = header_line(watch, name, line, left);
        if chosen
            .iter()
            .map(|span| span.content.width())
            .sum::<usize>()
            <= width
        {
            break;
        }
    }
    Paragraph::new(Line::from(chosen))
}

/// The header's parts, with or without the name and which epoch of this line it is.
fn header_line(
    watch: &Watch,
    name: Option<&String>,
    line: Option<&String>,
    left: &str,
) -> Vec<Span<'static>> {
    let bold = Style::new().add_modifier(Modifier::BOLD);
    let mut spans = vec![Span::styled(
        " 333 ",
        Style::new().add_modifier(Modifier::REVERSED),
    )];
    if let Some(name) = name {
        spans.push(Span::raw("  "));
        spans.push(Span::styled(name.clone(), bold));
    }
    spans.extend([
        Span::raw(format!("   {} ", words!("screen-draw-epoch"))),
        Span::styled(words!("screen-draw-number", number = watch.epoch.0), bold),
    ]);
    if let Some(line) = line {
        spans.push(Span::raw(line.clone()));
    }
    spans.push(Span::raw(format!("   {left}")));
    spans
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
        .flat_map(|line| right::flush(line, room))
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
    use crate::screen::Entry;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    use crate::words::count::Base;

    fn typed(typed: &str, refused: Option<&str>) -> Entry {
        Entry {
            typed: typed.to_owned(),
            refused: refused.map(str::to_owned),
        }
    }

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
        assert_eq!(watching[18], " no one has answered this node yet");
        assert_eq!(
            watching[19],
            " q  stop the node    s  say one of the 333    :  everything else    ?  all the keys   this node has not been given the file"
        );
        assert_eq!(
            rows(Saying::Which(typed("4", None)))[19],
            " say which of the 333? 4▏   enter to say it · esc to say nothing"
        );
        assert_eq!(
            rows(Saying::Typing(typed("pi", None)))[19],
            " : pi\u{258f}   ping · join · bootstrap · say · tor on · tor off · bridge · helper · status · quit"
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
        let narrow = crate::words::speaking("en", Base::Twelve, || drawn(48, 20, &Saying::Nothing));
        assert_eq!(
            narrow[1],
            " counting in twelve. \u{218A} is ten, \u{218B} is eleven, and"
        );
        assert_eq!(
            narrow[2], "          10 is twelve.",
            "all of it, under its words"
        );
    }

    #[test]
    fn in_twelve_the_333_the_keys_name_is_the_name_and_not_a_count() {
        let rows = crate::words::speaking("ko", Base::Twelve, || drawn(100, 20, &Saying::Nothing));
        // A letter two columns wide is drawn as the letter and a blank cell after it.
        let keys = rows[19].replace(' ', "");
        assert!(keys.contains("s333가운데하나말하기"), "{}", rows[19]);
        let asked = crate::words::speaking("en", Base::Twelve, || {
            drawn(100, 20, &Saying::Which(typed("", None)))
        });
        assert!(
            asked[19].starts_with(" say which of the 333? "),
            "{}",
            asked[19]
        );
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
            " q  stop the node    s  say one of the 333    :  everything else    ?  all the keys"
        );
        assert_eq!(at(48), " q  stop   s  say   :  more   ?  keys");
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
    fn the_box_of_keys_leaves_no_pane_border_showing_beside_it() {
        // Where the box is nearly as wide as the screen; on a wider one it stands in
        // the middle with the panes whole on either side of it.
        for (width, height) in [(48, 20), (50, 20), (58, 24)] {
            let rows =
                crate::words::speaking("en", Base::Ten, || drawn(width, height, &Saying::Keys));
            let at = rows.iter().position(|row| row.contains(" keys ")).unwrap();
            let opened = &rows[at];
            let right = opened.trim_end().chars().last().unwrap();
            assert_eq!(right, '\u{2510}', "{width}x{height}: {opened}");
            for row in &rows[at + 1..at + 4] {
                let beside = row.trim_end();
                assert!(beside.ends_with('\u{2502}'), "{width}x{height}: {row}");
                assert!(
                    !beside.ends_with("\u{2502}\u{2502}") && !beside.ends_with("\u{2502}\u{2510}"),
                    "a pane's border beside the box at {width}x{height}: {row}"
                );
            }
        }
    }

    #[test]
    fn a_refusal_stays_beside_what_was_typed_until_it_is_mended() {
        let refused = typed("ping", Some("that wants an address after it"));
        let rows =
            crate::words::speaking("en", Base::Ten, || drawn(48, 20, &Saying::Typing(refused)));
        assert_eq!(rows[18], " \u{25cf} that wants an address after it");
        assert_eq!(rows[19], " : ping\u{258f}");
    }

    #[test]
    fn a_line_typed_past_the_edge_keeps_its_end_and_the_cursor_in_sight() {
        let long = typed(&format!("join 333:{}:3333", "a".repeat(60)), None);
        let rows = crate::words::speaking("en", Base::Ten, || drawn(48, 20, &Saying::Typing(long)));
        assert!(
            rows[19].starts_with(" : \u{2026}") && rows[19].ends_with("a:3333\u{258f}"),
            "{}",
            rows[19]
        );
        assert!(rows[19].width() <= 48);
    }

    #[test]
    fn every_size_a_terminal_can_be_dragged_to_draws_without_stopping() {
        let copy = crate::node::sources::Sighting {
            address: "192.0.2.9:3333".into(),
            said_in: 8,
            heard: crate::node::sources::Heard {
                from: crate::node::sources::Source::ThisNetwork,
                epoch: 9,
            },
        };
        let watch = Watch::quiet(vec![copy]);
        let states = || {
            [
                Saying::Nothing,
                Saying::Keys,
                Saying::Which(typed("2", Some("there are 333 of them"))),
                Saying::Typing(typed("tor sideways", Some("that wants on or off after it"))),
            ]
        };
        let log = [crate::screen::Heard {
            at: "12:00:00".to_owned(),
            said: "증언     이 노드에 대해 서명된 것이 없습니다\n         두 번째 줄".to_owned(),
        }];
        for (tag, base) in [("en", Base::Ten), ("ko", Base::Twelve)] {
            crate::words::speaking(tag, base, || {
                for width in [0, 1, 2, 5, 12, 30, 47, 48, 61, 62, 63, 100, 200, 400] {
                    for height in [0, 1, 2, 3, 4, 7, 12, 20, 50] {
                        for saying in states() {
                            let mut terminal =
                                Terminal::new(TestBackend::new(width, height)).unwrap();
                            terminal
                                .draw(|frame| everything(frame, &watch, &log, &saying))
                                .unwrap();
                        }
                    }
                }
            });
        }
    }

    #[test]
    fn a_narrow_terminal_still_shows_the_count_and_the_standing_before_the_lines() {
        let mut watch = Watch::quiet(Vec::new());
        (watch.answering, watch.roll) = (3, 5);
        watch.name = format!("333{}c", "ab".repeat(30));
        watch.epoch = n333_core::Epoch::now();
        let rows = crate::words::speaking("en", Base::Ten, || {
            drawn_of(&watch, 48, 20, &Saying::Nothing)
        });
        // The name gives way to the epoch and the time left.
        assert!(!rows[0].contains("333abab"), "{}", rows[0]);
        let epoch = format!("epoch {} ", watch.epoch.0);
        assert!(rows[0].contains(&epoch), "{}", rows[0]);
        assert!(rows[0].ends_with("to the boundary"), "{}", rows[0]);
        assert_eq!(rows[1], " ANSWERING 3   silent 2   roll 5");
        assert_eq!(rows[2], " on nobody's roll.");
        assert!(rows[3].contains("log · times in UTC"), "{}", rows[3]);
    }

    #[test]
    fn the_header_says_which_epoch_of_the_line_where_there_is_room_for_it() {
        let mut watch = Watch::quiet(Vec::new());
        watch.line = Some(3);
        let top = |width| {
            crate::words::speaking("en", Base::Ten, || {
                drawn_of(&watch, width, 20, &Saying::Nothing)
            })[0]
                .clone()
        };
        let wide = top(130);
        assert!(
            wide.starts_with(" 333   333   epoch 9, this line's 3rd   "),
            "{wide}"
        );
        assert!(wide.ends_with(" to the boundary"), "{wide}");
        let narrow = top(48);
        assert!(!narrow.contains("this line"), "{narrow}");
        assert!(narrow.contains("epoch 9   "), "{narrow}");
    }

    #[test]
    fn in_english_the_countdown_is_said_as_it_was() {
        let said = crate::words::speaking("en", Base::Ten, || {
            [remaining(2 * 3600 + 5 * 60 + 9), remaining(4 * 60 + 5)]
        });
        assert_eq!(said, ["2h 05m".to_owned(), "4m 5s".to_owned()]);
    }
}
