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
        .context("launching the onion service")?;
    aloud!("raising  the unseen address. this can take minutes.");

    // The address is deliberately not shown until here. Handed to a peer before
    // the network holds the descriptor, it produces a connection failure that
    // looks like a bug in one of the two clients and is not one.
    let waiting = dialer.timeout();
    tokio::time::timeout(waiting, host.wait_until_reachable())
        .await
        .with_context(|| format!("not reachable after {} s", waiting.as_secs()))?
        .context("waiting for the service to be reachable")?;
    let address = PeerAddress::Onion {
        host: host.address()?,
        port: ONION_PORT,
    };
    aloud!("unseen   {address}");
    aloud!("invite   {}", Invite::to(address.clone()));
    // Written after the network holds the descriptor, so nobody is ever sent to an
    // address that does not answer yet.
    let _ = found_address.send(Some(address));

    loop {
        let stream = host.accept().await.context("accepting a peer")?;
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
    anyhow::bail!("this client was built without Tor, so it cannot publish an onion address")
}
