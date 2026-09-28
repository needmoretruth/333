//! Reading an address a person typed, and saying why one is not an address.
//!
//! The rules are `n333-net`'s. What is here is the saying: a refusal names what was
//! wrong and what the right form looks like, in the words this process speaks, before
//! anything is kept or knocked on. An address that could never answer, read and kept,
//! is knocked on by the vigil for as long as the node holds it.

use std::net::{IpAddr, SocketAddr};

use n333_net::PeerAddress;
use n333_net::invite::{self, InviteError, MAX_LEN};
use n333_net::peer::AddressError;

/// How many characters go before `.onion` in an onion address Tor still reaches.
const ONION_LETTERS: usize = 56;

/// An invitation or an address, for `join` and `ping` and the orders that do the same.
///
/// # Errors
/// Why it is neither, and how each is written.
pub(crate) fn typed(text: &str) -> Result<PeerAddress, String> {
    invite::address_or_invite(text)
        .map_err(|e| words!("typed-address-refused", why = why_not_an_invitation(&e)))
}

/// An address for `serve --announce`: where this node says it can be reached.
///
/// # Errors
/// Why it is not an address, and how one is written.
pub(crate) fn announced(text: &str) -> Result<PeerAddress, String> {
    text.parse::<PeerAddress>()
        .map_err(|e| words!("typed-address-refused-announce", why = why_not(&e)))
}

/// Where `serve --bind` listens: an address and a port, a bare address on the
/// port peers expect, or a bare `:port` on every interface.
///
/// # Errors
/// Why it is none of those, and how one is written.
pub(crate) fn bind(text: &str) -> Result<SocketAddr, String> {
    let text = text.trim();
    if let Ok(socket) = text.parse::<SocketAddr>() {
        return Ok(socket);
    }
    let bare = text
        .strip_prefix('[')
        .and_then(|rest| rest.strip_suffix(']'))
        .unwrap_or(text);
    if let Ok(address) = bare.parse::<IpAddr>() {
        return Ok(SocketAddr::new(address, n333_net::DEFAULT_PORT));
    }
    if let Some(port) = text.strip_prefix(':')
        && let Ok(port) = port.parse::<u16>()
    {
        return Ok(SocketAddr::from(([0, 0, 0, 0], port)));
    }
    Err(words!("typed-address-refused-bind", typed = text))
}

/// What was wrong with something that was meant to be an invitation or an address.
fn why_not_an_invitation(refused: &InviteError) -> String {
    match refused {
        InviteError::NotAnInvitation => words!("typed-address-no-tag"),
        InviteError::TooLong(length) => {
            words!("typed-address-too-long", most = MAX_LEN, length = *length)
        }
        InviteError::Address(refused) => why_not(refused),
        InviteError::NotCanonical { canonical } => {
            words!("typed-address-not-canonical", canonical = canonical)
        }
        InviteError::WrongPrefix(number) => words!("typed-address-wrong-tag", number = number),
    }
}

/// What was wrong with something that was meant to be an address.
fn why_not(refused: &AddressError) -> String {
    match refused {
        AddressError::Empty => words!("typed-address-empty"),
        AddressError::BadPort(port) => words!("typed-address-bad-port", port = port),
        AddressError::UnclosedBracket => words!("typed-address-unclosed"),
        AddressError::Scheme(scheme) => words!("typed-address-scheme", scheme = scheme),
        AddressError::NotAHost(host) => words!("typed-address-not-a-host", host = host),
        AddressError::NotAnOnion(host) => words!(
            "typed-address-not-an-onion",
            host = host,
            letters = ONION_LETTERS
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::words::count::Base;

    #[test]
    fn each_refusal_says_what_was_wrong_and_what_the_right_form_is() {
        let [scheme, number, onion, announce, listen] =
            crate::words::speaking("en", Base::Ten, || {
                [
                    typed("http://127.0.0.1:43331").expect_err("a web address"),
                    typed("334:127.0.0.1:43331").expect_err("the wrong tag"),
                    typed("abc.onion:3333").expect_err("not a v3 onion"),
                    announced("not an address").expect_err("not a host"),
                    bind("localhost").expect_err("a name is not something to listen on"),
                ]
            });
        let form = ". An address is host:port, as in node.example:3333, and\n\
                    an invitation is 333: and an address, as in 333:node.example:3333.";
        assert_eq!(
            scheme,
            format!("http:// belongs to a web address and not to an address here{form}")
        );
        assert_eq!(
            number,
            format!("an invitation starts with 333:, not 334:{form}")
        );
        assert_eq!(
            onion,
            format!(
                "abc.onion is not an onion address, which is 56 letters and digits\n\
                 before .onion{form}"
            )
        );
        assert_eq!(
            announce,
            "\"not an address\" is not a host name or an IP address. An address is \
             host:port, as in node.example:3333."
        );
        assert_eq!(
            listen,
            "localhost is not an address to listen on. It is an IP address and\n\
             a port, as in 0.0.0.0:3333. The address alone listens on port 3333,\n\
             and :port alone listens on every address."
        );
    }

    #[test]
    fn a_bare_address_listens_on_the_port_peers_expect() {
        let at = |text: &str| bind(text).expect("reads").to_string();
        assert_eq!(at("127.0.0.1"), "127.0.0.1:3333");
        assert_eq!(at("::1"), "[::1]:3333");
        assert_eq!(at("[::1]"), "[::1]:3333");
        assert_eq!(at(":4444"), "0.0.0.0:4444");
        assert_eq!(at("127.0.0.1:0"), "127.0.0.1:0");
    }
}
