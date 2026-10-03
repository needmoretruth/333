//! What a command line clap refused asks to be read in, and where its node is.

use std::ffi::OsString;
use std::path::PathBuf;

use crate::words::count::Base;

/// What a command line asks to be read in and where its node is, picked out by hand
/// from one clap refused.
#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct Asked {
    /// `--language`.
    pub(super) language: Option<String>,
    /// `--count-in`, if it names a base.
    pub(super) count_in: Option<Base>,
    /// `--data-dir`, where a folder of catalogs beside the node may be.
    pub(super) data_dir: Option<PathBuf>,
}

impl Asked {
    /// Every `--language`, `--count-in` and `--data-dir` before a `--`, the last of
    /// each winning, as clap would have it; given as `--flag value` or `--flag=value`.
    pub(super) fn from(args: &[OsString]) -> Self {
        let mut asked = Self::default();
        let mut words = args.iter().skip(1).map(|arg| arg.to_string_lossy());
        while let Some(word) = words.next() {
            if word == "--" {
                break;
            }
            let (flag, given) = match word.split_once('=') {
                Some((flag, value)) => (flag.to_owned(), Some(value.to_owned())),
                None => (word.into_owned(), None),
            };
            if !["--language", "--count-in", "--data-dir"].contains(&flag.as_str()) {
                continue;
            }
            let Some(value) = given.or_else(|| words.next().map(|value| value.into_owned())) else {
                break;
            };
            match flag.as_str() {
                "--language" => asked.language = Some(value),
                "--count-in" => asked.count_in = Base::named(&value).ok(),
                _ => asked.data_dir = Some(PathBuf::from(value)),
            }
        }
        asked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_refused_line_is_read_for_its_language_base_and_directory_by_hand() {
        let args = |line: &str| -> Vec<OsString> { line.split(' ').map(OsString::from).collect() };
        let asked = Asked::from(&args(
            "333 --bogus --language=ko serve --count-in twelve --data-dir /tmp/n --language es",
        ));
        assert_eq!(
            asked,
            Asked {
                language: Some("es".to_owned()),
                count_in: Some(Base::Twelve),
                data_dir: Some(PathBuf::from("/tmp/n")),
            }
        );
        let after_the_end = Asked::from(&args("333 tell -- --language ko"));
        assert_eq!(after_the_end, Asked::default());
        let unfinished = Asked::from(&args("333 --count-in nine --language"));
        assert_eq!(unfinished, Asked::default());
    }
}
