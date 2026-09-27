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
mod archive;
mod began;
mod claim;
mod commands;
mod control;
mod dial;
mod dwelling;
mod identity_file;
mod named;
mod node;
mod orders;
mod paths;
#[cfg(feature = "screen")]
mod screen;
mod typed;
mod version;

use std::process::ExitCode;
use std::time::Duration;

use clap::Parser as _;

use claim::Taken;
use commands::Common;
use paths::NodePaths;
use typed::{Cli, Command};

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
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
        paths: cli
            .data_dir
            .map_or_else(NodePaths::default_home, NodePaths::at),
        timeout: Duration::from_secs(cli.timeout),
        keeping: if cli.keep_everything {
            node::Keeping::Everything
        } else {
            node::Keeping::TheWindow
        },
        bridges: std::sync::Arc::new(std::sync::Mutex::new(n333_net::bridges::Bridges {
            lines: cli.bridges,
            helper: cli.bridge_helper,
        })),
        trust_directory_permissions: cli.dangerously_trust_directory_permissions,
    };

    // First of all, because it is the one thing a person who has stopped being counted
    // most needs to hear, and they hear it from whichever command they typed. Not from
    // the ones that keep the vigil or set it up: those are the answer to it. And not
    // while a vigil answers in this directory: it is being kept, whatever the stamp says.
    let keeps_or_installs = matches!(
        cli.command,
        Command::Serve { .. }
            | Command::Service {
                order: commands::service::Order::Install { .. }
                    | commands::service::Order::Uninstall
                    | commands::service::Order::Check
            }
    );
    if !keeps_or_installs && !control::answering(common.paths.root()).await {
        commands::service::say_if_not_kept(common.paths.root());
    }

    // Before anything is attempted, because everything that follows is stamped with an
    // epoch and a node whose clock is wrong is refused everywhere without being told
    // why.
    commands::check_the_clock(n333_core::Epoch::now());

    // The service manager is asked, and the awake stamp read, and nothing in the node's
    // directory is opened, so none of it waits for the directory or makes it.
    if let Command::Service { order } = cli.command {
        let done = commands::service::run(&common, order).await;
        return quietly_if_walked_away(done.map(|()| ExitCode::SUCCESS));
    }

    // Unpacking takes the directory itself: it may not exist yet, and it has to be
    // empty when the node is renamed into it, which a lock file inside it would not be.
    if let Command::Unpack { file } = &cli.command {
        let done = commands::unpack::run(&common, file).await;
        return quietly_if_walked_away(done);
    }

    // Before anything in the directory is read, and held until this process exits.
    let _claim = match claim::take(&common.mistrust(), common.paths.root())? {
        Taken::Ours(claim) => claim,
        Taken::Theirs(holder) => {
            let beside = commands::elsewhere::run(&common, holder, cli.command.wanted()).await;
            return quietly_if_walked_away(beside);
        }
    };

    let done = match cli.command {
        Command::Id => commands::id::run(&common),
        Command::Serve {
            bind,
            tor,
            no_direct,
            announce,
            no_mdns,
            no_router,
            meet,
            no_meet,
            plain,
        } => {
            commands::serve::run(
                &common,
                commands::serve::Vigil {
                    bind: (!no_direct).then_some(bind),
                    tor,
                    announce,
                    nearby: !no_mdns,
                    meet: (!no_meet).then_some(meet),
                    plain,
                    ask_the_router: !no_router,
                },
            )
            .await
        }
        Command::Bootstrap { meet, anyway } => {
            commands::bootstrap::run(&common, &meet, anyway).await
        }
        Command::Say { index } => commands::say::run(&common, index).await,
        Command::Status => commands::status::run(&common).await,
        Command::Join { address } => commands::join::run(&common, &address).await,
        Command::Ping { address } => commands::ping::run(&common, &address).await,
        Command::Pack { file, undo } => commands::pack::run(&common, file.as_deref(), undo),
        Command::Moved => commands::moved::run(&common),
        Command::Tell { .. } => return Ok(commands::elsewhere::nobody_to_tell()),
        // Dispatched above, before the directory is taken.
        Command::Service { .. } | Command::Unpack { .. } => Ok(()),
    };
    quietly_if_walked_away(done.map(|()| ExitCode::SUCCESS))
}

/// A reader that walked away — `333 status | head` — is not a failure and has nothing
/// to be told about it. Anything else is reported as it is.
fn quietly_if_walked_away(done: anyhow::Result<ExitCode>) -> anyhow::Result<ExitCode> {
    match done {
        Err(e) if walked_away(&e) => Ok(ExitCode::SUCCESS),
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
