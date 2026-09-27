//! The licences a released binary may carry: the ones it carries today, and no others
//! until somebody decides otherwise.
//!
//! Five hundred-odd packages are linked into every release, and each one's manifest says
//! what it may be taken under. Nothing used to look at that. A `cargo update` that
//! brought in a package under terms nobody had read would have shipped in the next
//! release with nobody knowing, which is the opposite of carrying every condition
//! knowingly.
//!
//! So the check is a ratchet. The list below is what the tree carried on the day it was
//! written, and a package whose expression cannot be satisfied from that list stops the
//! push that brought it, with its name. It is not a judgement about any licence left off
//! it: most of them are fine, and the first time one arrives the answer may well be to
//! take it. What the ratchet guarantees is only that taking it is something a person
//! did on purpose, in a pull request that says why, rather than something that happened.
//!
//! Every expression is read as SPDX, so `MIT OR Apache-2.0` is satisfied by either and
//! `(MIT OR Apache-2.0) AND Unicode-3.0` needs both halves. A package is refused when
//! there is no way to take it using only the licences on the list.

use crate::graph::Shipped;
use spdx::{Expression, LicenseReq, ParseMode};
use std::fmt::Write;

/// Every licence the released binaries carried when this list was written.
///
/// FROZEN FROM THE TREE, NOT CHOSEN. This is the dependency graph of 0.5.0 read off and
/// written down — MIT, Apache, BSD, ISC and the public-domain dedications that most of it
/// uses, and Unicode-3.0, MPL-2.0, Zlib, BSL-1.0 and CDLA-Permissive-2.0 that arrive
/// with Tor. A licence missing from it has not been judged, only not yet met. Adding one
/// is a decision: write down in the pull request why the package is worth its terms,
/// then add the line.
///
/// LGPL-2.1-or-later and LGPL-3.0-or-later also appear today, each only as one choice
/// beside a licence on this list, and that other choice is the one taken. They are left
/// off so that a package offering nothing but the LGPL is refused.
const CARRIED: [&str; 14] = [
    "0BSD",
    "Apache-2.0",
    "BSD-1-Clause",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "BSL-1.0",
    "CC0-1.0",
    "CDLA-Permissive-2.0",
    "ISC",
    "MIT",
    "MPL-2.0",
    "Unicode-3.0",
    "Unlicense",
    "Zlib",
];

/// The additions after `WITH` that the tree carried, frozen the same way.
const EXCEPTIONS: [&str; 1] = ["LLVM-exception"];

/// Kinds of licence that are refused wherever they appear in an expression, even as one
/// choice among several.
///
/// The code rules refuse these outright: GPL and AGPL reach into what is built beside
/// them, and SSPL and BUSL are not open source licences at all. Nothing in the tree
/// offers one even as an alternative today, so the first package that does is worth a
/// person reading, whatever else it also offers. Matched by the start of the SPDX
/// identifier, which is how every version of each is named; `LGPL-` does not start
/// with `GPL-`.
const REFUSED: [&str; 4] = ["GPL-", "AGPL-", "SSPL-", "BUSL-"];

/// Why one package stopped the check.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Refusal {
    /// The manifest gives no expression, so there is nothing a machine can read.
    Undeclared,
    /// The expression is not SPDX, even read leniently.
    Unreadable(String),
    /// A licence that is never taken appears in it.
    Refused(String),
    /// No way to take it uses only the licences carried; these are what it asks for
    /// instead.
    Outside(Vec<String>),
}

/// One package the check stopped at.
pub(crate) struct Stopped {
    /// The package, as `name version`.
    pub(crate) package: String,
    /// What its manifest says, as the generated file prints it.
    pub(crate) declared: String,
    /// Why.
    pub(crate) why: Refusal,
}

/// Every shipped package whose licence is not one the tree already carries.
pub(crate) fn check(shipped: &[Shipped]) -> Vec<Stopped> {
    shipped
        .iter()
        .filter_map(|package| {
            judge(package.licence.as_deref()).err().map(|why| Stopped {
                package: package.label(),
                declared: package.expression(),
                why,
            })
        })
        .collect()
}

/// Whether one declared expression may be taken using only what is carried.
fn judge(expression: Option<&str>) -> Result<(), Refusal> {
    let text = expression.ok_or(Refusal::Undeclared)?;
    let parsed = Expression::parse_mode(text, ParseMode::LAX)
        .map_err(|why| Refusal::Unreadable(why.to_string()))?;
    if let Some(refused) = parsed.requirements().find_map(|found| {
        let id = found.req.license.id()?;
        REFUSED
            .iter()
            .any(|kind| id.name.starts_with(kind))
            .then(|| id.name.to_owned())
    }) {
        return Err(Refusal::Refused(refused));
    }
    parsed.evaluate_with_failures(carried).map_err(|failed| {
        Refusal::Outside(failed.iter().map(|found| found.req.to_string()).collect())
    })
}

/// Whether one licence, with whatever follows its `WITH`, is on the lists.
fn carried(requirement: &LicenseReq) -> bool {
    let licence = requirement
        .license
        .id()
        .is_some_and(|id| CARRIED.contains(&id.name));
    let addition = requirement.addition.as_ref().is_none_or(|addition| {
        addition
            .id()
            .is_some_and(|id| EXCEPTIONS.contains(&id.name))
    });
    licence && addition
}

/// What to say when the check stops, naming every package that stopped it.
pub(crate) fn report(stopped: &[Stopped]) -> String {
    let mut out = String::from(
        "A released binary would carry a licence the tree has not carried before.\n\n",
    );
    for one in stopped {
        let why = match &one.why {
            Refusal::Undeclared => "declares no SPDX expression; read its licence file".to_owned(),
            Refusal::Unreadable(why) => format!("is not an SPDX expression: {why}"),
            Refusal::Refused(licence) => {
                format!("offers {licence}, which is refused even as one choice of several")
            }
            Refusal::Outside(asks) => format!(
                "cannot be taken under the licences already carried; it asks for {}",
                asks.join(", ")
            ),
        };
        let _ = writeln!(out, "  {}: `{}` {why}", one.package, one.declared);
    }
    out.push_str(
        "\nThe list in crates/notices/src/ratchet.rs is the tree as it was, frozen, and not a\n\
         judgement of any licence. Taking one that is not on it is a decision: say in the\n\
         pull request why the package is worth its terms, and add the licence to the list.\n",
    );
    out
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
    use cargo_metadata::semver::Version;
    use std::path::PathBuf;

    /// Every distinct expression the released graph declared when the list was frozen,
    /// copied out of THIRD-PARTY.md.
    const TODAY: [&str; 33] = [
        "MIT OR Apache-2.0",
        "MIT",
        "Apache-2.0 OR MIT",
        "MIT/Apache-2.0",
        "Unicode-3.0",
        "Apache-2.0",
        "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT",
        "ISC",
        "Unlicense OR MIT",
        "BSD-3-Clause",
        "Apache-2.0/MIT",
        "Zlib OR Apache-2.0 OR MIT",
        "Zlib",
        "Unlicense/MIT",
        "MPL-2.0",
        "MIT OR Apache-2.0 OR LGPL-2.1-or-later",
        "BSD-3-Clause OR MIT OR Apache-2.0",
        "BSD-2-Clause OR Apache-2.0 OR MIT",
        "Zlib OR MIT OR Apache-2.0",
        "Unlicense",
        "MIT OR Zlib OR Apache-2.0",
        "MIT OR BSD-3-Clause",
        "MIT OR Apache-2.0 OR BSD-1-Clause",
        "LGPL-3.0-or-later OR MPL-2.0",
        "CDLA-Permissive-2.0",
        "CC0-1.0",
        "BSL-1.0",
        "BSD-2-Clause",
        "Apache-2.0 OR ISC OR MIT",
        "Apache-2.0 OR BSL-1.0",
        "Apache-2.0 AND ISC",
        "Apache-2.0 / MIT",
        "0BSD OR MIT OR Apache-2.0",
    ];

    /// A package in a graph made up for the test.
    fn package(name: &str, licence: Option<&str>) -> Shipped {
        Shipped {
            name: name.to_owned(),
            version: Version::new(1, 0, 0),
            licence: licence.map(str::to_owned),
            licence_file: None,
            directory: PathBuf::new(),
        }
    }

    #[test]
    fn every_expression_the_tree_carried_when_frozen_still_passes() {
        let mut graph: Vec<Shipped> = TODAY
            .iter()
            .enumerate()
            .map(|(n, expression)| package(&format!("today{n}"), Some(expression)))
            .collect();
        graph.push(package("both", Some("(MIT OR Apache-2.0) AND Unicode-3.0")));
        let stopped = check(&graph);
        assert!(stopped.is_empty(), "{}", report(&stopped));
    }

    #[test]
    fn a_new_licence_stops_the_check_and_names_the_package_that_brought_it() {
        let graph = [
            package("fine", Some("MIT")),
            package("newcomer", Some("EUPL-1.2")),
        ];
        let stopped = check(&graph);
        assert_eq!(stopped.len(), 1);
        assert_eq!(stopped[0].package, "newcomer 1.0.0");
        assert_eq!(
            stopped[0].why,
            Refusal::Outside(vec!["EUPL-1.2".to_owned()])
        );
        let said = report(&stopped);
        assert!(said.contains("newcomer 1.0.0: `EUPL-1.2`"), "{said}");
        assert!(said.contains("it asks for EUPL-1.2"), "{said}");
    }

    #[test]
    fn a_new_licence_offered_beside_a_carried_one_is_taken_under_the_carried_one() {
        assert_eq!(judge(Some("EUPL-1.2 OR MIT")), Ok(()));
        assert!(
            judge(Some("EUPL-1.2 AND MIT")).is_err(),
            "both halves apply"
        );
    }

    #[test]
    fn the_refused_kinds_stop_it_even_as_one_choice_among_several() {
        for (expression, named) in [
            ("GPL-3.0-only", "GPL-3.0-only"),
            ("MIT OR GPL-2.0-or-later", "GPL-2.0-or-later"),
            ("AGPL-3.0-only OR Apache-2.0", "AGPL-3.0-only"),
            ("SSPL-1.0", "SSPL-1.0"),
            ("BUSL-1.1 OR MIT", "BUSL-1.1"),
        ] {
            assert_eq!(
                judge(Some(expression)),
                Err(Refusal::Refused(named.to_owned())),
                "{expression}"
            );
        }
    }

    #[test]
    fn the_lgpl_passes_only_beside_something_else() {
        assert_eq!(judge(Some("MIT OR LGPL-2.1-or-later")), Ok(()));
        assert_eq!(
            judge(Some("LGPL-2.1-or-later")),
            Err(Refusal::Outside(vec!["LGPL-2.1-or-later".to_owned()]))
        );
    }

    #[test]
    fn an_exception_the_tree_has_not_carried_is_a_new_licence_too() {
        assert_eq!(judge(Some("Apache-2.0 WITH LLVM-exception")), Ok(()));
        assert!(judge(Some("Apache-2.0 WITH Classpath-exception-2.0")).is_err());
    }

    #[test]
    fn a_package_that_says_nothing_readable_is_stopped_rather_than_passed() {
        assert_eq!(judge(None), Err(Refusal::Undeclared));
        assert!(matches!(
            judge(Some("MIT OR OR")),
            Err(Refusal::Unreadable(_))
        ));
        assert!(judge(Some("LicenseRef-proprietary")).is_err());
    }
}
