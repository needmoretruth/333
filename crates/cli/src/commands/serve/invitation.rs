//! The invitation a bound socket can honestly print, kept apart because it is the one
//! place that decides whether this node knows its own address well enough to hand out.

use std::net::SocketAddr;

use n333_net::{Invite, PeerAddress};
use tokio::sync::watch;

use crate::words::Arg;

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
        aloud_in!(
            "serve-invitation-somewhere",
            port = Arg::exact(bound.port())
        );
        return;
    }
    let address = PeerAddress::from(bound);
    aloud_in!(
        "serve-invitation-invite",
        invitation = Invite::to(address.clone()).to_string()
    );
    // Only an address this node can actually stand behind is signed and handed on.
    let _ = found_address.send(Some(address));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let pairs = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            [
                (
                    words!("serve-invitation-somewhere", port = Arg::exact(3333)),
                    "invite   333:<an address others can reach>:3333",
                ),
                (
                    words!("serve-invitation-invite", invitation = "333:192.0.2.7:3333"),
                    "invite   333:192.0.2.7:3333",
                ),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

    #[test]
    fn a_port_is_a_name_and_is_never_written_in_twelve() {
        let said = crate::words::speaking("en", crate::words::count::Base::Twelve, || {
            words!("serve-invitation-somewhere", port = Arg::exact(3333))
        });
        assert!(said.ends_with(":3333"), "{said}");
    }
}
