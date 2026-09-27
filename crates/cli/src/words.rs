//! The words a person reads, kept apart from the code that decides when to say them.
//!
//! The code says which thing it is saying, by a key, and with what: `aloud_in!("serve-
//! waiting", port = p)`. The words are in a catalog under `crates/cli/words/<tag>/`,
//! one file per command, in Project Fluent's `.ftl` format. A message with a
//! `.keyword` is a line in this client's voice: [`layout`] pads the keyword to the
//! column and indents every further line to it, so no catalog ever carries the spaces.
//! A message without one is a phrase, said inside some other line.
//!
//! WHY FLUENT RATHER THAN A FORMAT OF OUR OWN. Release builds of this tree (stripped,
//! thin LTO), in bytes, before any of this and with a probe of each choice reading one
//! embedded message, then with this module and its two catalogs as they are:
//!
//! | edition             | before     | own format probe | Fluent probe   | as built       |
//! |---------------------|-----------:|-----------------:|---------------:|---------------:|
//! | Standard (default)  | 18,988,848 |  +14,256         | +203,584       | +316,208       |
//! | Light (no features) |  7,071,904 |  +18,672         | +212,496       | +324,928       |
//!
//! Every column after the first includes the directory embedding and the width tables.
//! The own format was a probe with no plurals, no selectors and no error positions, so
//! its real cost is higher, but not by a hundred KB. What is left of "as built" over
//! the Fluent probe is this client's side: the code here, the language list, reading
//! a folder beside the node, and the catalogs themselves (twelve KB for two languages).
//!
//! The hundred and ninety KB between the two probes buys two things a small format
//! cannot have without Rust growing for every language. Plurals follow each language's
//! own rules (CLDR's, for every language it has), so Russian or Arabic arrive as a
//! folder like Korean did rather than as a new table here. And the files are a format
//! translators and their tools already read, with a parser that says which line it
//! could not read. The Vision's test is that a third language touches no Rust; with a
//! format of our own, the first language with a plural English does not have would.
//!
//! A catalog that is missing a message, or cannot say one, falls back to English for
//! that message alone. Numbers go through [`count`], which writes them in ten or in
//! twelve.

pub(crate) mod catalog;
pub(crate) mod choose;
pub(crate) mod count;
pub(crate) mod layout;

use std::borrow::Cow;
use std::path::Path;
use std::sync::OnceLock;

use fluent_bundle::types::{FluentNumber, FluentNumberOptions};
use fluent_bundle::{FluentArgs, FluentValue};

use catalog::Bundle;
use count::Base;

/// One message, in the words this process speaks, as a `String`.
///
/// `words!("id-name", name = n)` — the key, then each argument by the name the catalog
/// gives it. Counts are passed as numbers; anything else as text.
#[macro_export]
macro_rules! words {
    ($key:literal $(, $name:ident = $value:expr)* $(,)?) => {
        $crate::words::text($key, &[$((stringify!($name), $crate::words::Arg::from($value))),*])
    };
}

/// Say one message out loud: [`words!`] through [`aloud!`](crate::aloud).
#[macro_export]
macro_rules! aloud_in {
    ($key:literal $(, $name:ident = $value:expr)* $(,)?) => {
        $crate::aloud::say(format_args!("{}", $crate::words!($key $(, $name = $value)*)))
    };
}

/// The attribute that makes a message a line: its keyword.
const KEYWORD: &str = "keyword";

/// The words this process speaks, once they are chosen.
static WORDS: OnceLock<Words> = OnceLock::new();

/// One language, with English behind it.
pub(crate) struct Words {
    /// The tag of the folder these came from.
    tag: String,
    /// The base every count is written in.
    base: Base,
    /// The language chosen, with the node's own folder over what was built in.
    spoken: Bundle,
    /// What is said when the chosen language has nothing to say.
    english: Bundle,
}

impl Words {
    /// Read one language, and English to fall back on.
    pub(crate) fn open(
        tag: &str,
        base: Base,
        beside: Option<&Path>,
    ) -> (Self, Vec<catalog::Problem>) {
        let mut problems = Vec::new();
        let sources = catalog::sources(tag, beside, &mut problems);
        let spoken = catalog::bundle(tag, sources, &mut problems);
        let english = catalog::built_in(catalog::ENGLISH);
        let english = catalog::bundle(catalog::ENGLISH, english, &mut problems);
        let tag = tag.to_owned();
        let words = Self {
            tag,
            base,
            spoken,
            english,
        };
        (words, problems)
    }

    /// The language being spoken.
    pub(crate) fn tag(&self) -> &str {
        &self.tag
    }

    /// The base counts are written in.
    pub(crate) const fn base(&self) -> Base {
        self.base
    }

    /// Say one message.
    ///
    /// The chosen language only if it can say all of it: the message, every argument,
    /// and a keyword exactly when English has one. Otherwise English, which the checks
    /// make sure can say every key the code uses. A key nobody has is said as itself,
    /// which is a bug and is visible as one.
    fn say(&self, key: &str, args: &FluentArgs<'_>) -> String {
        let line = self
            .english
            .get_message(key)
            .is_some_and(|message| message.get_attribute(KEYWORD).is_some());
        said(&self.spoken, key, args, line, true)
            .or_else(|| said(&self.english, key, args, line, false))
            .unwrap_or_else(|| key.to_owned())
    }
}

/// One message from one catalog, laid out, if that catalog can say it.
fn said(
    bundle: &Bundle,
    key: &str,
    args: &FluentArgs<'_>,
    line: bool,
    strict: bool,
) -> Option<String> {
    let message = bundle.get_message(key)?;
    let mut errors = Vec::new();
    let words = bundle.format_pattern(message.value()?, Some(args), &mut errors);
    let keyword = message
        .get_attribute(KEYWORD)
        .map(|keyword| bundle.format_pattern(keyword.value(), Some(args), &mut errors));
    if strict && (!errors.is_empty() || keyword.is_some() != line) {
        return None;
    }
    Some(match keyword {
        Some(keyword) => layout::line(&keyword, &words),
        None => words.into_owned(),
    })
}

/// What is handed to a message: a count, which is written in the chosen base and can
/// choose a plural, or text, which is said exactly as it is.
#[derive(Debug, Clone)]
pub(crate) enum Arg<'a> {
    /// A count a person reads.
    Count {
        /// How many.
        number: u64,
        /// In threes, the way a large number is read.
        grouped: bool,
        /// Padded with zeros to this many digits.
        at_least: usize,
    },
    /// Anything that is not a count, numbers that are names included.
    Text(Cow<'a, str>),
}

impl Arg<'_> {
    /// A count with its digits in threes.
    #[expect(
        dead_code,
        reason = "the extinction line in status is its first user, once status is converted"
    )]
    pub(crate) const fn grouped(number: u64) -> Self {
        Self::Count {
            number,
            grouped: true,
            at_least: 0,
        }
    }

    /// A count padded with zeros, as the minutes of an hour are.
    pub(crate) const fn padded(number: u64, at_least: usize) -> Self {
        Self::Count {
            number,
            grouped: false,
            at_least,
        }
    }

    /// A number that names something rather than counting it: a port, a version, a
    /// byte count in an error. Never written in twelve.
    pub(crate) fn exact(number: impl std::fmt::Display) -> Self {
        Self::Text(Cow::Owned(number.to_string()))
    }

    /// What Fluent is handed. Fluent's numbers are f64, which holds every count this
    /// client has exactly: none comes near 2^53.
    fn fluent(&self) -> FluentValue<'_> {
        match self {
            Self::Count {
                number,
                grouped,
                at_least,
            } => FluentValue::Number(FluentNumber::new(
                *number as f64,
                FluentNumberOptions {
                    use_grouping: *grouped,
                    minimum_integer_digits: (*at_least != 0).then_some(*at_least),
                    ..FluentNumberOptions::default()
                },
            )),
            Self::Text(text) => FluentValue::String(Cow::Borrowed(text)),
        }
    }
}

/// Counts of every width a count comes in.
macro_rules! counts {
    ($($t:ty),*) => {$(
        impl From<$t> for Arg<'_> {
            fn from(number: $t) -> Self {
                Self::Count { number: u64::from(number), grouped: false, at_least: 0 }
            }
        }
    )*};
}
counts!(u8, u16, u32, u64);

impl From<usize> for Arg<'_> {
    fn from(number: usize) -> Self {
        Self::Count {
            number: u64::try_from(number).unwrap_or(u64::MAX),
            grouped: false,
            at_least: 0,
        }
    }
}

impl<'a> From<&'a str> for Arg<'a> {
    fn from(text: &'a str) -> Self {
        Self::Text(Cow::Borrowed(text))
    }
}

impl<'a> From<&'a String> for Arg<'a> {
    fn from(text: &'a String) -> Self {
        Self::Text(Cow::Borrowed(text))
    }
}

impl From<String> for Arg<'_> {
    fn from(text: String) -> Self {
        Self::Text(Cow::Owned(text))
    }
}

/// Say one message in the words this process speaks. What the macros call.
#[must_use]
pub(crate) fn text(key: &str, args: &[(&str, Arg<'_>)]) -> String {
    let mut fluent = FluentArgs::with_capacity(args.len());
    for (name, arg) in args {
        fluent.set(*name, arg.fluent());
    }
    current().say(key, &fluent)
}

/// The words this process speaks. English and ten until [`install`] chooses.
pub(crate) fn current() -> &'static Words {
    #[cfg(test)]
    if let Some(trial) = TRIAL.get() {
        return trial;
    }
    WORDS.get_or_init(|| Words::open(catalog::ENGLISH, Base::Ten, None).0)
}

/// Choose the language and the base, once, and say anything about the choice worth
/// saying: a catalog that could not be read, a language asked for by name that has no
/// words here, and that counts are in twelve if they are.
pub(crate) fn install(language: Option<&str>, count_in: Option<Base>, root: &Path) {
    let env = |name: &str| std::env::var(name).ok().filter(|value| !value.is_empty());
    let asked = choose::asked(language, &env);
    let beside = catalog::beside(root);
    let found = choose::matching(&asked.tag, &catalog::tags(Some(&beside)));
    let tag = found.as_deref().unwrap_or(catalog::ENGLISH);
    let (base, wrong_base) = choose::base(count_in, &env);
    let (words, problems) = Words::open(tag, base, Some(&beside));
    let _ = WORDS.set(words);
    for problem in problems {
        match problem {
            catalog::Problem::Broken { file, line } => {
                aloud_in!("words-broken", file = file, line = Arg::exact(line));
            }
            catalog::Problem::Unreadable { file, why } => {
                aloud_in!("words-unreadable", file = file, why = why);
            }
        }
    }
    if asked.by_name && found.is_none() {
        aloud_in!("words-no-catalog", tag = asked.tag);
    }
    if let Some(value) = wrong_base {
        aloud_in!("words-count-in-unknown", value = value);
    }
    if let Some((ten, eleven)) = base.past_nine() {
        aloud_in!(
            "words-counting-in-twelve",
            ten = ten.to_string(),
            eleven = eleven.to_string()
        );
    }
}

#[cfg(test)]
thread_local! {
    /// The words one test speaks, instead of the process's.
    static TRIAL: std::cell::Cell<Option<&'static Words>> = const { std::cell::Cell::new(None) };
}

/// Run `then` speaking another language or counting in another base, on this thread.
#[cfg(test)]
pub(crate) fn speaking<T>(tag: &str, base: Base, then: impl FnOnce() -> T) -> T {
    let words: &'static Words = Box::leak(Box::new(Words::open(tag, base, None).0));
    let before = TRIAL.replace(Some(words));
    let done = then();
    TRIAL.set(before);
    done
}

#[cfg(test)]
mod checks;
