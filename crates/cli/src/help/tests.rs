use std::ffi::OsString;

use crate::words::count::Base;

/// Every page of help the client had before its words moved to the catalogs, as it
/// printed them in English: `==== <what was typed after 333>`, then the page, with the
/// spaces clap leaves at the ends of blank lines taken off.
const BEFORE: &str = include_str!("english.txt");

/// What the client says for one command line, in one language, as it would print it.
fn page(tag: &str, typed: &str) -> String {
    let args: Vec<OsString> = std::iter::once("333")
        .chain(typed.split(' '))
        .map(OsString::from)
        .collect();
    let refused = crate::words::speaking(tag, Base::Ten, || {
        crate::typed::parse_from(args).expect_err("help is what clap refuses with")
    });
    let rendered = crate::words::speaking(tag, Base::Ten, || refused.rendered());
    rendered
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn in_english_every_page_of_help_says_exactly_what_it_said_before() {
    let pages: Vec<&str> = BEFORE
        .split("==== ")
        .filter(|one| !one.is_empty())
        .collect();
    assert_eq!(pages.len(), 47);
    for one in pages {
        let (typed, before) = one.split_once('\n').unwrap();
        assert_eq!(page("en", typed), before.trim_end(), "333 {typed}");
    }
}

#[test]
fn in_korean_no_page_of_help_has_an_english_heading_left() {
    for typed in [
        "--help",
        "-h",
        "run --help",
        "say -h",
        "service --help",
        "help help",
    ] {
        let page = page("ko", typed);
        for english in [
            "Usage:",
            "Options:",
            "Arguments:",
            "Commands:",
            "[default:",
            "[alias",
            "Print ",
        ] {
            assert!(!page.contains(english), "333 {typed}: {english}\n{page}");
        }
        assert!(page.contains("사용법:"), "333 {typed}\n{page}");
    }
}

#[test]
fn in_korean_what_clap_adds_after_a_flag_is_said_and_the_alias_still_works() {
    let page = page("ko", "run -h");
    assert!(page.contains("[기본값: 0.0.0.0:3333]"), "{page}");
    assert!(page.contains("[별칭: --no-upnp]"), "{page}");
    let args = ["333", "serve", "--no-upnp"].map(OsString::from).to_vec();
    let read = crate::words::speaking("ko", Base::Ten, || crate::typed::parse_from(args));
    assert!(matches!(
        read.map(|cli| cli.command),
        Ok(Some(crate::typed::Command::Serve {
            no_router: true,
            ..
        }))
    ));
}

#[test]
fn in_korean_a_refused_command_line_says_so_with_what_clap_hands_over() {
    let said = page("ko", "--bogus");
    assert!(
        said.starts_with("오류: 예상하지 않은 '--bogus' 인자가 있습니다"),
        "{said}"
    );
    assert!(
        said.contains("\n\n사용법: 333 [OPTIONS] [COMMAND]"),
        "{said}"
    );
    assert!(
        said.ends_with("더 알아보려면 '--help' 옵션을 쓰십시오."),
        "{said}"
    );
    let both = page("ko", "status --sources --json");
    assert!(
        both.contains("'--sources' 인자는 '--json' 인자와 함께 쓸 수 없습니다"),
        "{both}"
    );
}

#[test]
fn the_highest_index_say_takes_is_written_in_the_base_it_is_typed_in() {
    let twelve = crate::words::speaking("en", Base::TwelveAscii, || super::said("help-say-index"));
    assert!(
        twelve.starts_with("Which of them, from 0 to 238,"),
        "{twelve}"
    );
}

#[test]
fn every_help_key_the_definition_names_is_one_english_has() {
    let command = crate::words::speaking("en", Base::Ten, || {
        super::spoken(<crate::typed::Cli as clap::CommandFactory>::command())
    });
    let mut unsaid = Vec::new();
    let mut each = vec![&command];
    while let Some(command) = each.pop() {
        let texts = [command.get_about(), command.get_long_about()]
            .into_iter()
            .chain(
                command
                    .get_arguments()
                    .flat_map(|arg| [arg.get_help(), arg.get_long_help()]),
            );
        unsaid.extend(
            texts
                .flatten()
                .map(ToString::to_string)
                .filter(|t| t.starts_with("help-")),
        );
        each.extend(command.get_subcommands());
    }
    assert!(unsaid.is_empty(), "{unsaid:?}");
}

#[test]
fn an_edition_without_the_screen_is_told_nothing_about_one() {
    for (with, _) in super::WITHOUT_THE_SCREEN {
        let without = super::in_this_edition(with, false);
        assert_ne!(without, with);
        assert_eq!(super::in_this_edition(with, true), with);
        for tag in ["en", "ko"] {
            let said = crate::words::speaking(tag, Base::Ten, || crate::words::text(without, &[]));
            assert_ne!(said, without, "{tag} has no words for {without}");
            for about in ["the screen", "screen's", "화면을", "화면의", "화면에"] {
                assert!(!said.contains(about), "{tag} {without}: {said}");
            }
        }
    }
}

/// The lines of one section of a page: everything after its heading, up to the next
/// line that is not indented, blank lines left out.
fn section<'a>(page: &'a str, heading: &str) -> Vec<&'a str> {
    page.lines()
        .skip_while(|line| *line != heading)
        .skip(1)
        .take_while(|line| line.is_empty() || line.starts_with(' '))
        .filter(|line| !line.is_empty())
        .collect()
}

#[test]
fn the_first_page_lists_the_everyday_commands_first_and_the_rest_apart() {
    for typed in ["-h", "--help"] {
        let page = page("en", typed);
        let everyday: Vec<&str> = section(&page, "Commands:")
            .iter()
            .filter_map(|line| line.split_whitespace().next())
            .collect();
        assert!(
            page.find("More commands:") < page.find("Options:"),
            "{page}"
        );
        assert_eq!(everyday, super::listing::EVERYDAY, "{page}");
        let more = section(&page, "More commands:");
        assert!(
            more.iter().any(|line| line.starts_with("  service ")),
            "{page}"
        );
        assert!(
            !more.iter().any(|line| line.starts_with("  start ")),
            "{page}"
        );
        assert!(page.contains("Usage: 333 [OPTIONS] [COMMAND]"), "{page}");
        let options = section(&page, "Options:").join("\n");
        assert!(options.contains("--language"), "{page}");
        assert!(!options.contains("--timeout"), "{page}");
        let advanced = section(&page, "Advanced:").join("\n");
        for flag in [
            "--timeout",
            "--dangerously-trust-directory-permissions",
            "--keep-everything",
            "--bridge",
            "--bridge-helper",
            "--count-in",
            "--data-dir",
        ] {
            assert!(advanced.contains(flag), "{flag}: {page}");
        }
    }
}

#[test]
fn a_command_s_page_leaves_the_advanced_flags_and_their_essays_to_the_first_page() {
    let page = page("en", "status --help");
    for absent in [
        "--timeout",
        "--data-dir",
        "--bridge",
        "A ceiling rather than a delay",
    ] {
        assert!(!page.contains(absent), "{absent}: {page}");
    }
    assert!(page.contains("--language"), "{page}");
    assert!(!page.contains("THE333_LANGUAGE"), "{page}");
    assert!(page.lines().count() < 25, "{page}");
    // Still read where they are typed.
    let args = ["333", "status", "--data-dir", "/x", "--timeout", "9"]
        .map(OsString::from)
        .to_vec();
    let read = crate::words::speaking("en", Base::Ten, || crate::typed::parse_from(args));
    assert!(read.is_ok_and(|cli| cli.timeout == 9));
}

#[test]
fn every_command_shows_one_example_of_itself() {
    let command = crate::words::speaking("en", Base::Ten, || {
        super::spoken(<crate::typed::Cli as clap::CommandFactory>::command())
    });
    let mut each: Vec<(String, &clap::Command)> = command
        .get_subcommands()
        .map(|sub| (sub.get_name().to_owned(), sub))
        .collect();
    while let Some((path, sub)) = each.pop() {
        if sub.get_name() == "help" {
            continue;
        }
        let example = sub.get_after_help().map(ToString::to_string);
        let wanted = format!("Example: 333 {path}");
        assert!(
            example
                .as_deref()
                .is_some_and(|text| text.starts_with(&wanted)),
            "{path}: {example:?}"
        );
        each.extend(
            sub.get_subcommands()
                .map(|inner| (format!("{path} {}", inner.get_name()), inner)),
        );
    }
}
