//! The other way to be reachable: an onion address, for a node that is hiding. Kept
//! apart from the socket listener because only this half depends on Tor being built in.

use std::sync::Arc;

use n333_net::PeerAddress;
use tokio::sync::watch;

use crate::dial::Dialer;
use crate::node::Node;

use super::door::Door;

#[cfg(feature = "tor")]
use {
    super::door::{Caller, spawn_exchange},
    anyhow::Context as _,
    n333_net::Invite,
    n333_net::peer::ONION_PORT,
    n333_net::tor::SERVICE_NICKNAME,
    n333_net::tor::host::OnionHost,
};

/// Publish an onion address and answer every peer that arrives on it.
#[cfg(feature = "tor")]
pub(super) async fn answer(
    dialer: Dialer,
    node: Arc<Node>,
    door: Door,
    found_address: watch::Sender<Option<PeerAddress>>,
) -> anyhow::Result<()> {
    let client = dialer.tor().await?;
    let mut host = OnionHost::launch(&client, SERVICE_NICKNAME, ONION_PORT)
        .with_context(|| words!("serve-onion-launching"))?;
    aloud_in!("serve-onion-raising");

    // The address is deliberately not shown until here. Handed to a peer before
    // the network holds the descriptor, it produces a connection failure that
    // looks like a bug in one of the two clients and is not one.
    let waiting = dialer.timeout();
    tokio::time::timeout(waiting, host.wait_until_reachable())
        .await
        .map_err(|_| {
            anyhow::anyhow!(words!(
                "serve-onion-not-reachable",
                seconds = waiting.as_secs()
            ))
        })?
        .with_context(|| words!("serve-onion-waiting"))?;
    let address = PeerAddress::Onion {
        host: host.address()?,
        port: ONION_PORT,
    };
    aloud_in!("serve-onion-unseen", address = address.to_string());
    aloud_in!(
        "serve-onion-invite",
        invitation = Invite::to(address.clone()).to_string()
    );
    // Written after the network holds the descriptor, so nobody is ever sent to an
    // address that does not answer yet.
    let _ = found_address.send(Some(address));

    loop {
        let stream = host
            .accept()
            .await
            .with_context(|| words!("serve-onion-accepting"))?;
        // Through Tor there is no address to show, which is the point of it.
        spawn_exchange(stream, &node, &door, Caller::Unseen);
    }
}

/// Stands in for the onion listener when arti is not built in.
///
/// Refuse, rather than quietly listen on a socket the caller asked not to use.
#[cfg(not(feature = "tor"))]
pub(super) async fn answer(
    _dialer: Dialer,
    _node: Arc<Node>,
    _door: Door,
    _found_address: watch::Sender<Option<PeerAddress>>,
) -> anyhow::Result<()> {
    anyhow::bail!(words!("serve-onion-not-built"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let pairs = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            [
                (
                    words!("serve-onion-launching"),
                    "launching the onion service",
                ),
                (
                    words!("serve-onion-raising"),
                    "raising  the unseen address. this can take minutes.",
                ),
                (
                    words!("serve-onion-not-reachable", seconds = 180_u64),
                    "not reachable after 180 s",
                ),
                (
                    words!("serve-onion-waiting"),
                    "waiting for the service to be reachable",
                ),
                (
                    words!("serve-onion-unseen", address = "abc.onion:3333"),
                    "unseen   abc.onion:3333",
                ),
                (
                    words!("serve-onion-invite", invitation = "333:abc.onion:3333"),
                    "invite   333:abc.onion:3333",
                ),
                (words!("serve-onion-accepting"), "accepting a peer"),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

    #[test]
    fn a_client_without_tor_says_so_in_lines_a_terminal_can_hold() {
        // One line nine columns too wide under `failed` before; broken once now.
        let said = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            words!("serve-onion-not-built")
        });
        assert_eq!(
            said,
            "this client was built without Tor, so it cannot publish an onion\naddress"
        );
    }
}
