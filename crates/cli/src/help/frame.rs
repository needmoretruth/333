//! What clap says around the words of each command — `Usage:`, the headings, and what
//! it adds after a flag's words — for a person who does not read English.
//!
//! clap writes these in English and lets each be replaced: the heading of each flag,
//! the heading of the commands, and a template for the page, which is where `Usage:`
//! is. What it adds after a flag, `[default: …]` and `[alias: …]`, it cannot be told
//! the words for, so it is told not to add them, and they are added here to the flag's
//! own words, laid out and styled the way clap lays out and styles its own.
//!
//! Left in English, because clap writes them itself and has no way to be told other
//! words: `[OPTIONS]` in the usage line, and the names of the values (`<DIR>`), which
//! are part of what is typed rather than a description of it.

use clap::builder::Styles;
use clap::{Arg, Command};

use super::said;

/// A command's page, in words: `Usage:` and the heading of its commands.
///
/// clap's own template, with the heading written out instead of `{usage-heading}`,
/// styled as clap styles that heading.
pub(super) fn command(command: Command) -> Command {
    let usage = *command.get_styles().get_usage();
    let template = format!(
        "{{before-help}}{{about-with-newline}}\n\
         {usage}{}:{usage:#} {{usage}}\n\n{{all-args}}{{after-help}}",
        self::usage()
    );
    command
        .help_template(template)
        .subcommand_help_heading(said("help-frame-commands"))
}

/// The word before the usage line, which a refusal ends with too.
pub(super) fn usage() -> String {
    said("help-frame-usage")
}

/// One flag or argument, in words: its heading, and what clap would add after it.
pub(super) fn arg(arg: Arg, styles: &Styles) -> Arg {
    let heading = if arg.is_positional() {
        said("help-frame-arguments")
    } else {
        said("help-frame-options")
    };
    let specs = specs(&arg, styles);
    let arg = arg.help_heading(heading);
    if specs.is_empty() {
        return arg;
    }
    let short = arg.get_help().map(ToString::to_string).unwrap_or_default();
    let long = arg
        .get_long_help()
        .map_or_else(|| short.clone(), ToString::to_string);
    let hidden: Vec<String> = arg
        .get_visible_aliases()
        .unwrap_or_default()
        .into_iter()
        .map(str::to_owned)
        .collect();
    let short_hidden = arg.get_visible_short_aliases().unwrap_or_default();
    // Taken away and given back as aliases nobody is shown, because clap's list of
    // them is the only way to take a visible one away; `--no-upnp` still works.
    let joined = |words: &str, between: &str, specs: &str| match words {
        "" => specs.to_owned(),
        _ => format!("{words}{between}{specs}"),
    };
    arg.hide_default_value(true)
        .visible_alias(None)
        .visible_short_alias(None)
        .aliases(hidden)
        .short_aliases(short_hidden)
        .help(joined(&short, " ", &specs.join(" ")))
        .long_help(joined(&long, "\n\n", &specs.join("\n")))
}

/// What clap would add after a flag's words, each in brackets, in its order: the
/// default, then the aliases.
fn specs(arg: &Arg, styles: &Styles) -> Vec<String> {
    let (context, value) = (*styles.get_context(), *styles.get_context_value());
    let mut specs = Vec::new();
    let defaults = arg.get_default_values();
    if arg.get_action().takes_values() && !arg.is_hide_default_value_set() && !defaults.is_empty() {
        let defaults: Vec<String> = defaults
            .iter()
            .map(|default| default.to_string_lossy().into_owned())
            .collect();
        specs.push(format!(
            "{context}[{}: {context:#}{value}{}{value:#}{context}]{context:#}",
            said("help-frame-default"),
            defaults.join(" ")
        ));
    }
    let short = arg.get_visible_short_aliases().unwrap_or_default();
    let long = arg.get_visible_aliases().unwrap_or_default();
    let aliases: Vec<String> = short
        .iter()
        .map(|alias| format!("{value}-{alias}{value:#}"))
        .chain(
            long.iter()
                .map(|alias| format!("{value}--{alias}{value:#}")),
        )
        .collect();
    if !aliases.is_empty() {
        let named = words!("help-frame-alias", aliases = aliases.len());
        specs.push(format!(
            "{context}[{named}: {context:#}{}{context}]{context:#}",
            aliases.join(&format!("{context}, {context:#}"))
        ));
    }
    specs
}
