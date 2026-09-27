//! The right column — the vigil's log, folded to fit — kept apart because breaking
//! lines to a pane's width is its own small algorithm with its own tests.

use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;

use super::titled;

/// The right column: what this node has done and been told, newest at the bottom.
pub(super) fn vigil(log: &[String], area: Rect) -> Paragraph<'static> {
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
