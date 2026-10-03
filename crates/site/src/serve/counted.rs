//! How many of us are where: what the map draws and its table lists, counted once for
//! the page and for `GET /333/where-we-are` alike. Counts only; names nobody.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;
use serde_json::Value;
use sha2::{Digest as _, Sha256};

use crate::board::Board;
use crate::place::Place;

/// Where the onion's corner of the map is, in the map's own units: its left edge, its
/// top, its width and its height. Bottom left, in the empty South Pacific, beside the
/// onion drawn at (56, 556) in `map.html`.
const TOR_CORNER: [u32; 4] = [92, 526, 84, 66];

/// What the map shows.
#[derive(Debug, Serialize)]
pub(crate) struct Where {
    /// When this was counted, in milliseconds.
    pub(crate) as_of: u64,
    /// How many statements are held.
    pub(crate) saying: usize,
    /// How many name an onion address.
    pub(crate) tor: usize,
    /// How many the edge could not place.
    pub(crate) unplaced: usize,
    /// Countries, most first, then by code.
    pub(crate) countries: Vec<Country>,
    /// One `[x, y]` per placed statement, in whole degrees.
    pub(crate) dots: Vec<[i32; 2]>,
    /// One `[x, y]` per onion statement, in the map's units, inside the onion's corner.
    pub(crate) tor_dots: Vec<[u32; 2]>,
    /// The nodes this site's node knows of (its line's founder and everybody on the roll)
    /// that left nothing on the board; `null` when there is no observation to say.
    pub(crate) unsaid: Option<usize>,
}

/// One country and how many statements came from it.
#[derive(Debug, Serialize)]
pub(crate) struct Country {
    /// Two letters.
    pub(crate) c: String,
    /// How many.
    pub(crate) n: usize,
}

/// Count the board's live statements at `now`, and the observed nodes that made none.
pub(crate) fn count(board: &Board, observed: Option<&Value>, now: u64) -> Where {
    let mut counted: BTreeMap<String, usize> = BTreeMap::new();
    let mut said = BTreeSet::new();
    let mut found = Where {
        as_of: now,
        saying: 0,
        tor: 0,
        unplaced: 0,
        countries: Vec::new(),
        dots: Vec::new(),
        tor_dots: Vec::new(),
        unsaid: None,
    };
    for line in board.alive(now) {
        found.saying += 1;
        if let Some(statement) = line.said() {
            said.insert(statement.node.clone());
        }
        match line.place() {
            Some(Place::Tor) => {
                found.tor += 1;
                found.tor_dots.push(onion_spot(&line_name(line)));
            }
            None => found.unplaced += 1,
            Some(Place::At { country, y, x }) => {
                *counted.entry(country.clone()).or_default() += 1;
                found.dots.push([*x, *y]);
            }
        }
    }
    found.countries = counted.into_iter().map(|(c, n)| Country { c, n }).collect();
    found
        .countries
        .sort_by(|one, two| two.n.cmp(&one.n).then_with(|| one.c.cmp(&two.c)));
    found.unsaid = observed.map(|observed| unsaid(observed, &said));
    found
}

/// The name a line is about, from its signature if it has one.
fn line_name(line: &crate::board::Line) -> String {
    line.said()
        .map(|said| said.node.clone())
        .unwrap_or_default()
}

/// Where in the onion's corner a node's dot goes: the first bytes of a hash of its name,
/// so the same node is always in the same spot and nothing about it is told by where.
pub(crate) fn onion_spot(node: &str) -> [u32; 2] {
    let digest = Sha256::digest(node.as_bytes());
    let [left, top, wide, high] = TOR_CORNER;
    let across = digest.first().copied().map_or(0, u32::from);
    let down = digest.get(1).copied().map_or(0, u32::from);
    [left + across * wide / 256, top + down * high / 256]
}

/// How many of the observed founders and roll members said nothing on the board.
fn unsaid(observed: &Value, said: &BTreeSet<String>) -> usize {
    observed
        .get("nodes")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|node| {
            let founder = node.get("founder").and_then(Value::as_bool) == Some(true);
            let admitted = node
                .get("admitted")
                .is_some_and(|admitted| !admitted.is_null());
            founder || admitted
        })
        .filter_map(|node| node.get("id").and_then(Value::as_str))
        .filter(|id| !said.contains(*id))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_onion_spot_is_inside_the_corner_and_always_the_same() {
        let one = onion_spot(&"3".repeat(64));
        assert_eq!(one, onion_spot(&"3".repeat(64)));
        assert_ne!(one, onion_spot(&"4".repeat(64)));
        for name in ["", "a", "333abc", &"f".repeat(64)] {
            let [x, y] = onion_spot(name);
            assert!((92..176).contains(&x) && (526..592).contains(&y), "{x} {y}");
        }
    }

    #[test]
    fn the_unsaid_are_the_founder_and_the_roll_less_whoever_spoke() {
        let observed = serde_json::json!({ "nodes": [
            { "id": "f", "founder": true, "admitted": null },
            { "id": "a", "admitted": 3 },
            { "id": "b", "admitted": 4 },
            { "id": "s", "admitted": null },
        ] });
        let said: BTreeSet<String> = ["a".to_owned()].into();
        assert_eq!(unsaid(&observed, &said), 2);
    }
}
