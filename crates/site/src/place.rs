//! Where a node was when it left a statement, as coarsely as the map needs.
//!
//! Cloudflare places a request and says so in three headers; Caddy passes them on, and
//! only Cloudflare can reach Caddy, so they are Cloudflare's word and nobody else's.
//! Whole degrees are about a hundred kilometres, which is less than the address on the
//! board already gives away to anybody who looks it up.
//!
//! A STATEMENT NAMING AN ONION ADDRESS IS PLACED NOWHERE. The request that carried it
//! came over the ordinary internet, so the edge knows the country it came from, and that
//! is exactly the fact the onion address exists to withhold. `"tor"` is all that is
//! kept, so there is nothing in the record to leak afterwards.

use hyper::HeaderMap;
use serde::{Deserialize, Serialize};

/// Where a statement came from, or the fact that it will not say.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Stored", into = "Stored")]
pub(crate) enum Place {
    /// The statement names an onion address.
    Tor,
    /// A country and whole degrees.
    At {
        /// Two upper-case letters.
        country: String,
        /// Degrees north.
        y: i32,
        /// Degrees east.
        x: i32,
    },
}

/// The shape the Worker stored a place in: the word `"tor"` or `{c, y, x}`.
///
/// Kept so a board exported from the Worker reads here unchanged.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Stored {
    /// Only ever `"tor"`.
    Word(String),
    /// A placed statement.
    At {
        /// Country.
        c: String,
        /// Degrees north.
        y: i32,
        /// Degrees east.
        x: i32,
    },
}

impl TryFrom<Stored> for Place {
    type Error = String;
    fn try_from(stored: Stored) -> Result<Self, Self::Error> {
        match stored {
            Stored::Word(word) if word == "tor" => Ok(Self::Tor),
            Stored::Word(word) => Err(format!("a place is \"tor\" or a country, not {word:?}")),
            Stored::At { c, y, x } => Ok(Self::At { country: c, y, x }),
        }
    }
}

impl From<Place> for Stored {
    fn from(place: Place) -> Self {
        match place {
            Place::Tor => Self::Word("tor".to_owned()),
            Place::At { country, y, x } => Self::At { c: country, y, x },
        }
    }
}

/// Place a statement naming `address`, from the headers of the request that carried it.
///
/// `None` where the edge could not place it. Nothing is guessed.
pub(crate) fn of(headers: &HeaderMap, address: &str) -> Option<Place> {
    if host_of(address).ends_with(".onion") {
        return Some(Place::Tor);
    }
    let country = header(headers, "cf-ipcountry")?;
    if country.len() != 2 || !country.bytes().all(|byte| byte.is_ascii_uppercase()) {
        return None;
    }
    let y = degrees(header(headers, "cf-iplatitude")?)?;
    let x = degrees(header(headers, "cf-iplongitude")?)?;
    if !(-90.0..=90.0).contains(&y) || !(-180.0..=180.0).contains(&x) {
        return None;
    }
    Some(Place::At {
        country: country.to_owned(),
        y: rounded(y),
        x: rounded(x),
    })
}

/// One header as text, if it is there and is text.
fn header<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers.get(name).and_then(|value| value.to_str().ok())
}

/// A number of degrees, if the header holds a finite one.
fn degrees(text: &str) -> Option<f64> {
    text.trim()
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
}

/// Rounded the way the Worker's `Math.round` did: halves go up, so -2.5 is -2.
#[allow(clippy::cast_possible_truncation)] // in range: checked to be within ±180 first
fn rounded(degrees: f64) -> i32 {
    (degrees + 0.5).floor() as i32
}

/// The host out of a `host:port` address, lower-cased.
///
/// Split on the last colon so a bracketed IPv6 host keeps everything before its port.
pub(crate) fn host_of(address: &str) -> String {
    let host = address
        .rfind(':')
        .map_or(address, |cut| address.get(..cut).unwrap_or(address));
    host.to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn headers(pairs: &[(&'static str, &'static str)]) -> HeaderMap {
        let mut map = HeaderMap::new();
        for (name, value) in pairs {
            map.insert(*name, value.parse().unwrap());
        }
        map
    }

    #[test]
    fn an_onion_address_is_tor_whatever_the_edge_says() {
        let edge = headers(&[
            ("cf-ipcountry", "KR"),
            ("cf-iplatitude", "37.5"),
            ("cf-iplongitude", "127"),
        ]);
        assert_eq!(of(&edge, "abc.onion:3333"), Some(Place::Tor));
    }

    #[test]
    fn a_placed_request_is_whole_degrees_rounded_as_the_worker_rounded() {
        let edge = headers(&[
            ("cf-ipcountry", "KR"),
            ("cf-iplatitude", "37.5"),
            ("cf-iplongitude", "-2.5"),
        ]);
        let place = of(&edge, "1.2.3.4:3333");
        assert_eq!(
            place,
            Some(Place::At {
                country: "KR".to_owned(),
                y: 38,
                x: -2
            })
        );
        assert_eq!(
            serde_json::to_string(&place).unwrap(),
            r#"{"c":"KR","y":38,"x":-2}"#
        );
    }

    #[test]
    fn a_request_the_edge_could_not_place_is_not_placed() {
        assert_eq!(
            of(&headers(&[("cf-ipcountry", "KR")]), "1.2.3.4:3333"),
            None
        );
    }
}
