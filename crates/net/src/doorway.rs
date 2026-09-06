//! Asking the router to send a port to this machine.
//!
//! This is the thing that makes a BitTorrent client work on a home connection without
//! anybody opening anything. The router in front of a household drops every connection
//! nobody inside asked for, and it keeps doing that until it is told otherwise — but
//! most of them will be told, by a program on the inside, over a protocol they already
//! speak. It is called UPnP-IGD and it is on by default on a great many of them.
//!
//! NOTHING IS ASSUMED FROM A YES. The router saying it added the mapping is the router
//! saying it added the mapping. Whether anything actually arrives depends on the rest
//! of the path — a second router behind the first, a provider that shares one address
//! between many households — and neither of those tells you anything here. So this is
//! asked, and then the answer is measured by knocking, and the knock is what decides.
//!
//! IT IS A DOOR IN SOMEBODY'S HOUSE. What this asks for is real: a port on the
//! household's address, pointed at this machine, until the router forgets it. So it
//! says exactly what it asked for and exactly what it was told, in the same words a
//! person would need to go and undo it.

use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

use igd_next::aio::tokio as igd;
use igd_next::{PortMappingProtocol, SearchOptions};

/// What the router is told this mapping is for.
///
/// It shows up in the router's own list of forwarded ports, which is where somebody
/// goes to find out what asked for what. A description nobody recognises is a
/// description that gets left in place for years.
const WHAT_FOR: &str = "333";

/// How long to wait for a router to answer the search at all.
///
/// The search is a UDP broadcast onto the local network. A router that speaks this
/// answers in milliseconds; one that does not, never answers, and every second past
/// the first few is a second the node spends not starting.
const PATIENCE: Duration = Duration::from_secs(5);

/// How long the mapping is asked for, in seconds. Zero means until the router forgets.
///
/// Asked for permanently because the alternative is a node that was reachable this
/// morning and is not this afternoon, for a reason nothing on the screen would ever
/// mention. A router that refuses to make one permanent is asked for a day instead.
const FOR_EVER: u32 = 0;

/// A day, for a router that will not make one permanent.
const A_DAY: u32 = 86_400;

/// What came of asking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asked {
    /// The router says the port now comes here.
    ///
    /// It says so. Whether anything arrives is a different question and is answered by
    /// knocking, not by this.
    Forwarded {
        /// The address the router says the household is at.
        outside: IpAddr,
        /// The port that was asked for, on both sides.
        port: u16,
    },
    /// No router answered the search.
    ///
    /// Ordinary rather than broken: plenty of routers have this turned off, plenty of
    /// networks have no router that speaks it, and a machine with a public address of
    /// its own has nothing to ask.
    NobodyAnswered,
    /// A router answered and would not do it.
    Refused(String),
}

/// Ask the router to send `port` to this machine.
///
/// `port` is used on both sides: the port the world would knock on and the port this
/// node is listening on are the same number, so an invitation says one thing rather
/// than two.
///
/// Nothing here fails in a way a caller should stop for. A node that cannot be
/// forwarded is a node that carries on and finds out by knocking.
pub async fn ask_the_router(port: u16) -> Asked {
    let options = SearchOptions {
        timeout: Some(PATIENCE),
        ..SearchOptions::default()
    };
    let Ok(gateway) = igd::search_gateway(options).await else {
        return Asked::NobodyAnswered;
    };
    let Some(local) = address_the_router_sees(gateway.addr) else {
        return Asked::Refused("this machine has no address on the router's network".to_owned());
    };
    let here = SocketAddr::new(local, port);
    let asked = gateway
        .add_port(PortMappingProtocol::TCP, port, here, FOR_EVER, WHAT_FOR)
        .await;
    if let Err(for_ever) = asked {
        // Some routers will only make a mapping that expires. A day is far longer than
        // the epoch this node is in and the ask is made again the next time it starts.
        if let Err(a_day) = gateway
            .add_port(PortMappingProtocol::TCP, port, here, A_DAY, WHAT_FOR)
            .await
        {
            return Asked::Refused(format!("{for_ever}, and for a day: {a_day}"));
        }
    }
    match gateway.get_external_ip().await {
        Ok(outside) => Asked::Forwarded { outside, port },
        // The mapping was made and the router will not say what address it is on.
        // That is the mapping, so it is reported as one; the knock finds the address.
        Err(_) => Asked::Forwarded {
            outside: gateway.addr.ip(),
            port,
        },
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

    #[tokio::test]
    async fn the_way_out_is_found_or_it_is_not_and_neither_is_an_error() {
        // Whatever this machine is on, asking produces one of the three and never a
        // panic or a hang. A build machine with no router answers `NobodyAnswered`.
        let asked = ask_the_router(0).await;
        assert!(matches!(
            asked,
            Asked::Forwarded { .. } | Asked::NobodyAnswered | Asked::Refused(_)
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
