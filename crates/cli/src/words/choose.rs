//! Which language to speak and which base to count in, chosen once at the start.
//!
//! The language: `--language`, then `THE333_LANGUAGE`, then the system's own locale
//! (`LC_ALL`, `LC_MESSAGES`, `LANG`, the order POSIX gives them), then English. The
//! first two are a person asking by name; the locale is the system guessing for them,
//! and a guess that finds no words is not worth a line every time the client starts.
//!
//! A tag is matched the way RFC 4647 looks one up: `ko-KR` is tried as itself, then as
//! `ko`. Any tag can be asked for. The ones that exist are whatever folders there are.

use super::count::Base;

/// A language somebody asked for, and how.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Asked {
    /// The tag, as asked for.
    pub(crate) tag: String,
    /// Whether a person named it, rather than the system's locale.
    pub(crate) by_name: bool,
}

/// What the environment says, one variable at a time. Empty is not set.
pub(crate) type Environment<'a> = &'a dyn Fn(&str) -> Option<String>;

/// The language asked for, by the flag, the variable or the locale.
#[must_use]
pub(crate) fn asked(flag: Option<&str>, env: Environment<'_>) -> Asked {
    let named = flag
        .map(str::to_owned)
        .filter(|tag| !tag.trim().is_empty())
        .or_else(|| env("THE333_LANGUAGE"));
    if let Some(tag) = named {
        return Asked {
            tag: tag.trim().to_owned(),
            by_name: true,
        };
    }
    let tag = ["LC_ALL", "LC_MESSAGES", "LANG"]
        .into_iter()
        .find_map(env)
        .and_then(|locale| from_locale(&locale))
        .unwrap_or_else(|| super::catalog::ENGLISH.to_owned());
    Asked {
        tag,
        by_name: false,
    }
}

/// A POSIX locale name as a language tag: `ko_KR.UTF-8` is `ko-KR`.
///
/// `C` and `POSIX` are the locale that means no locale, which is English.
#[must_use]
pub(crate) fn from_locale(locale: &str) -> Option<String> {
    let bare = locale.split(['.', '@']).next().unwrap_or_default().trim();
    match bare {
        "" => None,
        "C" | "POSIX" => Some(super::catalog::ENGLISH.to_owned()),
        tag => Some(tag.replace('_', "-")),
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
    fn the_flag_comes_before_the_variable_and_the_variable_before_the_locale() {
        let env = with(&[("THE333_LANGUAGE", "es"), ("LANG", "ko_KR.UTF-8")]);
        assert_eq!(asked(Some("jbo"), &env).tag, "jbo");
        assert_eq!(asked(None, &env).tag, "es");
        let env = with(&[("LANG", "ko_KR.UTF-8")]);
        let from_the_system = asked(None, &env);
        assert_eq!(from_the_system.tag, "ko-KR");
        assert!(!from_the_system.by_name);
    }

    #[test]
    fn lc_all_wins_over_lang_even_when_it_says_c() {
        let env = with(&[("LC_ALL", "C"), ("LANG", "ko_KR.UTF-8")]);
        assert_eq!(asked(None, &env).tag, "en");
        assert_eq!(asked(None, &with(&[])).tag, "en");
    }

    #[test]
    fn a_locale_name_becomes_a_tag() {
        assert_eq!(from_locale("ko_KR.UTF-8").as_deref(), Some("ko-KR"));
        assert_eq!(from_locale("sr_RS@latin").as_deref(), Some("sr-RS"));
        assert_eq!(from_locale("POSIX").as_deref(), Some("en"));
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
