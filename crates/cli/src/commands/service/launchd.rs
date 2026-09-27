//! launchd: a launch agent that keeps the vigil, and a second that asks every hour
//! whether it is being kept.
//!
//! The agent is `packaging/dev.the333.vigil.plist` with its program, its arguments and
//! its log written for this machine. That file stays the one a person reads and copies
//! by hand; this is the same file, filled in.
//!
//! Compiled on every system so that what it writes is tested on every system; only
//! macOS runs it.
#![cfg_attr(not(target_os = "macos"), allow(dead_code))]

use std::path::{Path, PathBuf};

use anyhow::Context as _;

use super::programs::{ask, outcome, run_aloud};
use super::receipt::Receipt;
use super::vigil::Vigil;
use super::{Seen, say};

/// The documented agent, which the installed one is filled in from.
const CANONICAL: &str = include_str!("../../../../../packaging/dev.the333.vigil.plist");

/// The vigil's label, which is also its file's name.
const LABEL: &str = "dev.the333.vigil";
/// The check's.
const CHECK_LABEL: &str = "dev.the333.vigil.check";

/// Where every file install writes begins, and how uninstall knows the file is its own.
const MARK: &str = "<!-- Written by `333 service install`. `333 service uninstall` stops the vigil\n     \
                    and removes this file and the one beside it that checks on it. -->";

/// The agent: the documented one, with its program, arguments and log written in.
#[must_use]
pub(crate) fn agent(exe: &Path, args: &[String], log: &Path) -> String {
    let body = CANONICAL
        .find("<plist")
        .and_then(|at| CANONICAL.get(at..))
        .unwrap_or(CANONICAL);
    let mut agent = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{MARK}\n");
    let mut in_arguments = false;
    let mut after_key = false;
    for line in body.lines() {
        if after_key && line.trim() == "<array>" {
            in_arguments = true;
        }
        after_key = line.trim() == "<key>ProgramArguments</key>";
        if in_arguments && line.trim().starts_with("<string>") {
            continue;
        }
        if in_arguments && line.trim() == "</array>" {
            in_arguments = false;
            agent.push_str(&strings(exe, args));
        }
        let line = line.replace(
            "/Users/USERNAME/Library/Logs/333.log",
            &escaped(&log.display()),
        );
        agent.push_str(&line);
        agent.push('\n');
    }
    agent
}

/// The agent that asks, every hour, whether the vigil is being kept.
#[must_use]
pub(crate) fn check_agent(exe: &Path, args: &[String], log: &Path) -> String {
    let log = escaped(&log.display());
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         {MARK}\n\
         <plist version=\"1.0\">\n\
         <dict>\n\
         \x20 <key>Label</key>\n\
         \x20 <string>{CHECK_LABEL}</string>\n\
         \x20 <key>ProgramArguments</key>\n\
         \x20 <array>\n\
         {}\
         \x20 </array>\n\
         \x20 <key>StartInterval</key>\n\
         \x20 <integer>3600</integer>\n\
         \x20 <key>StandardOutPath</key>\n\
         \x20 <string>{log}</string>\n\
         \x20 <key>StandardErrorPath</key>\n\
         \x20 <string>{log}</string>\n\
         </dict>\n\
         </plist>\n",
        strings(exe, args)
    )
}

/// A program and its arguments as the lines of a plist array.
fn strings(exe: &Path, args: &[String]) -> String {
    std::iter::once(exe.display().to_string())
        .chain(args.iter().cloned())
        .map(|arg| format!("    <string>{}</string>\n", escaped(&arg)))
        .collect()
}

/// Text as XML carries it.
fn escaped(text: &(impl std::fmt::Display + ?Sized)) -> String {
    text.to_string()
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// This user's home, and their uid as launchd names their session.
fn home_and_uid() -> anyhow::Result<(PathBuf, String)> {
    let home = directories::BaseDirs::new()
        .map(|dirs| dirs.home_dir().to_path_buf())
        .context("this system names no home directory for this user")?;
    let uid = ask("id", &["-u"])
        .map(|uid| uid.trim().to_owned())
        .context("asking `id -u` which user this is")?;
    Ok((home, uid))
}

/// Write both agents and start them in this user's session.
///
/// # Errors
/// Fails if an agent cannot be written, or launchd will not take it.
pub(crate) fn install(vigil: &Vigil) -> anyhow::Result<Receipt> {
    let (home, uid) = home_and_uid()?;
    let agents = home.join("Library").join("LaunchAgents");
    let logs = home.join("Library").join("Logs");
    for dir in [&agents, &logs] {
        std::fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
    }
    let log = logs.join("333.log");
    let files = [
        (LABEL, agent(&vigil.exe, &vigil.serve, &log)),
        (CHECK_LABEL, check_agent(&vigil.exe, &vigil.check, &log)),
    ];
    let mut wrote = Vec::new();
    for (label, text) in files {
        let path = agents.join(format!("{label}.plist"));
        std::fs::write(&path, text).with_context(|| format!("writing {}", path.display()))?;
        say(format_args!("wrote    {}", path.display()))?;
        // An earlier install of the same agent is still loaded, and launchd refuses to
        // load a label twice. Quietly, because there is usually nothing to unload.
        let _ = outcome("launchctl", &["bootout", &format!("gui/{uid}/{label}")]);
        run_aloud("launchctl", &["enable", &format!("gui/{uid}/{label}")])?;
        run_aloud(
            "launchctl",
            &[
                "bootstrap",
                &format!("gui/{uid}"),
                &path.display().to_string(),
            ],
        )?;
        wrote.push(path);
    }
    say(format_args!(
        "login    launchd keeps the vigil from the moment you log in until you log out,\n\
         \x20        and after a restart it begins again when you log in. What it says is in\n\
         \x20        {}.",
        log.display()
    ))?;
    Ok(Receipt {
        node: vigil.node.clone(),
        wrote,
        tasks: Vec::new(),
        log: Some(log),
        linger_turned_on: false,
    })
}

/// Stop both agents and remove what install wrote.
///
/// # Errors
/// Fails if a file install wrote cannot be removed.
pub(crate) fn uninstall(_receipt: Option<&Receipt>) -> anyhow::Result<()> {
    let (home, uid) = home_and_uid()?;
    for label in [CHECK_LABEL, LABEL] {
        let path = home
            .join("Library")
            .join("LaunchAgents")
            .join(format!("{label}.plist"));
        if !path.exists() {
            continue;
        }
        if !std::fs::read_to_string(&path).is_ok_and(|text| text.contains(MARK)) {
            say(format_args!(
                "left     {}. `333 service install` did not write it.",
                path.display()
            ))?;
            continue;
        }
        if let Err(e) = run_aloud("launchctl", &["bootout", &format!("gui/{uid}/{label}")]) {
            say(format_args!("failed   {e:#}"))?;
        }
        std::fs::remove_file(&path).with_context(|| format!("removing {}", path.display()))?;
        say(format_args!("removed  {}", path.display()))?;
    }
    Ok(())
}

/// What launchd says of the vigil, and the last lines of its log.
#[must_use]
pub(crate) fn status(receipt: Option<&Receipt>) -> Seen {
    let state = home_and_uid()
        .ok()
        .and_then(|(_, uid)| ask("launchctl", &["print", &format!("gui/{uid}/{LABEL}")]))
        .map_or_else(|| "not installed".to_owned(), |printed| state(&printed));
    Seen {
        state,
        log: receipt
            .and_then(|receipt| receipt.log.as_deref())
            .map(super::last_lines)
            .unwrap_or_default(),
    }
}

/// What `launchctl print` said, in words.
fn state(printed: &str) -> String {
    let field = |name: &str| {
        printed.lines().find_map(|line| {
            let (key, value) = line.split_once(" = ")?;
            (key.trim() == name).then(|| value.trim().to_owned())
        })
    };
    match (field("state").as_deref(), field("last exit code")) {
        (Some("running"), _) => "running".to_owned(),
        (Some(other), Some(code)) => format!("{other}, and it last ended with {code}"),
        (Some(other), None) => other.to_owned(),
        (None, _) => "loaded, and launchd did not say what it is doing".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_filled_in_agent_runs_this_program_for_this_node_and_logs_where_it_says() {
        let agent = agent(
            Path::new("/Users/some one/bin/333"),
            &[
                "--data-dir".to_owned(),
                "/Users/some one/node & co".to_owned(),
            ],
            Path::new("/Users/some one/Library/Logs/333.log"),
        );
        assert!(agent.contains(
            "  <array>\n    <string>/Users/some one/bin/333</string>\n    \
             <string>--data-dir</string>\n    <string>/Users/some one/node &amp; co</string>\n  \
             </array>\n"
        ));
        assert!(!agent.contains("USERNAME"));
        assert!(agent.contains("<string>/Users/some one/Library/Logs/333.log</string>"));
        assert!(agent.contains("<key>ThrottleInterval</key>\n  <integer>333</integer>"));
        assert!(agent.contains(MARK));
    }

    #[test]
    fn the_check_asks_every_hour_and_says_so_in_the_same_log() {
        let check = check_agent(
            Path::new("/Users/x/bin/333"),
            &["service".to_owned(), "check".to_owned()],
            Path::new("/Users/x/Library/Logs/333.log"),
        );
        assert!(check.contains("<key>StartInterval</key>\n  <integer>3600</integer>"));
        assert!(check.contains("<string>/Users/x/Library/Logs/333.log</string>"));
        assert!(check.contains("<string>check</string>"));
    }

    #[test]
    fn what_launchd_says_is_said_in_words() {
        assert_eq!(state("\tstate = running\n"), "running");
        assert_eq!(
            state("\tstate = not running\n\tlast exit code = 1\n"),
            "not running, and it last ended with 1"
        );
    }
}
