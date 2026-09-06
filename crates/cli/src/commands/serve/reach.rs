//! Whether anybody outside can actually get in.
//!
//! A socket that is bound is not a socket that can be reached. On a home network there
//! is a router in front of it that drops everything nobody inside asked for, and it
//! keeps doing that until somebody opens its settings and sends the port to this
//! machine. Most people never do, and nothing in this client can do it for them.
//!
//! So rather than print an address and let the person find out weeks later that
//! nobody ever arrived, this knocks on that address from here and says which of the
//! three things happened.
//!
//! THE ANSWER IS ONLY EVER WRONG IN ONE DIRECTION. When the knock comes back to this
//! node the port reaches this machine, and that is settled: the far end proved it by
//! holding this node's key, which nothing else on the internet can do. When it does
//! not come back, either the port is shut or the router will not let a machine inside
//! it dial its own outside address, and plenty of routers will not. A knock that works
//! is proof. A knock that fails is a warning.

use std::net::SocketAddr;
use std::sync::Arc;

use n333_net::doorway::{self, Asked};
use n333_net::{Invite, PeerAddress, initiate};
use tokio::sync::watch;

use crate::commands::hours::Board;
use crate::dial::Dialer;
use crate::node::Node;

/// Find out whether the bound port can be reached from outside, and say so.
///
/// Runs on its own, because it takes a round trip to the meeting point and another to
/// this node's own front door, and nothing else should be waiting on either. `announce`
/// means the person already told this node what to hand out, so the address found here
/// is reported and not published over the top of theirs.
pub(super) fn tell_them(
    board: Board,
    dialer: Dialer,
    node: Arc<Node>,
    bound: SocketAddr,
    found_address: watch::Sender<Option<PeerAddress>>,
    ask_the_router: bool,
) {
    tokio::spawn(async move {
        if ask_the_router {
            ask(bound.port()).await;
        }
        let Some(seen) = board.what_address_do_i_arrive_from().await else {
            return;
        };
        let outside = PeerAddress::from(SocketAddr::new(seen, bound.port()));
        match knock(&dialer, &node, &outside).await {
            Answer::ItWasUs => {
                aloud!(
                    "open     port {} reaches this machine from outside. This node knocked at\n\
                     \x20        {outside} and answered itself, so that address is one you can\n\
                     \x20        hand to anybody.",
                    bound.port()
                );
                aloud!("invite   {}", Invite::to(outside.clone()));
                // Only now, and only if nothing better is already standing: an onion
                // address is reachable from everywhere and this one is reachable from
                // wherever the router allows, so the onion address wins if it arrives.
                if found_address.borrow().is_none() {
                    let _ = found_address.send(Some(outside));
                }
            }
            Answer::SomebodyElse => aloud!(
                "shut     something answered at {outside} and it was not this node. That port\n\
                 \x20        on your address belongs to something else, so an invitation naming\n\
                 \x20        it would send people to the wrong machine."
            ),
            Answer::Nothing => aloud!(
                "shut     nothing answered at {outside}, so as far as the outside world can\n\
                 \x20        tell this node is not listening. Either the router in front of it\n\
                 \x20        was never told to send port {} here, or it will not let a machine\n\
                 \x20        inside it dial its own outside address. `333 serve --tor` needs no\n\
                 \x20        router change at all and works from any network, including the\n\
                 \x20        ones that hand out no reachable address in the first place.",
                bound.port()
            ),
        }
    });
}

/// Ask the router to send this port here, and say what it said.
///
/// Said and not acted on: the knock that follows is what decides, and a router that
/// says yes and is wrong looks exactly like a router that says yes and is right.
async fn ask(port: u16) {
    match doorway::ask_the_router(port).await {
        Asked::Forwarded { outside, port } => aloud!(
            "opened   the router says port {port} on {outside} now comes to this machine. It\n\
             \x20        is listed there as `333` if you want to take it away again. Whether\n\
             \x20        anything arrives is the next line."
        ),
        Asked::NobodyAnswered => aloud!(
            "closed   no router here answered the request to open a port. That is ordinary:\n\
             \x20        plenty have it turned off, and a machine with an address of its own\n\
             \x20        has nothing to ask. `--no-upnp` stops this node asking at all."
        ),
        Asked::Refused(why) => aloud!("closed   the router would not open port {port}: {why}"),
    }
}

/// What knocking on this node's own outside address found.
enum Answer {
    /// This node answered itself: the port reaches this machine.
    ItWasUs,
    /// Something answered and it held a different key.
    SomebodyElse,
    /// Nothing answered at all.
    Nothing,
}

/// Knock once, and see who is there.
async fn knock(dialer: &Dialer, node: &Node, outside: &PeerAddress) -> Answer {
    let Ok(mut stream) = dialer.dial(outside).await else {
        return Answer::Nothing;
    };
    let identity = node.identity();
    match initiate(&mut stream, identity).await {
        Ok(exchange) if exchange.peer.node_id == identity.node_id() => Answer::ItWasUs,
        Ok(_) => Answer::SomebodyElse,
        Err(_) => Answer::Nothing,
    }
}
