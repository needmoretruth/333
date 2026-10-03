//! `333` with nothing after it: how this node is, and what to type next.
//!
//! A person who has just installed this types `333` first. A refusal that lists every
//! command tells them nothing about their own machine, so this says the three things
//! they need — whether there is a node here, whether it is running and how, and its
//! name — and the two commands that make sense from there. Everything else is
//! `333 --help`.

use std::process::ExitCode;

use n333_core::epoch::unix_now_seconds;

use crate::commands::Common;
use crate::commands::running::{self, Running};

/// Say how this node is, and what comes next.
///
/// # Errors
/// Never, as written: a name that cannot be read is left out rather than refused, so
/// that this always says what it can.
pub(crate) async fn run(common: &Common) -> anyhow::Result<ExitCode> {
    let root = common.paths.root();
    if !crate::identity_file::holds_a_name(root) {
        aloud_in!("overview-no-node", home = root.display().to_string());
        crate::aloud::line(&next(false, Running::Not));
        return Ok(ExitCode::SUCCESS);
    }
    let running = running::here(root).await;
    crate::aloud::line(&running.line(unix_now_seconds()));
    if let Ok(Some(identity)) = crate::identity_file::load(&common.mistrust(), root) {
        let name = crate::commands::shorten(&identity.node_id().to_string());
        aloud_in!("overview-name", name = name);
    }
    crate::aloud::line(&next(true, running));
    Ok(ExitCode::SUCCESS)
}

/// The commands that make sense next, for a node that is `named` or not and runs as
/// `running` does.
fn next(named: bool, running: Running) -> String {
    match (named, running.is_running()) {
        (false, _) => words!("overview-next-new"),
        (true, false) => words!("overview-next-stopped"),
        (true, true) => words!("overview-next-running"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::words::count::Base;

    #[test]
    fn in_english_what_comes_next_follows_from_how_the_node_is() {
        let said =
            |named, running| crate::words::speaking("en", Base::Ten, || next(named, running));
        let new = said(false, Running::Not);
        assert!(
            new.contains("`333 join <invitation>`") && new.contains("`333 begin`"),
            "{new}"
        );
        let stopped = said(true, Running::Not);
        assert!(
            stopped.contains("`333 start`") && stopped.contains("`333 run`"),
            "{stopped}"
        );
        let running = said(true, Running::Terminal { since: None });
        assert!(
            running.contains("`333 status`") && running.contains("`333 stop`"),
            "{running}"
        );
    }
}
