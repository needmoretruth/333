//! The checks that keep the catalogs and the code saying the same things.
//!
//! Several people convert lines to keys at once and several more translate them, and
//! every way the two can drift apart shows up as a line said as its own key, a
//! placeholder printed as `{$name}`, or a translation nobody's code ever asks for.
//! Each of those is a test here rather than something a reviewer has to notice.

mod built;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use fluent_syntax::ast::{
    Entry, Expression, InlineExpression, Pattern, PatternElement, VariantKey,
};
use unicode_width::UnicodeWidthStr as _;

use super::catalog::{self, ENGLISH, Source};

/// What one message asks for: its attributes and the variables it uses anywhere.
#[derive(Debug, PartialEq, Eq)]
struct Shape {
    attributes: BTreeSet<String>,
    variables: BTreeSet<String>,
}

/// Every message in some files, by key, with the file each came from.
fn messages(sources: &[Source]) -> BTreeMap<String, (String, Shape)> {
    let mut found = BTreeMap::new();
    for source in sources {
        let resource = fluent_syntax::parser::parse_runtime(source.text.as_str())
            .unwrap_or_else(|(resource, _)| resource);
        for entry in resource.body {
            let Entry::Message(message) = entry else {
                continue;
            };
            let mut variables = BTreeSet::new();
            let attributes = message
                .attributes
                .iter()
                .map(|a| a.id.name.to_owned())
                .collect();
            for pattern in message
                .value
                .iter()
                .chain(message.attributes.iter().map(|a| &a.value))
            {
                in_pattern(pattern, &mut variables);
            }
            let shape = Shape {
                attributes,
                variables,
            };
            let before = found.insert(message.id.name.to_owned(), (source.name.clone(), shape));
            assert!(
                before.is_none(),
                "{} is said twice in one language",
                message.id.name
            );
        }
    }
    found
}

fn in_pattern(pattern: &Pattern<&str>, variables: &mut BTreeSet<String>) {
    for element in &pattern.elements {
        if let PatternElement::Placeable { expression } = element {
            in_expression(expression, variables);
        }
    }
}

fn in_expression(expression: &Expression<&str>, variables: &mut BTreeSet<String>) {
    match expression {
        Expression::Select { selector, variants } => {
            in_inline(selector, variables);
            for variant in variants {
                // A number is handed to Fluent with options of its own, and Fluent
                // matches `[0]` only against a number with none, so it never matches.
                // Where the code means a case, the code picks the key.
                assert!(
                    matches!(variant.key, VariantKey::Identifier { .. }),
                    "a variant keyed by a number never matches: use a key of its own"
                );
                in_pattern(&variant.value, variables);
            }
        }
        Expression::Inline(inline) => in_inline(inline, variables),
    }
}

fn in_inline(inline: &InlineExpression<&str>, variables: &mut BTreeSet<String>) {
    match inline {
        InlineExpression::VariableReference { id } => {
            variables.insert(id.name.to_owned());
        }
        InlineExpression::Placeable { expression } => in_expression(expression, variables),
        InlineExpression::FunctionReference { arguments, .. } => {
            for argument in &arguments.positional {
                in_inline(argument, variables);
            }
            for argument in &arguments.named {
                in_inline(&argument.value, variables);
            }
        }
        _ => {}
    }
}

/// One place in the code that says a key, and the arguments it names.
#[derive(Debug)]
struct Said {
    at: String,
    key: String,
    arguments: BTreeSet<String>,
}

/// Every Rust file of this crate's source.
fn rust_files(dir: &Path, found: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_files(&path, found);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            found.push(path);
        }
    }
}

/// The source with its comments taken out, so an example in a doc is not a use.
fn code_of(path: &Path) -> String {
    let text = std::fs::read_to_string(path).unwrap();
    text.lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Every `words!("…", a = …)` and `aloud_in!("…", a = …)` in the code.
fn said_in_the_code() -> Vec<Said> {
    let mut files = Vec::new();
    rust_files(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        &mut files,
    );
    let mut said = Vec::new();
    for file in files {
        let code = code_of(&file);
        for opening in ["words!(", "aloud_in!("] {
            for (at, _) in code.match_indices(opening) {
                if code.get(..at).is_some_and(|before| before.ends_with('"')) {
                    continue; // this file's own list of what to look for
                }
                let rest = code
                    .get(at + opening.len()..)
                    .unwrap_or_default()
                    .trim_start();
                let Some(after_quote) = rest.strip_prefix('"') else {
                    continue; // the macro's own definition, passing `$key` on
                };
                let (key, rest) = after_quote.split_once('"').unwrap();
                said.push(Said {
                    at: format!("{}", file.display()),
                    key: key.to_owned(),
                    arguments: argument_names(rest),
                });
            }
        }
    }
    said
}

/// The names before each `=` at the top level of a call, up to its closing bracket.
fn argument_names(rest: &str) -> BTreeSet<String> {
    let (mut depth, mut quoted, mut segment, mut names) =
        (0_i32, false, String::new(), BTreeSet::new());
    let mut take = |segment: &mut String| {
        if let Some((name, _)) = segment.split_once('=') {
            let name = name.trim();
            if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                names.insert(name.to_owned());
            }
        }
        segment.clear();
    };
    for c in rest.chars() {
        match c {
            '"' => quoted = !quoted,
            _ if quoted => {}
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' if depth == 0 => break,
            ')' | ']' | '}' => depth -= 1,
            ',' if depth == 0 => {
                take(&mut segment);
                continue;
            }
            _ => {}
        }
        segment.push(c);
    }
    take(&mut segment);
    names
}

#[test]
fn every_catalog_can_be_read_whole() {
    for (_, files) in catalog::every_built_in() {
        for source in files {
            let mut problems = Vec::new();
            let name = source.name.clone();
            let _ = catalog::parsed(source, &mut problems);
            assert!(problems.is_empty(), "{name}: {problems:?}");
        }
    }
}

#[test]
fn every_key_lives_in_the_file_named_for_it() {
    for (_, files) in catalog::every_built_in() {
        for (key, (file, _)) in messages(&files) {
            let stem = Path::new(&file)
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .into_owned();
            assert!(key.starts_with(&format!("{stem}-")), "{key} is in {file}");
        }
    }
}

#[test]
fn every_translation_says_only_what_english_says_and_asks_for_the_same_things() {
    let english = messages(&catalog::built_in(ENGLISH));
    for (tag, files) in catalog::every_built_in() {
        for (key, (file, shape)) in messages(&files) {
            let Some((_, wanted)) = english.get(&key) else {
                panic!("{file}: {key} is not a key English has");
            };
            assert_eq!(&shape, wanted, "{tag}: {key} in {file}");
        }
    }
}

#[test]
fn every_english_line_has_its_korean_beside_it() {
    let english = messages(&catalog::built_in(ENGLISH));
    let korean = messages(&catalog::built_in("ko"));
    let missing: Vec<&str> = english
        .iter()
        .filter(|(key, (_, shape))| {
            korean
                .get(*key)
                .is_none_or(|(_, said)| said.attributes != shape.attributes)
        })
        .map(|(key, _)| key.as_str())
        .collect();
    assert!(
        missing.is_empty(),
        "no Korean, or Korean without the same attributes, for: {}.\n\
         A Korean translation is expected alongside every new English line, in the \
         file of the same name under words/ko/.",
        missing.join(", ")
    );
}

/// Words that stay in Rust on purpose: the file, the opening, and why. None do: what
/// a program reads (a unit file, a receipt, `status --json`) is not handed to any of
/// these, and everything a person reads belongs in a catalog.
const LEFT_IN_RUST: &[(&str, &str, &str)] = &[];

#[test]
fn nothing_a_person_reads_is_left_written_in_rust() {
    let mut files = Vec::new();
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    rust_files(&src, &mut files);
    let read: Vec<(PathBuf, built::Built)> = files
        .into_iter()
        .map(|file| {
            let code = built::built(&std::fs::read_to_string(&file).unwrap());
            (file, code)
        })
        .collect();
    let for_tests: BTreeSet<PathBuf> = read
        .iter()
        .flat_map(|(file, code)| {
            code.test_modules
                .iter()
                .map(|name| built::module_file(file, name))
        })
        .collect();
    assert!(for_tests.iter().all(|file| file.exists()), "{for_tests:?}");
    let mut left = Vec::new();
    for (file, code) in &read {
        if for_tests.contains(file) {
            continue;
        }
        let name = file.strip_prefix(&src).unwrap().display().to_string();
        for (line, opening) in built::said_in_rust(code) {
            let excused = LEFT_IN_RUST
                .iter()
                .any(|(at, what, _)| *at == name && *what == opening);
            if !excused {
                left.push(format!("{name}:{line}: {opening}"));
            }
        }
    }
    assert!(
        left.is_empty(),
        "words a person reads are still written in Rust. Give each a key in the \
         catalog named for its file, under words/en/ with its Korean under words/ko/, \
         and say it with words! or aloud_in!:\n{}",
        left.join("\n")
    );
}

#[test]
fn every_key_the_code_says_is_in_english_and_given_what_it_asks_for() {
    let english = messages(&catalog::built_in(ENGLISH));
    let said = said_in_the_code();
    assert!(said.len() > 5, "the scan found almost nothing: {said:?}");
    for one in said {
        let Some((_, shape)) = english.get(&one.key) else {
            panic!("{}: {} is not in the English catalog", one.at, one.key);
        };
        assert_eq!(one.arguments, shape.variables, "{}: {}", one.at, one.key);
    }
}

#[test]
fn every_english_key_is_said_somewhere_in_the_code() {
    let mut files = Vec::new();
    rust_files(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        &mut files,
    );
    let code: String = files.iter().map(|file| code_of(file)).collect();
    for key in messages(&catalog::built_in(ENGLISH)).keys() {
        assert!(code.contains(&format!("\"{key}\"")), "nothing says {key}");
    }
}

/// How wide one line of a catalog comes out, with each placeable counted as one digit.
fn shown_width(line: &str) -> Option<usize> {
    let mut text = line.trim_end();
    if text.is_empty() || text.starts_with('#') {
        return None;
    }
    if !text.starts_with(' ') {
        text = text.split_once(" = ").map_or("", |(_, value)| value);
    }
    let text = text.trim_start();
    if text.starts_with('.') || text.contains("->") {
        return None;
    }
    let text = text
        .trim_start_matches('*')
        .trim_start_matches('}')
        .trim_start();
    let text = match text.strip_prefix('[') {
        Some(variant) => variant.split_once(']').map_or("", |(_, rest)| rest),
        None => text,
    };
    let mut shown = String::new();
    let mut inside = false;
    for c in text.chars() {
        match c {
            '{' => {
                inside = true;
                shown.push('0');
            }
            '}' => inside = false,
            _ if !inside => shown.push(c),
            _ => {}
        }
    }
    Some(shown.trim().width())
}

#[test]
fn no_line_of_any_catalog_is_wider_than_a_terminal_leaves_after_the_keyword() {
    const ROOM: usize = 80 - super::layout::COLUMN;
    for (_, files) in catalog::every_built_in() {
        for source in files {
            for (at, line) in source.text.lines().enumerate() {
                let width = shown_width(line).unwrap_or(0);
                assert!(width <= ROOM, "{}:{}: {width} columns", source.name, at + 1);
            }
        }
    }
}

#[test]
fn the_scan_reads_argument_names_and_not_what_is_inside_them() {
    let names = argument_names(r#", a = f(b = 1, "c = 2"), d = x[0]) + e = 3"#);
    assert_eq!(names, BTreeSet::from(["a".to_owned(), "d".to_owned()]));
    assert_eq!(shown_width("k = { $n } keys").unwrap(), 6);
    assert_eq!(shown_width("       *[other] { $n } 키").unwrap(), 4);
    assert_eq!(shown_width("    .keyword = name"), None);
}
