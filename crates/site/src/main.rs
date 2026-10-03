//! `333-site`: what runs the333.dev, on one machine beside one node.
//!
//! The site is a convenience and nothing depends on it. It does three things:
//!
//! - `serve` — the meeting point (the same paths, bodies and status codes the old
//!   Cloudflare Worker gave, so released clients keep working) and the pages.
//! - `observe` — run by the node's owner on a timer, it reads the node's files without
//!   touching them, asks the running node for `status --json`, and writes one public
//!   JSON file for the pages. The site process never reads the node's directory.
//! - `deploy` — run by the site's own user on a timer, it builds whatever `main` holds
//!   and switches to it, keeping the release that runs when a build fails.
//!
//! WHAT IT NEVER DOES. It never relays traffic between nodes, never holds a key, and
//! never writes a visitor's address anywhere: the per-address wait between statements is
//! keyed by an HMAC whose key lives only in this process's memory.

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

mod atomic;
mod board;
mod deploy;
mod frames;
mod gate;
mod history;
mod observe;
mod place;
mod serve;
mod site;
mod words;

use std::process::ExitCode;

use clap::{Parser, Subcommand};

/// The command line.
#[derive(Parser)]
#[command(name = "333-site", about = "Serves, observes and deploys the333.dev")]
struct Cli {
    /// What to do.
    #[command(subcommand)]
    command: Command,
}

/// The three things this binary is run for.
#[derive(Subcommand)]
enum Command {
    /// Serve the meeting point and the pages.
    Serve(serve::Args),
    /// Write the public observation of the site's node, once.
    Observe(observe::Args),
    /// Build and switch to what `origin/main` holds, if it changed.
    Deploy(deploy::Args),
}

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .with_target(false)
        // The journal shows escape codes as they are.
        .with_ansi(false)
        // journald stamps every line already.
        .without_time()
        .init();

    let done = match Cli::parse().command {
        Command::Serve(args) => serve::run(&args),
        Command::Observe(args) => observe::run(&args),
        Command::Deploy(args) => deploy::run(&args),
    };
    match done {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            tracing::error!("{error:#}");
            ExitCode::FAILURE
        }
    }
}
