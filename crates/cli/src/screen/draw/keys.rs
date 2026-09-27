//! Every key and every order word, in a small box over the screen, until any key is
//! pressed — kept apart because it is drawn over everything else rather than in a
//! place of its own.
//!
//! The bottom line has room for the four keys and little more, and the order words
//! are otherwise only in the README. A person keeping the vigil should not need a
//! second window to learn what they can type into the first.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Clear, Paragraph};
use unicode_width::UnicodeWidthStr as _;

use super::right::flush;
use super::{padded, titled};

/// Each key and each order as it is typed, with what it does.
///
/// What is typed is never translated: the keys are the keys, and the order words are
/// the command line's own. The capitals stand for what goes there.
fn rows() -> [(&'static str, String); 13] {
    [
        ("q  esc  ctrl-c", words!("screen-draw-keys-leave")),
        ("s", words!("screen-draw-keys-say")),
        (":", words!("screen-draw-keys-order")),
        ("?", words!("screen-draw-keys-these")),
        ("ping ADDRESS", words!("screen-draw-keys-ping")),
        ("join INVITATION", words!("screen-draw-keys-join")),
        ("bootstrap", words!("screen-draw-keys-bootstrap")),
        ("say N", words!("screen-draw-keys-say")),
        ("tor on  tor off", words!("screen-draw-keys-tor")),
        ("bridge LINE", words!("screen-draw-keys-bridge")),
        ("helper PROGRAM", words!("screen-draw-keys-helper")),
        ("status", words!("screen-draw-keys-status")),
        ("quit", words!("screen-draw-keys-quit")),
    ]
}

/// How many of [`rows`] are keys; the rest are typed after `:`.
const KEYS: usize = 4;

/// Draw the box over whatever is in `area`, as large as it needs and no larger.
pub(super) fn every_key(frame: &mut Frame<'_>, area: Rect) {
    let rows = rows();
    let typed = rows
        .iter()
        .map(|(typed, _)| typed.width())
        .max()
        .unwrap_or(0);
    // Two for the frame and two for its padding, then what is typed, two spaces, and
    // the rest of the room for what it does.
    let inner = usize::from(area.width).saturating_sub(4);
    let widest = rows.iter().map(|(_, does)| does.width()).max().unwrap_or(0);
    let inner = inner.min(typed + 2 + widest);
    let beside = inner.saturating_sub(typed + 2);
    let bold = Style::new().add_modifier(Modifier::BOLD);
    let mut lines = Vec::new();
    for (at, (keys, does)) in rows.iter().enumerate() {
        if at == KEYS {
            lines.push(Line::raw(""));
            lines.push(Line::styled(words!("screen-draw-keys-typed-after"), bold));
        }
        lines.extend(row(keys, does, typed, beside));
    }
    let height = u16::try_from(lines.len() + 2).unwrap_or(u16::MAX);
    let width = u16::try_from(inner + 4).unwrap_or(u16::MAX);
    let place = centred(area, width, height);
    frame.render_widget(Clear, place);
    frame.render_widget(
        Paragraph::new(lines).block(
            titled(words!("screen-draw-keys-title"))
                .title_bottom(format!(" {} ", words!("screen-draw-keys-close"))),
        ),
        place,
    );
}

/// One key or order, with what it does beside it, or under it when there is no room
/// beside it for more than a word or two.
fn row(keys: &str, does: &str, typed: usize, beside: usize) -> Vec<Line<'static>> {
    let bold = Style::new().add_modifier(Modifier::BOLD);
    if beside < 12 {
        let mut lines = vec![Line::styled(keys.to_owned(), bold)];
        let under = (typed + 2 + beside).saturating_sub(2);
        lines.extend(
            flush(does, under)
                .into_iter()
                .map(|line| Line::raw(format!("  {line}"))),
        );
        return lines;
    }
    let mut lines = Vec::new();
    for (at, line) in flush(does, beside).into_iter().enumerate() {
        let lead = if at == 0 { keys } else { "" };
        lines.push(Line::from(vec![
            Span::styled(padded(lead, typed + 2), bold),
            Span::raw(line.trim_start().to_owned()),
        ]));
    }
    lines
}

/// A box of this size in the middle of `area`, or all of `area` when it is smaller.
fn centred(area: Rect, width: u16, height: u16) -> Rect {
    let (width, height) = (width.min(area.width), height.min(area.height));
    Rect {
        x: area.x + (area.width - width) / 2,
        y: area.y + (area.height - height) / 2,
        width,
        height,
    }
}
