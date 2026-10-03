//! The minute one address waits between two statements.
//!
//! A node has one thing to say and 333 minutes to say it in, so a minute between words is
//! generous three hundred times over and still stops one machine writing in a loop.
//!
//! THE ADDRESS IS NEVER KEPT. What is remembered is an HMAC of it under a key drawn when
//! the process starts and held only in memory, so neither this process's memory after a
//! restart nor anything on disk can be turned back into who wrote. Entries older than the
//! minute are dropped as soon as anybody else is checked.
//!
//! The Worker's other limit, 900 writes a day, is gone: it existed because the store the
//! Worker ran on refused writes after a thousand a day. A file on this disk does not.

use std::collections::HashMap;

use hmac::digest::generic_array::GenericArray;
use hmac::{Hmac, Mac as _};
use rand_core::{OsRng, RngCore as _};
use sha2::Sha256;

/// How long one address waits before it may leave another statement, in seconds.
pub(crate) const BETWEEN_WORDS: u64 = 60;

/// One HMAC-SHA256 block: the key length HMAC uses without hashing it first.
const KEY_LEN: usize = 64;

/// Who wrote when, by keyed name.
pub(crate) struct Gate {
    /// The key, drawn at start and never written anywhere.
    key: [u8; KEY_LEN],
    /// When each keyed name last opened the gate, in milliseconds.
    opened: HashMap<[u8; 32], u64>,
}

impl Gate {
    /// A gate with a fresh key from the operating system.
    pub(crate) fn new() -> Self {
        let mut key = [0_u8; KEY_LEN];
        OsRng.fill_bytes(&mut key);
        Self {
            key,
            opened: HashMap::new(),
        }
    }

    /// The keyed name of an address.
    fn name(&self, address: &str) -> [u8; 32] {
        // `from_slice` asserts the length, and KEY_LEN is the HMAC-SHA256 key size.
        let mut mac = Hmac::<Sha256>::new(GenericArray::from_slice(&self.key));
        mac.update(address.as_bytes());
        mac.finalize().into_bytes().into()
    }

    /// How many seconds `address` has to wait, or `None` if it may write now — in which
    /// case the minute starts again from `now_ms`.
    ///
    /// A request without an address (one that did not come through the edge) is never
    /// held back, as the Worker never held one back.
    pub(crate) fn too_soon(&mut self, address: Option<&str>, now_ms: u64) -> Option<u64> {
        let address = address?;
        let minute = BETWEEN_WORDS * 1000;
        self.opened
            .retain(|_, since| now_ms.saturating_sub(*since) < minute);
        let name = self.name(address);
        if let Some(since) = self.opened.get(&name) {
            return Some(seconds_left(*since, now_ms));
        }
        self.opened.insert(name, now_ms);
        None
    }
}

/// The whole seconds left of the minute for a gate opened at `since`: never under one,
/// because the gate was still there when it was asked, and never over the minute.
fn seconds_left(since: u64, now_ms: u64) -> u64 {
    let ends = since.saturating_add(BETWEEN_WORDS * 1000);
    let left = ends.saturating_sub(now_ms).div_ceil(1000);
    left.clamp(1, BETWEEN_WORDS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_same_address_waits_out_the_rest_of_the_minute() {
        let mut gate = Gate::new();
        assert_eq!(gate.too_soon(Some("192.0.2.1"), 1_000_000), None);
        assert_eq!(
            gate.too_soon(Some("192.0.2.1"), 1_000_000 + 20_500),
            Some(40)
        );
        assert_eq!(gate.too_soon(Some("192.0.2.2"), 1_000_000 + 20_500), None);
        assert_eq!(gate.too_soon(Some("192.0.2.1"), 1_000_000 + 60_000), None);
    }
}
