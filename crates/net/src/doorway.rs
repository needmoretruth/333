//! Asking the router to send a port to this machine.
//!
//! This is the thing that makes a BitTorrent client work on a home connection without
//! anybody opening anything. The router in front of a household drops every connection
//! nobody inside asked for, and it keeps doing that until it is told otherwise — but
//! most of them will be told, by a program on the inside, over a protocol they already
//! speak. There are three such protocols and a router speaks one or two of them:
//! UPnP-IGD ([`upnp`]) on most home routers, and PCP or its predecessor NAT-PMP
//! ([`lease`]) on Apple's, on many providers' boxes and on OpenWrt.
//!
//! UPNP FIRST, THEN PCP, THEN NAT-PMP. UPnP is asked first because it is what most
//! routers speak and what this client has always asked, so a router that said yes to
//! it before says yes to it again and nothing about that household changes. The other
//! two are asked only when UPnP found nobody or was refused: a router that already sent
//! the port here has nothing more to be asked, and asking it twice over two protocols
//! would leave two mappings to take away instead of one.
//!
//! NOTHING IS ASSUMED FROM A YES. The router saying it added the mapping is the router
//! saying it added the mapping. Whether anything actually arrives depends on the rest
//! of the path — a second router behind the first, a provider that shares one address
//! between many households — and neither of those tells you anything here. So this is
//! asked, and then the answer is measured by knocking, and the knock is what decides.
//!
//! IT IS A DOOR IN SOMEBODY'S HOUSE. What this asks for is real: a port on the
//! household's address, pointed at this machine, until the router forgets it. So it
//! says exactly what it asked for, over which protocol, what it was told and for how
//! long, in the same words a person would need to go and undo it.

pub mod gateway;
mod lease;
mod upnp;

use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

pub use lease::{ASK_FOR, Lease};

/// What the router is told this mapping is for.
///
/// It shows up in the router's own list of forwarded ports, which is where somebody
/// goes to find out what asked for what. A description nobody recognises is a
/// description that gets left in place for years. PCP and NAT-PMP carry no
/// description, so a mapping made over them is listed by port alone.
const WHAT_FOR: &str = "333";

/// Which of the three protocols the router said yes over.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Way {
    /// UPnP Internet Gateway Device, found by a broadcast and asked over HTTP.
    Upnp,
    /// The Port Control Protocol, RFC 6887.
    Pcp,
    /// The NAT Port Mapping Protocol, RFC 6886.
    NatPmp,
}

impl std::fmt::Display for Way {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Upnp => "UPnP-IGD",
            Self::Pcp => "PCP",
            Self::NatPmp => "NAT-PMP",
        })
    }
}

/// How long a mapping the router made stays made.
///
/// Said because it is part of what was changed in somebody's house: a mapping that
/// stays until it is taken away is a different thing to leave behind from one that
/// goes by itself in two hours.
#[derive(Debug, Clone)]
pub enum Lasts {
    /// Until somebody takes it away in the router's settings.
    UntilTakenAway,
    /// For this long, and then the router drops it by itself. It is not asked again.
    For(Duration),
    /// For as long as it is asked again, which is up to whoever holds this.
    WhileKept(Lease),
}

/// What came of asking.
#[derive(Debug, Clone)]
pub enum Asked {
    /// The router says the port now comes here.
    ///
    /// It says so. Whether anything arrives is a different question and is answered by
    /// knocking, not by this.
    Forwarded {
        /// The address the router says the household is at, if it would say.
        outside: Option<IpAddr>,
        /// The port the world would knock on. UPnP is always asked for this node's own
        /// port; PCP and NAT-PMP routers may choose another, and this is what they chose.
        port: u16,
        /// Which protocol the router said yes over.
        by: Way,
        /// How long it said yes for.
        lasts: Lasts,
    },
    /// No router answered any of the three.
    ///
    /// Ordinary rather than broken: plenty of routers have all of them turned off,
    /// plenty of networks have no router that speaks any, and a machine with a public
    /// address of its own has nothing to ask.
    NobodyAnswered,
    /// A router answered and would not do it, in its own words.
    Refused(String),
}

/// Ask the router to send `port` to this machine.
///
/// `port` is the port this node is listening on, and the port it asks for outside, so
/// that an invitation says one thing rather than two wherever the router agrees.
///
/// Nothing here fails in a way a caller should stop for. A node that cannot be
/// forwarded is a node that carries on and finds out by knocking.
pub async fn ask_the_router(port: u16) -> Asked {
    let upnp = upnp::ask(port).await;
    if matches!(upnp, Asked::Forwarded { .. }) {
        return upnp;
    }
    let Some(gateway) = gateway::default_gateway() else {
        return upnp;
    };
    either(upnp, lease::ask(gateway, port).await)
}

/// Put what UPnP said together with what PCP and NAT-PMP said.
///
/// A yes from either is the answer. Otherwise every refusal is kept, in the order it
/// was asked, because a person reading one of them would go and look in the wrong place.
fn either(upnp: Asked, then: Asked) -> Asked {
    match (upnp, then) {
        (_, forwarded @ Asked::Forwarded { .. }) | (forwarded @ Asked::Forwarded { .. }, _) => {
            forwarded
        }
        (Asked::Refused(first), Asked::Refused(second)) => {
            Asked::Refused(format!("{first}; {second}"))
        }
        (Asked::Refused(why), Asked::NobodyAnswered) => {
            Asked::Refused(format!("{why}; PCP and NAT-PMP were not answered"))
        }
        (Asked::NobodyAnswered, refused @ Asked::Refused(_)) => refused,
        (Asked::NobodyAnswered, Asked::NobodyAnswered) => Asked::NobodyAnswered,
    }
}

/// This machine's address on the network the router is on.
///
/// Found by asking the operating system which of this machine's addresses it would use
/// to reach the router. Nothing is sent: a datagram socket that has been given a
/// destination has picked an interface, and the interface is the answer. A machine with
/// several networks gets the one the router is actually on, which is the point.
fn address_the_router_sees(gateway: SocketAddr) -> Option<IpAddr> {
    let bind = if gateway.is_ipv6() {
        "[::]:0"
    } else {
        "0.0.0.0:0"
    };
    let socket = std::net::UdpSocket::bind(bind).ok()?;
    socket.connect(gateway).ok()?;
    Some(socket.local_addr().ok()?.ip())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_description_is_the_one_a_person_will_read_in_their_router() {
        assert_eq!(WHAT_FOR, "333");
    }

    #[test]
    fn the_protocols_are_named_the_way_router_settings_name_them() {
        let named = [Way::Upnp, Way::Pcp, Way::NatPmp].map(|way| way.to_string());
        assert_eq!(named, ["UPnP-IGD", "PCP", "NAT-PMP"]);
    }

    #[tokio::test]
    async fn the_way_out_is_found_or_it_is_not_and_neither_is_an_error() {
        // Whatever this machine is on, asking produces one of the three and never a
        // panic or a hang. A build machine with no router answers `NobodyAnswered`.
        // Port 0 is refused on this side before any router is asked to map it, by
        // igd-next and by the PCP half alike, so nothing is left behind on one.
        let asked = ask_the_router(0).await;
        assert!(matches!(
            asked,
            Asked::Forwarded { .. } | Asked::NobodyAnswered | Asked::Refused(_)
        ));
    }

    #[test]
    fn both_refusals_are_kept_in_the_order_they_were_asked() {
        let asked = either(
            Asked::Refused("UPnP-IGD said no".to_owned()),
            Asked::Refused("PCP: not authorised".to_owned()),
        );
        assert!(
            matches!(asked, Asked::Refused(why) if why == "UPnP-IGD said no; PCP: not authorised")
        );
    }

    #[test]
    fn a_refusal_is_not_hidden_behind_silence_on_the_other_two() {
        let asked = either(
            Asked::Refused("UPnP-IGD said no".to_owned()),
            Asked::NobodyAnswered,
        );
        assert!(matches!(asked, Asked::Refused(why) if why.starts_with("UPnP-IGD said no")));
    }

    #[test]
    fn silence_everywhere_is_nobody_answering() {
        assert!(matches!(
            either(Asked::NobodyAnswered, Asked::NobodyAnswered),
            Asked::NobodyAnswered
        ));
    }

    #[test]
    fn a_gateway_on_this_machine_has_an_address_to_answer_from() {
        // Loopback is a network like any other as far as this question goes, and it is
        // the one every machine has.
        let gateway: SocketAddr = "127.0.0.1:1900".parse().expect("an address");
        assert_eq!(
            address_the_router_sees(gateway),
            Some(IpAddr::from([127, 0, 0, 1]))
        );
    }
}
