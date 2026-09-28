//! What a command does when another 333 already has this node's directory.
//!
//! It does not wait for it and it does not open anything it would write. If the other
//! one is keeping the vigil and what was asked is something a vigil can be told, it is
//! handed over and carried out there, and this prints what the vigil said about it.
//! Anything else is refused, naming who has the directory and what to do instead.

use std::io::Write as _;
use std::process::ExitCode;

use crate::claim::Holder;
use crate::commands::Common;
use crate::words::Arg;

/// What a command wants, put the way a running vigil could be asked for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Wanted {
    /// Something a vigil carries out, in the screen's own words.
    Order(String),
    /// Something a vigil answers with a page that is the whole of what this prints,
    /// so nothing is added after it: `status --json` is read by programs.
    Page(String),
    /// This node's name, which can be read beside a vigil without writing anything.
    Name,
    /// A vigil of its own. There is one per directory.
    Vigil,
    /// Something that cannot be handed over, and why not.
    Kept(&'static str),
    /// The same, with why not already in the words of whoever asked, which a
    /// `&'static str` cannot be.
    KeptSaid(String),
}

/// Do what can be done beside the holder, and say so.
///
/// # Errors
/// Fails if the vigil stops answering part way, or what it said cannot be printed.
pub(crate) async fn run(
    common: &Common,
    holder: Holder,
    wanted: Wanted,
) -> anyhow::Result<ExitCode> {
    let home = common.paths.root();
    let who = who(holder);
    match wanted {
        Wanted::Order(order) => match crate::control::hand_over(home, &order).await? {
            Some(true) => {
                aloud_in!("elsewhere-done", vigil = the_vigil(holder));
                Ok(ExitCode::SUCCESS)
            }
            Some(false) => {
                aloud_in!("elsewhere-failed", vigil = the_vigil(holder));
                Ok(ExitCode::FAILURE)
            }
            None => Ok(busy(&who)),
        },
        Wanted::Page(order) => match crate::control::hand_over(home, &order).await? {
            Some(true) => Ok(ExitCode::SUCCESS),
            Some(false) => {
                // Beside the page and not in it: a program reads the page.
                let failed = words!("elsewhere-failed", vigil = the_vigil(holder));
                let _ = writeln!(std::io::stderr().lock(), "{failed}");
                Ok(ExitCode::FAILURE)
            }
            None => Ok(busy(&who)),
        },
        Wanted::Name => {
            let Some(identity) = crate::identity_file::load(&common.mistrust(), home)? else {
                aloud_in!("elsewhere-finding-its-name", who = &who);
                return Ok(ExitCode::FAILURE);
            };
            crate::commands::id::beside_the_vigil(&identity.node_id().to_string(), home);
            Ok(ExitCode::SUCCESS)
        }
        Wanted::Vigil if crate::control::answering(home).await => {
            aloud_in!("elsewhere-already-keeping", who = &who);
            Ok(ExitCode::FAILURE)
        }
        Wanted::Kept(why) if crate::control::answering(home).await => Ok(keeping(&who, why)),
        Wanted::KeptSaid(why) if crate::control::answering(home).await => Ok(keeping(&who, &why)),
        Wanted::Vigil | Wanted::Kept(_) | Wanted::KeptSaid(_) => Ok(busy(&who)),
    }
}

/// Refuse, when the vigil is kept here and what was asked is not for it to do.
fn keeping(who: &str, why: &str) -> ExitCode {
    crate::aloud::line(&kept(who, why));
    ExitCode::FAILURE
}

/// The line that refuses, with the reason in the column however it came.
///
/// A reason may come with the column already in it, and the line puts every further
/// line in the column itself: kept as it came, those lines would be in it twice.
fn kept(who: &str, why: &str) -> String {
    let why = why
        .lines()
        .map(str::trim_start)
        .collect::<Vec<_>>()
        .join("\n");
    words!("elsewhere-keeping", who = who, why = why)
}

/// What `333 tell` says when nobody holds the directory to be told.
#[must_use]
pub(crate) fn nobody_to_tell() -> ExitCode {
    aloud_in!("elsewhere-nobody-to-tell");
    ExitCode::FAILURE
}

/// Refuse, when whoever holds the directory cannot be handed anything.
fn busy(who: &str) -> ExitCode {
    if cfg!(unix) {
        aloud_in!("elsewhere-busy", who = who);
    } else {
        aloud_in!("elsewhere-busy-cannot-be-handed", who = who);
    }
    ExitCode::FAILURE
}

/// The other 333, by its process number when it wrote one down.
fn who(holder: Holder) -> String {
    match holder.pid {
        Some(pid) => words!("elsewhere-another-by-number", pid = Arg::exact(pid)),
        None => words!("elsewhere-another"),
    }
}

/// The vigil that was handed the order, by its process number when it wrote one down.
fn the_vigil(holder: Holder) -> String {
    match holder.pid {
        Some(pid) => words!("elsewhere-the-vigil-by-number", pid = Arg::exact(pid)),
        None => words!("elsewhere-the-vigil"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::words::count::Base;
    use crate::words::layout::{COLUMN, where_the_words_begin};

    const BY_NUMBER: Holder = Holder { pid: Some(1234) };
    const UNNUMBERED: Holder = Holder { pid: None };

    fn lines() -> Vec<String> {
        let other = who(BY_NUMBER);
        vec![
            words!("elsewhere-done", vigil = the_vigil(BY_NUMBER)),
            words!("elsewhere-failed", vigil = the_vigil(UNNUMBERED)),
            words!("elsewhere-finding-its-name", who = &other),
            words!("elsewhere-already-keeping", who = &other),
            kept(
                &other,
                "it looks for people only where it always does.\n\
                 \x20        Leave --meet out.",
            ),
            words!("elsewhere-nobody-to-tell"),
            words!("elsewhere-busy", who = &other),
            words!("elsewhere-busy-cannot-be-handed", who = who(UNNUMBERED)),
        ]
    }

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let said = crate::words::speaking("en", Base::Ten, lines);
        assert_eq!(
            said,
            [
                "done     carried out by the vigil kept in this directory (process 1234).",
                "failed   the vigil kept in this directory did not do that.",
                "busy     another 333 (process 1234) is still finding\n\
                 \x20        this node's name. Run this again when it has one.",
                "busy     another 333 (process 1234) is already keeping the vigil here, and\n\
                 \x20        one directory is one node. It can be told things from here:\n\
                 \x20        `333 say 7`, `333 join <invitation>`, `333 tell 'tor on'`. A\n\
                 \x20        second node needs a directory of its own, given with --data-dir.",
                "busy     another 333 (process 1234) is keeping the vigil here, and\n\
                 \x20        it looks for people only where it always does.\n\
                 \x20        Leave --meet out.",
                "unheard  nobody is keeping the vigil in this directory, so there is nobody to\n\
                 \x20        tell. `333 serve` keeps it, and then this works.",
                "busy     another 333 (process 1234) has this node's\n\
                 \x20        directory and is not a vigil that can be handed this. Nothing here\n\
                 \x20        was read or written. Run this again when it has finished.",
                "busy     another 333 has this node's\n\
                 \x20        directory. On this system a running 333 cannot be handed anything\n\
                 \x20        from another terminal yet, so nothing here was read or written. Type\n\
                 \x20        it into that one's screen after `:`, or stop it and run this again.",
            ]
        );
    }

    #[test]
    fn in_korean_every_line_of_elsewhere_begins_its_words_in_the_same_column() {
        for line in crate::words::speaking("ko", Base::Ten, lines) {
            assert!(!line.is_ascii(), "{line:?}");
            assert_eq!(where_the_words_begin(&line), COLUMN, "{line:?}");
        }
    }
}
