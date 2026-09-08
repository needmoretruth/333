//! Writes `THIRD-PARTY.md`, and can tell you that the one on disk is out of date.
//!
//! The released binaries are statically linked and carry 500-odd packages that other
//! people wrote. Every one of those licences — MIT, BSD, ISC, Apache-2.0 — asks that
//! its notice go along with the copies. The repository's own `LICENSE` and `NOTICE`
//! say nothing about them, so without this file every release breaks the terms it was
//! given the code under.
//!
//! Keeping such a file by hand does not work; it goes stale on the first `cargo
//! update` and nobody notices for a year. So it is generated from the dependency
//! graph, and `--check` regenerates it in memory and fails if the file on disk has
//! drifted, which is what runs on every push.
//!
//! This crate is a tool. Nothing that ships depends on it.

mod graph;
mod licences;
mod render;

use anyhow::{Context, Result};
use clap::Parser;
use std::collections::BTreeSet;

/// How many changed packages are worth naming before a count says the rest.
const NAMED_IN_SUMMARY: usize = 20;

/// Two modes and no more: write the file, or say whether it is current.
#[derive(Parser)]
#[command(about = "Writes THIRD-PARTY.md from the dependency graph of the released binaries.")]
struct Args {
    /// Regenerate in memory and fail if what is on disk differs. Changes nothing.
    #[arg(long)]
    check: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let repository = graph::locate()?;
    let shipped = graph::shipped(&repository.released_manifest)?;
    let corpus = licences::gather(&shipped);
    let regenerated = render::document(&shipped, &corpus);
    let path = repository.root.join(render::FILE);

    if args.check {
        let on_disk = std::fs::read_to_string(&path)
            .with_context(|| format!("{} is missing; run this without --check", path.display()))?;
        if on_disk == regenerated {
            println!(
                "{} is current: {} packages, {} licence texts.",
                render::FILE,
                shipped.len(),
                corpus.texts.len()
            );
            return Ok(());
        }
        eprint!("{}", difference(&on_disk, &regenerated));
        // A stale attribution file has to stop a release, so the exit code carries it.
        std::process::exit(1);
    }

    std::fs::write(&path, &regenerated)
        .with_context(|| format!("could not write {}", path.display()))?;
    println!(
        "Wrote {}: {} packages, {} distinct licence texts, {} packages shipping none.",
        path.display(),
        shipped.len(),
        corpus.texts.len(),
        corpus.gaps.len()
    );
    Ok(())
}

/// What changed, said so that a person can tell a new dependency from an edited word.
///
/// The package table is the part worth naming, so it is compared as a set of rows.
/// When those match, the difference is somewhere in the prose or a licence text, and
/// the first line that differs is more use than a list.
fn difference(on_disk: &str, regenerated: &str) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    let _ = writeln!(
        out,
        "{} is out of date. Run `cargo run -p n333-notices` and commit the result.",
        render::FILE
    );

    let was = rows(on_disk);
    let now = rows(regenerated);
    let gone: Vec<&String> = was.difference(&now).collect();
    let arrived: Vec<&String> = now.difference(&was).collect();
    let _ = writeln!(out, "\n{} packages listed, {} now.", was.len(), now.len());
    list(&mut out, "No longer shipped", &gone);
    list(&mut out, "Newly shipped", &arrived);

    if gone.is_empty() && arrived.is_empty() {
        let at = on_disk
            .lines()
            .zip(regenerated.lines())
            .position(|(before, after)| before != after);
        let _ = match at {
            Some(at) => writeln!(
                out,
                "\nThe same packages, so the change is in the text. First difference at line {}.",
                at + 1
            ),
            None => writeln!(
                out,
                "\nThe same packages and no differing line: one file is longer than the other."
            ),
        };
    }
    out
}

/// One heading and its rows, up to the point where a count says more than a list.
fn list(out: &mut String, heading: &str, rows: &[&String]) {
    use std::fmt::Write;
    if rows.is_empty() {
        return;
    }
    let _ = writeln!(out, "\n{heading} ({}):", rows.len());
    for row in rows.iter().take(NAMED_IN_SUMMARY) {
        let _ = writeln!(out, "  {row}");
    }
    if let Some(rest) = rows.len().checked_sub(NAMED_IN_SUMMARY).filter(|n| *n > 0) {
        let _ = writeln!(out, "  ...and {rest} more");
    }
}

/// The header of the package table, so that it is not counted as a package.
const HEADER: &str = "| Package | Version | Licence |";

/// The rows of the package table, told apart from the table of packages that ship no
/// text by having three cells rather than four, and from the header and the rule under
/// it by not being either.
fn rows(document: &str) -> BTreeSet<String> {
    document
        .lines()
        .filter(|line| line.starts_with("| ") && line.ends_with(" |"))
        .filter(|line| line.matches(" | ").count() == 2)
        .filter(|line| *line != HEADER && !line.contains("--- | ---"))
        .map(str::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::expect_used,
        clippy::unwrap_used,
        clippy::panic,
        clippy::indexing_slicing
    )]

    use super::*;

    const TABLE: &str = "\
# Third-party notices

## Packages

| Package | Version | Licence |
| --- | --- | --- |
| alpha | 1.0.0 | MIT |
| beta | 2.0.0 | Apache-2.0 |

## Packages that ship no licence text

| Package | Version | Licence | Why |
| --- | --- | --- | --- |
| gamma | 3.0.0 | MIT | ships nothing |
";

    #[test]
    fn only_the_package_table_is_read_as_packages() {
        let found = rows(TABLE);
        assert_eq!(
            found.len(),
            2,
            "the four-cell table and the rules are left out"
        );
        assert!(found.contains("| alpha | 1.0.0 | MIT |"));
        assert!(found.contains("| beta | 2.0.0 | Apache-2.0 |"));
    }

    #[test]
    fn a_package_that_came_and_one_that_went_are_both_named() {
        let after = TABLE.replace("| beta | 2.0.0 | Apache-2.0 |", "| delta | 4.0.0 | ISC |");
        let said = difference(TABLE, &after);
        assert!(said.contains("out of date"));
        assert!(said.contains("No longer shipped (1):"));
        assert!(said.contains("| beta | 2.0.0 | Apache-2.0 |"));
        assert!(said.contains("Newly shipped (1):"));
        assert!(said.contains("| delta | 4.0.0 | ISC |"));
    }

    #[test]
    fn a_changed_version_is_both_a_departure_and_an_arrival() {
        let after = TABLE.replace("| alpha | 1.0.0 | MIT |", "| alpha | 1.0.1 | MIT |");
        let said = difference(TABLE, &after);
        assert!(said.contains("| alpha | 1.0.0 | MIT |"));
        assert!(said.contains("| alpha | 1.0.1 | MIT |"));
    }

    #[test]
    fn the_same_packages_with_different_words_points_at_the_line() {
        let after = TABLE.replace("# Third-party notices", "# Third party notices");
        let said = difference(TABLE, &after);
        assert!(said.contains("the change is in the text"));
        assert!(said.contains("line 1"), "{said}");
    }

    #[test]
    fn a_long_list_stops_naming_and_starts_counting() {
        use std::fmt::Write;
        let mut after = String::from("| Package | Version | Licence |\n");
        for n in 0..30 {
            let _ = writeln!(after, "| crate{n} | 1.0.0 | MIT |");
        }
        let said = difference("", &after);
        assert!(said.contains("Newly shipped (30):"), "{said}");
        assert!(said.contains("...and 10 more"), "{said}");
    }

    #[test]
    fn a_file_that_only_got_longer_is_reported_as_that() {
        let after = format!("{TABLE}a trailing paragraph\n");
        let said = difference(TABLE, &after);
        assert!(!said.contains("No longer shipped"));
        assert!(!said.contains("Newly shipped"));
        assert!(said.contains("one file is longer than the other"), "{said}");
    }
}
