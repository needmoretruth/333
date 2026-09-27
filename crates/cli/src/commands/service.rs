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

/// What `333 service` can be asked to do.
#[derive(Debug, clap::Subcommand)]
pub(crate) enum Order {
    /// Keep this node's vigil through logouts and reboots, with this system's own
    /// service manager.
    ///
    /// Give it the flags you would give `serve`: the service runs `serve` with exactly
    /// those, for this node's directory, and an hourly check beside it says so on this
    /// machine when the vigil stops. Every file it writes and every command it runs is
    /// said as it happens, and `333 service uninstall` undoes all of it.
    Install {
        /// The flags for `serve`, as you would type them after it.
        #[arg(
            trailing_var_arg = true,
            allow_hyphen_values = true,
            value_name = "SERVE FLAGS"
        )]
        flags: Vec<String>,
    },
    /// Stop the vigil's service, and remove everything `service install` wrote.
    Uninstall,
    /// What the service manager says of the vigil, when the vigil last said it was
    /// awake, and the last things it said.
    Status,
    /// Say so on this machine if the vigil is not being kept. The service runs this
    /// every hour; it says nothing when all is well.
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
            say(format_args!("{line}"))
        };
    }
}

/// Say one line on standard output.
///
/// Through one locked handle rather than `println!`, so that a reader that walked away
/// ends this quietly instead of panicking inside the print macro.
///
/// # Errors
/// Fails if standard output is closed.
pub(crate) fn say(text: std::fmt::Arguments<'_>) -> std::io::Result<()> {
    writeln!(std::io::stdout().lock(), "{text}")
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
        say(format_args!(
            "mind     {} is somewhere this system empties, and this node's name is kept\n\
             \x20        nowhere else. The service keeps the vigil there until it is emptied.",
            vigil.node.display()
        ))?;
    }
    let args: Vec<&str> = vigil.serve.iter().map(String::as_str).collect();
    say(format_args!(
        "vigil    {}",
        programs::typed(&vigil.exe.display().to_string(), &args)
    ))?;
    let receipt = match manager::install(&vigil) {
        Ok(receipt) => receipt,
        Err(e) => {
            say(format_args!(
                "undo     `333 service uninstall` removes whatever of this was done."
            ))?;
            return Err(e);
        }
    };
    let Some(path) = receipt::path() else {
        anyhow::bail!("this system names no configuration directory to keep the receipt in");
    };
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(&path, receipt.to_text())?;
    say(format_args!(
        "wrote    {}, which is how `333 service uninstall` knows what to undo.\n\
         undo     `333 service uninstall` stops the vigil and undoes all of the above.\n\
         \x20        The node's own directory is not touched by either.",
        path.display()
    ))?;
    Ok(())
}

/// Stop the service, remove what install wrote, and forget that it was installed.
fn uninstall() -> anyhow::Result<()> {
    let receipt = receipt::read();
    manager::uninstall(receipt.as_ref())?;
    if let Some(path) = receipt::path().filter(|path| path.exists()) {
        std::fs::remove_file(&path)?;
        say(format_args!("removed  {}", path.display()))?;
        // The directory too, if install is what made it and nothing else is in it.
        if let Some(dir) = path.parent() {
            let _ = std::fs::remove_dir(dir);
        }
    }
    match receipt {
        Some(receipt) => say(format_args!(
            "vigil    no longer kept by a service. {} is left as the vigil left it:\n\
             \x20        `333 serve` keeps the vigil by hand, and `333 service install` sets\n\
             \x20        the service up again.",
            receipt.node.display()
        ))?,
        None => say(format_args!(
            "service  none was installed by `333 service install` for this user."
        ))?,
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
    say(format_args!("service  {}", seen.state))?;
    say(format_args!("node     {}", node.display()))?;
    match awake::read(&node) {
        Some(last) => say(format_args!(
            "awake    said so last at {}, {} ago",
            awake::iso(last),
            awake::how_long(unix_now_seconds().saturating_sub(last))
        ))?,
        None => say(format_args!("awake    never said so, in this directory"))?,
    }
    if seen.log.is_empty() {
        return Ok(say(format_args!("said     nothing that was kept"))?);
    }
    say(format_args!("said     the last {} lines:", seen.log.len()))?;
    for line in &seen.log {
        say(format_args!("         {line}"))?;
    }
    Ok(())
}

/// What there is on a system with no service manager this knows how to ask.
#[cfg(not(any(target_os = "linux", target_os = "macos", windows)))]
mod manager {
    use super::Seen;
    use super::receipt::Receipt;
    use super::vigil::Vigil;

    /// The same sentence for all three, because it is the same fact.
    const NONE: &str = "this system has no service manager `333 service` knows how to ask. \
                        `333 serve --plain` keeps the vigil under whatever keeps programs \
                        running here.";

    pub(crate) fn install(_: &Vigil) -> anyhow::Result<Receipt> {
        anyhow::bail!(NONE)
    }

    pub(crate) fn uninstall(_: Option<&Receipt>) -> anyhow::Result<()> {
        anyhow::bail!(NONE)
    }

    pub(crate) fn status(_: Option<&Receipt>) -> Seen {
        Seen {
            state: "not installed: there is no service manager here this knows".to_owned(),
            log: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
