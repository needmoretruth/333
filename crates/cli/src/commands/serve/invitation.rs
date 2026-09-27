//! The invitation a bound socket can honestly print, kept apart because it is the one
//! place that decides whether this node knows its own address well enough to hand out.

use std::net::SocketAddr;

use n333_net::{Invite, PeerAddress};
use tokio::sync::watch;

/// Say what to hand somebody so they can find this node.
///
/// A wildcard bind is the ordinary case and it is the one where this node genuinely
/// does not know the answer: it is listening on every interface and has no idea which
/// address of the machine, if any, a stranger can reach. Printing `333:0.0.0.0:3333`
/// would look like an invitation and work for nobody, so it says what is missing and
/// leaves [`reach`](super::reach) to fill it in once the knock has come back.
pub(super) fn say_the_invitation(
    bound: SocketAddr,
    found_address: &watch::Sender<Option<PeerAddress>>,
) {
    if bound.ip().is_unspecified() {
        aloud!(
            "invite   333:<an address others can reach>:{}",
            bound.port()
        );
        return;
    }
    let address = PeerAddress::from(bound);
    aloud!("invite   {}", Invite::to(address.clone()));
    // Only an address this node can actually stand behind is signed and handed on.
    let _ = found_address.send(Some(address));
}
