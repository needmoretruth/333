//! `333 service` — keeping the vigil through logouts and reboots, when asked to.
//!
//! A node that stops when a laptop closes is absent from every hour its owner slept
//! through. Every system already has a program whose job is to keep other programs
//! running — systemd, launchd, Task Scheduler — and this asks that program, in its own
//! terms, to keep this one. Nothing is added beside the client: no daemon of its own,
//! no helper, nothing to install first.
//!
//! NOTHING HAPPENS UNTIL IT IS ASKED FOR. The client never installs itself: this runs
//! only when a person types `333 service install`, it says every file it writes and
//! every command it runs as it runs them, and `333 service uninstall` undoes exactly
//! that, from a record of what was done ([`receipt`]).
//!
//! One file per service manager. Each is compiled everywhere, so that what it writes is
//! tested on every system; which one runs is decided here, by the system this was
//! built for.

pub(crate) mod awake;
mod check;
mod exe;
mod launchd;
mod notify;
mod programs;
mod receipt;
mod schtasks;
mod systemd;
mod vigil;

use std::io::Write as _;
use std::path::Path;

use n333_core::epoch::unix_now_seconds;

use crate::commands::Common;

#[cfg(target_os = "macos")]
use launchd as manager;
#[cfg(windows)]
use schtasks as manager;
#[cfg(target_os = "linux")]
use systemd as manager;

// What `333 service` can be asked to do.
//
// What each is for is in the catalogs, under the key each names (`crate::help`).
#[derive(Debug, clap::Subcommand)]
pub(crate) enum Order {
    // Keep this node's vigil through logouts and reboots.
    #[command(
        about = "help-service-install",
        long_about = "help-service-install-long"
    )]
    Install {
        // The flags for `serve`, as they would be typed after it.
        #[arg(
            trailing_var_arg = true,
            allow_hyphen_values = true,
            value_name = "SERVE FLAGS",
            help = "help-service-install-flags"
        )]
        flags: Vec<String>,
    },
    // Stop the vigil's service, and remove everything install wrote.
    #[command(about = "help-service-uninstall")]
    Uninstall,
    // What the service manager says of the vigil.
    #[command(about = "help-service-status")]
    Status,
    // Say so on this machine if the vigil is not being kept.
    #[command(about = "help-service-check")]
    Check,
}

/// What a service manager says about the vigil it keeps.
pub(crate) struct Seen {
    /// Running, stopped, failed or not installed, in words.
    pub(crate) state: String,
    /// The last lines the vigil said, oldest first.
    pub(crate) log: Vec<String>,
}

/// How many of the vigil's last lines `service status` shows.
const LAST_LINES: usize = 20;

/// Do what was asked.
///
/// # Errors
/// Fails if the service manager refuses, a file cannot be written or removed, or the
/// flags given to `install` are ones `serve` would refuse.
pub(crate) async fn run(common: &Common, order: Order) -> anyhow::Result<()> {
    match order {
        Order::Install { flags } => install(common, &flags),
        Order::Uninstall => uninstall(),
        Order::Status => status(common),
        Order::Check => check::run(common).await,
    }
}

/// Say, before anything else, that the vigil for this node is not being kept.
///
/// Every command but the ones that keep or install the vigil begins with this, so that
/// a person who stopped being counted three days ago hears it the next time they
/// type anything at all. `aside` puts it on standard error instead, for a command whose
/// standard output is read by a program.
pub(crate) fn say_if_not_kept(root: &Path, aside: bool) {
    let installed = receipt::keeps(root);
    if let Some(line) = awake::not_kept(awake::read(root), installed, unix_now_seconds()) {
        let _ = if aside {
            writeln!(std::io::stderr().lock(), "{line}")
        } else {
            say(&line)
        };
    }
}

/// Say one line on standard output, already in words.
///
/// Through one locked handle rather than `println!`, so that a reader that walked away
/// ends this quietly instead of panicking inside the print macro.
///
/// # Errors
/// Fails if standard output is closed.
pub(crate) fn say(line: &str) -> std::io::Result<()> {
    writeln!(std::io::stdout().lock(), "{line}")
}

/// The last lines of a log file, oldest first.
#[must_use]
pub(crate) fn last_lines(path: &Path) -> Vec<String> {
    let Ok(text) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    let lines: Vec<&str> = text.lines().collect();
    lines
        .get(lines.len().saturating_sub(LAST_LINES)..)
        .unwrap_or_default()
        .iter()
        .map(|line| (*line).to_owned())
        .collect()
}

/// Write the service, start it, and write down what was done.
fn install(common: &Common, flags: &[String]) -> anyhow::Result<()> {
    let cli = vigil::read(common, flags).unwrap_or_else(|refused| refused.exit());
    let vigil = vigil::write(cli, exe::lasting()?)?;
    if exe::fleeting(&vigil.node) {
        say(&words!(
            "service-mind",
            node = vigil.node.display().to_string()
        ))?;
    }
    let args: Vec<&str> = vigil.serve.iter().map(String::as_str).collect();
    let command = programs::typed(&vigil.exe.display().to_string(), &args);
    say(&words!("service-runs", command = command))?;
    let receipt = match manager::install(&vigil) {
        Ok(receipt) => receipt,
        Err(e) => {
            say(&words!("service-undo-partial"))?;
            return Err(e);
        }
    };
    let Some(path) = receipt::path() else {
        anyhow::bail!(words!("service-no-receipt-directory"));
    };
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(&path, receipt.to_text())?;
    say(&words!(
        "service-wrote-receipt",
        path = path.display().to_string()
    ))?;
    say(&words!("service-undo"))?;
    Ok(())
}

/// Stop the service, remove what install wrote, and forget that it was installed.
fn uninstall() -> anyhow::Result<()> {
    let receipt = receipt::read();
    manager::uninstall(receipt.as_ref())?;
    if let Some(path) = receipt::path().filter(|path| path.exists()) {
        std::fs::remove_file(&path)?;
        say(&words!(
            "service-removed",
            path = path.display().to_string()
        ))?;
        // The directory too, if install is what made it and nothing else is in it.
        if let Some(dir) = path.parent() {
            let _ = std::fs::remove_dir(dir);
        }
    }
    match receipt {
        Some(receipt) => say(&words!(
            "service-uninstalled",
            node = receipt.node.display().to_string()
        ))?,
        None => say(&words!("service-none-installed"))?,
    }
    Ok(())
}

/// What the service manager says, when the vigil last said it was awake, and what it
/// said last.
fn status(common: &Common) -> anyhow::Result<()> {
    let receipt = receipt::read();
    let seen = manager::status(receipt.as_ref());
    let node = receipt.as_ref().map_or_else(
        || common.paths.root().to_path_buf(),
        |receipt| receipt.node.clone(),
    );
    say(&words!("service-state", state = seen.state))?;
    say(&words!("service-node", node = node.display().to_string()))?;
    match awake::read(&node) {
        Some(last) => say(&words!(
            "service-last-awake",
            at = awake::iso(last),
            ago = awake::how_long(unix_now_seconds().saturating_sub(last))
        ))?,
        None => say(&words!("service-never-awake"))?,
    }
    if seen.log.is_empty() {
        return Ok(say(&words!("service-said-nothing"))?);
    }
    say(&words!("service-said-last", lines = seen.log.len()))?;
    for line in &seen.log {
        say(&format!("         {line}"))?;
    }
    Ok(())
}

/// What there is on a system with no service manager this knows how to ask.
#[cfg(not(any(target_os = "linux", target_os = "macos", windows)))]
mod manager {
    use super::Seen;
    use super::receipt::Receipt;
    use super::vigil::Vigil;

    // The same sentence for all three, because it is the same fact.

    pub(crate) fn install(_: &Vigil) -> anyhow::Result<Receipt> {
        anyhow::bail!(words!("service-no-manager"))
    }

    pub(crate) fn uninstall(_: Option<&Receipt>) -> anyhow::Result<()> {
        anyhow::bail!(words!("service-no-manager"))
    }

    pub(crate) fn status(_: Option<&Receipt>) -> Seen {
        Seen {
            state: words!("service-not-installed-here"),
            log: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let pairs = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            [
                (
                    words!("service-mind", node = "/tmp/333-node"),
                    "mind     /tmp/333-node is somewhere this system empties, and this node's name is kept\n\
                     \x20        nowhere else. The service keeps the vigil there until it is emptied.",
                ),
                (
                    words!("service-runs", command = "/usr/bin/333 serve --plain"),
                    "vigil    /usr/bin/333 serve --plain",
                ),
                (
                    words!("service-undo-partial"),
                    "undo     `333 service uninstall` removes whatever of this was done.",
                ),
                (
                    words!("service-no-receipt-directory"),
                    "this system names no configuration directory to keep the receipt in",
                ),
                (
                    format!(
                        "{}\n{}",
                        words!("service-wrote-receipt", path = "/c/333/receipt"),
                        words!("service-undo")
                    ),
                    "wrote    /c/333/receipt, which is how `333 service uninstall` knows what to undo.\n\
                     undo     `333 service uninstall` stops the vigil and undoes all of the above.\n\
                     \x20        The node's own directory is not touched by either.",
                ),
                (
                    words!("service-removed", path = "/c/333/receipt"),
                    "removed  /c/333/receipt",
                ),
                (
                    words!("service-uninstalled", node = "/tmp/333-node"),
                    "vigil    no longer kept by a service. /tmp/333-node is left as the vigil left it:\n\
                     \x20        `333 serve` keeps the vigil by hand, and `333 service install` sets\n\
                     \x20        the service up again.",
                ),
                (
                    words!("service-none-installed"),
                    "service  none was installed by `333 service install` for this user.",
                ),
                (
                    words!("service-state", state = "running"),
                    "service  running",
                ),
                (
                    words!("service-node", node = "/tmp/333-node"),
                    "node     /tmp/333-node",
                ),
                (
                    words!(
                        "service-last-awake",
                        at = "2026-09-22T03:10:00Z",
                        ago = "7 minutes"
                    ),
                    "awake    said so last at 2026-09-22T03:10:00Z, 7 minutes ago",
                ),
                (
                    words!("service-never-awake"),
                    "awake    never said so, in this directory",
                ),
                (
                    words!("service-said-nothing"),
                    "said     nothing that was kept",
                ),
                (
                    words!("service-said-last", lines = 20_usize),
                    "said     the last 20 lines:",
                ),
                (
                    words!("service-no-manager"),
                    "this system has no service manager `333 service` knows how to ask. \
                     `333 serve --plain` keeps the vigil under whatever keeps programs \
                     running here.",
                ),
                (
                    words!("service-not-installed-here"),
                    "not installed: there is no service manager here this knows",
                ),
                (
                    words!("service-creating", path = "/h/.config/systemd/user"),
                    "creating /h/.config/systemd/user",
                ),
                (
                    words!("service-writing", path = "/h/333.service"),
                    "writing /h/333.service",
                ),
                (
                    words!("service-removing", path = "/h/333.service"),
                    "removing /h/333.service",
                ),
                (
                    words!("service-wrote", path = "/h/333.service"),
                    "wrote    /h/333.service",
                ),
                (
                    words!("service-left", path = "/h/333.service"),
                    "left     /h/333.service. `333 service install` did not write it.",
                ),
                (
                    words!("service-failed", why = "`systemctl` did not succeed: no"),
                    "failed   `systemctl` did not succeed: no",
                ),
                (words!("service-not-installed"), "not installed"),
                (words!("service-running"), "running"),
                (words!("service-starting"), "starting"),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

    #[test]
    fn a_long_log_is_shown_as_its_last_twenty_lines() {
        let path = std::env::temp_dir().join(format!("333-log-{}", std::process::id()));
        let text: String = (1..=25).map(|n| format!("line {n}\n")).collect();
        std::fs::write(&path, text).unwrap();
        let last = last_lines(&path);
        std::fs::remove_file(&path).unwrap();
        assert_eq!(last.len(), 20);
        assert_eq!(last.first().map(String::as_str), Some("line 6"));
        assert_eq!(last.last().map(String::as_str), Some("line 25"));
    }
}
