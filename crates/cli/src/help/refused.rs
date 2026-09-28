//! What clap says when it refuses a command line, for a person who does not read English.
//!
//! clap takes a formatter for its refusals, and hands it what went wrong as parts:
//! which kind of refusal, which flag, which value, what it might have meant. This
//! writes those parts in the catalog's words, in the shape clap's own formatter gives
//! them: the refusal, any tip, the usage line, and where to read more. English is left
//! to clap's own formatter, whose words these are.
//!
//! Left in English: what clap's own suggestions say (`to pass '--x' as a value, use
//! '-- --x'`), because clap hands them over already written; and the reason a value was
//! refused, which the code that read the value wrote.

use std::fmt::Write as _;

use clap::builder::{StyledStr, Styles};
use clap::error::{ContextKind, ContextValue, Error, ErrorFormatter, ErrorKind};

/// The formatter a refusal is said with, in any language but English.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Spoken;

impl ErrorFormatter for Spoken {
    fn format_error(error: &Error<Self>) -> StyledStr {
        let styles = Styles::default();
        let (bad, literal) = (*styles.get_error(), *styles.get_literal());
        let mut text = format!("{bad}{}:{bad:#} ", line(words!("help-refused-error")));
        text.push_str(&what(error, &styles));
        tips(error, &styles, &mut text);
        if let Some(ContextValue::StyledStr(usage)) = error.get(ContextKind::Usage) {
            // clap hands the usage over already headed, in English.
            let usage = usage.ansi().to_string();
            let heading = format!("{}:", super::frame::usage());
            text.push_str("\n\n");
            text.push_str(&usage.replacen("Usage:", &heading, 1));
        }
        let help = format!("{literal}--help{literal:#}");
        let _ = write!(
            text,
            "\n\n{}\n",
            line(words!("help-refused-try", help = help))
        );
        StyledStr::from(text)
    }
}

/// One message as one line: clap lays out what it says itself, so a line break in a
/// catalog is a space here.
fn line(text: String) -> String {
    super::paragraphs(&text)
}

/// A string from the refusal's parts.
fn part(error: &Error<Spoken>, kind: ContextKind) -> Option<&str> {
    match error.get(kind) {
        Some(ContextValue::String(text)) => Some(text),
        _ => None,
    }
}

/// A count from the refusal's parts.
fn count(error: &Error<Spoken>, kind: ContextKind) -> Option<u64> {
    match error.get(kind) {
        Some(ContextValue::Number(n)) => Some(n.unsigned_abs() as u64),
        _ => None,
    }
}

/// What was refused, in one sentence and any list that goes with it.
fn what(error: &Error<Spoken>, styles: &Styles) -> String {
    let (bad, literal, good) = (
        *styles.get_invalid(),
        *styles.get_literal(),
        *styles.get_valid(),
    );
    let arg = part(error, ContextKind::InvalidArg).map(|arg| format!("{bad}{arg}{bad:#}"));
    let named =
        part(error, ContextKind::InvalidArg).map(|arg| format!("{literal}{arg}{literal:#}"));
    let value = part(error, ContextKind::InvalidValue);
    let sub = part(error, ContextKind::InvalidSubcommand).map(|sub| format!("{bad}{sub}{bad:#}"));
    match (error.kind(), arg, value) {
        (ErrorKind::ArgumentConflict, ..) => conflict(error, styles),
        (ErrorKind::NoEquals, Some(arg), _) => line(words!("help-refused-no-equals", arg = arg)),
        (ErrorKind::InvalidValue, Some(arg), Some("")) => {
            line(words!("help-refused-value-missing", arg = arg))
        }
        (ErrorKind::InvalidValue | ErrorKind::ValueValidation, Some(_), Some(value)) => {
            let value = format!("{bad}{value}{bad:#}");
            let arg = named.unwrap_or_default();
            let mut said = line(words!(
                "help-refused-value-invalid",
                value = value,
                arg = arg
            ));
            if let Some(why) = std::error::Error::source(error) {
                let _ = write!(said, ": {why}");
            }
            said.push_str(&listed(error, ContextKind::ValidValue, "possible", &good));
            said
        }
        (ErrorKind::TooManyValues, Some(_), Some(value)) => line(words!(
            "help-refused-too-many",
            value = format!("{bad}{value}{bad:#}"),
            arg = named.unwrap_or_default()
        )),
        (ErrorKind::TooFewValues, Some(_), _) => line(words!(
            "help-refused-too-few",
            wanted = count(error, ContextKind::MinValues).unwrap_or_default(),
            arg = named.unwrap_or_default(),
            given = count(error, ContextKind::ActualNumValues).unwrap_or_default()
        )),
        (ErrorKind::WrongNumberOfValues, Some(_), _) => line(words!(
            "help-refused-wrong-number",
            wanted = count(error, ContextKind::ExpectedNumValues).unwrap_or_default(),
            arg = named.unwrap_or_default(),
            given = count(error, ContextKind::ActualNumValues).unwrap_or_default()
        )),
        (ErrorKind::UnknownArgument, Some(arg), _) => {
            line(words!("help-refused-unknown", arg = arg))
        }
        (ErrorKind::InvalidSubcommand, ..) if sub.is_some() => line(words!(
            "help-refused-unknown-subcommand",
            subcommand = sub.unwrap_or_default()
        )),
        (ErrorKind::MissingSubcommand, ..) if sub.is_some() => {
            let mut said = line(words!(
                "help-refused-no-subcommand",
                command = sub.unwrap_or_default()
            ));
            said.push_str(&listed(
                error,
                ContextKind::ValidSubcommand,
                "subcommands",
                &good,
            ));
            said
        }
        (ErrorKind::MissingRequiredArgument, ..) => {
            let mut said = line(words!("help-refused-missing"));
            if let Some(ContextValue::Strings(missing)) = error.get(ContextKind::InvalidArg) {
                for one in missing {
                    let _ = write!(said, "\n  {good}{one}{good:#}");
                }
            }
            said
        }
        (ErrorKind::InvalidUtf8, ..) => line(words!("help-refused-not-utf8")),
        // Nothing this knows the parts of: what clap would say without them.
        (kind, ..) => std::error::Error::source(error).map_or_else(
            || kind.as_str().unwrap_or_default().to_owned(),
            ToString::to_string,
        ),
    }
}

/// Two things that cannot be given together.
fn conflict(error: &Error<Spoken>, styles: &Styles) -> String {
    let bad = *styles.get_invalid();
    let arg = part(error, ContextKind::InvalidArg);
    let prior = error.get(ContextKind::PriorArg);
    if let (Some(arg), Some(ContextValue::String(before))) = (arg, prior)
        && arg == before
    {
        return line(words!(
            "help-refused-twice",
            arg = format!("{bad}{arg}{bad:#}")
        ));
    }
    let Some(arg) = arg.or_else(|| part(error, ContextKind::InvalidSubcommand)) else {
        return line(words!("help-refused-conflict-unnamed"));
    };
    let arg = format!("{bad}{arg}{bad:#}");
    match prior {
        Some(ContextValue::String(with)) => line(words!(
            "help-refused-conflict",
            arg = arg,
            with = format!("{bad}{with}{bad:#}")
        )),
        Some(ContextValue::Strings(with)) => {
            let mut said = line(words!("help-refused-conflict-list", arg = arg));
            for one in with {
                let _ = write!(said, "\n  {bad}{one}{bad:#}");
            }
            said
        }
        _ => line(words!("help-refused-conflict-others", arg = arg)),
    }
}

/// The values or commands that would have done, as clap lists them after the refusal.
fn listed(
    error: &Error<Spoken>,
    kind: ContextKind,
    which: &str,
    good: &clap::builder::styling::Style,
) -> String {
    let Some(ContextValue::Strings(names)) = error.get(kind) else {
        return String::new();
    };
    if names.is_empty() {
        return String::new();
    }
    let names: Vec<String> = names
        .iter()
        .map(|name| format!("{good}{name}{good:#}"))
        .collect();
    let heading = match which {
        "possible" => line(words!("help-refused-possible-values")),
        _ => line(words!("help-refused-subcommands")),
    };
    format!("\n  [{heading}: {}]", names.join(", "))
}

/// What clap thinks might have been meant, each on a line after the refusal.
fn tips(error: &Error<Spoken>, styles: &Styles, text: &mut String) {
    let good = *styles.get_valid();
    let tip = format!("{good}{}:{good:#}", line(words!("help-refused-tip")));
    let mut first = true;
    for kind in [
        ContextKind::SuggestedSubcommand,
        ContextKind::SuggestedArg,
        ContextKind::SuggestedValue,
    ] {
        let names: Vec<String> = match error.get(kind) {
            Some(ContextValue::String(name)) => vec![name.clone()],
            Some(ContextValue::Strings(names)) => names.clone(),
            _ => continue,
        };
        let many = names.len();
        let names = names
            .iter()
            .map(|name| format!("'{good}{name}{good:#}'"))
            .collect::<Vec<_>>()
            .join(", ");
        let similar = match kind {
            ContextKind::SuggestedSubcommand => line(words!(
                "help-refused-similar-subcommand",
                many = many,
                names = names
            )),
            ContextKind::SuggestedArg => line(words!(
                "help-refused-similar-argument",
                many = many,
                names = names
            )),
            _ => line(words!(
                "help-refused-similar-value",
                many = many,
                names = names
            )),
        };
        text.push_str(if first { "\n\n" } else { "\n" });
        first = false;
        let _ = write!(text, "  {tip} {similar}");
    }
    if let Some(ContextValue::StyledStrs(suggestions)) = error.get(ContextKind::Suggested) {
        if first {
            text.push('\n');
        }
        for suggestion in suggestions {
            let _ = write!(text, "\n  {tip} {}", suggestion.ansi());
        }
    }
}
