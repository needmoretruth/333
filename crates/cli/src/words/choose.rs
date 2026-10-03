//! Which language to speak and which base to count in, chosen once at the start.
//!
//! The language: `--language`, then `THE333_LANGUAGE`, then the one saved with
//! `333 language <TAG>` in the node's directory, then English. Each of the first three
//! is a person asking by name. The system's locale is not asked: a machine set up in
//! one language is often used by somebody who reads another, and a client that changed
//! its words because of a setting nobody made for it was the wrong default. The locale
//! is still read for the one thing it does describe, whether the terminal can show
//! more than ASCII.
//!
//! A tag is matched the way RFC 4647 looks one up: `ko-KR` is tried as itself, then as
//! `ko`. Any tag can be asked for. The ones that exist are whatever folders there are.

use super::count::Base;

/// A language somebody asked for, and how.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Asked {
    /// The tag, as asked for.
    pub(crate) tag: String,
    /// Whether a person named it, rather than English being taken for want of one.
    pub(crate) by_name: bool,
}

/// What the environment says, one variable at a time. Empty is not set.
pub(crate) type Environment<'a> = &'a dyn Fn(&str) -> Option<String>;

/// The language asked for, by the flag, the variable or the saved choice.
#[must_use]
pub(crate) fn asked(flag: Option<&str>, env: Environment<'_>, saved: Option<String>) -> Asked {
    let named = [flag.map(str::to_owned), env("THE333_LANGUAGE"), saved]
        .into_iter()
        .flatten()
        .map(|tag| tag.trim().to_owned())
        .find(|tag| !tag.is_empty());
    match named {
        Some(tag) => Asked { tag, by_name: true },
        None => Asked {
            tag: super::catalog::ENGLISH.to_owned(),
            by_name: false,
        },
    }
}

/// The folder that answers a tag, trying less of the tag each time.
#[must_use]
pub(crate) fn matching(asked: &str, there: &[String]) -> Option<String> {
    let mut tag = asked;
    loop {
        if let Some(found) = there.iter().find(|one| one.eq_ignore_ascii_case(tag)) {
            return Some(found.clone());
        }
        tag = tag.get(..tag.rfind('-')?)?;
    }
}

/// Whether this terminal's locale says it can show UTF-8.
///
/// `LC_ALL`, then `LC_CTYPE`, then `LANG`: the first one set decides. With none set a
/// Unix terminal is in the C locale, which is ASCII; Windows' own console is not
/// described by these at all and shows Unicode.
#[must_use]
pub(crate) fn utf8(env: Environment<'_>) -> bool {
    ["LC_ALL", "LC_CTYPE", "LANG"]
        .into_iter()
        .find_map(env)
        .map_or(cfg!(windows), |locale| {
            let locale = locale.to_ascii_lowercase();
            locale.contains("utf-8") || locale.contains("utf8")
        })
}

/// The base asked for, by the flag or `THE333_COUNT_IN`, and what was wrong with the
/// variable if it named no base at all.
#[must_use]
pub(crate) fn base(flag: Option<Base>, env: Environment<'_>) -> (Base, Option<String>) {
    let (asked, wrong) = match flag {
        Some(base) => (base, None),
        None => match env("THE333_COUNT_IN") {
            None => (Base::Ten, None),
            Some(value) => match Base::named(&value) {
                Ok(base) => (base, None),
                Err(_) => (Base::Ten, Some(value)),
            },
        },
    };
    (asked.shown(utf8(env)), wrong)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with(pairs: &'static [(&'static str, &'static str)]) -> impl Fn(&str) -> Option<String> {
        move |name| {
            pairs
                .iter()
                .find(|(key, _)| *key == name)
                .map(|(_, value)| (*value).to_owned())
        }
    }

    #[test]
    fn the_flag_comes_before_the_variable_and_the_variable_before_the_saved_choice() {
        let env = with(&[("THE333_LANGUAGE", "es")]);
        let saved = || Some("ko".to_owned());
        assert_eq!(asked(Some("jbo"), &env, saved()).tag, "jbo");
        assert_eq!(asked(None, &env, saved()).tag, "es");
        let from_the_file = asked(None, &with(&[]), saved());
        assert_eq!(from_the_file.tag, "ko");
        assert!(from_the_file.by_name);
        assert_eq!(asked(Some(" "), &with(&[]), None).tag, "en");
    }

    #[test]
    fn the_system_locale_never_chooses_the_language() {
        for locale in ["LC_ALL", "LC_MESSAGES", "LANG"] {
            let env = |name: &str| (name == locale).then(|| "ko_KR.UTF-8".to_owned());
            let asked = asked(None, &env, None);
            assert_eq!(asked.tag, "en", "{locale}");
            assert!(!asked.by_name, "{locale}");
        }
    }

    #[test]
    fn a_tag_falls_back_to_less_of_itself_and_then_to_nothing() {
        let there = vec!["en".to_owned(), "ko".to_owned(), "zh-Hant".to_owned()];
        assert_eq!(matching("ko-KR", &there).as_deref(), Some("ko"));
        assert_eq!(matching("zh-hant-TW", &there).as_deref(), Some("zh-Hant"));
        assert_eq!(matching("KO", &there).as_deref(), Some("ko"));
        assert_eq!(matching("de-DE", &there), None);
    }

    #[test]
    fn twelve_on_a_terminal_that_cannot_show_its_digits_uses_x_and_e() {
        let ascii = with(&[("LANG", "C")]);
        assert_eq!(base(Some(Base::Twelve), &ascii).0, Base::TwelveAscii);
        let unicode = with(&[("LC_ALL", "ko_KR.utf8")]);
        assert_eq!(base(Some(Base::Twelve), &unicode).0, Base::Twelve);
        let asked = with(&[("THE333_COUNT_IN", "dozen"), ("LANG", "en_US.UTF-8")]);
        assert_eq!(base(None, &asked), (Base::Ten, Some("dozen".to_owned())));
    }
}
