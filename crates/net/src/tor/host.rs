//! Being a Tor onion service: publishing an address and accepting streams on it.
//!
//! WHY THE ADDRESS IS STABLE. The identity key behind an onion address is generated
//! on first launch and stored under arti's state directory, keyed by the service
//! nickname. Same directory and same nickname means the same address after a restart;
//! a temporary directory means a new address every run and a node nobody can find twice.
//!
//! WHY WRONG PORTS ARE REJECTED THE WAY THEY ARE. Answering a port this service does
//! not offer — or refusing one differently from the way C tor refuses it — makes the
//! service distinguishable from every other onion service on the network. The refusal
//! below is the ordinary one, on purpose.

use std::pin::Pin;
use std::sync::Arc;

use arti_client::DataStream;
use futures::{Stream, StreamExt as _};
use safelog::DisplayRedacted as _;
use tor_cell::relaycell::msg::{Connected, End, EndReason};
use tor_hsservice::{HsNickname, RunningOnionService, StreamRequest, handle_rend_requests};
use tor_proto::stream::IncomingStreamRequest;

use super::{Client, Error as TorError};

/// Things that can go wrong hosting a service.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Being a Tor client failed.
    #[error(transparent)]
    Tor(#[from] TorError),
    /// Arti refused an operation. Boxed for the same reason as in the parent module.
    #[error("tor: {0}")]
    Arti(#[from] Box<arti_client::Error>),
    /// A peer's connection attempt could not be completed. This is the peer's
    /// circuit failing, not this node's service failing, so the service goes on.
    #[error("incoming stream: {0}")]
    IncomingStream(#[from] Box<tor_hsservice::ClientError>),
    /// The service configuration is not valid.
    #[error("service configuration: {0}")]
    Config(#[from] arti_client::config::ConfigBuildError),
    /// The nickname is not one arti accepts: lowercase letters, digits, `_` and `-`.
    #[error("service nickname: {0}")]
    Nickname(#[from] tor_hsservice::InvalidNickname),
    /// The service was configured off. Nothing here ever does that, so reaching this
    /// means the configuration was assembled by something that did.
    #[error("the onion service is disabled in its own configuration")]
    Disabled,
    /// The service launched but has no identity key to name itself with.
    #[error("the onion service has no address")]
    NoAddress,
    /// The stream of incoming requests ended, which arti only does at shutdown.
    #[error("the onion service stopped accepting requests")]
    Stopped,
}

/// A launched onion service, and the requests arriving on it.
pub struct OnionHost {
    service: Arc<RunningOnionService>,
    requests: Pin<Box<dyn Stream<Item = StreamRequest> + Send>>,
    port: u16,
}

impl OnionHost {
    /// Launch the service. The address is available as soon as this returns.
    ///
    /// # Errors
    /// Fails if the nickname is invalid, the configuration is rejected, or arti
    /// cannot launch the service.
    pub fn launch(client: &Client, nickname: &str, port: u16) -> Result<Self, Error> {
        let config = tor_hsservice::config::OnionServiceConfigBuilder::default()
            .nickname(HsNickname::new(nickname.to_owned())?)
            .build()?;
        let (service, rendezvous) = client.launch_onion_service(config).map_err(Box::new)?.ok_or(Error::Disabled)?;
        Ok(Self {
            service,
            // The stream arti returns is not Unpin, and `handle_rend_requests` wants
            // to own it. Boxing once here is cheaper than pinning at every use.
            requests: Box::pin(handle_rend_requests(Box::pin(rendezvous))),
            port,
        })
    }

    /// This service's onion address.
    ///
    /// # Errors
    /// Fails if the identity key is missing, which should not happen after a
    /// successful launch.
    pub fn address(&self) -> Result<String, Error> {
        self.service
            .onion_address()
            // The address is not a secret — it is the thing peers must be told — so
            // the redacted rendering that arti defaults to is not what we want here.
            .map(|id| id.display_unredacted().to_string())
            .ok_or(Error::NoAddress)
    }

    /// Wait until the network is believed to have this service's descriptor.
    ///
    /// Handing a peer the address before this returns produces a connection failure
    /// that looks like a bug and is not one. Arti is explicit that reachability is an
    /// implication in one direction only: `false` does not prove unreachable.
    ///
    /// Returns a future that borrows nothing — `use<>` captures no lifetime — rather
    /// than being an `async fn` taking `&self`. The stream of incoming requests is
    /// `Send` but not `Sync`, so a future holding a shared reference to this whole
    /// object could not be moved to another task, and waiting is exactly the thing a
    /// caller wants to do on a task of its own.
    ///
    /// # Errors
    /// Fails if the service stops before it becomes reachable.
    pub fn wait_until_reachable(&self) -> impl Future<Output = Result<(), Error>> + Send + use<> {
        let service = Arc::clone(&self.service);
        async move {
            if service.status().state().is_fully_reachable() {
                return Ok(());
            }
            let mut events = service.status_events();
            while let Some(status) = events.next().await {
                if status.state().is_fully_reachable() {
                    return Ok(());
                }
            }
            Err(Error::Stopped)
        }
    }

    /// Accept the next stream addressed to this service's port.
    ///
    /// Requests for any other port, and requests that are not connection attempts,
    /// are refused here and never reach the caller.
    ///
    /// # Errors
    /// Fails if arti stops accepting requests.
    pub async fn accept(&mut self) -> Result<DataStream, Error> {
        loop {
            let request = self.requests.next().await.ok_or(Error::Stopped)?;
            let wanted = matches!(
                request.request(),
                IncomingStreamRequest::Begin(begin) if begin.port() == self.port
            );
            if !wanted {
                // A failed rejection means the circuit is already gone; the next
                // request is still worth waiting for.
                let _ = request.reject(End::new_with_reason(EndReason::DONE)).await;
                continue;
            }
            return Ok(request.accept(Connected::new_empty()).await.map_err(Box::new)?);
        }
    }
}
