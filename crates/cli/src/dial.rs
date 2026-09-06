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

/// Which roads a node is willing to travel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Roads {
    /// Whatever the address says. The ordinary case.
    Whichever,
    /// Only the unseen ones.
    ///
    /// For a node that is hiding. It is not enough to refuse to *answer* on a socket:
    /// a node that keeps its address off the wire and then opens a clearnet connection
    /// to every peer it knows, once an epoch, has put its address on the wire itself —
    /// at the far end, where it can be written down. A hiding node reaches only peers
    /// that are also hiding, and does not reach the rest at all.
    OnlyUnseen,
}

/// The one thing in a process that knows how to reach a peer.
#[derive(Clone)]
pub(crate) struct Dialer {
    /// Where this node's files are and how long it is willing to wait.
    common: Common,
    /// Which roads this node will travel.
    roads: Roads,
    /// The Tor client, started at most once and only if something needs it.
    #[cfg(feature = "tor")]
    tor: std::sync::Arc<tokio::sync::OnceCell<n333_net::tor::Client>>,
}

impl Dialer {
    /// A dialler that has not started anything yet, and may never need to.
    pub(crate) fn new(common: Common) -> Self {
        Self::travelling(common, Roads::Whichever)
    }

    /// A dialler restricted to the roads a node is willing to be seen on.
    pub(crate) fn travelling(common: Common, roads: Roads) -> Self {
        Self {
            common,
            roads,
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
        if self.roads == Roads::OnlyUnseen {
            anyhow::bail!(
                "this node keeps its address unseen, so it will not open a connection \
                 to {address}, which would show it"
            );
        }
        // Neither of these names the address. Every caller that prints one of these
        // has already said which peer it was reaching, and a line that says the
        // address three times is a line nobody reads to the end.
        let stream = tokio::time::timeout(self.common.timeout, direct::connect(address))
            .await
            .map_err(|_| anyhow::Error::msg(self.gave_up_on()))??;
        Ok(Box::new(stream))
    }

    /// The sentence both transports use when the peer never answers.
    fn gave_up_on(&self) -> String {
        format!("no answer after {} s", self.common.timeout.as_secs())
    }
}

/// What a build with arti in it can do.
#[cfg(feature = "tor")]
use anyhow::Context as _;

/// What a build with arti in it can do.
#[cfg(feature = "tor")]
impl Dialer {
    /// How long this node is willing to wait on any one step that talks to Tor.
    ///
    /// Shared so that hosting an onion address gives up on the same schedule as
    /// dialling one. Two different deadlines for the same Tor client would mean a node
    /// that has given up on being reachable while still waiting to reach others.
    pub(crate) const fn timeout(&self) -> std::time::Duration {
        self.common.timeout
    }

    /// Start Tor now, if any of these addresses will need it and it is not up yet.
    ///
    /// WHY THIS EXISTS. A round with one peer is bounded by the response window,
    /// because an answer that arrives later would not count anyway. A Tor bootstrap
    /// takes seconds to minutes and is allowed longer than that on purpose. Put the
    /// first onion dial inside a round and the bootstrap is cut off by the shorter of
    /// the two deadlines, and what this node then says is that the peer did not
    /// answer — which is a sentence about somebody else's node describing a state of
    /// this one. Waking here, before any round starts, spends the time where it is
    /// actually being spent and lets a failure to wake say so.
    pub(crate) async fn wake_for(&self, addresses: &[String]) {
        if self.tor.get().is_some() {
            return;
        }
        let any_unseen = addresses
            .iter()
            .filter_map(|address| address.parse::<PeerAddress>().ok())
            .any(|address| address.needs_tor());
        if !any_unseen {
            return;
        }
        aloud!(
            "waking   somebody worth reaching is at an unseen address and Tor is not up.\n\
             \x20        The first bootstrap takes seconds to minutes, and nothing is\n\
             \x20        asked of anybody until it is over."
        );
        if let Err(e) = self.tor().await {
            aloud!(
                "unwoken  Tor did not start: {e:#}\n\
                 \x20        Unseen addresses are skipped this epoch. The nodes behind them\n\
                 \x20        have not failed to answer — nothing reached them to ask."
            );
        }
    }

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
        .with_context(|| self.gave_up_on())?
        .with_context(|| format!("connecting to {address}"))?;
        Ok(Box::new(stream))
    }
}

/// What a build without arti can do, which is say so.
#[cfg(not(feature = "tor"))]
impl Dialer {
    /// Nothing to wake. An onion address is refused by name when it is dialled.
    pub(crate) async fn wake_for(&self, _addresses: &[String]) {}

    /// Refuse an onion address by name, rather than fail further down with a
    /// name-resolution error that reads like a broken network.
    async fn through_tor(&self, address: &PeerAddress) -> anyhow::Result<Stream> {
        anyhow::bail!("this client was built without Tor, so it cannot reach {address}")
    }
}
