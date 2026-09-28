//! systemd: a user unit that keeps the vigil, and a timer that asks every hour whether
//! it is being kept.
//!
//! The unit is `packaging/333.service` with its `ExecStart` line written for this
//! program, this node and these flags. That file stays the one a person reads and
//! copies by hand; this is the same file, filled in.
//!
//! Compiled on every system so that what it writes is tested on every system; only
//! Linux runs it.
#![cfg_attr(not(target_os = "linux"), allow(dead_code))]

use std::path::{Path, PathBuf};

use anyhow::{Context as _, bail};

use super::programs::{ask, outcome, run_aloud};
use super::receipt::Receipt;
use super::vigil::Vigil;
use super::{Seen, say};

/// The documented unit, which the installed one is filled in from.
const CANONICAL: &str = include_str!("../../../../../packaging/333.service");

/// The vigil.
const UNIT: &str = "333.service";
/// What the timer runs.
const CHECK: &str = "333-check.service";
/// What asks, every hour.
const TIMER: &str = "333-check.timer";

/// The first lines of everything install writes, which is also how uninstall knows a
/// file is its own to remove.
const HEADER: &str = "# Written by `333 service install`. `333 service uninstall` stops the vigil\n\
                      # and removes this file and the two beside it that check on it.\n";

/// The unit: the documented one, with its program line written for this vigil.
#[must_use]
pub(crate) fn unit(exec: &str, header: &str) -> String {
    let body = CANONICAL
        .find("[Unit]")
        .and_then(|at| CANONICAL.get(at..))
        .unwrap_or(CANONICAL);
    let mut unit = format!("{header}\n");
    for line in body.lines() {
        if line.starts_with("ExecStart=") {
            unit.push_str(&format!("ExecStart={exec}\n"));
        } else {
            unit.push_str(line);
            unit.push('\n');
        }
    }
    unit
}

/// The one-shot the timer runs.
#[must_use]
pub(crate) fn check_unit(exec: &str) -> String {
    format!(
        "{HEADER}\n\
         [Unit]\n\
         Description=333, asking whether the vigil is being kept\n\
         Documentation=https://the333.dev\n\
         \n\
         [Service]\n\
         Type=oneshot\n\
         ExecStart={exec}\n\
         NoNewPrivileges=yes\n"
    )
}

/// The timer.
#[must_use]
pub(crate) fn timer() -> String {
    format!(
        "{HEADER}\n\
         [Unit]\n\
         Description=333, asking every hour whether the vigil is being kept\n\
         \n\
         [Timer]\n\
         # An hour after it starts and every hour after that. Not at once: the vigil beside\n\
         # it is starting too, and has not yet had a moment to say it is awake.\n\
         OnActiveSec=1h\n\
         OnUnitActiveSec=1h\n\
         \n\
         [Install]\n\
         WantedBy=timers.target\n"
    )
}

/// A program and its arguments as one `ExecStart` value.
#[must_use]
pub(crate) fn exec_line(exe: &Path, args: &[String]) -> String {
    std::iter::once(exe.display().to_string())
        .chain(args.iter().cloned())
        .map(|arg| quoted(&arg))
        .collect::<Vec<_>>()
        .join(" ")
}

/// One word of an `ExecStart` line, escaped the way systemd reads one.
///
/// `%` and `$` are systemd's own substitutions and are doubled to mean themselves; a
/// word with a space in it — a bridge line always has several — is quoted whole.
fn quoted(word: &str) -> String {
    let escaped = word
        .replace('\\', r"\\")
        .replace('"', "\\\"")
        .replace('%', "%%")
        .replace('$', "$$");
    if word.is_empty() || word.contains(|c: char| c.is_whitespace() || "\"'\\;".contains(c)) {
        format!("\"{escaped}\"")
    } else {
        escaped
    }
}

/// Where this user's own units live.
fn unit_directory() -> anyhow::Result<PathBuf> {
    directories::BaseDirs::new()
        .map(|dirs| dirs.config_dir().join("systemd").join("user"))
        .with_context(|| words!("service-systemd-no-configuration"))
}

/// Write the units, start the vigil, and make sure it outlives the session.
///
/// # Errors
/// Fails if systemd has no session for this user, or a unit cannot be written or
/// started.
pub(crate) fn install(vigil: &Vigil) -> anyhow::Result<Receipt> {
    if let Err(why) = outcome("systemctl", &["--user", "show-environment"]) {
        bail!(words!("service-systemd-no-session", why = why));
    }
    let dir = unit_directory()?;
    std::fs::create_dir_all(&dir)
        .with_context(|| words!("service-creating", path = dir.display().to_string()))?;
    let files = [
        (
            dir.join(UNIT),
            unit(&exec_line(&vigil.exe, &vigil.serve), HEADER),
        ),
        (
            dir.join(CHECK),
            check_unit(&exec_line(&vigil.exe, &vigil.check)),
        ),
        (dir.join(TIMER), timer()),
    ];
    for (path, text) in &files {
        let replacing = path.exists() && !ours(path);
        let shown = path.display().to_string();
        std::fs::write(path, text).with_context(|| words!("service-writing", path = &shown))?;
        if replacing {
            say(&words!("service-systemd-wrote-over", path = &shown))?;
        } else {
            say(&words!("service-wrote", path = &shown))?;
        }
    }
    run_aloud("systemctl", &["--user", "daemon-reload"])?;
    run_aloud("systemctl", &["--user", "enable", UNIT, TIMER])?;
    // Restarted rather than started, so that installing again with other flags keeps
    // the vigil with those flags from now rather than from the next boot.
    run_aloud("systemctl", &["--user", "restart", UNIT])?;
    run_aloud("systemctl", &["--user", "start", TIMER])?;
    Ok(Receipt {
        node: vigil.node.clone(),
        wrote: files.into_iter().map(|(path, _)| path).collect(),
        tasks: Vec::new(),
        log: None,
        linger_turned_on: linger_on()?,
    })
}

/// Make sure systemd keeps this user's services after they log out. True if this is
/// what turned it on.
fn linger_on() -> anyhow::Result<bool> {
    let user = std::env::var("USER").unwrap_or_else(|_| "$USER".to_owned());
    let uid = ask("id", &["-u"]).unwrap_or_default();
    let now = ask(
        "loginctl",
        &["show-user", uid.trim(), "-p", "Linger", "--value"],
    );
    if now.as_deref().map(str::trim) == Some("yes") {
        say(&words!("service-systemd-linger-already", user = &user))?;
        return Ok(false);
    }
    match run_aloud("loginctl", &["enable-linger"]) {
        Ok(_) => {
            say(&words!("service-systemd-linger-on", user = &user))?;
            Ok(true)
        }
        Err(e) => {
            say(&words!(
                "service-systemd-linger-not-on",
                why = format!("{e:#}"),
                user = &user
            ))?;
            Ok(false)
        }
    }
}

/// Whether install wrote this file.
fn ours(path: &Path) -> bool {
    std::fs::read_to_string(path).is_ok_and(|text| text.starts_with(HEADER))
}

/// Stop the vigil and the check, remove what install wrote, and put linger back.
///
/// # Errors
/// Fails if a file install wrote cannot be removed.
pub(crate) fn uninstall(receipt: Option<&Receipt>) -> anyhow::Result<()> {
    let dir = unit_directory()?;
    for name in [TIMER, UNIT, CHECK] {
        let path = dir.join(name);
        if !path.exists() {
            continue;
        }
        if !ours(&path) {
            say(&words!("service-left", path = path.display().to_string()))?;
            continue;
        }
        if name != CHECK
            && let Err(e) = run_aloud("systemctl", &["--user", "disable", "--now", name])
        {
            say(&words!("service-failed", why = format!("{e:#}")))?;
        }
        std::fs::remove_file(&path)
            .with_context(|| words!("service-removing", path = path.display().to_string()))?;
        say(&words!(
            "service-removed",
            path = path.display().to_string()
        ))?;
    }
    run_aloud("systemctl", &["--user", "daemon-reload"])?;
    // What systemd remembers of a unit that failed outlives the unit's file. Quietly,
    // because there is nothing to forget when it did not fail.
    let _ = outcome("systemctl", &["--user", "reset-failed", UNIT, CHECK]);
    match receipt {
        Some(receipt) if receipt.linger_turned_on => {
            run_aloud("loginctl", &["disable-linger"])?;
            say(&words!("service-systemd-linger-off"))?;
        }
        Some(_) => say(&words!("service-systemd-linger-left"))?,
        None => {}
    }
    Ok(())
}

/// What systemd says of the vigil, and the last lines it and the check said.
#[must_use]
pub(crate) fn status(_receipt: Option<&Receipt>) -> Seen {
    let shown = ask(
        "systemctl",
        &[
            "--user",
            "show",
            UNIT,
            "-p",
            "LoadState,ActiveState,SubState,UnitFileState,Result",
        ],
    );
    let log = ask(
        "journalctl",
        &[
            "--user",
            "-u",
            UNIT,
            "-u",
            CHECK,
            "-n",
            "20",
            "--no-pager",
            "-o",
            "short-iso",
        ],
    );
    Seen {
        state: shown.map_or_else(
            || words!("service-systemd-not-answering"),
            |shown| state(&shown),
        ),
        log: log
            .map(|log| log.lines().map(str::to_owned).collect())
            .unwrap_or_default(),
    }
}

/// What `systemctl show` said, in words.
fn state(shown: &str) -> String {
    let property = |name: &str| {
        shown
            .lines()
            .find_map(|line| line.strip_prefix(name)?.strip_prefix('='))
            .unwrap_or_default()
    };
    if property("LoadState") == "not-found" {
        return words!("service-not-installed");
    }
    let what = match (property("ActiveState"), property("SubState")) {
        ("active", _) => words!("service-running"),
        ("activating", "auto-restart") => words!("service-systemd-restarting"),
        ("activating", _) => words!("service-starting"),
        ("deactivating", _) => words!("service-systemd-stopping"),
        ("failed", _) => words!("service-systemd-failed", result = property("Result")),
        ("inactive", _) => words!("service-systemd-stopped"),
        (other, sub) => format!("{other} ({sub})"),
    };
    match property("UnitFileState") {
        "enabled" => words!("service-systemd-at-every-boot", state = what),
        "" => what,
        other => words!(
            "service-systemd-not-again",
            state = what,
            file_state = other
        ),
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
                    words!("service-systemd-no-configuration"),
                    "this system names no configuration directory for this user",
                ),
                (
                    words!("service-systemd-no-session", why = "no bus"),
                    "systemd is not keeping a session for this user here (no bus). It starts one \
                     when this user logs in, at the console or over ssh, and not through su or \
                     sudo. Log in as this user and run this again.",
                ),
                (
                    words!("service-systemd-wrote-over", path = "/h/333.service"),
                    "wrote    /h/333.service, in place of the one there",
                ),
                (
                    words!("service-systemd-linger-already", user = "a"),
                    "linger   already on for a. It is what keeps the vigil running after you log\n\
                     \x20        out, and starts it at boot with nobody logged in.",
                ),
                (
                    words!("service-systemd-linger-on", user = "a"),
                    "linger   on for a. It is what keeps the vigil running after you log out,\n\
                     \x20        and starts it at boot with nobody logged in.",
                ),
                (
                    words!("service-systemd-linger-not-on", why = "denied", user = "a"),
                    "linger   not on: denied. Without it the vigil stops when you log out and waits\n\
                     \x20        for you to log in again after a reboot. `sudo loginctl enable-linger\n\
                     \x20        a` turns it on.",
                ),
                (
                    words!("service-systemd-linger-off"),
                    "linger   off again, as it was before install.",
                ),
                (
                    words!("service-systemd-linger-left"),
                    "linger   left as it was. Install did not turn it on.",
                ),
                (
                    words!("service-systemd-not-answering"),
                    "unknown: systemd is not answering for this user",
                ),
                (
                    state("ActiveState=active\nUnitFileState=enabled\n"),
                    "running, and started at every boot",
                ),
                (
                    state("ActiveState=activating\nSubState=start\n"),
                    "starting",
                ),
                (state("ActiveState=deactivating\n"), "stopping"),
                (
                    state("ActiveState=failed\nResult=exit-code\n"),
                    "failed (exit-code)",
                ),
                (state("ActiveState=inactive\n"), "stopped"),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

    const PACKAGED: &str = include_str!("../../../../../packaging/package/333.service");

    /// The header the package's copy of the unit carries instead of install's.
    fn package_header() -> &'static str {
        PACKAGED.get(..PACKAGED.find("\n[Unit]").unwrap()).unwrap()
    }

    #[test]
    fn the_filled_in_unit_is_the_documented_one_with_its_program_line_written() {
        let exec = "%h/.local/bin/333 serve --plain";
        let body = CANONICAL.get(CANONICAL.find("[Unit]").unwrap()..).unwrap();
        assert_eq!(unit(exec, HEADER), format!("{HEADER}\n{body}"));
    }

    #[test]
    fn the_packaged_unit_is_the_documented_one_run_from_usr_bin() {
        // Kept as a file because a package can only carry files, and pinned here so
        // that a change to the documented unit cannot leave the packaged one behind.
        let exec = exec_line(
            Path::new("/usr/bin/333"),
            &["serve".to_owned(), "--plain".to_owned()],
        );
        assert_eq!(exec, "/usr/bin/333 serve --plain");
        assert_eq!(unit(&exec, package_header()), PACKAGED);
    }

    #[test]
    fn nothing_in_the_unit_puts_the_vigil_in_a_namespace_of_its_own() {
        // In a user unit each of these runs the process where / belongs to nobody, and
        // a node refuses a directory whose ancestry it cannot trust: the unit that had
        // them never got as far as opening a socket.
        for making_a_namespace in [
            "PrivateTmp=",
            "ProtectSystem=",
            "ProtectHome=",
            "PrivateUsers=",
            "PrivateDevices=",
        ] {
            let found = CANONICAL
                .lines()
                .any(|line| line.starts_with(making_a_namespace));
            assert!(!found, "{making_a_namespace} is in the unit");
        }
    }

    #[test]
    fn a_bridge_line_and_a_path_with_a_space_each_stay_one_word() {
        let exec = exec_line(
            Path::new("/home/some one/bin/333"),
            &[
                "--bridge".to_owned(),
                "obfs4 192.0.2.1:443 cert=a%b iat-mode=0".to_owned(),
            ],
        );
        assert_eq!(
            exec,
            "\"/home/some one/bin/333\" --bridge \"obfs4 192.0.2.1:443 cert=a%%b iat-mode=0\""
        );
    }

    #[test]
    fn the_check_runs_hourly_and_not_at_once() {
        let timer = timer();
        assert!(timer.contains("\nOnActiveSec=1h\n"));
        assert!(timer.contains("\nOnUnitActiveSec=1h\n"));
        assert!(check_unit("/usr/bin/333 service check").contains("\nType=oneshot\n"));
    }

    #[test]
    fn what_systemd_says_is_said_in_words() {
        let shown = "LoadState=loaded\nActiveState=activating\nSubState=auto-restart\n\
                     UnitFileState=enabled\nResult=exit-code\n";
        assert_eq!(
            state(shown),
            "stopped, and starting again 333 seconds after it stopped, and started at every boot"
        );
        assert_eq!(
            state("LoadState=not-found\nActiveState=inactive\n"),
            "not installed"
        );
        assert_eq!(
            state("LoadState=loaded\nActiveState=inactive\nUnitFileState=disabled\n"),
            "stopped, and disabled: it does not start again by itself"
        );
    }
}
