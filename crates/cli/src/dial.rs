//! Reaching another node, whichever way its address says to.
//!
//! THE ADDRESS DECIDES, not a setting: a plain host opens a socket, a `.onion`
//! address goes through Tor. That rule lives in `n333-net`; what lives here is the
//! one Tor client a process is allowed to have.
//!
//! WHY ONE CLIENT. Arti holds directory state and a pool of circuits, and starting a
//! second one in the same process pays the whole bootstrap again and fights over the
//! same state directory. So it is started at most once, on the first onion address
//! anybody dials or hosts, and never at all by a node that does neither. That is what
//! makes Tor a thing this client carries rather than a thing it costs.

use anyhow::Context as _;
use futures::{AsyncRead, AsyncWrite};
use n333_net::{PeerAddress, direct};

use crate::commands::Common;

/// Anything a node can be spoken to over.
///
/// The two transports have nothing in common but this, and the code above them is
/// written once because of it.
pub(crate) trait Wire: AsyncRead + AsyncWrite + Unpin + Send {}

impl<T: AsyncRead + AsyncWrite + Unpin + Send> Wire for T {}

/// An open connection to a peer, however it was reached.
pub(crate) type Stream = Box<dyn Wire>;

/// The one thing in a process that knows how to reach a peer.
#[derive(Clone)]
pub(crate) struct Dialer {
    /// Where this node's files are and how long it is willing to wait.
    common: Common,
    /// The Tor client, started at most once and only if something needs it.
    #[cfg(feature = "tor")]
    tor: std::sync::Arc<tokio::sync::OnceCell<n333_net::tor::Client>>,
}

impl Dialer {
    /// A dialler that has not started anything yet, and may never need to.
    pub(crate) fn new(common: Common) -> Self {
        Self {
            common,
            #[cfg(feature = "tor")]
            tor: std::sync::Arc::default(),
        }
    }

    /// Open a connection to `address`, giving up after the shared timeout.
    ///
    /// # Errors
    /// Fails if the address cannot be reached in time, or if it needs Tor and this
    /// build has none.
    pub(crate) async fn dial(&self, address: &PeerAddress) -> anyhow::Result<Stream> {
        if address.needs_tor() {
            return self.through_tor(address).await;
        }
        let stream = tokio::time::timeout(self.common.timeout, direct::connect(address))
            .await
            .with_context(|| self.gave_up_on(address))?
            .with_context(|| format!("connecting to {address}"))?;
        Ok(Box::new(stream))
    }

    /// How long this node is willing to wait on any one step that talks to a network.
    ///
    /// Shared so that hosting an onion address gives up on the same schedule as
    /// dialling one. Two different deadlines for the same Tor client would mean a node
    /// that has given up on being reachable while still waiting to reach others.
    pub(crate) const fn timeout(&self) -> std::time::Duration {
        self.common.timeout
    }

    /// The sentence both transports use when the peer never answers.
    fn gave_up_on(&self, address: &PeerAddress) -> String {
        format!(
            "no answer from {address} after {} s",
            self.common.timeout.as_secs()
        )
    }
}

/// What a build with arti in it can do.
#[cfg(feature = "tor")]
impl Dialer {
    /// The Tor client, started on the first call and shared from then on.
    ///
    /// # Errors
    /// Fails if arti cannot start, or does not bootstrap inside the timeout.
    pub(crate) async fn tor(&self) -> anyhow::Result<n333_net::tor::Client> {
        self.tor
            .get_or_try_init(|| crate::commands::bootstrap(&self.common))
            .await
            .cloned()
    }

    /// Reach an onion address, starting Tor if this is the first one.
    async fn through_tor(&self, address: &PeerAddress) -> anyhow::Result<Stream> {
        let client = self.tor().await?;
        let stream = tokio::time::timeout(
            self.common.timeout,
            n333_net::tor::connect(&client, address.host(), address.port()),
        )
        .await
        .with_context(|| self.gave_up_on(address))?
        .with_context(|| format!("connecting to {address}"))?;
        Ok(Box::new(stream))
    }
}

/// What a build without arti can do, which is say so.
#[cfg(not(feature = "tor"))]
impl Dialer {
    /// Refuse an onion address by name, rather than fail further down with a
    /// name-resolution error that reads like a broken network.
    async fn through_tor(&self, address: &PeerAddress) -> anyhow::Result<Stream> {
        anyhow::bail!("this client was built without Tor, so it cannot reach {address}")
    }
}
