//! `333-site serve`: the meeting point and the pages, on one loopback port behind Caddy.
//!
//! NO ACCESS LOG. Nothing here writes a line per request, and the only use of a
//! visitor's address is the per-address minute in [`crate::gate`], which keeps an HMAC
//! of it in memory and never the address.

mod board_view;
mod counted;
mod head;
mod machine;
mod map_view;
mod meeting;
mod network_view;
mod pages;
mod respond;
mod routes;
mod sitemap;
mod state;
mod statics;
mod status_view;
mod template;
mod values;

use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::{TokioIo, TokioTimer};

use state::State;

/// How long a connection may take to send its headers. Caddy sends them at once; this is
/// for anything else that reaches the port.
const HEADERS_WITHIN: Duration = Duration::from_secs(10);

/// Where the server listens unless told otherwise. The unit does not say, so this is
/// also where `deploy` asks a new release whether it serves.
pub(crate) const LISTEN: &str = "127.0.0.1:8080";

/// The messages the server says itself, for the check that every English message is
/// used. The scripts say most of these too; a key here that nothing says is a key the
/// check cannot catch, so this list stays the server's and no longer.
#[cfg(test)]
pub(crate) const SAID_BY_SERVER: &[&str] = &[
    "language-name",
    "languages",
    "js-state-awake",
    "js-state-not-running",
    "js-line-epoch",
    "js-copy",
    "js-board-said",
    "js-board-site",
    "js-board-tor",
    "js-map-read-at",
    "js-map-tor",
    "js-map-nowhere",
    "js-map-nobody",
    "js-map-all",
    "js-map-unsaid",
    "js-map-dot",
    "js-network-awake",
    "js-network-not-running",
    "js-network-empty",
    "js-network-epoch",
    "js-network-reach-direct",
    "js-network-reach-tor-short",
    "js-network-tag-founder",
    "js-network-tag-site",
    "js-network-this-node",
    "js-network-yes",
    "js-network-no",
    "js-network-none",
    "status-chart-summary",
    "status-chart-too-few",
    "status-age",
    "status-observed-running",
    "status-observed-not-running",
    "status-uptime",
    "status-uptime-value",
];

/// What `serve` is told.
#[derive(clap::Args)]
pub(crate) struct Args {
    /// The directory of pages and assets.
    #[arg(long)]
    site: PathBuf,
    /// The directory the board is kept in. Made, private, if it is not there.
    #[arg(long)]
    state: PathBuf,
    /// The JSON file `333-site observe` writes.
    #[arg(long)]
    observation: PathBuf,
    /// Where to listen. Loopback: only Caddy should reach this.
    #[arg(long, default_value = LISTEN)]
    listen: SocketAddr,
    /// The version pages show and name their assets by.
    #[arg(long, conflicts_with = "version_file")]
    version: Option<String>,
    /// A file holding the version, as `deploy` writes it into each release (`VERSION`).
    #[arg(long)]
    version_file: Option<PathBuf>,
}

/// Serve until the process is stopped.
///
/// Stopping it at any moment is safe: the board is written by rename, so the file on
/// disk is always one whole board.
///
/// # Errors
/// Fails if the site directory is missing, the state directory cannot be made, the
/// version file cannot be read, or the address cannot be listened on.
pub(crate) fn run(args: &Args) -> anyhow::Result<()> {
    let version = match (&args.version, &args.version_file) {
        (Some(version), _) => version.trim().to_owned(),
        (None, Some(file)) => std::fs::read_to_string(file)
            .with_context(|| format!("reading the version from {}", file.display()))?
            .trim()
            .to_owned(),
        (None, None) => "dev".to_owned(),
    };
    let release = args
        .version_file
        .as_deref()
        .map(machine::Release::beside)
        .unwrap_or_default();
    let state = Arc::new(
        State::open(&args.site, &args.state, args.observation.clone(), version)?
            .with_release(release),
    );
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("starting the runtime")?
        .block_on(listen(state, args.listen))
}

/// Take connections for ever.
async fn listen(state: Arc<State>, address: SocketAddr) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .with_context(|| format!("listening on {address}"))?;
    tracing::info!(
        "serving {} on {address}, version {}",
        state.site.display(),
        state.version
    );
    loop {
        let (stream, _) = match listener.accept().await {
            Ok(accepted) => accepted,
            Err(error) => {
                // Out of file descriptors, most likely. Waiting a moment beats spinning.
                tracing::warn!("a connection could not be taken: {error}");
                tokio::time::sleep(Duration::from_millis(100)).await;
                continue;
            }
        };
        let state = Arc::clone(&state);
        tokio::spawn(async move {
            let service = service_fn(move |request| {
                let state = Arc::clone(&state);
                async move { Ok::<_, Infallible>(routes::handle(state, request).await) }
            });
            // A connection that breaks off is the visitor's business, not the log's.
            let _ = http1::Builder::new()
                .timer(TokioTimer::new())
                .header_read_timeout(HEADERS_WITHIN)
                .serve_connection(TokioIo::new(stream), service)
                .await;
        });
    }
}
