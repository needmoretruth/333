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
    assert_eq!(pages.len(), 37);
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
        "serve --help",
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
    let page = page("ko", "serve -h");
    assert!(page.contains("[기본값: 0.0.0.0:3333]"), "{page}");
    assert!(page.contains("[별칭: --no-upnp]"), "{page}");
    let args = ["333", "serve", "--no-upnp"].map(OsString::from).to_vec();
    let read = crate::words::speaking("ko", Base::Ten, || crate::typed::parse_from(args));
    assert!(matches!(
        read.map(|cli| cli.command),
        Ok(crate::typed::Command::Serve {
            no_router: true,
            ..
        })
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
        said.contains("\n\n사용법: 333 [OPTIONS] <COMMAND>"),
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
