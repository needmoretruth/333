//! `333 ping` — reach another node and exchange one heartbeat.
//!
//! The address decides how. A plain host opens a socket and the exchange is done in
//! milliseconds; a `.onion` address starts Tor first, which costs seconds to minutes
//! before the first byte moves. Nothing else differs between the two.

use anyhow::Context as _;
use futures::{AsyncRead, AsyncWrite};
use n333_core::Identity;
use n333_net::{Exchange, PeerAddress, direct, initiate};

use crate::commands::{Common, describe};
use crate::identity_file;

/// Exchange one heartbeat with the node at `address`.
///
/// # Errors
/// Fails if the identity cannot be read, the peer cannot be reached, or the answer
/// does not check out.
pub(crate) async fn run(common: &Common, address: &PeerAddress) -> anyhow::Result<()> {
    let (identity, _origin) =
        identity_file::load_or_create(&common.mistrust(), common.paths.root())?;
    println!("name     {}", identity.node_id());
    println!("dialing  {address}");

    let exchange = if address.needs_tor() {
        onion::exchange(common, address, &identity).await?
    } else {
        let stream = tokio::time::timeout(common.timeout, direct::connect(address))
            .await
            .with_context(|| timed_out(address, common))?
            .with_context(|| format!("connecting to {address}"))?;
        speak(common, stream, &identity).await?
    };

    println!("{}", describe(&exchange));
    Ok(())
}

/// Run the exchange over an open stream, under the shared deadline.
async fn speak<S>(common: &Common, mut stream: S, identity: &Identity) -> anyhow::Result<Exchange>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    tokio::time::timeout(common.timeout, initiate(&mut stream, identity))
        .await
        .context("the peer accepted the connection but did not finish the exchange")?
        .context("exchanging heartbeats")
}

/// The one sentence both transports use when the peer never answers.
fn timed_out(address: &PeerAddress, common: &Common) -> String {
    format!(
        "no answer from {address} after {} s",
        common.timeout.as_secs()
    )
}

/// Reaching a peer that is hiding.
#[cfg(feature = "tor")]
mod onion {
    use anyhow::Context as _;
    use n333_core::Identity;
    use n333_net::{Exchange, PeerAddress, tor};

    use crate::commands::{Common, bootstrap};

    /// Start Tor, open a stream to the onion address, and exchange one heartbeat.
    pub(super) async fn exchange(
        common: &Common,
        address: &PeerAddress,
        identity: &Identity,
    ) -> anyhow::Result<Exchange> {
        let client = bootstrap(common).await?;
        let stream = tokio::time::timeout(
            common.timeout,
            tor::connect(&client, address.host(), address.port()),
        )
        .await
        .with_context(|| super::timed_out(address, common))?
        .with_context(|| format!("connecting to {address}"))?;
        super::speak(common, stream, identity).await
    }
}

/// Stands in for the Tor dialler when arti is not built in.
#[cfg(not(feature = "tor"))]
mod onion {
    use n333_core::Identity;
    use n333_net::{Exchange, PeerAddress};

    use crate::commands::Common;

    /// Say why, rather than fail further down with a name-resolution error.
    pub(super) async fn exchange(
        _common: &Common,
        address: &PeerAddress,
        _identity: &Identity,
    ) -> anyhow::Result<Exchange> {
        anyhow::bail!("this client was built without Tor, so it cannot reach {address}")
    }
}
