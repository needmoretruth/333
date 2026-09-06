//! Being a Tor client: where the node keeps its Tor state, how it bootstraps, and
//! how it reaches another node's onion address.
//!
//! Hosting is the other half and lives in [`host`].
//!
//! WHY THE PATHS ARE EXPLICIT. Arti's defaults put state and cache under the user's
//! home directory, shared by every arti-based program on the machine. Two nodes
//! started with defaults fight over the same lock files, and the failure is quiet
//! rather than loud. Every node here names its own directories.

pub mod host;

use std::path::PathBuf;
use std::sync::Arc;

use arti_client::config::CfgPath;
use arti_client::{DataStream, TorClient, TorClientConfig};
use tor_rtcompat::PreferredRuntime;

/// The nickname arti files this node's onion-service keys under.
///
/// FROZEN in effect: the nickname is a path component in the keystore, so changing
/// it makes arti look in a different place, find no key, generate a new one, and
/// give the node a different onion address.
pub const SERVICE_NICKNAME: &str = "n333";

/// A running Tor client, shared between the hosting and dialling halves.
pub type Client = Arc<TorClient<PreferredRuntime>>;

/// Where this node keeps the state that has to survive a restart.
#[derive(Debug, Clone)]
pub struct Paths {
    /// Arti's state directory. The onion-service identity key lives under it, so
    /// losing this directory means losing the node's onion address for good.
    pub state_dir: PathBuf,
    /// Arti's directory cache. Losing it costs one slow bootstrap, nothing more.
    pub cache_dir: PathBuf,
}

/// Things that can go wrong being a Tor client.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Arti refused an operation.
    ///
    /// Boxed because arti's error type is 168 bytes; unboxed it would make every
    /// `Result` in this module that large, on the success path too.
    #[error("tor: {0}")]
    Tor(#[from] Box<arti_client::Error>),
    /// The configuration this node assembled is not valid.
    #[error("tor configuration: {0}")]
    Config(#[from] arti_client::config::ConfigBuildError),
    /// The peer's address is not one arti will dial.
    #[error("address: {0}")]
    Address(#[from] arti_client::TorAddrError),
}

/// Start a Tor client and wait until it has bootstrapped.
///
/// Bootstrapping is minutes on a cold cache and seconds on a warm one. Arti retries
/// on its own — 128 times by default — so a caller that needs to give up must impose
/// its own deadline rather than wait for this to return an error.
///
/// `trust_directory_permissions` relaxes arti's refusal to start when the state or
/// cache directory is group- or world-accessible. It exists for scratch directories
/// and containers with odd ownership; a real node should leave it off and fix the
/// permissions instead.
///
/// # Errors
/// Fails if the configuration is invalid or arti cannot start.
///
/// # Panics
/// Arti panics rather than returning an error when there is no async runtime in
/// context. Call this from inside a Tokio runtime.
pub async fn bootstrap(paths: &Paths, trust_directory_permissions: bool) -> Result<Client, Error> {
    let mut builder = TorClientConfig::builder();
    builder
        .storage()
        .state_dir(CfgPath::new_literal(paths.state_dir.clone()))
        .cache_dir(CfgPath::new_literal(paths.cache_dir.clone()));
    if trust_directory_permissions {
        builder.storage().permissions().dangerously_trust_everyone();
    }
    let config = builder.build()?;
    Ok(TorClient::create_bootstrapped(config)
        .await
        .map_err(Box::new)?)
}

/// Open a stream to another node's onion address.
///
/// The connection is deliberately made with the client's default preferences. Every
/// form of per-stream isolation arti offers becomes part of the key under which it
/// caches rendezvous circuits, so isolating streams would mean building a fresh
/// circuit — seconds of it — for every peer contact.
///
/// # Errors
/// Fails if the address is malformed, the peer never published a descriptor, or the
/// circuit cannot be built.
pub async fn connect(client: &Client, onion_address: &str, port: u16) -> Result<DataStream, Error> {
    Ok(client
        .connect((onion_address, port))
        .await
        .map_err(Box::new)?)
}
