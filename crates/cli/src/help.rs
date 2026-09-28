//! The words of `333 --help`: what each command and each flag is for, from the catalogs.
//!
//! The definition in [`crate::typed`] names, where clap would take words, the key of
//! those words instead: `help = "help-serve-tor"` for `--tor` under `serve`. Once the
//! words are chosen, each key is replaced by what it says, before clap builds the
//! definition, because the build is where clap decides whether a command has a longer
//! account to offer and copies the flags every command takes into each one.
//!
//! What clap makes for itself at the build, `--help`, `--version` and `help`, is put in
//! words after it, as clap would build it for a parse; nothing is added or taken away
//! then, because the build is also where clap writes down where each flag is.
//!
//! What clap itself says around them, `Usage:` and the headings and the rest, is in
//! [`frame`] for every language but English, where clap's own words are left as they
//! are. What it says when a command line is refused is in [`refused`].

mod frame;
pub(crate) mod refused;

use clap::builder::StyledStr;
use clap::{Arg, ArgAction, Command};

use crate::words::catalog::ENGLISH;

/// The definition, built, with every word a person reads written in.
#[must_use]
pub(crate) fn spoken(command: Command) -> Command {
    let command = in_words(command.long_version(crate::version::long()));
    let english = crate::words::current().tag().eq_ignore_ascii_case(ENGLISH);
    built(command, english)
}

/// One command and everything under it, with each key in the definition replaced by
/// what it says.
///
/// Before the build, because the build is where clap decides whether there is a
/// longer account to offer with `--help`, and copies the flags every command takes.
fn in_words(command: Command) -> Command {
    let mut command = command;
    if let Some(key) = key(command.get_about()) {
        command = command.about(said(&key));
    }
    if let Some(key) = key(command.get_long_about()) {
        command = command.long_about(said(&key));
    }
    command.mut_args(explained).mut_subcommands(in_words)
}

/// One flag or argument, with its keys replaced by what they say.
fn explained(arg: Arg) -> Arg {
    let mut arg = arg;
    if let Some(key) = key(arg.get_help()) {
        arg = arg.help(said(&key));
    }
    if let Some(key) = key(arg.get_long_help()) {
        arg = arg.long_help(said(&key));
    }
    arg
}

/// The key a definition names instead of words, if it names one.
fn key(text: Option<&StyledStr>) -> Option<String> {
    let text = text?.to_string();
    text.starts_with("help-").then_some(text)
}

/// What clap makes for itself at the build — `--help`, `--version` and `help` — in
/// words, and in any language but English what clap says around them, for this
/// command and every one under it.
///
/// Each is built first, the way a parse builds it, which is what makes them; not with
/// `Command::build`, which would give `help` a copy of every command instead of their
/// names. Nothing is added or taken away after that: clap has written down where each
/// flag is.
fn built(command: Command, english: bool) -> Command {
    let mut command = command;
    let _ = command.render_usage();
    let helping = command.get_name() == "help";
    let styles = command.get_styles().clone();
    if helping {
        command = command.about(said("help-print-this"));
    }
    let mut command = command.mut_args(|arg| {
        let arg = own(arg, helping);
        if english {
            arg
        } else {
            frame::arg(arg, &styles)
        }
    });
    if !english {
        command = frame::command(command);
    }
    for sub in command.get_subcommands_mut() {
        *sub = built(std::mem::take(sub), english);
    }
    command
}

/// A flag clap makes for itself, in words; any other as it is.
fn own(arg: Arg, helping: bool) -> Arg {
    let id = arg.get_id().as_str().to_owned();
    match (id.as_str(), arg.get_action()) {
        ("help", ArgAction::Help) if arg.get_long_help().is_some() => arg
            .help(said("help-print-help-more"))
            .long_help(said("help-print-help-summary")),
        ("help", ArgAction::Help) => arg.help(said("help-print-help")),
        ("version", ArgAction::Version) => arg.help(said("help-print-version")),
        // The one argument of clap's own `help`.
        ("subcommand", _) if helping => arg.help(said("help-print-for")),
        _ => arg,
    }
}

/// One message, as clap shows it. The one key that is given something is the one
/// that counts: which of the 333 there are is a count, written in the base this counts
/// in, like the index it asks for.
fn said(key: &str) -> String {
    let text = if key == "help-say-index" {
        words!("help-say-index", last = n333_core::signal::SIGNAL_COUNT - 1)
    } else {
        crate::words::text(key, &[])
    };
    paragraphs(&text)
}

/// A catalog's lines as clap's paragraphs: a line break inside a paragraph is a space,
/// because clap lays the words out itself, and a blank line begins a new one.
fn paragraphs(text: &str) -> String {
    text.split("\n\n")
        .map(|paragraph| paragraph.split('\n').collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[cfg(test)]
mod tests;
