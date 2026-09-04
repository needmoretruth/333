//! The 333 command line client.
//!
//! Three things it can do at this version: show this node's name, publish an onion
//! address and answer heartbeats on it, and dial another node to exchange one.
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

use std::path::PathBuf;
use std::time::Duration;

use clap::{Parser, Subcommand};

use commands::Common;
use paths::NodePaths;

/// One node of the 333 network.
#[derive(Debug, Parser)]
#[command(name = "333", version, about = "A node of the 333 network.")]
struct Cli {
    /// Directory holding this node's identity and Tor state.
    #[arg(long, global = true, value_name = "DIR")]
    data_dir: Option<PathBuf>,

    /// Seconds to wait for any single step that talks to the Tor network.
    #[arg(long, global = true, default_value_t = 300, value_name = "SECONDS")]
    timeout: u64,

    /// Accept a data directory that other users on this machine can read.
    ///
    /// Arti refuses to start on a loosely permissioned directory, which is the right
    /// default. This exists for scratch directories and containers with odd
    /// ownership, and it does what its name says.
    #[arg(long, global = true)]
    dangerously_trust_directory_permissions: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Show this node's name, creating an identity on first run.
    Id,
    /// Publish an onion address and answer heartbeats until interrupted.
    Serve,
    /// Dial another node and exchange one heartbeat.
    Ping {
        /// The peer's onion address, without the port.
        address: String,
        /// The peer's port.
        #[arg(long, default_value_t = commands::ping::default_port())]
        port: u16,
    },
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
        Command::Serve => commands::serve::run(&common).await,
        Command::Ping { address, port } => commands::ping::run(&common, &address, port).await,
    }
}
