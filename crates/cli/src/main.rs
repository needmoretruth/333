//! The 333 command line client.
//!
//! What a person reaches for first is `333`, `333 start`, `333 stop` and `333 status`:
//! whether this node is running, running it in the background, and stopping it. The
//! rest is what a node can be asked to do while it runs, and moving one.
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
// Second, so that everything below it can say something in the reader's language.
#[macro_use]
mod words;
mod archive;
mod began;
mod claim;
mod commands;
mod control;
mod dial;
mod dwelling;
mod failed;
mod help;
mod identity_file;
mod named;
mod node;
mod orders;
mod paths;
#[cfg(feature = "screen")]
mod screen;
mod typed;
mod version;

use std::io::Write as _;
use std::process::ExitCode;
use std::time::Duration;

use claim::Taken;
use commands::Common;
use paths::NodePaths;
use typed::{Cli, Command};

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(code) => code,
        // A reader that walked away — `333 status | head` — is not a failure and has
        // nothing to be told about it. Anything else is reported as it is.
        Err(e) if walked_away(&e) => ExitCode::SUCCESS,
        Err(e) => {
            // Straight to the error stream, and past the screen: by the time a command
            // has failed the screen is gone, and a line sent to it would be lost. A
            // stream nobody is reading is not a reason to fail harder.
            let _ = writeln!(std::io::stderr(), "{}", failed::said(&e));
            ExitCode::FAILURE
        }
    }
}

/// Read the command line and do what it asks, with the exit code it ends in.
async fn run() -> anyhow::Result<ExitCode> {
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

    let cli = typed::read();
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

    // Before anything is said, because everything that is said is said in these.
    words::install(cli.language.as_deref(), cli.count_in, common.paths.root());

    // `333` alone says how this node is, and reads only what it can without taking the
    // directory.
    let Some(command) = cli.command else {
        commands::check_the_clock(n333_core::Epoch::now());
        return commands::overview::run(&common).await;
    };

    // First of all, because it is the one thing a person who has stopped being counted
    // most needs to hear, and they hear it from whichever command they typed. Not while
    // a node answers in this directory: it is running, whatever the stamp says.
    if !answers_it_already(&command) && !control::answering(common.paths.root()).await {
        let read_by_a_program = matches!(command, Command::Status { json: true, .. });
        commands::service::say_if_not_kept(common.paths.root(), read_by_a_program);
    }

    // Before anything is attempted, because everything that follows is stamped with an
    // epoch and a node whose clock is wrong is refused everywhere without being told
    // why.
    commands::check_the_clock(n333_core::Epoch::now());

    let command = match without_the_directory(&common, command).await? {
        Ok(code) => return Ok(code),
        Err(command) => command,
    };

    // Whether it is running comes first, and only this terminal can say how: the page
    // after it may be handed back by the running node itself.
    if matches!(
        command,
        Command::Status {
            all: false,
            sources: false,
            json: false
        }
    ) && identity_file::holds_a_name(common.paths.root())
    {
        let running = commands::running::here(common.paths.root()).await;
        aloud::line(&running.line(n333_core::epoch::unix_now_seconds()));
    }

    // Before the directory is taken, which would make it: a command that only reads a
    // node, pointed at a directory with none, says so and makes nothing.
    command.needs_a_node_in(common.paths.root())?;
    // Nobody keeps a vigil in a directory that is not there, and there is nothing to
    // take.
    if matches!(command, Command::Tell { .. }) && !common.paths.root().exists() {
        return Ok(commands::elsewhere::nobody_to_tell());
    }

    // Before anything in the directory is read, and held until this process exits.
    let _claim = match claim::take(&common.mistrust(), common.paths.root())? {
        Taken::Ours(claim) => claim,
        Taken::Theirs(holder) => {
            return commands::elsewhere::run(&common, holder, command.wanted()).await;
        }
    };

    let done = match command {
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
        Command::Say { index } => match commands::say::read_index(&index) {
            Ok(index) => commands::say::run(&common, index).await,
            Err(e) => Err(e),
        },
        Command::Status { all, sources, json } => {
            let show = commands::status::Show::of(all, sources, json);
            commands::status::run(&common, show).await
        }
        Command::Join { address } => commands::join::run(&common, &address).await,
        Command::Ping { address } => commands::ping::run(&common, &address).await,
        Command::Pack { file, undo } => commands::pack::run(&common, file.as_deref(), undo),
        Command::Moved => commands::moved::run(&common),
        Command::Tell { .. } => return Ok(commands::elsewhere::nobody_to_tell()),
        // Dispatched above, before the directory is taken.
        Command::Service { .. }
        | Command::Unpack { .. }
        | Command::Languages { .. }
        | Command::Start { .. }
        | Command::Stop
        | Command::Restart { .. }
        | Command::Logs { .. }
        | Command::Invite => Ok(()),
    };
    done.map(|()| ExitCode::SUCCESS)
}

/// Whether a command is itself the answer to a node not running: the ones that run it
/// or set it up, and the ones whose first line says whether it is running.
fn answers_it_already(command: &Command) -> bool {
    matches!(
        command,
        Command::Serve { .. }
            | Command::Start { .. }
            | Command::Stop
            | Command::Restart { .. }
            | Command::Logs { .. }
            | Command::Status {
                all: false,
                sources: false,
                json: false
            }
            | Command::Service {
                order: commands::service::Order::Install { .. }
                    | commands::service::Order::Uninstall
                    | commands::service::Order::Check
            }
    )
}

/// Do a command that asks the service manager, reads the catalogs, or reads only files
/// a running node writes whole, and so neither waits for the node's directory nor makes
/// it; or hand any other command back.
///
/// Unpacking is one of these too: it takes the directory itself, which may not exist
/// yet and has to be empty when the node is renamed into it, which a lock file inside
/// it would not be.
async fn without_the_directory(
    common: &Common,
    command: Command,
) -> anyhow::Result<Result<ExitCode, Command>> {
    let done = |done: anyhow::Result<()>| done.map(|()| Ok(ExitCode::SUCCESS));
    match command {
        Command::Service { order } => done(commands::service::run(common, order).await),
        Command::Languages { tag } => done(commands::languages::run(common, tag.as_deref())),
        Command::Start { flags } => commands::start::start(common, &flags).await.map(Ok),
        Command::Stop => commands::start::stop(common).await.map(Ok),
        Command::Restart { flags } => commands::start::restart(common, &flags).await.map(Ok),
        Command::Logs { follow } => commands::logs::run(follow).map(Ok),
        Command::Invite => commands::invite::run(common).map(Ok),
        Command::Unpack { file } => commands::unpack::run(common, &file).await.map(Ok),
        other => Ok(Err(other)),
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
