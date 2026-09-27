//! `333 ping` — reach another node and exchange one heartbeat.
//!
//! The address decides how. A plain host opens a socket and the exchange is done in
//! milliseconds; a `.onion` address starts Tor first, which costs seconds to minutes
//! before the first byte moves. Nothing else differs between the two, and neither
//! this file nor anything above it knows which one happened.

use std::time::Duration;

use anyhow::Context as _;
use n333_core::{Identity, NodeId};
use n333_net::{PeerAddress, initiate};

use crate::commands::{Common, describe};
use crate::dial::Dialer;
use crate::identity_file;
use crate::node::sources;

/// Exchange one heartbeat with the node at `address`.
///
/// # Errors
/// Fails if the identity cannot be read, the peer cannot be reached, or the answer
/// does not check out.
pub(crate) async fn run(common: &Common, address: &PeerAddress) -> anyhow::Result<()> {
    let (identity, origin) =
        identity_file::load_or_create(&common.mistrust(), common.paths.root())?;
    aloud!("name     {}", identity.node_id());
    crate::named::report(origin, common.paths.root());
    // Written down before the knock, and again with a name once somebody answers: it
    // is an address this node was given, and the vigil goes on knocking there.
    let home = common.paths.root();
    let typed = address.to_string();
    sources::typed(home, &typed, None).context("writing down the address")?;
    let answered = knock(
        &identity,
        &Dialer::new(common.clone()),
        common.timeout,
        address,
    )
    .await?;
    sources::typed(home, &typed, Some(answered)).context("writing down who answered")
}

/// Exchange one heartbeat, as a node that is already running.
///
/// Shared with the vigil, which has the key and a dialler of its own already and must
/// not start a second of either.
///
/// The name that answered, for the caller to write down where it keeps addresses.
///
/// # Errors
/// Fails if the peer cannot be reached or the answer does not check out.
pub(crate) async fn knock(
    identity: &Identity,
    dialer: &Dialer,
    timeout: Duration,
    address: &PeerAddress,
) -> anyhow::Result<NodeId> {
    aloud!("knocking {address}");

    let mut stream = dialer.dial(address).await?;
    let exchange = tokio::time::timeout(timeout, initiate(&mut stream, identity))
        .await
        .context("the peer accepted the connection but did not finish the exchange")?
        .context("exchanging heartbeats")?;

    aloud!("{}", describe(&exchange));
    Ok(exchange.peer.node_id)
}
