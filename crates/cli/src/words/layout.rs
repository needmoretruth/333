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
}
