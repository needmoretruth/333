//! The commands `333 --help` lists: the everyday ones first, and the rest under a
//! heading of their own.
//!
//! clap gives all the commands under one command a single heading, so the list is
//! written here, laid out and styled the way clap lays out and styles its own, and clap
//! is told not to list them. The usage line is written here too, because clap writes
//! `[COMMAND]` in it only for a command whose commands it lists.

use clap::Command;
use unicode_width::UnicodeWidthStr as _;

use super::said;

/// The commands a person reaches for first, in the order they are listed.
pub(super) const EVERYDAY: [&str; 11] = [
    "start", "stop", "restart", "status", "logs", "run", "invite", "join", "begin", "name",
    "language",
];

/// The first command, built, with its commands listed in two parts above its flags.
#[must_use]
pub(super) fn listed(command: Command, english: bool) -> Command {
    let styles = command.get_styles().clone();
    let (header, literal) = (*styles.get_header(), *styles.get_literal());
    let everyday: Vec<&Command> = EVERYDAY
        .iter()
        .filter_map(|name| command.find_subcommand(name))
        .collect();
    let rest: Vec<&Command> = command
        .get_subcommands()
        .filter(|sub| !EVERYDAY.contains(&sub.get_name()))
        .collect();
    let longest = command
        .get_subcommands()
        .map(|sub| sub.get_name().width())
        .max()
        .unwrap_or(0);
    let rows = |commands: &[&Command]| -> String {
        let row = |sub: &&Command| {
            let name = sub.get_name();
            let about = sub.get_about().map(ToString::to_string).unwrap_or_default();
            let gap = " ".repeat(longest - name.width() + 2);
            format!("  {literal}{name}{literal:#}{gap}{about}")
        };
        commands.iter().map(row).collect::<Vec<_>>().join("\n")
    };
    let list = format!(
        "{header}{}:{header:#}\n{}\n\n{header}{}:{header:#}\n{}",
        said("help-frame-commands"),
        rows(&everyday),
        said("help-frame-more-commands"),
        rows(&rest)
    );
    let (template, line) = framed(&command, english);
    let mut command = command
        .before_help(list)
        .help_template(template)
        .override_usage(line);
    for sub in command.get_subcommands_mut() {
        *sub = std::mem::take(sub).hide(true);
    }
    command
}

/// The page's template, with the list where clap would put what goes before the help,
/// and the usage line clap would have written had it listed the commands itself.
fn framed(command: &Command, english: bool) -> (String, String) {
    let styles = command.get_styles();
    let (usage, literal, placeholder) = (
        *styles.get_usage(),
        *styles.get_literal(),
        *styles.get_placeholder(),
    );
    let heading = if english {
        "{usage-heading}".to_owned()
    } else {
        format!("{usage}{}:{usage:#}", super::frame::usage())
    };
    let template = format!(
        "{{about-with-newline}}\n{heading} {{usage}}\n\n{{before-help}}{{all-args}}{{after-help}}"
    );
    let name = command.get_name();
    let line = format!(
        "{literal}{name}{literal:#} {placeholder}[OPTIONS]{placeholder:#} \
         {placeholder}[COMMAND]{placeholder:#}"
    );
    (template, line)
}
