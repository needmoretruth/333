//! One line as this client says it: a keyword in a column of nine, then the words.
//!
//! The padding is made here and never written in a catalog. A translator writes the
//! keyword and the sentence; the column comes out right in every language, including
//! the ones whose letters take two columns on a terminal, which counting characters
//! would get wrong by exactly the width of the keyword.

use unicode_width::UnicodeWidthStr as _;

/// The column the words begin in, counted from zero. A keyword of eight columns and
/// one space fills it exactly.
pub(crate) const COLUMN: usize = 9;

/// Lay out a keyword and what follows it.
///
/// A keyword wider than eight columns gets one space and pushes the words over rather
/// than touching them. Every further line of the words starts in the same column as
/// the first, so a sentence broken over three lines still reads as one entry in a log.
#[must_use]
pub(crate) fn line(keyword: &str, words: &str) -> String {
    let keyword = keyword.trim();
    let width = keyword.width();
    let column = COLUMN.max(width + 1);
    let mut laid = String::with_capacity(keyword.len() + words.len() + column * 4);
    laid.push_str(keyword);
    laid.extend(std::iter::repeat_n(' ', column.saturating_sub(width)));
    for (at, text) in words.split('\n').enumerate() {
        if at != 0 {
            laid.push('\n');
            if !text.is_empty() {
                laid.extend(std::iter::repeat_n(' ', column));
            }
        }
        laid.push_str(text);
    }
    laid
}

/// The display column the words of one laid-out line begin in: after the keyword and
/// the spaces that follow it.
#[must_use]
pub(crate) fn where_the_words_begin(first_line: &str) -> usize {
    let keyword = first_line.split(' ').next().unwrap_or_default();
    let spaces = first_line
        .get(keyword.len()..)
        .unwrap_or_default()
        .chars()
        .take_while(|c| *c == ' ')
        .count();
    keyword.width() + spaces
}

/// The fewest columns worth lining a further line up under the words for. Fewer, and
/// a sentence comes out a word or two a line down a strip at the right; it reads
/// better starting at the edge.
const NARROWEST: usize = 12;

/// What was said, laid out again for a place `width` columns wide.
///
/// A catalog breaks its lines where the file is wide enough, not where a terminal or a
/// pane is, so a line shown as it was written breaks mid-sentence wherever the two
/// differ. Here the words of each sentence are put back together and broken again at
/// the width they are shown in, with every further line in the column the first line's
/// words begin in, as [`line`] would have laid them out for that width.
///
/// What is not a sentence is kept as it is and only broken if it does not fit: a line
/// set in further than the words (a row of a table), a line with two spaces running in
/// it (columns: an address, then an epoch, then a clock), and a line with no keyword
/// before it, which begins a thing said of its own. A blank line still parts two
/// paragraphs.
#[must_use]
pub(crate) fn fold(said: &str, width: usize) -> Vec<String> {
    let width = width.max(1);
    let mut laid = Vec::new();
    let mut lines = said.split('\n').peekable();
    while let Some(first) = lines.next() {
        if first.trim().is_empty() {
            laid.push(String::new());
            continue;
        }
        let mut after = Vec::new();
        while let Some(next) = lines.next_if(|line| line.is_empty() || line.starts_with(' ')) {
            after.push(next);
        }
        one_entry(first, &after, width, &mut laid);
    }
    laid
}

/// One piece of an entry, as it is laid out again.
enum Block<'a> {
    /// Words of a sentence, joined from however many lines they were broken over.
    Prose(String),
    /// A line kept as it is, set in by this many columns.
    Set(usize, &'a str),
    /// A blank line between two paragraphs.
    Blank,
}

/// One keyword and everything said after it, laid out for `width`.
fn one_entry(first: &str, after: &[&str], width: usize, laid: &mut Vec<String>) {
    if after.is_empty() && first.width() <= width {
        laid.push(first.trim_end().to_owned());
        return;
    }
    let (head, column, words) = keyword_of(first);
    let hang = if width >= column + NARROWEST {
        column
    } else {
        0
    };
    let lead = if hang == column {
        head.to_owned()
    } else {
        format!("{} ", head.trim_end()).trim_start().to_owned()
    };
    let mut blocks: Vec<Block<'_>> = Vec::new();
    placed(&mut blocks, column, column, words);
    for line in after {
        let text = line.trim();
        let indent = line.len() - line.trim_start_matches(' ').len();
        placed(&mut blocks, column, indent, text);
    }
    if blocks.is_empty() {
        laid.push(head.trim_end().to_owned());
        return;
    }
    for (at, block) in blocks.iter().enumerate() {
        match block {
            Block::Blank => laid.push(String::new()),
            Block::Prose(text) => {
                let start = if at == 0 {
                    lead.clone()
                } else {
                    " ".repeat(hang)
                };
                laid.extend(wrapped(&start, text, hang, width));
            }
            Block::Set(indent, text) => {
                let indent = if width >= indent + NARROWEST {
                    *indent
                } else {
                    0
                };
                let start = if at == 0 {
                    lead.clone()
                } else {
                    " ".repeat(indent)
                };
                laid.extend(wrapped(&start, text, indent, width));
            }
        }
    }
}

/// Where one line of an entry goes: onto the sentence before it, when it is more of
/// one in the words' own column, or on a line of its own.
fn placed<'a>(blocks: &mut Vec<Block<'a>>, column: usize, indent: usize, text: &'a str) {
    let text = text.trim_end();
    if text.is_empty() {
        if !blocks.is_empty() {
            blocks.push(Block::Blank);
        }
        return;
    }
    if indent != column || text.contains("  ") {
        blocks.push(Block::Set(indent, text));
        return;
    }
    match blocks.last_mut() {
        Some(Block::Prose(sentence)) => {
            sentence.push(' ');
            sentence.push_str(text);
        }
        _ => blocks.push(Block::Prose(text.to_owned())),
    }
}

/// The keyword and its padding, the column the words begin in, and the words, of the
/// first line of an entry. A line that starts with spaces is words said under the line
/// before it, in that column; a line with no keyword is words from the edge.
fn keyword_of(first: &str) -> (&str, usize, &str) {
    let leading = first.len() - first.trim_start_matches(' ').len();
    if leading > 0 {
        let (head, words) = first.split_at_checked(leading).unwrap_or(("", first));
        return (head, leading, words);
    }
    let keyword = first.split(' ').next().unwrap_or_default();
    let after = first.get(keyword.len()..).unwrap_or_default();
    let spaces = after.len() - after.trim_start_matches(' ').len();
    let words = after.get(spaces..).unwrap_or_default();
    let is_one = spaces >= 2 || (spaces == 1 && keyword.width() + 1 >= COLUMN);
    if words.is_empty() || !is_one {
        return ("", 0, first);
    }
    let head = first.get(..keyword.len() + spaces).unwrap_or_default();
    (head, keyword.width() + spaces, words)
}

/// `text` after `start`, broken on spaces to fit `width`, every further line set in by
/// `hang` columns.
#[must_use]
pub(crate) fn wrapped(start: &str, text: &str, hang: usize, width: usize) -> Vec<String> {
    let mut laid = Vec::new();
    let mut lead = start.to_owned();
    let mut rest = text.trim_end();
    loop {
        let room = width.saturating_sub(lead.width()).max(1);
        let Some(ends_at) = cut(rest, room) else {
            laid.push(format!("{lead}{rest}").trim_end().to_owned());
            return laid;
        };
        let (head, tail) = rest.split_at_checked(ends_at).unwrap_or((rest, ""));
        laid.push(format!("{lead}{}", head.trim_end()));
        rest = tail.trim_start();
        if rest.is_empty() {
            return laid;
        }
        lead = " ".repeat(hang);
    }
}

/// Where to cut a line that does not fit in `room` columns: after the last space that
/// fits, or at the edge when no space does. Nothing when all of it fits.
///
/// Measured in the columns a terminal gives each letter, not in letters: Korean and
/// Chinese take two each. A letter wider than the whole room is let through on a line
/// of its own rather than never placed, which would loop for ever.
fn cut(line: &str, room: usize) -> Option<usize> {
    use unicode_width::UnicodeWidthChar as _;
    let (mut covered, mut space) = (0_usize, None);
    for (at, letter) in line.char_indices() {
        let wide = letter.width().unwrap_or(0);
        if covered + wide > room {
            let edge = if at == 0 { at + letter.len_utf8() } else { at };
            return Some(space.unwrap_or(edge));
        }
        if letter == ' ' && covered > 0 {
            space = Some(at);
        }
        covered += wide;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_short_keyword_is_padded_to_the_column() {
        assert_eq!(line("name", "333abc"), "name     333abc");
        assert_eq!(line("rejoined", "x"), "rejoined x");
    }

    #[test]
    fn a_keyword_in_wide_letters_is_padded_by_what_it_covers() {
        // 이름 is two letters and four columns.
        let laid = line("이름", "333abc");
        assert_eq!(laid, "이름     333abc");
        assert_eq!(where_the_words_begin(&laid), COLUMN);
    }

    #[test]
    fn a_keyword_too_wide_for_the_column_still_leaves_one_space() {
        assert_eq!(line("remembered", "x\ny"), "remembered x\n           y");
    }

    #[test]
    fn every_further_line_starts_where_the_first_line_s_words_did() {
        assert_eq!(
            line("keep", "that directory.\nlose it"),
            "keep     that directory.\n         lose it"
        );
        assert_eq!(
            line("keep", "a\n\nb"),
            "keep     a\n\n         b",
            "no trailing spaces"
        );
    }

    /// The catalog's `serve-waiting-for-the-file`, broken where the file breaks it.
    const WAITING: &str = "waiting  this node has not been given the file, so nothing is counted for it\n\
        \x20        yet and there is nothing yet for anybody to witness. It cannot make\n\
        \x20        one. It only ever arrives from somebody who already holds it, and\n\
        \x20        the two of you sign for the handover. Ask for an\n\
        \x20        invitation, then `333 join 333:their.address:3333`.";

    #[test]
    fn a_sentence_broken_for_the_file_is_broken_again_for_the_width_it_is_shown_in() {
        let laid = fold(WAITING, 48);
        assert!(laid.iter().all(|line| line.width() <= 48), "{laid:#?}");
        assert!(
            laid.iter()
                .skip(1)
                .all(|line| line.starts_with("         ") && !line.starts_with("          ")),
            "{laid:#?}"
        );
        let words = |text: &str| text.split_whitespace().collect::<Vec<_>>().join(" ");
        assert_eq!(
            words(&laid.join(" ")),
            words(WAITING),
            "no word lost or moved"
        );
        // The file's short line, "Ask for an", is not kept as a short line.
        assert!(
            !laid
                .iter()
                .any(|line| line.trim_end().ends_with("Ask for an")),
            "{laid:#?}"
        );
        let wide = fold(WAITING, 200);
        assert_eq!(wide.len(), 2, "{wide:#?}");
    }

    #[test]
    fn a_line_that_fits_the_width_is_left_as_it_was() {
        assert_eq!(
            fold("answer   127.0.0.1:3333", 80),
            ["answer   127.0.0.1:3333"]
        );
        assert_eq!(fold("roll     1 of us", 16), ["roll     1 of us"]);
    }

    #[test]
    fn rows_and_columns_are_kept_and_a_blank_line_still_parts_paragraphs() {
        let said = "witness  333ab…cd  epoch 9  clocks together  (answered)\n\
            \x20        you said: I received the file.\n\
            \x20        it is written in two hands.\n\
            \n\
            \x20        That is all.\n\
            \x20 by hand                1";
        assert_eq!(
            fold(said, 100),
            [
                "witness  333ab…cd  epoch 9  clocks together  (answered)",
                "         you said: I received the file. it is written in two hands.",
                "",
                "         That is all.",
                "  by hand                1",
            ]
        );
    }

    #[test]
    fn a_row_too_wide_is_broken_on_a_space_under_its_own_column() {
        let name = "3336".repeat(16);
        let said = format!("witness  epoch 9 answered by {name}, who came here to be asked");
        let laid = fold(&said, 100);
        assert_eq!(laid.len(), 2, "{laid:#?}");
        assert!(laid.iter().all(|line| line.width() <= 100), "{laid:#?}");
        assert!(
            laid[1].starts_with("         ") && laid[1].ends_with(" to be asked"),
            "{laid:#?}"
        );
    }

    #[test]
    fn letters_two_columns_wide_are_laid_out_by_the_columns_they_cover() {
        let said = "대기     이 노드는 아직 파일을 받지 못했으므로 아무것도 세어지지 않고,\n\
            \x20        아직 누구도 증언할 것이 없습니다.";
        let laid = fold(said, 40);
        assert!(laid.iter().all(|line| line.width() <= 40), "{laid:#?}");
        assert!(laid[0].starts_with("대기     이 노드는"), "{laid:#?}");
        assert!(
            laid.iter()
                .skip(1)
                .all(|line| line.starts_with("         ")),
            "{laid:#?}"
        );
    }

    #[test]
    fn too_narrow_to_line_up_under_the_words_starts_at_the_edge() {
        let laid = fold("answer   one two three four five six", 16);
        assert!(laid.iter().all(|line| line.width() <= 16), "{laid:#?}");
        assert_eq!(laid[0], "answer one two");
        assert!(!laid[1].starts_with(' '), "{laid:#?}");
    }

    #[test]
    fn words_said_under_the_line_before_keep_its_column() {
        let said = "         It goes out to everyone this node reaches, and they pass it on.\n\
            \x20        Once every 333 minutes you may say one of 333 things.";
        let laid = fold(said, 60);
        assert!(
            laid.iter().all(|line| line.starts_with("         ")),
            "{laid:#?}"
        );
        assert!(laid.iter().all(|line| line.width() <= 60), "{laid:#?}");
    }
}
