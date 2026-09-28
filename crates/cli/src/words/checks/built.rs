//! What one Rust file builds outside its tests, read closely enough to tell a string a
//! person reads from code that only mentions one.
//!
//! Not a parser: comments, string and character literals, and the items under
//! `#[cfg(test)]` are all that is recognised, because that is all it takes to know
//! whether a `println!` or a `bail!("…")` is in the code that ships. Lines are kept
//! where they were, so that a finding names the line to go and change.

use std::ops::Range;
use std::path::{Path, PathBuf};

/// One file's code as it is built outside tests.
#[derive(Debug, Default)]
pub(super) struct Built {
    /// The code, comments out and every test item out, each string literal whole.
    pub(super) text: String,
    /// Where each string literal is in `text`, quotes and prefix included.
    pub(super) literals: Vec<Range<usize>>,
    /// The modules declared here only for tests, `#[cfg(test)] mod name;`.
    pub(super) test_modules: Vec<String>,
}

/// The item under a `#[cfg(test)]` being passed over.
struct Skipping {
    /// How many brackets of any kind deep inside it.
    depth: usize,
    /// What it said, to find the name of a module it declares.
    said: String,
}

impl Skipping {
    /// Pass over one character, and say whether the item ended with it: a `;` or a
    /// `,` outside every bracket, the brace that closes its body, or a bracket that
    /// closes whatever it stood inside.
    fn ended_by(&mut self, c: char) -> bool {
        self.said.push(c);
        match c {
            '{' | '[' | '(' => {
                self.depth += 1;
                false
            }
            '}' | ']' | ')' => match self.depth.checked_sub(1) {
                None => true,
                Some(depth) => {
                    self.depth = depth;
                    depth == 0 && c == '}'
                }
            },
            ';' | ',' => self.depth == 0,
            _ => false,
        }
    }
}

/// Read a file's code the way the compiler outside tests would see it.
pub(super) fn built(source: &str) -> Built {
    let test_only: Vec<char> = "#[cfg(test)]".chars().collect();
    let chars: Vec<char> = source.chars().collect();
    let mut built = Built::default();
    let mut skipping: Option<Skipping> = None;
    let mut at = 0;
    while at < chars.len() {
        let (c, next) = (chars[at], chars.get(at + 1).copied());
        if c == '/' && matches!(next, Some('/' | '*')) {
            at = past_comment(&chars, at, &mut built.text);
            continue;
        }
        if let Some(end) = literal_end(&chars, at) {
            let literal: String = chars[at..end].iter().collect();
            let string = !literal.trim_start_matches('b').starts_with('\'');
            match &mut skipping {
                Some(skip) => {
                    skip.said.push_str(&literal);
                    built.text.extend(literal.chars().filter(|c| *c == '\n'));
                }
                None => {
                    let start = built.text.len();
                    built.text.push_str(&literal);
                    if string {
                        built.literals.push(start..built.text.len());
                    }
                }
            }
            at = end;
            continue;
        }
        let Some(skip) = &mut skipping else {
            if chars[at..].starts_with(&test_only) {
                skipping = Some(Skipping {
                    depth: 0,
                    said: String::new(),
                });
                at += test_only.len();
            } else {
                built.text.push(c);
                at += 1;
            }
            continue;
        };
        if c == '\n' {
            built.text.push('\n');
        }
        if skip.ended_by(c) {
            if let Some(name) = declared_module(&skip.said) {
                built.test_modules.push(name);
            }
            skipping = None;
        }
        at += 1;
    }
    built
}

/// Past one comment, keeping the line breaks inside it. Block comments nest.
fn past_comment(chars: &[char], from: usize, text: &mut String) -> usize {
    let mut at = from + 2;
    if chars[from + 1] == '/' {
        while at < chars.len() && chars[at] != '\n' {
            at += 1;
        }
        return at;
    }
    let mut depth = 1;
    while at < chars.len() && depth > 0 {
        match (chars[at], chars.get(at + 1)) {
            ('/', Some('*')) => (depth, at) = (depth + 1, at + 2),
            ('*', Some('/')) => (depth, at) = (depth - 1, at + 2),
            (c, _) => {
                if c == '\n' {
                    text.push('\n');
                }
                at += 1;
            }
        }
    }
    at
}

/// Where a string or character literal starting here ends, if one starts here.
fn literal_end(chars: &[char], at: usize) -> Option<usize> {
    let before = at.checked_sub(1).and_then(|b| chars.get(b));
    if before.is_some_and(|c| c.is_alphanumeric() || *c == '_') {
        return None;
    }
    let mut start = at;
    if chars.get(start) == Some(&'b') {
        start += 1;
    }
    if chars.get(start) == Some(&'r') {
        let hashes = chars[start + 1..].iter().take_while(|c| **c == '#').count();
        if chars.get(start + 1 + hashes) == Some(&'"') {
            let close: Vec<char> = std::iter::once('"')
                .chain(std::iter::repeat_n('#', hashes))
                .collect();
            let body = start + 2 + hashes;
            let found = (body..chars.len()).find(|i| chars[*i..].starts_with(&close))?;
            return Some(found + close.len());
        }
        return None;
    }
    match chars.get(start) {
        Some('"') => {
            let mut i = start + 1;
            while i < chars.len() {
                match chars[i] {
                    '\\' => i += 2,
                    '"' => return Some(i + 1),
                    _ => i += 1,
                }
            }
            Some(chars.len())
        }
        // A character, not a lifetime: `'x'` or an escape like `'\''`.
        Some('\'') if chars.get(start + 1) == Some(&'\\') => (start + 3..chars.len())
            .find(|i| chars[*i] == '\'')
            .map(|i| i + 1),
        Some('\'') if chars.get(start + 2) == Some(&'\'') => Some(start + 3),
        _ => None,
    }
}

/// The name in `mod name;`, after whatever attributes stand in front of it.
fn declared_module(item: &str) -> Option<String> {
    let mut rest = item.trim();
    while let Some(after) = rest.strip_prefix("#[") {
        rest = after.split_once(']')?.1.trim_start();
    }
    let rest = rest.strip_prefix("pub(crate) ").unwrap_or(rest);
    let name = rest.strip_prefix("mod ")?.strip_suffix(';')?.trim();
    Some(name.to_owned())
}

/// The file a module declared in `declaring` lives in.
pub(super) fn module_file(declaring: &Path, name: &str) -> PathBuf {
    let dir = declaring.parent().unwrap_or(declaring);
    let stem = declaring.file_stem().unwrap_or_default();
    let dir = if ["main", "lib", "mod"].iter().any(|own| stem == *own) {
        dir.to_path_buf()
    } else {
        dir.join(stem)
    };
    let flat = dir.join(format!("{name}.rs"));
    if flat.exists() {
        flat
    } else {
        dir.join(name).join("mod.rs")
    }
}

/// What says a line to a person, and so must be handed words from a catalog.
#[derive(Debug, Clone, Copy)]
enum Saying {
    /// Anything at all: a macro that prints.
    Always,
    /// A literal with words in it, as what the call is given first.
    Literal,
    /// A literal with words in it, after what it writes to.
    LiteralAfterWriter,
}

/// Every opening that says a line or makes the text of an error a person reads.
const OPENINGS: &[(&str, Saying)] = &[
    ("aloud!(", Saying::Always),
    ("println!(", Saying::Always),
    ("eprintln!(", Saying::Always),
    ("print!(", Saying::Always),
    ("eprint!(", Saying::Always),
    ("bail!(", Saying::Literal),
    ("anyhow!(", Saying::Literal),
    (".context(", Saying::Literal),
    (".with_context(", Saying::Literal),
    ("format_args!(", Saying::Literal),
    ("write!(", Saying::LiteralAfterWriter),
    ("writeln!(", Saying::LiteralAfterWriter),
];

/// Every place in built code where words a person reads are written in Rust: the line,
/// and the opening that says them.
pub(super) fn said_in_rust(built: &Built) -> Vec<(usize, &'static str)> {
    let text = &built.text;
    let inside = |at: usize| built.literals.iter().any(|l| l.start < at && at < l.end);
    let mut found = Vec::new();
    for (opening, saying) in OPENINGS {
        for (at, _) in text.match_indices(opening) {
            let before = text.get(..at).and_then(|b| b.chars().next_back());
            let joined = before.is_some_and(|c| c.is_alphanumeric() || c == '_');
            if (joined && !opening.starts_with('.')) || inside(at) {
                continue;
            }
            let mut after = at + opening.len();
            if matches!(saying, Saying::LiteralAfterWriter) {
                let Some(comma) = first_argument_end(text, after, &inside) else {
                    continue;
                };
                after = comma + 1;
            }
            let said = matches!(saying, Saying::Always)
                || literal_at(text, after, &built.literals).is_some_and(has_words);
            if said {
                let line = text.get(..at).map_or(0, |b| b.matches('\n').count()) + 1;
                found.push((line, *opening));
            }
        }
    }
    found.sort_unstable();
    found
}

/// Where the first argument of a call ends, at the comma after it, if there is one.
fn first_argument_end(text: &str, from: usize, inside: &impl Fn(usize) -> bool) -> Option<usize> {
    let mut depth = 0_usize;
    for (at, c) in text.get(from..)?.char_indices() {
        if inside(from + at) {
            continue;
        }
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth = depth.checked_sub(1)?,
            ',' if depth == 0 => return Some(from + at),
            _ => {}
        }
    }
    None
}

/// The literal a call is handed first, past a closure's bars or a `format!(`.
fn literal_at<'a>(text: &'a str, from: usize, literals: &[Range<usize>]) -> Option<&'a str> {
    let mut rest = text.get(from..)?.trim_start();
    for prefix in ["move ", "||", "format!("] {
        rest = rest.strip_prefix(prefix).unwrap_or(rest).trim_start();
    }
    let at = text.len() - rest.len();
    let literal = literals.iter().find(|l| l.start == at)?;
    text.get(literal.clone())
}

/// Whether a literal holds any words: letters outside its placeholders and escapes.
pub(super) fn has_words(literal: &str) -> bool {
    let body = literal.trim_start_matches(['b', 'r', '#']);
    let raw = literal.starts_with('r') || literal.starts_with("br");
    let mut chars = body.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' if !raw => match chars.next() {
                Some('x') => drop((chars.next(), chars.next())),
                Some('u') => {
                    for c in chars.by_ref() {
                        if c == '}' {
                            break;
                        }
                    }
                }
                _ => {}
            },
            '{' if chars.peek() == Some(&'{') => drop(chars.next()),
            '{' => {
                for c in chars.by_ref() {
                    if c == '}' {
                        break;
                    }
                }
            }
            c if c.is_alphabetic() => return true,
            _ => {}
        }
    }
    false
}

#[test]
fn comments_and_test_items_are_not_built_and_their_lines_are_kept() {
    let source =
        "a(); // println!\n/* x\n */ b();\n#[cfg(test)]\nmod t {\n  fn f() { \"}\"; }\n}\nc();\n";
    let built = built(source);
    assert_eq!(built.text, "a(); \n\n b();\n\n\n\n\nc();\n");
    assert!(built.test_modules.is_empty());
}

#[test]
fn a_module_declared_for_tests_is_named_and_a_literal_is_kept_whole() {
    let built = built("#[cfg(test)]\nmod learning;\nlet x = r#\"a \" b\"#; '{'; 'a;\n");
    assert_eq!(built.test_modules, ["learning"]);
    let literal = &built.text[built.literals[0].clone()];
    assert_eq!(literal, "r#\"a \" b\"#");
    assert_eq!(built.literals.len(), 1, "a char is not a string literal");
}

#[test]
fn words_are_letters_outside_placeholders_and_escapes() {
    assert!(has_words("\"writing {}\""));
    assert!(has_words("\"이름\""));
    assert!(!has_words("\"{line}\""));
    assert!(!has_words("\"\\n\\x20  {tip} {similar:#}\""));
    assert!(!has_words("\"{{{name}}}\""));
}

#[test]
fn a_line_is_found_where_words_are_handed_to_what_says_them_and_nowhere_else() {
    let source = "e.context(\"reading\")?;\n\
                  e.with_context(|| format!(\"writing {}\", p))?;\n\
                  e.with_context(|| words!(\"k\"))?;\n\
                  writeln!(out, \"{}\", words!(\"k\"))?;\n\
                  write!(f(a, b), \"by hand\")?;\n\
                  eprintln!(\"{x}\");\n\
                  let s = \"println!(\";\n\
                  bail!(NONE);\n";
    let found = said_in_rust(&built(source));
    assert_eq!(
        found,
        [
            (1, ".context("),
            (2, ".with_context("),
            (5, "write!("),
            (6, "eprintln!(")
        ]
    );
}
