//! The right column — the vigil's log, laid out to fit — kept apart because laying a
//! message out to a pane's width is its own small algorithm with its own tests.

use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;

use unicode_width::UnicodeWidthStr as _;

use super::titled;
use crate::screen::Heard;
use crate::words::layout;

/// The right column: what this node has done and been told, newest at the bottom.
pub(super) fn vigil(log: &[Heard], area: Rect) -> Paragraph<'static> {
    let width = usize::from(area.width).saturating_sub(4);
    let room = usize::from(area.height).saturating_sub(2);
    // Built from the newest backwards, because the newest line is the one that must be
    // on the screen. A pane filled from the top loses whatever happened last.
    let mut upward: Vec<String> = Vec::new();
    for heard in log.iter().rev() {
        let mut laid = laid_out(heard, width);
        laid.reverse();
        upward.append(&mut laid);
        if upward.len() >= room {
            break;
        }
    }
    upward.truncate(room);
    upward.reverse();
    Paragraph::new(upward.into_iter().map(Line::raw).collect::<Vec<Line<'_>>>())
        .block(titled(words!("screen-draw-right-title")))
}

/// The columns the time takes before the words: `12:00:00` and two spaces.
const STAMP: usize = 10;

/// One thing said, with its time before it, laid out to fit `width`.
///
/// The words are laid out again for the room after the time, so a sentence the
/// catalog broke for a file eighty columns wide reads as one sentence in a pane of
/// forty or of a hundred and sixty, and every further line starts in its words'
/// column. Where the pane is too narrow for the time to stand beside the words, the
/// time goes on a line of its own and the words take the whole width: ten columns of
/// indent out of fifteen would leave five for the words, and a node's name would come
/// out two letters at a time. Blank lines are left out: on a pane, a paragraph that
/// starts on a new line is parted enough, and the rows are worth more.
fn laid_out(heard: &Heard, width: usize) -> Vec<String> {
    if width == 0 {
        return Vec::new();
    }
    let stamp = STAMP.max(heard.at.width() + 2);
    let beside = width >= stamp + layout::COLUMN + 12;
    let words = layout::fold(&heard.said, if beside { width - stamp } else { width });
    let words = words.into_iter().filter(|line| !line.is_empty());
    if !beside {
        return std::iter::once(heard.at.clone()).chain(words).collect();
    }
    words
        .enumerate()
        .map(|(at, line)| {
            if at == 0 {
                format!("{}  {line}", heard.at)
            } else {
                format!("{}{line}", " ".repeat(stamp))
            }
        })
        .collect()
}

/// Words broken on spaces to fit a column, every line starting at the edge: for a
/// sentence in a column of its own rather than an entry in the log. Nothing when
/// there is no column.
pub(super) fn flush(text: &str, width: usize) -> Vec<String> {
    if width == 0 || text.trim().is_empty() {
        return Vec::new();
    }
    layout::wrapped("", text, 0, width)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn heard(said: &str) -> Heard {
        Heard {
            at: "12:00:00".to_owned(),
            said: said.to_owned(),
        }
    }

    #[test]
    fn a_message_is_laid_out_again_for_the_pane_under_its_words() {
        let said = "hand     an invitation names a place, not a person. it swears to nothing;\n\
                    \x20        whoever answers there proves themselves by holding a key.";
        let laid = laid_out(&heard(said), 44);
        assert_eq!(
            laid,
            [
                "12:00:00  hand     an invitation names a",
                "                   place, not a person. it",
                "                   swears to nothing;",
                "                   whoever answers there",
                "                   proves themselves by",
                "                   holding a key.",
            ]
        );
        let wide = laid_out(&heard(said), 160);
        assert_eq!(wide.len(), 1, "{wide:#?}");
    }

    #[test]
    fn a_pane_too_narrow_for_the_time_beside_the_words_puts_it_above_them() {
        let laid = laid_out(&heard("answer   127.0.0.1:43331"), 24);
        assert_eq!(laid, ["12:00:00", "answer   127.0.0.1:43331"]);
        assert!(
            laid_out(&heard("anything"), 0).is_empty(),
            "no pane, nothing"
        );
    }

    #[test]
    fn a_word_longer_than_the_column_is_cut_rather_than_lost() {
        // A node's name is sixty-four characters with nothing to break on.
        let folded = flush("name 333abcdefghijklmnop", 12);
        assert!(
            folded.iter().all(|line| line.chars().count() <= 12),
            "{folded:?}"
        );
        assert_eq!(folded.concat().replace(' ', ""), "name333abcdefghijklmnop");
    }

    #[test]
    fn letters_two_columns_wide_are_folded_by_the_columns_they_cover() {
        let folded = flush("이 노드에 대해 서명된 것이 없습니다 在任何时代都没有", 14);
        assert!(folded.iter().all(|line| line.width() <= 14), "{folded:?}");
        assert_eq!(
            folded.concat().replace(' ', ""),
            "이노드에대해서명된것이없습니다在任何时代都没有"
        );
    }

    #[test]
    fn a_line_that_fits_is_left_exactly_as_it_was() {
        assert_eq!(flush("short enough", 40), vec!["short enough".to_owned()]);
        assert_eq!(
            flush("exactly fourteen", 16),
            vec!["exactly fourteen".to_owned()],
            "a line as wide as the column fits it"
        );
        assert!(
            flush("anything", 0).is_empty(),
            "no column, nothing to draw"
        );
    }
}
