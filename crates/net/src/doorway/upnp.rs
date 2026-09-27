//! Asking over UPnP-IGD, the protocol most home routers already speak.
//!
//! The router is found by a broadcast onto the local network rather than by address,
//! and it is asked for the mapping permanently. Nothing here renews it: a permanent
//! mapping needs no renewing, and a router that would only give a day is asked again
//! the next time the node starts.

use std::net::SocketAddr;
use std::time::Duration;

use igd_next::aio::tokio as igd;
use igd_next::{PortMappingProtocol, SearchOptions};

use super::{Asked, Lasts, WHAT_FOR, Way, address_the_router_sees};

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

/// Ask whatever answers the search to send `port` here, on the same port outside.
pub(super) async fn ask(port: u16) -> Asked {
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
    let mut lasts = Lasts::UntilTakenAway;
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
            return Asked::Refused(format!("UPnP-IGD said {for_ever}, and for a day: {a_day}"));
        }
        lasts = Lasts::For(Duration::from_secs(u64::from(A_DAY)));
    }
    Asked::Forwarded {
        // The mapping was made and the router may still not say what address it is
        // on. That is the mapping, so it is reported as one; the knock finds the rest.
        outside: gateway.get_external_ip().await.ok(),
        port,
        by: Way::Upnp,
        lasts,
    }
}
