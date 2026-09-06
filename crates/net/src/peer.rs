//! How a peer's address is written, and what it says about how to reach it.
//!
//! THE ADDRESS DECIDES THE TRANSPORT. There is no flag for it and no negotiation. An
//! address ending in `.onion` is reached through Tor; anything else is reached by
//! opening a socket to it. This is the whole rule, and it is a rule rather than an
//! option for two reasons: a node that thought it was hidden because it passed a flag
//! somewhere is one misconfiguration away from not being, and sending clearnet
//! traffic through Tor would put an exit node in the middle of every exchange, which
//! buys nothing here — the peer already knows who it is talking to.
//!
//! Most nodes never touch Tor. It is for people who need their own address not to be
//! seen, and it costs a bootstrap of seconds to minutes before the first byte moves.

use std::fmt;
use std::str::FromStr;

/// The port a node listens on when it is reachable directly.
///
/// Not 333: ports below 1024 need privileges the client should never ask for, and a
/// client that asks to be run as root has failed at something more important than a
/// memorable number.
pub const DEFAULT_PORT: u16 = 3333;

/// The virtual port used inside an onion address.
///
/// FROZEN, and unrelated to [`DEFAULT_PORT`]. Onion-service ports are virtual: they
/// are numbers inside the Tor protocol, not sockets on the host, so 333 collides with
/// nothing and needs no privileges.
pub const ONION_PORT: u16 = 333;

/// The suffix that marks an address as reachable only through Tor.
pub const ONION_SUFFIX: &str = ".onion";

/// Where a peer is, and therefore how to reach it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PeerAddress {
    /// A host and port to open a socket to. The default.
    Direct {
        /// A hostname or an IP literal, without brackets even if it is IPv6.
        host: String,
        /// The port to connect to.
        port: u16,
    },
    /// An onion address, reached through Tor.
    Onion {
        /// The full address including the `.onion` suffix.
        host: String,
        /// The virtual port inside the onion service.
        port: u16,
    },
}

/// Why an address could not be read.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum AddressError {
    /// Nothing but whitespace, or nothing at all.
    #[error("no address given")]
    Empty,
    /// The part after the last colon is not a port number.
    #[error("{0:?} is not a port number between 1 and 65535")]
    BadPort(String),
    /// An IPv6 literal was opened with `[` and never closed.
    #[error("an address starting with '[' must contain a matching ']'")]
    UnclosedBracket,
}

impl PeerAddress {
    /// The host as it should be handed to whatever opens the connection.
    #[must_use]
    pub fn host(&self) -> &str {
        match self {
            Self::Direct { host, .. } | Self::Onion { host, .. } => host,
        }
    }

    /// Whether an address is worth handing to somebody on another network.
    ///
    /// A node binds to what it can bind to, and what it can bind to is often a place that
    /// means something only where it is standing. Loopback is the machine itself. A private
    /// range is the building it is in. A link local address is the wire it is plugged into.
    /// Every one of them is a correct address for a neighbour and a wrong one for a stranger,
    /// and a stranger who dials it reaches something of their own or nothing at all.
    ///
    /// A NAME IS ALWAYS WORTH HANDING OVER, because a name is resolved by whoever dials it
    /// and only they can say where it points. This refuses what it can prove is local and
    /// nothing else.
    #[must_use]
    pub fn worth_telling_a_stranger(&self) -> bool {
        let Ok(address) = self.host().parse::<std::net::IpAddr>() else {
            // A hostname, or an onion address, which is reachable from anywhere by design.
            return true;
        };
        match address {
            std::net::IpAddr::V4(four) => {
                !(four.is_loopback()
                    || four.is_private()
                    || four.is_link_local()
                    || four.is_unspecified()
                    || four.is_broadcast()
                    || four.is_documentation()
                    || four.is_multicast()
                    // 100.64.0.0/10, the range an internet provider puts between a household
                    // and the rest of the world. A node here is behind two layers of address
                    // translation and cannot be reached at this address by anybody.
                    || (four.octets()[0] == 100 && (64..128).contains(&four.octets()[1])))
            }
            std::net::IpAddr::V6(six) => {
                let first = six.segments()[0];
                !(six.is_loopback()
                    || six.is_unspecified()
                    || six.is_multicast()
                    // fc00::/7, addresses that route inside one organisation and nowhere else.
                    || (first & 0xfe00) == 0xfc00
                    // fe80::/10, addresses that mean nothing off the wire they are on.
                    || (first & 0xffc0) == 0xfe80)
            }
        }
    }

    /// The port to connect to.
    #[must_use]
    pub const fn port(&self) -> u16 {
        match self {
            Self::Direct { port, .. } | Self::Onion { port, .. } => *port,
        }
    }

    /// Does reaching this peer require a Tor client?
    #[must_use]
    pub const fn needs_tor(&self) -> bool {
        matches!(self, Self::Onion { .. })
    }
}

impl FromStr for PeerAddress {
    type Err = AddressError;

    /// Read an address in any of the forms a person writes one.
    ///
    /// `host`, `host:port`, `1.2.3.4`, `1.2.3.4:port`, `[::1]`, `[::1]:port`, `::1`,
    /// and the same with a `.onion` host. A bare IPv6 literal is recognised by
    /// holding more than one colon, which is why the bracketed form exists at all:
    /// without brackets there is no way to tell `::1:3333` apart from a port.
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let text = text.trim();
        if text.is_empty() {
            return Err(AddressError::Empty);
        }
        let (host, port) = split_host_and_port(text)?;
        if host.is_empty() {
            return Err(AddressError::Empty);
        }
        let host = host.to_ascii_lowercase();
        if host.ends_with(ONION_SUFFIX) {
            Ok(Self::Onion {
                port: port.unwrap_or(ONION_PORT),
                host,
            })
        } else {
            Ok(Self::Direct {
                port: port.unwrap_or(DEFAULT_PORT),
                host,
            })
        }
    }
}

/// Split an address into its host and its port, if it names one.
fn split_host_and_port(text: &str) -> Result<(&str, Option<u16>), AddressError> {
    if let Some(rest) = text.strip_prefix('[') {
        let (host, after) = rest
            .split_once(']')
            .ok_or(AddressError::UnclosedBracket)?;
        return match after.strip_prefix(':') {
            Some(port) => Ok((host, Some(parse_port(port)?))),
            None if after.is_empty() => Ok((host, None)),
            None => Err(AddressError::BadPort(after.to_owned())),
        };
    }
    // More than one colon and no brackets: a bare IPv6 literal, which cannot also
    // carry a port. One colon: a host and a port.
    match text.rsplit_once(':') {
        Some((host, port)) if !host.contains(':') => Ok((host, Some(parse_port(port)?))),
        _ => Ok((text, None)),
    }
}

/// Read a port, refusing zero: it means "any port" to the operating system and can
/// never be the port a peer is waiting on.
fn parse_port(text: &str) -> Result<u16, AddressError> {
    match text.parse::<u16>() {
        Ok(port) if port != 0 => Ok(port),
        _ => Err(AddressError::BadPort(text.to_owned())),
    }
}

impl From<std::net::SocketAddr> for PeerAddress {
    /// A bound socket, said the way a peer would be told to reach it.
    ///
    /// Always `Direct`: a socket address is a place on the network, and an onion
    /// address is not one.
    fn from(socket: std::net::SocketAddr) -> Self {
        Self::Direct {
            host: socket.ip().to_string(),
            port: socket.port(),
        }
    }
}

impl fmt::Display for PeerAddress {
    /// Written back the way it would be typed, brackets restored where needed.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let host = self.host();
        if host.contains(':') {
            write!(f, "[{host}]:{}", self.port())
        } else {
            write!(f, "{host}:{}", self.port())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(text: &str) -> PeerAddress {
        text.parse().expect("a readable address")
    }

    #[test]
    fn the_ports_are_the_agreed_ones() {
        assert_eq!(DEFAULT_PORT, 3333);
        assert_eq!(ONION_PORT, 333);
    }

    #[test]
    fn a_plain_host_is_reached_without_tor() {
        let address = parse("node.example:3333");
        assert_eq!(
            address,
            PeerAddress::Direct {
                host: "node.example".into(),
                port: 3333
            }
        );
        assert!(!address.needs_tor());
    }

    #[test]
    fn an_address_that_means_something_only_here_is_not_handed_to_a_stranger() {
        for local in [
            "127.0.0.1:3333",
            "0.0.0.0:3333",
            "10.0.0.4:3333",
            "192.168.1.5:3333",
            "172.16.9.9:3333",
            "169.254.7.7:3333",
            "100.64.3.3:3333",
            "203.0.113.9:3333",
            "[::1]:3333",
            "[fe80::1]:3333",
            "[fc00::1]:3333",
        ] {
            assert!(
                !parse(local).worth_telling_a_stranger(),
                "{local} should not be published"
            );
        }
    }

    #[test]
    fn an_address_the_world_can_reach_is_handed_over() {
        for reachable in [
            "9.9.9.9:3333",
            "8.8.8.8:3333",
            "[2606:4700:4700::1111]:3333",
            "node.example.org:3333",
            "abcdefghij.onion",
        ] {
            assert!(
                parse(reachable).worth_telling_a_stranger(),
                "{reachable} should be published"
            );
        }
    }

    #[test]
    fn an_onion_host_is_reached_through_tor() {
        let address = parse("abcdefghij.onion");
        assert!(address.needs_tor());
        assert_eq!(address.port(), ONION_PORT);
        assert!(parse("abcdefghij.onion:333").needs_tor());
    }

    #[test]
    fn each_kind_has_its_own_default_port() {
        assert_eq!(parse("node.example").port(), DEFAULT_PORT);
        assert_eq!(parse("abcdefghij.onion").port(), ONION_PORT);
        assert_eq!(parse("node.example:9").port(), 9);
        assert_eq!(parse("abcdefghij.onion:9").port(), 9);
    }

    #[test]
    fn ipv6_needs_brackets_only_when_it_carries_a_port() {
        assert_eq!(parse("::1").host(), "::1");
        assert_eq!(parse("::1").port(), DEFAULT_PORT);
        assert_eq!(parse("[::1]").host(), "::1");
        assert_eq!(parse("[::1]:4444").host(), "::1");
        assert_eq!(parse("[::1]:4444").port(), 4444);
        // Without brackets the last group of an address is indistinguishable from a
        // port, so an unbracketed literal never gets one.
        assert_eq!(parse("2001:db8::1").host(), "2001:db8::1");
        assert_eq!(parse("2001:db8::1").port(), DEFAULT_PORT);
    }

    #[test]
    fn an_address_survives_being_written_and_read_again() {
        for text in ["node.example:3333", "[::1]:4444", "1.2.3.4:3333"] {
            assert_eq!(parse(text).to_string(), text);
            assert_eq!(parse(&parse(text).to_string()), parse(text));
        }
        assert_eq!(parse("abcdefghij.onion").to_string(), "abcdefghij.onion:333");
    }

    #[test]
    fn the_suffix_is_recognised_whatever_case_it_is_typed_in() {
        assert!(parse("ABCDEFGHIJ.ONION").needs_tor());
        assert_eq!(parse("Node.Example").host(), "node.example");
    }

    #[test]
    fn a_bound_socket_becomes_the_address_a_peer_would_be_given() {
        use std::net::SocketAddr;
        let v4: SocketAddr = "1.2.3.4:3333".parse().expect("a socket address");
        assert_eq!(PeerAddress::from(v4).to_string(), "1.2.3.4:3333");
        // The brackets come back on the way out, and only there.
        let v6: SocketAddr = "[2001:db8::1]:4444".parse().expect("a socket address");
        assert_eq!(PeerAddress::from(v6).host(), "2001:db8::1");
        assert_eq!(PeerAddress::from(v6).to_string(), "[2001:db8::1]:4444");
        assert_eq!(parse(&PeerAddress::from(v6).to_string()), PeerAddress::from(v6));
    }

    #[test]
    fn an_address_that_cannot_be_read_says_which_part_failed() {
        assert_eq!("".parse::<PeerAddress>(), Err(AddressError::Empty));
        assert_eq!("   ".parse::<PeerAddress>(), Err(AddressError::Empty));
        assert_eq!(
            "node.example:0".parse::<PeerAddress>(),
            Err(AddressError::BadPort("0".into())),
            "port zero means 'any port' and no peer can be waiting on it"
        );
        assert_eq!(
            "node.example:99999".parse::<PeerAddress>(),
            Err(AddressError::BadPort("99999".into()))
        );
        assert_eq!(
            "node.example:http".parse::<PeerAddress>(),
            Err(AddressError::BadPort("http".into()))
        );
        assert_eq!(
            "[::1".parse::<PeerAddress>(),
            Err(AddressError::UnclosedBracket)
        );
        assert_eq!(":3333".parse::<PeerAddress>(), Err(AddressError::Empty));
    }
}
