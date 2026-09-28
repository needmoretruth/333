//! The left column — the count, this node's standing, and what was said — kept apart
//! because it is the part of the screen that turns this node's numbers into words.

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use unicode_width::UnicodeWidthStr as _;

use n333_core::presence::{Standing, WINDOW_EPOCHS};

use super::right::flush;
use super::{ahead, marked, padded, titled};
use crate::node::sources::Counts;
use crate::screen::watch::{Said, Watch, Where};
use crate::words::Arg;

/// The left column: the count, then this node, then what was said.
///
/// Above all of it, when there is one, another copy of this node's name: it is the
/// one thing on this screen that the person has to act on and that nothing else
/// will act on for them.
pub(super) fn this_node<'a>(watch: &'a Watch, area: Rect) -> Paragraph<'a> {
    let width = usize::from(area.width).saturating_sub(4);
    let mut lines = another_copy(watch, width);
    lines.extend(numbers(watch, width));
    let you = words!("screen-draw-left-you");
    lines.extend([Line::raw(""), Line::styled(you, Modifier::BOLD)]);
    lines.extend(standing(&watch.standing, width));
    if watch.unseen {
        lines.extend(unseen(width));
    }
    lines.push(Line::raw(""));
    let rows = usize::from(area.height).saturating_sub(2 + lines.len());
    lines.extend(said(&watch.said, rows, width));
    Paragraph::new(lines).block(titled(words!("screen-draw-left-title")))
}

/// The count, the roll, the addresses and what was witnessed, each number in one
/// column, with the ways the addresses were first heard of set in under their count.
fn numbers(watch: &Watch, width: usize) -> Vec<Line<'static>> {
    let silent = watch.roll.saturating_sub(watch.answering);
    let rows = [
        (words!("screen-draw-left-answering"), watch.answering),
        (words!("screen-draw-left-silent"), silent),
        (words!("screen-draw-left-roll"), watch.roll),
        (words!("screen-draw-left-known-where"), watch.addresses),
        (words!("screen-draw-left-witnessed"), watch.witnessed),
    ];
    let ways = heard_from(&watch.known);
    let column = number_column(&rows, &ways, width);
    let [answering, silent, roll, known, witnessed] = rows;
    let grey = Style::new().fg(Color::DarkGray);
    let mut lines = vec![
        counted(answering, true, column),
        counted(silent, false, column),
        Line::from(Span::styled("─────────", grey)),
        counted(roll, false, column),
        counted(known, false, column),
    ];
    lines.extend(ways.into_iter().map(|way| way_row(way, column)));
    lines.push(counted(witnessed, false, column));
    lines
}

/// What a terminal too narrow for the column still shows above the vigil, in the
/// column's own order: the count, another copy of this name if there is one, and the
/// first line of where this node stands.
pub(super) fn at_a_glance(watch: &Watch, width: usize) -> Vec<Line<'static>> {
    let bold = Style::new().add_modifier(Modifier::BOLD);
    let silent = watch.roll.saturating_sub(watch.answering);
    let mut lines = vec![Line::from(vec![
        Span::styled(
            format!(
                "{} {}",
                words!("screen-draw-left-answering"),
                words!("screen-draw-left-number", number = watch.answering)
            ),
            bold,
        ),
        Span::raw(format!(
            "   {} {}   {} {}",
            words!("screen-draw-left-silent"),
            words!("screen-draw-left-number", number = silent),
            words!("screen-draw-left-roll"),
            words!("screen-draw-left-number", number = watch.roll)
        )),
    ])];
    if !watch.copies.is_empty() {
        lines.extend(marked(
            &words!("screen-draw-left-another-copy"),
            width,
            Color::Red,
            bold,
        ));
    }
    lines.extend(standing(&watch.standing, width).into_iter().take(1));
    if watch.unseen {
        lines.extend(unseen(width).into_iter().take(1));
    }
    lines
}

/// Where the addresses this node holds were first heard of, as rows under the count of
/// them, naming only the ways some of them came. Nothing when it holds none.
fn heard_from(known: &Counts) -> Vec<(String, usize)> {
    [
        (words!("screen-draw-left-heard-by-hand"), known.by_hand),
        (words!("screen-draw-left-heard-nearby"), known.this_network),
        (
            words!("screen-draw-left-heard-meeting-point"),
            known.meeting_point,
        ),
        (
            words!("screen-draw-left-heard-from-us", peers = known.peers),
            known.from_peers,
        ),
        (words!("screen-draw-left-heard-not-noted"), known.unrecorded),
    ]
    .into_iter()
    .filter(|(_, count)| *count != 0)
    .collect()
}

/// How far in the rows' numbers start: past the widest name among them, the ways set
/// in under "known where" included, so every number stands in one column in every
/// language. Never less than the thirteen the column was drawn for, and never so far
/// that a number no longer fits.
fn number_column(rows: &[(String, usize)], ways: &[(String, usize)], width: usize) -> usize {
    let widest = rows
        .iter()
        .map(|(name, _)| name.width())
        .chain(ways.iter().map(|(name, _)| WAY_IN.width() + name.width()))
        .max()
        .unwrap_or(0);
    (widest + 1).max(13).min(width.saturating_sub(6).max(13))
}

/// How far a way an address was heard of is set in under the count of them.
const WAY_IN: &str = "  ";

/// One way addresses were first heard of, and how many, in the rows' number column.
fn way_row((name, count): (String, usize), column: usize) -> Line<'static> {
    let grey = Style::new().fg(Color::DarkGray);
    Line::from(vec![
        Span::styled(padded(&format!("{WAY_IN}{name}"), column), grey),
        Span::styled(words!("screen-draw-left-number", number = count), grey),
    ])
}

/// What this screen says while this node is unseen: that being reached is what is
/// counted, and the one order that reaches it from behind any router.
fn unseen(width: usize) -> Vec<Line<'static>> {
    let mut lines = marked(
        &words!("screen-draw-left-unseen"),
        width,
        Color::Yellow,
        Style::new().add_modifier(Modifier::BOLD),
    );
    lines.extend(broken(
        &words!("screen-draw-left-unseen-why"),
        width,
        Style::new().fg(Color::DarkGray),
    ));
    lines
}

/// What this screen says while another copy of this name is out there.
fn another_copy(watch: &Watch, width: usize) -> Vec<Line<'static>> {
    let Some(latest) = watch.copies.iter().max_by_key(|copy| copy.said_in) else {
        return Vec::new();
    };
    let loud = Style::new().add_modifier(Modifier::BOLD);
    let mut lines = marked(
        &words!("screen-draw-left-another-copy"),
        width,
        Color::Red,
        loud,
    );
    for (said, style) in [
        (words!("screen-draw-left-says-it-is-at"), Style::new()),
        (latest.address.clone(), Style::new()),
        (
            words!("screen-draw-left-in-epoch", epoch = latest.said_in),
            Style::new(),
        ),
        (words!("screen-draw-left-never-said-so"), Style::new()),
        (words!("screen-draw-left-stop-one"), loud),
        (
            words!("screen-draw-left-status-says-more"),
            Style::new().fg(Color::DarkGray),
        ),
    ] {
        lines.extend(broken(&said, width, style));
    }
    lines.push(Line::raw(""));
    lines
}

/// One number with its name, the number in the rows' column.
fn counted((name, count): (String, usize), loud: bool, column: usize) -> Line<'static> {
    let style = if loud {
        Style::new().add_modifier(Modifier::BOLD)
    } else {
        Style::new()
    };
    Line::from(vec![
        Span::styled(padded(&name, column), style),
        Span::styled(words!("screen-draw-left-number", number = count), style),
    ])
}

/// A phrase broken over several lines, each line styled alike and none wider than the
/// column: a translation broken for a wider column than this one is folded again.
fn broken(text: &str, width: usize, style: Style) -> Vec<Line<'static>> {
    text.split('\n')
        .flat_map(|line| flush(line, width))
        .map(|line| Line::styled(line.trim_start().to_owned(), style))
        .collect()
}

/// What this node's own record says about it, in the words that are true of it.
fn standing(standing: &Where, width: usize) -> Vec<Line<'static>> {
    let grey = Style::new().fg(Color::DarkGray);
    match standing {
        Where::OnNobodysRoll => {
            let mut lines = broken(
                &words!("screen-draw-left-on-nobodys-roll"),
                width,
                Style::new(),
            );
            lines.push(Line::styled(crate::commands::THE_PLACE, grey));
            lines
        }
        Where::BeganAlone => broken(&words!("screen-draw-left-began-alone"), width, Style::new()),
        Where::Waiting {
            joined,
            counted_from,
        } => {
            let mut lines = vec![
                Line::raw(words!("screen-draw-left-given-in", epoch = joined.0)),
                Line::raw(words!(
                    "screen-draw-left-counted-from",
                    epoch = counted_from.0
                )),
            ];
            lines.extend(broken(&words!("screen-draw-left-until-then"), width, grey));
            lines
        }
        Where::Counted {
            standing,
            silent_on,
        } => counted_standing(standing, *silent_on, width),
    }
}

/// The share of the window this node was present for, as a person reads a share: per
/// hundred, and so in ten whatever the person counts in, the way `%` is read.
fn share(per_mille: Option<u64>) -> String {
    per_mille.map_or_else(
        || words!("screen-draw-left-no-share"),
        |per_mille| {
            words!(
                "screen-draw-left-share",
                whole = Arg::exact(per_mille / 10),
                tenth = Arg::exact(per_mille % 10)
            )
        },
    )
}

/// The lines for a node its own record counts over the window.
fn counted_standing(standing: &Standing, silent_on: u64, width: usize) -> Vec<Line<'static>> {
    let mut lines = vec![Line::from(vec![
        Span::raw(words!(
            "screen-draw-left-present",
            present = standing.present,
            counted = standing.counted
        )),
        Span::styled(
            share(standing.per_mille()),
            Style::new().add_modifier(Modifier::BOLD),
        ),
    ])];
    if standing.qualifies() {
        lines.push(Line::styled(
            words!("screen-draw-left-counted"),
            Style::new().fg(Color::DarkGray),
        ));
    } else {
        lines.extend(marked(
            &words!("screen-draw-left-not-counted"),
            width,
            Color::Red,
            Style::new(),
        ));
    }
    if silent_on != 0 {
        lines.push(Line::styled(
            words!(
                "screen-draw-left-silent-on",
                epochs = silent_on,
                window = WINDOW_EPOCHS
            ),
            Style::new().fg(Color::DarkGray),
        ));
    }
    lines
}

/// The shape of what everybody said this epoch, in the rows left for it.
///
/// Nothing at all when not even a heading and one row fit: the vigil and `333 status`
/// both say it, and a heading over nothing would read as nothing having been said.
fn said(said: &Said, rows: usize, width: usize) -> Vec<Line<'static>> {
    let bold = Style::new().add_modifier(Modifier::BOLD);
    let grey = Style::new().fg(Color::DarkGray);
    if said.spoken == 0 {
        let mut lines = vec![Line::from(Span::styled(
            words!("screen-draw-left-said"),
            bold,
        ))];
        lines.extend(broken(
            &words!("screen-draw-left-nothing-said"),
            width,
            grey,
        ));
        return if lines.len() <= rows {
            lines
        } else {
            Vec::new()
        };
    }
    let mut lines = vec![Line::from(vec![
        Span::styled(padded(&words!("screen-draw-left-said"), 6), bold),
        Span::raw(words!(
            "screen-draw-left-spoke",
            spoken = said.spoken,
            observed = said.observed
        )),
    ])];
    // However many rows are left on this column, and a line saying what did not fit.
    // Cutting the tail off in silence would make a distribution look like the whole of
    // one, which is the one thing this screen must never do.
    let room = rows.saturating_sub(lines.len() + usize::from(said.mine.is_some()));
    let shown = if said.rows.len() <= room {
        said.rows.len()
    } else {
        room.saturating_sub(1)
    };
    if room < 2 && said.rows.len() > room {
        return Vec::new();
    }
    lines.extend(said.rows.iter().take(shown).map(|row| said_row(*row)));
    if said.rows.len() > shown {
        lines.push(Line::styled(
            format!(
                " {}",
                words!("screen-draw-left-more-said", rows = said.rows.len() - shown)
            ),
            grey,
        ));
    }
    if let Some(mine) = said.mine {
        lines.push(Line::styled(
            format!(" {}", words!("screen-draw-left-you-said", index = mine)),
            grey,
        ));
    }
    lines
}

/// One signal said this epoch: which, how many said it, their share, and whether that
/// share reached a third.
fn said_row((index, count, per_mille, reached): (u16, u64, Option<u64>, bool)) -> Line<'static> {
    let mark = if reached {
        format!("  {}", words!("screen-draw-left-a-third"))
    } else {
        String::new()
    };
    let style = if reached {
        Style::new().add_modifier(Modifier::BOLD)
    } else {
        Style::new()
    };
    Line::styled(
        format!(
            " {}{} {}{mark}",
            padded(&words!("screen-draw-left-signal", index = index), 6),
            ahead(&words!("screen-draw-left-number", number = count), 3),
            ahead(&share(per_mille), 6),
        ),
        style,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::sources::{Heard, Sighting, Source};

    fn watching(copies: Vec<Sighting>) -> Watch {
        Watch::quiet(copies)
    }

    fn text(lines: &[Line<'_>]) -> String {
        lines
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        use n333_core::Epoch;
        let english = |then: &dyn Fn() -> Vec<Line<'static>>| {
            crate::words::speaking("en", crate::words::count::Base::Ten, || text(&then()))
        };
        let copy = Sighting {
            address: "192.0.2.9:3333".into(),
            said_in: 8,
            heard: Heard {
                from: Source::ThisNetwork,
                epoch: 9,
            },
        };
        let watch = watching(vec![copy]);
        assert_eq!(
            english(&|| another_copy(&watch, 30)),
            "\u{25cf} ANOTHER COPY OF THIS NAME\nsays it is at\n192.0.2.9:3333\nin epoch 8.\n\
             this node never said so.\nstop one of them.\n`333 status` says more.\n"
        );
        assert_eq!(
            english(&|| vec![counted((words!("screen-draw-left-answering"), 7), true, 13)]),
            "ANSWERING    7"
        );
        assert_eq!(
            english(&|| standing(&Where::OnNobodysRoll, 30)),
            "on nobody's roll.\nnobody has handed you\nthe file yet. it takes\n\
             an invitation.\nthe333.dev"
        );
        assert_eq!(
            english(&|| standing(&Where::BeganAlone, 30)),
            "the start of your own line.\nnobody handed you the\nfile, so you are on no\n\
             roll. hand it on: whoever\ntakes it is counted."
        );
        let waiting = Where::Waiting {
            joined: Epoch(5),
            counted_from: Epoch(8),
        };
        assert_eq!(
            english(&|| standing(&waiting, 30)),
            "given the file in 5\ncounted from 8\nanswer everything until\n\
             then. none of it is banked."
        );
        let short = Standing {
            counted: 300,
            present: 100,
        };
        assert_eq!(
            english(&|| counted_standing(&short, 33, 30)),
            "present in 100 of 300 — 33.3%\n\u{25cf} not counted. two of every\n  \
             three is all that is asked\nsilent on 33 of 333"
        );
        let enough = Standing {
            counted: 3,
            present: 3,
        };
        assert_eq!(
            english(&|| counted_standing(&enough, 0, 30)),
            "present in 3 of 3 — 100.0%\nby your own record, counted"
        );
        let nothing = watch.said;
        assert_eq!(
            english(&|| said(&nothing, 40, 30)),
            "SAID\nnothing yet. 333 things\ncan be said."
        );
        let some = Said {
            rows: vec![
                (42, 2, Some(666), true),
                (7, 1, Some(333), false),
                (8, 1, Some(333), false),
            ],
            spoken: 3,
            observed: 9,
            mine: Some(42),
        };
        assert_eq!(
            english(&|| said(&some, 4, 30)),
            "SAID  3 of 9 spoke\n #42     2  66.6%  a third\n and 2 more said\n you said #42"
        );
    }

    #[test]
    fn a_share_is_per_hundred_in_every_base() {
        let short = Standing {
            counted: 300,
            present: 150,
        };
        let said = crate::words::speaking("en", crate::words::count::Base::Twelve, || {
            text(&counted_standing(&short, 0, 30))
        });
        assert!(said.contains(" — 50.0%"), "{said}");
    }

    #[test]
    fn where_addresses_came_from_names_only_the_ways_some_came() {
        let known = Counts {
            by_hand: 1,
            meeting_point: 2,
            from_peers: 4,
            peers: 2,
            ..Counts::default()
        };
        let said =
            crate::words::speaking("en", crate::words::count::Base::Ten, || heard_from(&known));
        let names: Vec<&str> = said.iter().map(|(name, _)| name.as_str()).collect();
        assert_eq!(names, ["by hand", "a meeting point", "from 2 of us"]);
        assert!(heard_from(&Counts::default()).is_empty());
    }

    #[test]
    fn every_number_in_the_rows_stands_in_one_column_in_every_language() {
        let mut watch = watching(Vec::new());
        (watch.roll, watch.addresses) = (3, 3);
        watch.known = Counts {
            by_hand: 1,
            meeting_point: 2,
            from_peers: 4,
            peers: 2,
            ..Counts::default()
        };
        for tag in ["en", "ko"] {
            let rows = crate::words::speaking(tag, crate::words::count::Base::Ten, || {
                text(&numbers(&watch, 30))
            });
            // Where the number of each row stands, as a terminal counts columns. The
            // way "from 2 of us" has a digit of its own before its number.
            let columns: Vec<usize> = rows
                .lines()
                .filter(|row| !row.starts_with('\u{2500}'))
                .map(|row| {
                    let at = row.rfind(' ').map_or(0, |at| at + 1);
                    row[..at].width()
                })
                .collect();
            assert_eq!(columns.len(), 8, "{tag}: {rows}");
            assert!(
                columns.windows(2).all(|two| two[0] == two[1]),
                "{tag}: {columns:?}\n{rows}"
            );
        }
    }

    #[test]
    fn an_unseen_node_is_told_the_order_that_reaches_it() {
        let mut watch = watching(Vec::new());
        watch.unseen = true;
        let area = Rect::new(0, 0, 34, 40);
        let said = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            text(&at_a_glance(&watch, 30)) + "\n" + &text(&unseen(30))
        });
        assert!(said.contains("\u{25cf} UNSEEN"), "{said}");
        assert!(said.contains("`: tor on` needs no router."), "{said}");
        let _ = this_node(&watch, area);
    }

    #[test]
    fn another_copy_of_this_name_is_the_first_thing_in_the_column() {
        let copy = Sighting {
            address: "192.0.2.9:3333".into(),
            said_in: 8,
            heard: Heard {
                from: Source::ThisNetwork,
                epoch: 9,
            },
        };
        let said = text(&another_copy(&watching(vec![copy]), 30));
        assert!(
            said.starts_with("\u{25cf} ANOTHER COPY OF THIS NAME"),
            "{said}"
        );
        assert!(said.contains("192.0.2.9:3333") && said.contains("epoch 8"));
        assert!(
            another_copy(&watching(Vec::new()), 30).is_empty(),
            "and nothing when there is none"
        );
    }
}
