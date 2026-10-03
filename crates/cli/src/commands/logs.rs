//! `333 logs` — the last lines the node said while it ran in the background.
//!
//! The same lines `333 service status` shows, from wherever the service manager keeps
//! them: systemd's journal, or the file launchd and Task Scheduler append to. A node run
//! in a terminal said its lines there, and nothing kept them.

use std::process::ExitCode;

use crate::commands::service::{self, say};

/// Say the last lines, or follow them as they are said where that can be done.
///
/// # Errors
/// Fails if standard output is closed, or the program that follows them fails.
pub(crate) fn run(follow: bool) -> anyhow::Result<ExitCode> {
    let Some(lines) = service::last_said() else {
        aloud_in!("logs-none");
        return Ok(ExitCode::SUCCESS);
    };
    if follow && let Some(followed) = service::follow() {
        followed?;
        return Ok(ExitCode::SUCCESS);
    }
    if lines.is_empty() {
        aloud_in!("logs-nothing-yet");
    }
    for line in &lines {
        say(line)?;
    }
    if follow {
        aloud_in!("logs-cannot-follow");
    }
    Ok(ExitCode::SUCCESS)
}
