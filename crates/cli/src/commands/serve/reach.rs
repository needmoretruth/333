//! Whether anybody outside can actually get in.
//!
//! A socket that is bound is not a socket that can be reached. On a home network there
//! is a router in front of it that drops everything nobody inside asked for, and it
//! keeps doing that until somebody opens its settings and sends the port to this
//! machine, or a program inside asks it to. Most people never open the settings, so
//! this node asks ([`router`]) unless it was told not to.
//!
//! Then, rather than print an address and let the person find out weeks later that
//! nobody ever arrived, this knocks on that address from here and says which of the
//! three things happened.
//!
//! THE ANSWER IS ONLY EVER WRONG IN ONE DIRECTION. When the knock comes back to this
//! node the port reaches this machine, and that is settled: the far end proved it by
//! holding this node's key, which nothing else on the internet can do. When it does
//! not come back, either the port is shut or the router will not let a machine inside
//! it dial its own outside address, and plenty of routers will not. A knock that works
//! is proof. A knock that fails is a warning.

mod router;

use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use n333_net::doorway::{Asked, Lasts, Lease};
use n333_net::{Invite, PeerAddress, initiate};
use tokio::sync::{oneshot, watch};

use crate::commands::hours::Board;
use crate::dial::Dialer;
use crate::node::Node;

/// How long the end of the vigil waits for the router to take its port back.
///
/// Long enough for a router that is still being asked to finish being asked and then
/// be asked to forget it, which is two rounds of a few seconds; short enough that a
/// person who pressed Ctrl-C is not left wondering whether it heard.
const GIVING_BACK: Duration = Duration::from_secs(20);

/// What the router lent this vigil, to be given back when it ends.
pub(super) struct Kept {
    /// Tells the keeper the vigil is over.
    leaving: oneshot::Sender<()>,
    /// The keeper, which asks the router, keeps the lease and gives it back.
    keeper: tokio::task::JoinHandle<()>,
}

impl Kept {
    /// Give back whatever the router lent, and wait a little while for it to say so.
    ///
    /// A router that never answers leaves a mapping that runs out by itself, so this
    /// does not wait for ever, and says so if it stops waiting.
    pub(super) async fn give_back(self) {
        let _ = self.leaving.send(());
        if tokio::time::timeout(GIVING_BACK, self.keeper)
            .await
            .is_err()
        {
            println!(
                "closed   the router had not answered when the vigil ended. Whatever it\n\
                 \x20        agreed to runs out by itself within {}.",
                router::how_long(n333_net::doorway::ASK_FOR)
            );
        }
    }
}

/// Find out whether the bound port can be reached from outside, and say so.
///
/// Runs on its own, because it takes a round trip to the router, another to the
/// meeting point and another to this node's own front door, and nothing else should be
/// waiting on any of them. The router is asked whether or not there is a meeting
/// point: it is the one thing that changes whether the door opens, and it says what
/// address the household is at, which is enough to knock on when nobody else is asked.
pub(super) fn tell_them(
    board: Option<Board>,
    dialer: Dialer,
    node: Arc<Node>,
    bound: SocketAddr,
    found_address: watch::Sender<Option<PeerAddress>>,
    ask_the_router: bool,
) -> Kept {
    let (leaving, left) = oneshot::channel();
    let keeper = tokio::spawn(async move {
        let asked = if ask_the_router {
            router::ask(bound.port()).await
        } else {
            Asked::NobodyAnswered
        };
        let (said, lease) = lent(asked);
        // Knocked on alongside, not before: the lease has to be kept from the moment
        // it is made, and the knock can take as long as a dial takes.
        let port = said.map_or(bound.port(), |(_, port)| port);
        let router_said = said.and_then(|(at, _)| at);
        tokio::spawn(async move {
            let Some((seen, only_the_router)) = where_from(board.as_ref(), router_said).await
            else {
                return;
            };
            let outside = PeerAddress::from(SocketAddr::new(seen, port));
            if only_the_router && !outside.worth_telling_a_stranger() {
                aloud!(
                    "shut     the router says this household is at {seen}, which is not an\n\
                     \x20        address on the open internet: another router, or the provider's\n\
                     \x20        shared address, stands between it and everybody else, and\n\
                     \x20        nothing here can ask that one. `333 serve --tor` needs no router\n\
                     \x20        change at all."
                );
                return;
            }
            knock_and_say(&dialer, &node, outside, &found_address).await;
        });
        router::keep(lease, left).await;
    });
    Kept { leaving, keeper }
}

/// What the router's answer means here: where it says the port is, and what to keep.
///
/// Only a lease is kept. A UPnP mapping is left exactly as it was made.
fn lent(asked: Asked) -> (Option<(Option<IpAddr>, u16)>, Option<Lease>) {
    match asked {
        Asked::Forwarded {
            outside,
            port,
            lasts: Lasts::WhileKept(lease),
            ..
        } => (Some((outside, port)), Some(lease)),
        Asked::Forwarded { outside, port, .. } => (Some((outside, port)), None),
        Asked::NobodyAnswered | Asked::Refused(_) => (None, None),
    }
}

/// Where the household is, and whether it was only the router that said so.
///
/// The meeting point first, because it saw the address a stranger would actually
/// arrive from; a router behind a second router reports an address that is only
/// outside the first one, which is why the router's word is marked as the router's.
async fn where_from(board: Option<&Board>, router_said: Option<IpAddr>) -> Option<(IpAddr, bool)> {
    let seen = match board {
        Some(board) => board.what_address_do_i_arrive_from().await,
        None => None,
    };
    seen.map(|seen| (seen, false))
        .or(router_said.map(|said| (said, true)))
}

/// Knock on `outside`, and say which of the three things happened.
async fn knock_and_say(
    dialer: &Dialer,
    node: &Node,
    outside: PeerAddress,
    found_address: &watch::Sender<Option<PeerAddress>>,
) {
    match knock(dialer, node, &outside).await {
        Answer::ItWasUs => {
            aloud!(
                "open     port {} reaches this machine from outside. This node knocked at\n\
                 \x20        {outside} and answered itself, so that address is one you can\n\
                 \x20        hand to anybody.",
                outside.port()
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
            outside.port()
        ),
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
