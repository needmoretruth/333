//! `333 ping` — reach another node and exchange one heartbeat.
//!
//! The address decides how. A plain host opens a socket and the exchange is done in
//! milliseconds; a `.onion` address starts Tor first, which costs seconds to minutes
//! before the first byte moves. Nothing else differs between the two, and neither
//! this file nor anything above it knows which one happened.

use anyhow::Context as _;
use n333_net::{PeerAddress, initiate};

use crate::commands::{Common, describe};
use crate::dial::Dialer;
use crate::identity_file;

/// Exchange one heartbeat with the node at `address`.
///
/// # Errors
/// Fails if the identity cannot be read, the peer cannot be reached, or the answer
/// does not check out.
pub(crate) async fn run(common: &Common, address: &PeerAddress) -> anyhow::Result<()> {
    let (identity, _origin) =
        identity_file::load_or_create(&common.mistrust(), common.paths.root())?;
    aloud!("name     {}", identity.node_id());
    aloud!("knocking {address}");

    let mut stream = Dialer::new(common.clone()).dial(address).await?;
    let exchange = tokio::time::timeout(common.timeout, initiate(&mut stream, &identity))
        .await
        .context("the peer accepted the connection but did not finish the exchange")?
        .context("exchanging heartbeats")?;

    aloud!("{}", describe(&exchange));
    Ok(())
}
