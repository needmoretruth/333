//! `333 serve` — answer heartbeats until interrupted.
//!
//! By default this opens a socket and nothing else: no Tor, no bootstrap, no wait.
//! `--tor` additionally publishes an onion address, for a node whose own address
//! should not be visible to the peers that reach it.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context as _, bail};
use futures::{AsyncRead, AsyncWrite};
use n333_core::Identity;
use n333_net::{direct, respond, session};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

use crate::commands::{Common, describe};
use crate::identity_file;

/// How long one exchange may take before this node stops waiting on it.
///
/// The connection is already open by the time an exchange starts and a few hundred
/// bytes travel each way, so seconds is the honest scale even through Tor. A minute
/// is generous, and it is what stops a peer that connects and then says nothing for
/// ever.
const EXCHANGE_TIMEOUT: Duration = Duration::from_secs(60);

/// How many exchanges may be in flight at once.
///
/// A peer that opens connections and stalls on each costs this node a task and a
/// buffer per connection with nothing to shed them. The cap has to live here, because
/// nothing below it knows what an exchange is worth. Refusing is deliberate and
/// visible: a peer over the cap is told nothing and the operator sees a line.
const MAX_CONCURRENT_EXCHANGES: usize = 64;

/// Run until interrupted, answering every heartbeat that arrives.
///
/// `bind` is the socket to listen on, or `None` to listen only through Tor. `tor`
/// publishes an onion address as well.
///
/// # Errors
/// Fails if the identity cannot be read, if neither way of listening was asked for,
/// if a socket cannot be bound, or if Tor was asked for and cannot start.
pub(crate) async fn run(
    common: &Common,
    bind: Option<SocketAddr>,
    tor: bool,
) -> anyhow::Result<()> {
    if bind.is_none() && !tor {
        bail!("nothing would be listening: --no-direct needs --tor");
    }
    let (identity, _origin) =
        identity_file::load_or_create(&common.mistrust(), common.paths.root())?;
    let identity = Arc::new(identity);
    println!("name     {}", identity.node_id());

    let gate = Arc::new(Semaphore::new(MAX_CONCURRENT_EXCHANGES));
    let mut listening = tokio::task::JoinSet::new();

    if let Some(bind) = bind {
        let listener = direct::Listener::bind(bind)
            .await
            .with_context(|| format!("listening on {bind}"))?;
        // True the instant the socket is bound, which is why it is printed here.
        println!("answer   {}", listener.address()?);
        let (identity, gate) = (Arc::clone(&identity), Arc::clone(&gate));
        listening.spawn(async move { answer_direct(listener, identity, gate).await });
    }

    if tor {
        let (identity, gate) = (Arc::clone(&identity), Arc::clone(&gate));
        listening.spawn(onion::answer(common.clone(), identity, gate));
    }

    // No line here saying the vigil has begun: with --no-direct it would not be true
    // yet. Each listener announces itself at the moment it can actually answer.
    while let Some(finished) = listening.join_next().await {
        finished.context("a listener stopped unexpectedly")??;
    }
    Ok(())
}

/// Answer every peer that opens a socket to this node.
async fn answer_direct(
    listener: direct::Listener,
    identity: Arc<Identity>,
    gate: Arc<Semaphore>,
) -> anyhow::Result<()> {
    loop {
        let (stream, from) = listener.accept().await.context("accepting a peer")?;
        // A peer's address is not a name and is not recorded; it is shown so that the
        // operator of this node can see who is reaching it right now.
        spawn_exchange(stream, &identity, &gate, &from.to_string());
    }
}

/// Give one peer its own task, its own deadline and one of the permits.
fn spawn_exchange<S>(mut stream: S, identity: &Arc<Identity>, gate: &Arc<Semaphore>, from: &str)
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let Ok(permit) = Arc::clone(gate).try_acquire_owned() else {
        println!("turned away {from}: {MAX_CONCURRENT_EXCHANGES} are already being answered");
        return;
    };
    let identity = Arc::clone(identity);
    // One slow or hostile peer must not hold up the next one, so each exchange runs on
    // its own task, under a deadline, and its failure is reported rather than
    // propagated. The permit is released when the task ends, whichever way.
    tokio::spawn(async move {
        let _permit: OwnedSemaphorePermit = permit;
        match tokio::time::timeout(EXCHANGE_TIMEOUT, respond(&mut stream, &identity)).await {
            Ok(Ok(exchange)) => println!("{}", describe(&exchange)),
            Ok(Err(e)) => report(&e),
            Err(_elapsed) => println!(
                "silence  {} s, so we let go",
                EXCHANGE_TIMEOUT.as_secs()
            ),
        }
    });
}

/// A failed exchange is the peer's problem, not this node's, so it is printed and
/// forgotten. Distinguishing the kinds matters: a stream that died mid-frame is a bad
/// connection, while a bad signature is someone doing it on purpose.
fn report(error: &session::Error) {
    match error {
        session::Error::Frame(e) => println!("broken   the connection failed mid-message: {e}"),
        other => println!("refused  {other}"),
    }
}

/// The other way to be reachable: an onion address, for a node that is hiding.
#[cfg(feature = "tor")]
mod onion {
    use std::sync::Arc;

    use anyhow::Context as _;
    use n333_core::Identity;
    use n333_net::peer::ONION_PORT;
    use n333_net::tor::SERVICE_NICKNAME;
    use n333_net::tor::host::OnionHost;
    use tokio::sync::Semaphore;

    use crate::commands::{Common, bootstrap};

    /// Publish an onion address and answer every peer that arrives on it.
    pub(super) async fn answer(
        common: Common,
        identity: Arc<Identity>,
        gate: Arc<Semaphore>,
    ) -> anyhow::Result<()> {
        let client = bootstrap(&common).await?;
        let mut host = OnionHost::launch(&client, SERVICE_NICKNAME, ONION_PORT)
            .context("launching the onion service")?;
        println!("raising  the unseen address. this can take minutes.");

        // The address is deliberately not shown until here. Handed to a peer before
        // the network holds the descriptor, it produces a connection failure that
        // looks like a bug in one of the two clients and is not one.
        tokio::time::timeout(common.timeout, host.wait_until_reachable())
            .await
            .with_context(|| format!("not reachable after {} s", common.timeout.as_secs()))?
            .context("waiting for the service to be reachable")?;
        println!("unseen   {}:{ONION_PORT}", host.address()?);

        loop {
            let stream = host.accept().await.context("accepting a peer")?;
            // Through Tor there is no address to show, which is the point of it.
            super::spawn_exchange(stream, &identity, &gate, "over tor");
        }
    }
}

/// Stands in for the onion listener when arti is not built in.
#[cfg(not(feature = "tor"))]
mod onion {
    use std::sync::Arc;

    use n333_core::Identity;
    use tokio::sync::Semaphore;

    use crate::commands::Common;

    /// Refuse, rather than quietly listen on a socket the caller asked not to use.
    pub(super) async fn answer(
        _common: Common,
        _identity: Arc<Identity>,
        _gate: Arc<Semaphore>,
    ) -> anyhow::Result<()> {
        anyhow::bail!("this client was built without Tor, so it cannot publish an onion address")
    }
}
