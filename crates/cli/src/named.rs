//! What a person is told on the run that makes their node's name.
//!
//! Beside the commands rather than inside one, because every command that can make a
//! name says it: `id`, `serve`, `join`, `bootstrap`, `status`, `say` and `ping`.

use std::path::Path;

use crate::identity_file::Origin;

/// Say, on the run that made this node's name, that it was made and where to keep it.
///
/// Every command that can make a name says this, and only on that run: it is the one
/// run on which the warning can still be acted on, and a person who began with `serve`
/// or `join` instead of `id` is owed it just the same. One function, so that nobody is
/// told less because of which command they typed first.
pub(crate) fn report(origin: Origin, home: &Path) {
    for line in said(origin, home) {
        aloud!("{line}");
    }
}

/// The lines [`report`] says, none unless this run made the name.
fn said(origin: Origin, home: &Path) -> Vec<String> {
    let Origin::Created { not_called } = origin else {
        return Vec::new();
    };
    vec![
        crate::commands::naming(not_called),
        words!("named-home", home = home.display().to_string()),
        words!("named-keep"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::words::count::Base;
    use crate::words::layout::{COLUMN, where_the_words_begin};

    fn first_run() -> Vec<String> {
        said(
            Origin::Created { not_called: 1_234 },
            Path::new("/tmp/333-node"),
        )
    }

    #[test]
    fn in_english_the_naming_says_exactly_what_it_said_before_the_words_were_moved() {
        let lines = crate::words::speaking("en", Base::Ten, first_run);
        assert_eq!(
            lines,
            [
                "called   1234 keys were made and not called. this one was.",
                "home     /tmp/333-node",
                "keep     that directory. lose it and you lose this name, every hour\n\
                 \x20        anyone ever witnessed for you, and any way of proving you\n\
                 \x20        were here. There is no recovery and there is no appeal.\n\
                 \x20        To move it to another machine: `333 pack <FILE>` here, then\n\
                 \x20        `333 unpack <FILE>` there.",
            ]
        );
    }

    #[test]
    fn a_name_that_was_already_there_is_not_named_again() {
        assert!(said(Origin::Loaded, Path::new("/tmp/333-node")).is_empty());
    }

    #[test]
    fn in_korean_every_line_of_the_naming_begins_its_words_in_the_same_column() {
        let lines = crate::words::speaking("ko", Base::Ten, first_run);
        for line in &lines {
            let mut rows = line.split('\n');
            let first = rows.next().unwrap();
            assert_eq!(where_the_words_begin(first), COLUMN, "{first:?}");
            for row in rows {
                let indent = row.chars().take_while(|c| *c == ' ').count();
                assert_eq!(indent, COLUMN, "{row:?}");
            }
        }
    }
}
