//! `333 ping` — dial another node's onion address and exchange heartbeats.

use anyhow::Context as _;
use n333_net::initiate;
use n333_net::tor::{self, HEARTBEAT_PORT};

use crate::commands::{Common, bootstrap, describe};
use crate::identity_file;

/// Exchange one heartbeat with the node at `address`.
///
/// # Errors
/// Fails if the identity cannot be read, Tor cannot start, the peer cannot be
/// reached, or the answer does not check out.
pub(crate) async fn run(common: &Common, address: &str, port: u16) -> anyhow::Result<()> {
    let (identity, _origin) = identity_file::load_or_create(&common.paths.identity_file())?;
    println!("name     {}", identity.node_id());

    let client = bootstrap(common).await?;

    println!("dialing  {address}:{port}");
    let mut stream = tokio::time::timeout(common.timeout, tor::connect(&client, address, port))
        .await
        .with_context(|| format!("no answer from {address} after {} s", common.timeout.as_secs()))?
        .with_context(|| format!("connecting to {address}:{port}"))?;

    let exchange = tokio::time::timeout(common.timeout, initiate(&mut stream, &identity))
        .await
        .context("the peer opened a stream but did not complete the exchange")?
        .context("exchanging heartbeats")?;

    println!("{}", describe(&exchange));
    Ok(())
}

/// The port to use when the caller does not name one.
#[must_use]
pub(crate) const fn default_port() -> u16 {
    HEARTBEAT_PORT
}
