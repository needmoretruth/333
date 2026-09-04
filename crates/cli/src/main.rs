//! The 333 command line client.
//!
//! Three things it can do at this version: show this node's name, listen for
//! heartbeats, and reach another node to exchange one.
//!
//! Reaching a peer is direct by default. Tor is carried for the nodes that need
//! their own address unseen, and starting it is the slowest thing this program can
//! be asked to do, so it happens only when something asks for it by name.
//!
//! Everything printed here is deliberately plain. The vocabulary the project speaks
//! to people in belongs to the screens that come later; a client at version 0.0.1 is
//! read by someone who needs to know what actually happened.

// Tests assert by panicking, so the lints that forbid panicking in shipped code are
// off inside them. Nothing else in the workspace gets this exemption.
#![cfg_attr(
    test,
    allow(
        clippy::expect_used,
        clippy::unwrap_used,
        clippy::panic,
        clippy::indexing_slicing
    )
)]

mod commands;
mod identity_file;
mod paths;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use clap::{Parser, Subcommand};

use commands::Common;
use n333_net::PeerAddress;
use paths::NodePaths;

/// One node of the 333 network.
#[derive(Debug, Parser)]
#[command(name = "333", version, about = "A node of the 333 network.")]
struct Cli {
    /// Directory holding everything this node owns: its name, and Tor's state if it
    /// uses Tor.
    #[arg(long, global = true, value_name = "DIR")]
    data_dir: Option<PathBuf>,

    /// Seconds to wait for any single step that talks to the network.
    ///
    /// A ceiling rather than a delay. A direct connection is done in milliseconds
    /// and fails on its own; this is sized for a Tor bootstrap, which is the one
    /// step here that can legitimately take minutes.
    #[arg(long, global = true, default_value_t = 300, value_name = "SECONDS")]
    timeout: u64,

    /// Accept a data directory that other users on this machine can read.
    ///
    /// Both this client and arti refuse to start on a loosely permissioned
    /// directory, which is the right default: that directory holds the only copy of
    /// this node's name. The flag exists for scratch directories and containers with
    /// odd ownership, and it does what its name says.
    #[arg(long, global = true)]
    dangerously_trust_directory_permissions: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Show this node's name, creating an identity on first run.
    Id,
    /// Answer heartbeats until interrupted.
    Serve {
        /// Address and port to listen on.
        #[arg(long, default_value_t = default_bind(), value_name = "ADDR:PORT")]
        bind: SocketAddr,

        /// Also publish an onion address, so peers can reach this node without
        /// learning where it is. Starting Tor takes seconds to minutes.
        #[arg(long)]
        tor: bool,

        /// Do not listen on a socket at all. Only useful together with --tor, and
        /// the only way to run a node whose address is nowhere on the wire.
        #[arg(long)]
        no_direct: bool,
    },
    /// Reach another node and exchange one heartbeat.
    Ping {
        /// The peer, as `host`, `host:port`, `[::1]:port` or `something.onion`.
        /// An onion address is reached through Tor; everything else directly.
        address: PeerAddress,
    },
}

/// Listen on every interface, on the port peers expect.
fn default_bind() -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], n333_net::DEFAULT_PORT))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();

    let cli = Cli::parse();
    let common = Common {
        paths: cli.data_dir.map_or_else(NodePaths::default_home, NodePaths::at),
        timeout: Duration::from_secs(cli.timeout),
        trust_directory_permissions: cli.dangerously_trust_directory_permissions,
    };

    match cli.command {
        Command::Id => commands::id::run(&common),
        Command::Serve {
            bind,
            tor,
            no_direct,
        } => commands::serve::run(&common, (!no_direct).then_some(bind), tor).await,
        Command::Ping { address } => commands::ping::run(&common, &address).await,
    }
}
