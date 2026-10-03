//! Whether this node is running, and how: in the background, kept by the service
//! manager, or in a terminal somebody started it in.
//!
//! Found out from outside, without taking the node's directory: whatever asks must not
//! stop a node from starting by holding its directory while it looks. The service
//! manager is asked only when the service installed for this user is the one for this
//! directory, so that a command pointed at some other directory never runs it.
//!
//! WHEN IT STARTED is when the running node took its directory's lock, which is the
//! one moment every system records the same way: the lock file is written then and
//! never again while it is held ([`crate::claim::taken_at`]).

use std::path::Path;

use crate::commands::service::awake;

/// How the node in one directory is running, if it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Running {
    /// The service manager runs it, and will again after a reboot.
    Background {
        /// When it took its directory, in seconds since 1970.
        since: Option<u64>,
    },
    /// Something other than the service manager runs it, most often a terminal.
    Terminal {
        /// When it took its directory, in seconds since 1970.
        since: Option<u64>,
    },
    /// Nothing runs it.
    Not,
}

/// How the node at `root` is running now.
///
/// Where a running node can be asked through its socket, that is what says it is in a
/// terminal; elsewhere its lock is the only sign, and it cannot tell a node from any
/// other command holding the directory.
pub(crate) async fn here(root: &Path) -> Running {
    let since = crate::claim::taken_at(root);
    let kept_here = matches!(
        crate::commands::service::keeper(root),
        crate::commands::service::Keeper::Here { .. }
    );
    if kept_here && crate::commands::service::active() {
        return Running::Background { since };
    }
    let answering = crate::control::answering(root).await;
    if answering || (!cfg!(unix) && crate::claim::held(root)) {
        return Running::Terminal { since };
    }
    Running::Not
}

impl Running {
    /// Whether anything is running it.
    #[must_use]
    pub(crate) const fn is_running(self) -> bool {
        !matches!(self, Self::Not)
    }

    /// The line that says so, at `now` seconds since 1970.
    #[must_use]
    pub(crate) fn line(self, now: u64) -> String {
        let since = |at: u64| (awake::iso(at), awake::how_long(now.saturating_sub(at)));
        match self {
            Self::Background { since: Some(at) } => {
                let (at, ago) = since(at);
                words!("running-background-since", since = at, ago = ago)
            }
            Self::Background { since: None } => words!("running-background"),
            Self::Terminal { since: Some(at) } => {
                let (at, ago) = since(at);
                words!("running-terminal-since", since = at, ago = ago)
            }
            Self::Terminal { since: None } => words!("running-terminal"),
            Self::Not => words!("running-not"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::words::count::Base;

    #[test]
    fn in_english_the_line_says_how_it_runs_and_since_when() {
        let at = 1_790_000_000;
        let lines = crate::words::speaking("en", Base::Ten, || {
            [
                Running::Background { since: Some(at) }.line(at + 600),
                Running::Terminal { since: Some(at) }.line(at + 30),
                Running::Background { since: None }.line(at),
                Running::Not.line(at),
            ]
        });
        assert_eq!(
            lines[0],
            "running  in the background since 2026-09-21T14:13:20Z, 10 minutes ago"
        );
        assert_eq!(
            lines[1],
            "running  in a terminal since 2026-09-21T14:13:20Z, under a minute ago"
        );
        assert_eq!(lines[2], "running  in the background");
        assert!(lines[3].starts_with("running  no."), "{}", lines[3]);
        assert!(lines[3].contains("`333 start`"), "{}", lines[3]);
    }

    #[test]
    fn a_directory_nothing_holds_is_not_running() {
        let root = std::env::temp_dir().join(format!("333-running-{}", std::process::id()));
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        assert_eq!(runtime.block_on(here(&root)), Running::Not);
        assert!(!Running::Not.is_running());
    }
}
