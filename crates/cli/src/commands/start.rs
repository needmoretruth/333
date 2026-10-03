//! `333 start`, `333 stop` and `333 restart`: the node in the background, after every
//! reboot, or not at all, in the words a person reaches for first.
//!
//! In the background is the service manager's job, and `333 service install` already
//! knows how to ask each one; `start` installs through it the first time and only
//! switches it on and off after that, so that stopping keeps every file install wrote
//! and starting again writes nothing. A node somebody runs in a terminal is theirs: it
//! is never started a second time in the background beside them, and is stopped by
//! asking it through its socket, the way the screen's own `quit` asks.

use std::path::Path;
use std::process::ExitCode;
use std::time::{Duration, Instant};

use crate::commands::Common;
use crate::commands::running::{self, Running};
use crate::commands::service::{self, Keeper, say};

/// How long a node that was asked to stop has to stop before this says it has not.
///
/// Stopping gives back what it asked of the router and says its last line, which on a
/// slow router is seconds; this is far beyond that.
const PATIENCE: Duration = Duration::from_secs(30);

/// Refuse to do anything for a directory with no node in it, saying what makes one.
#[must_use]
pub(crate) fn no_node(home: &Path) -> anyhow::Error {
    let none = anyhow::anyhow!(words!("start-no-node", home = home.display().to_string()));
    crate::failed::next_step(none, words!("start-no-node-next"))
}

/// Run the node in the background now and after every reboot, with `flags` for `run`
/// if any were given.
///
/// Flags are what `service install` takes, and are kept: given once, every later
/// `start` without flags runs the node with them. Given again and different from what
/// is installed, the service is installed again with them, as `service install` would.
///
/// # Errors
/// Fails if there is no node, the flags are ones `run` refuses, the service belongs to
/// another directory, or the service manager refuses.
pub(crate) async fn start(common: &Common, flags: &[String]) -> anyhow::Result<ExitCode> {
    let root = common.paths.root();
    if !crate::identity_file::holds_a_name(root) {
        return Err(no_node(root));
    }
    let wanted = match flags {
        [] => None,
        flags => Some(service::would_run(common, flags)?),
    };
    let keeper = service::keeper(root);
    let again = matches!(keeper, Keeper::Here { .. })
        && installs_again(wanted.as_deref(), &service::installed_run());
    match running::here(root).await {
        Running::Terminal { .. } => {
            aloud_in!("start-in-a-terminal");
            return Ok(ExitCode::SUCCESS);
        }
        Running::Background { .. } if !again => {
            aloud_in!("start-already");
            // Running, so not stopped, whatever was written down before.
            service::running_after_all();
            return Ok(ExitCode::SUCCESS);
        }
        Running::Background { .. } | Running::Not => {}
    }
    match keeper {
        Keeper::Nobody => service::install(common, flags)?,
        Keeper::Here { .. } if again => service::install(common, flags)?,
        Keeper::Here { .. } => service::resume()?,
        Keeper::Elsewhere(other) => anyhow::bail!(words!(
            "start-elsewhere",
            other = other.display().to_string()
        )),
    }
    aloud_in!("start-started");
    Ok(ExitCode::SUCCESS)
}

/// Whether `start` installs the service again: only when flags were given, and what
/// they make the service run is not what the installed one runs. An install whose
/// receipt does not say what it runs is installed again whenever flags are given.
fn installs_again(wanted: Option<&[String]>, installed: &[String]) -> bool {
    wanted.is_some_and(|wanted| wanted != installed)
}

/// Stop the node now and keep it stopped after a reboot.
///
/// # Errors
/// Fails if the service manager refuses, or a node in a terminal will not stop.
pub(crate) async fn stop(common: &Common) -> anyhow::Result<ExitCode> {
    let root = common.paths.root();
    let mut stopped = false;
    if let Keeper::Here {
        stopped: on_purpose,
    } = service::keeper(root)
    {
        let active = service::active();
        if active || !on_purpose {
            service::pause()?;
            stopped = active;
            if active {
                quiet(root).await;
            }
        }
    }
    if running::here(root).await.is_running() {
        if !cfg!(unix) {
            anyhow::bail!(words!("stop-cannot-ask-here"));
        }
        ask_to_quit(root).await?;
        aloud_in!("stop-stopped-in-a-terminal");
        return Ok(ExitCode::SUCCESS);
    }
    if stopped {
        aloud_in!("stop-stopped");
    } else {
        aloud_in!("stop-not-running");
    }
    Ok(ExitCode::SUCCESS)
}

/// Stop the node, then run it in the background, with `flags` for `run` as `start`
/// takes them.
///
/// # Errors
/// Whatever stopping or starting failed with.
pub(crate) async fn restart(common: &Common, flags: &[String]) -> anyhow::Result<ExitCode> {
    stop(common).await?;
    start(common, flags).await
}

/// Ask the node running at `root` to stop through its socket, and wait until it has.
///
/// A node that stops while it answers closes the socket before it has said it is done,
/// and that is the answer too.
async fn ask_to_quit(root: &Path) -> anyhow::Result<()> {
    if let Ok(Some((false, said))) = crate::control::ask(root, "quit").await {
        say(said.trim_end())?;
        anyhow::bail!(words!("stop-refused"));
    }
    if quiet(root).await {
        return Ok(());
    }
    anyhow::bail!(words!("stop-still-running", seconds = PATIENCE.as_secs()))
}

/// Wait until nothing runs at `root`, saying whether that happened in time.
async fn quiet(root: &Path) -> bool {
    let began = Instant::now();
    while running::here(root).await.is_running() {
        if began.elapsed() > PATIENCE {
            return false;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::words::count::Base;

    #[test]
    fn the_service_is_installed_again_only_for_flags_that_differ_from_what_it_runs() {
        let installed = ["--data-dir", "/n", "serve", "--plain", "--tor"].map(str::to_owned);
        let other = ["--data-dir", "/n", "serve", "--plain"].map(str::to_owned);
        assert!(
            !installs_again(None, &installed),
            "no flags keeps what is installed"
        );
        assert!(!installs_again(Some(&installed), &installed));
        assert!(installs_again(Some(&other), &installed));
        assert!(
            installs_again(Some(&other), &[]),
            "a receipt that does not say what it runs"
        );
    }

    #[test]
    fn the_flags_are_compared_as_the_service_would_run_them() {
        let common = Common {
            paths: crate::paths::NodePaths::at(std::path::PathBuf::from("/srv/node")),
            timeout: Duration::from_secs(300),
            keeping: crate::node::Keeping::TheWindow,
            bridges: std::sync::Arc::default(),
            trust_directory_permissions: false,
        };
        let flags = ["--tor", "--bind", "127.0.0.1:4444"].map(str::to_owned);
        let run = crate::words::speaking("en", Base::Ten, || service::would_run(&common, &flags))
            .unwrap();
        // Written as the system spells an absolute path: `D:\srv\node` on Windows.
        let home = std::path::absolute("/srv/node").unwrap();
        assert_eq!(
            run[..2],
            ["--data-dir".to_owned(), home.display().to_string()]
        );
        let after = run.iter().position(|arg| arg == "serve").unwrap();
        assert_eq!(
            run[after..],
            ["serve", "--plain", "--bind", "127.0.0.1:4444", "--tor"]
        );
    }

    #[test]
    fn with_no_node_it_says_how_to_get_one() {
        let said = crate::words::speaking("en", Base::Ten, || {
            crate::failed::said(&no_node(Path::new("/tmp/nowhere")))
        });
        assert!(said.contains("/tmp/nowhere"), "{said}");
        assert!(said.contains("`333 join <invitation>`"), "{said}");
        assert!(said.contains("`333 begin`"), "{said}");
    }
}
