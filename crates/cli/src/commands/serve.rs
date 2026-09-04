//! `333 serve` — publish an onion address and answer heartbeats on it.

use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use n333_net::tor::host::OnionHost;
use n333_net::tor::{HEARTBEAT_PORT, SERVICE_NICKNAME};
use n333_net::{respond, session};

use crate::commands::{Common, bootstrap, describe};
use crate::identity_file;

/// How long one exchange may take before this node stops waiting on it.
///
/// The circuit is already open by the time an exchange starts and 139 bytes travel
/// each way, so seconds is the honest scale. A minute is generous, and it is what
/// stops a peer that opens a stream and then says nothing for ever.
const EXCHANGE_TIMEOUT: Duration = Duration::from_secs(60);

/// How many exchanges may be in flight at once.
///
/// Arti will carry 65,535 streams on one circuit, so a peer that opens streams and
/// stalls on each costs this node a task and a buffer per stream with nothing to shed
/// them. The cap has to live here, because nothing below it knows what an exchange
/// is worth. Refusing is deliberate and visible: a peer over the cap is told nothing
/// and the operator sees a line.
const MAX_CONCURRENT_EXCHANGES: usize = 64;

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

    let in_flight = Arc::new(tokio::sync::Semaphore::new(MAX_CONCURRENT_EXCHANGES));
    loop {
        let mut stream = host.accept().await.context("accepting a peer")?;
        let Ok(permit) = Arc::clone(&in_flight).try_acquire_owned() else {
            println!("peer -        refused: {MAX_CONCURRENT_EXCHANGES} exchanges already open");
            drop(stream);
            continue;
        };
        let identity = Arc::clone(&identity);
        // One slow or hostile peer must not hold up the next one, so each exchange
        // runs on its own task, under a deadline, and its failure is reported rather
        // than propagated. The permit is released when the task ends, whichever way.
        tokio::spawn(async move {
            let _permit = permit;
            match tokio::time::timeout(EXCHANGE_TIMEOUT, respond(&mut stream, &identity)).await {
                Ok(Ok(exchange)) => println!("{}", describe(&exchange)),
                Ok(Err(e)) => report(&e),
                Err(_elapsed) => println!(
                    "peer -        gave up after {} s",
                    EXCHANGE_TIMEOUT.as_secs()
                ),
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
