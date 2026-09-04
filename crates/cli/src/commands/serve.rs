//! `333 serve` — publish an onion address and answer heartbeats on it.

use std::sync::Arc;

use anyhow::Context as _;
use n333_net::tor::host::OnionHost;
use n333_net::tor::{HEARTBEAT_PORT, SERVICE_NICKNAME};
use n333_net::{respond, session};

use crate::commands::{Common, bootstrap, describe};
use crate::identity_file;

/// Run until interrupted, answering every heartbeat that arrives.
///
/// # Errors
/// Fails if the identity cannot be read, Tor cannot start, or the service stops
/// accepting requests.
pub(crate) async fn run(common: &Common) -> anyhow::Result<()> {
    let (identity, _origin) = identity_file::load_or_create(&common.paths.identity_file())?;
    let identity = Arc::new(identity);
    println!("name     {}", identity.node_id());

    let client = bootstrap(common).await?;
    let mut host = OnionHost::launch(&client, SERVICE_NICKNAME, HEARTBEAT_PORT)
        .context("launching the onion service")?;
    println!("address  {}:{HEARTBEAT_PORT}", host.address()?);

    println!("publishing the address to the network...");
    tokio::time::timeout(common.timeout, host.wait_until_reachable())
        .await
        .with_context(|| format!("not reachable after {} s", common.timeout.as_secs()))?
        .context("waiting for the service to be reachable")?;
    println!("reachable. waiting for peers.");

    loop {
        let mut stream = host.accept().await.context("accepting a peer")?;
        let identity = Arc::clone(&identity);
        // One slow or hostile peer must not hold up the next one, so each exchange
        // runs on its own task and its failure is reported rather than propagated.
        tokio::spawn(async move {
            match respond(&mut stream, &identity).await {
                Ok(exchange) => println!("{}", describe(&exchange)),
                Err(e) => report(&e),
            }
        });
    }
}

/// A failed exchange is the peer's problem, not this node's, so it is printed and
/// forgotten. Distinguishing the kinds matters: a stream that died mid-frame is a
/// bad circuit, while a bad signature is someone doing it on purpose.
fn report(error: &session::Error) {
    match error {
        session::Error::Frame(e) => println!("peer -        stream failed: {e}"),
        other => println!("peer -        refused: {other}"),
    }
}
