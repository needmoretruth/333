//! The 333 command line client.
//!
//! Three things it can do at this version: show this node's name, listen for
//! heartbeats, and reach another node to exchange one.
//!
//! Reaching a peer is direct by default. Tor is carried for the nodes that need
//! their own address unseen, and starting it is the slowest thing this program can
//! be asked to do, so it happens only when something asks for it by name.
//!
//! Every line printed here is a log line and a liturgy at once, and it has to be both
//! or it is neither. A keyword, then what happened, in the words this network uses for
//! it: nothing is dressed up, nothing is understated, and nothing is explained. An
//! operator reading the output at three in the morning needs to know exactly what their
//! node did. That is the same sentence.

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

// First, so that everything below it can say something out loud.
#[macro_use]
mod aloud;
mod commands;
mod dial;
mod identity_file;
mod node;
mod paths;
#[cfg(feature = "screen")]
mod screen;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use clap::{Parser, Subcommand};

use commands::Common;
use n333_net::PeerAddress;
use paths::NodePaths;

/// One node of the 333 network.
#[derive(Debug, Parser)]
#[command(
    name = "333",
    version,
    about = "One node of 333. It keeps the hours, answers when asked, and passes the file on."
)]
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

    /// Accept a directory that others on this machine can enter.
    ///
    /// Both this client and arti refuse to start on a loosely permissioned
    /// directory, which is the right default: that directory holds the only copy of
    /// this node's name. The flag exists for scratch directories and containers with
    /// odd ownership, and it does what its name says.
    #[arg(long, global = true)]
    dangerously_trust_directory_permissions: bool,

    /// Keep every statement for ever, instead of the window standing is read over.
    ///
    /// It confers nothing. Every statement carries its own signature and verifies the
    /// same wherever it was kept, so there is no archive of record and nobody becomes
    /// an archivist by doing this. It is for people who would rather the bytes still
    /// existed somewhere, which nothing here requires of anyone.
    #[arg(long, global = true)]
    keep_everything: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Show this node's name, asking for one on first run.
    Id,
    /// Keep the vigil: answer whoever asks, until interrupted.
    Serve {
        /// Address and port to listen on.
        #[arg(long, default_value_t = default_bind(), value_name = "ADDR:PORT")]
        bind: SocketAddr,

        /// Also raise an onion address, so others can reach this node without
        /// learning where it is. Waking Tor takes seconds to minutes.
        #[arg(long)]
        tor: bool,

        /// Do not open a socket at all. Only useful with --tor, and the only way to
        /// keep the vigil with your address nowhere on the wire.
        #[arg(long)]
        no_direct: bool,

        /// The address to tell other nodes to reach this one at.
        ///
        /// Needed when the socket cannot say: listening on every interface, or behind
        /// something that forwards a port. Without it a node on a wildcard bind can
        /// answer whoever finds it and can never be found.
        #[arg(long, value_name = "HOST:PORT")]
        announce: Option<PeerAddress>,

        /// Do not say on the local network that this node is here.
        ///
        /// What goes out otherwise is that something on this machine speaks 333 and
        /// on which port — not this node's name — which is what a port scan of the
        /// same network would find anyway. It is how two nodes in one house find each
        /// other with nobody typing an invitation. A node listening only through Tor
        /// never does this at all.
        #[arg(long)]
        no_mdns: bool,

        /// Say the lines instead of drawing the screen.
        ///
        /// The screen is what this client does on a terminal. Anywhere else — a pipe,
        /// a service manager's log, a file — it says the lines instead, and this flag
        /// asks for that on a terminal too.
        #[arg(long)]
        plain: bool,
    },
    /// Speak one of the 333, once in this epoch. What travels is the number.
    Say {
        /// Which of them, from 0 to 332. The words are not written yet.
        #[arg(value_name = "INDEX")]
        index: u16,
    },
    /// Show what this node has seen: how many of us are answering, where this node
    /// stands over the window, and how much of the silence is left if it has begun.
    Status,
    /// Ask a node that has the file to hand it over. Write the three bytes yourself and
    /// you hold three bytes: you are one of us from the moment somebody gives them to
    /// you and you both sign for it.
    Join {
        /// An invitation (`333:host:port`) from somebody who already has it.
        #[arg(value_parser = n333_net::invite::address_or_invite)]
        address: PeerAddress,
    },
    /// Knock on another node, and exchange one heartbeat with it.
    Ping {
        /// An invitation (`333:host:port`), or an address typed by hand as `host`,
        /// `host:port`, `[::1]:port` or `something.onion`. An onion address is
        /// reached through Tor; everything else directly.
        #[arg(value_parser = n333_net::invite::address_or_invite)]
        address: PeerAddress,
    },
}

/// Listen on every interface, on the port peers expect.
fn default_bind() -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], n333_net::DEFAULT_PORT))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Everything the libraries under this say goes where everything this client says
    // goes. Otherwise arti writes a warning straight into a terminal the screen is
    // drawing on, and what a person sees is a bootstrap message wearing a border.
    tracing_subscriber::fmt()
        .with_writer(aloud::Voice)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();

    let cli = Cli::parse();
    let common = Common {
        paths: cli.data_dir.map_or_else(NodePaths::default_home, NodePaths::at),
        timeout: Duration::from_secs(cli.timeout),
        keeping: if cli.keep_everything {
            node::Keeping::Everything
        } else {
            node::Keeping::TheWindow
        },
        trust_directory_permissions: cli.dangerously_trust_directory_permissions,
    };

    // Before anything is attempted, because everything that follows is stamped with an
    // epoch and a node whose clock is wrong is refused everywhere without being told
    // why.
    commands::check_the_clock(n333_core::Epoch::now());

    let done = match cli.command {
        Command::Id => commands::id::run(&common),
        Command::Serve {
            bind,
            tor,
            no_direct,
            announce,
            no_mdns,
            plain,
        } => {
            commands::serve::run(
                &common,
                (!no_direct).then_some(bind),
                tor,
                announce,
                !no_mdns,
                plain,
            )
            .await
        }
        Command::Say { index } => commands::say::run(&common, index).await,
        Command::Status => commands::status::run(&common).await,
        Command::Join { address } => commands::join::run(&common, &address).await,
        Command::Ping { address } => commands::ping::run(&common, &address).await,
    };
    match done {
        // A reader that walked away — `333 status | head` — is not a failure and has
        // nothing to be told about it. Anything else is reported as it is.
        Err(e) if walked_away(&e) => Ok(()),
        other => other,
    }
}

/// Did this end because whoever was reading the output closed the pipe?
fn walked_away(error: &anyhow::Error) -> bool {
    error.chain().any(|cause| {
        cause
            .downcast_ref::<std::io::Error>()
            .is_some_and(|io| io.kind() == std::io::ErrorKind::BrokenPipe)
    })
}
