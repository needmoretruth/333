//! Windows: a scheduled task that keeps the vigil from logon, and a second that asks
//! every hour whether it is being kept.
//!
//! A scheduled task rather than a Windows service, because a service that starts at
//! boot runs as an account of its own, and a node lives in one person's directory
//! where that account has no business. The price is that the vigil is kept while this
//! user is logged in, and install says so.
//!
//! Task Scheduler keeps no log of what a program says, so each task runs through
//! `cmd.exe` to append it to a file in the node's directory, which is where `service
//! status` reads it back from.
//!
//! Compiled on every system so that what it writes is tested on every system; only
//! Windows runs it.
#![cfg_attr(not(windows), allow(dead_code))]

use std::path::Path;

use anyhow::{Context as _, bail};

use super::programs::{ask, outcome, run_aloud};
use super::receipt::Receipt;
use super::vigil::Vigil;
use super::{Seen, say};

/// The vigil's task.
const VIGIL_TASK: &str = "333 vigil";
/// The check's.
const CHECK_TASK: &str = "333 check";
/// The file in the node's directory both tasks write what they say into.
const LOG: &str = "vigil.log";

/// What starts a task, and how it is kept going. The rest of a task is the same for both.
struct Schedule<'a> {
    /// What the task is, in Task Scheduler's own list.
    description: &'a str,
    /// How long after logon it first runs.
    delay: &'a str,
    /// How often it is started again. A task still running is left alone.
    every: &'a str,
    /// How long one run may last before Windows ends it. `PT0S` is for ever.
    limit: &'a str,
}

/// The vigil: from logon, for ever, and started again 333 seconds after it stops.
///
/// Restarting is done twice over. `RestartOnFailure` is what Task Scheduler offers
/// and stops after 999 tries; a trigger that repeats every 333 seconds with
/// `IgnoreNew` does nothing while the vigil runs and starts it again when it is not,
/// indefinitely.
#[must_use]
pub(crate) fn vigil_task(user: &str, arguments: &str, directory: &str) -> String {
    let schedule = Schedule {
        description: "Runs the 333 node while you are logged in. `333 stop` stops it; `333 service uninstall` removes it.",
        delay: "PT0S",
        every: "PT5M33S",
        limit: "PT0S",
    };
    task(user, arguments, directory, &schedule).replace(
        "  </Settings>",
        "    <RestartOnFailure>\n      <Interval>PT5M33S</Interval>\n      \
         <Count>999</Count>\n    </RestartOnFailure>\n  </Settings>",
    )
}

/// The check: an hour after logon and every hour after that.
#[must_use]
pub(crate) fn check_task(user: &str, arguments: &str, directory: &str) -> String {
    let schedule = Schedule {
        description: "Asks every hour whether the 333 node is running. `333 service uninstall` removes it.",
        delay: "PT1H",
        every: "PT1H",
        limit: "PT10M",
    };
    task(user, arguments, directory, &schedule)
}

/// One task, as Task Scheduler's own XML.
fn task(user: &str, arguments: &str, directory: &str, schedule: &Schedule<'_>) -> String {
    // The fixed values first and the arguments last, so that nothing a person typed is
    // ever read again as a place to fill in.
    TASK.replace("{description}", &escaped(schedule.description))
        .replace("{delay}", schedule.delay)
        .replace("{every}", schedule.every)
        .replace("{limit}", schedule.limit)
        .replace("{user}", &escaped(user))
        .replace("{directory}", &escaped(directory))
        .replace("{arguments}", &escaped(arguments))
}

/// A task, with a name in braces wherever [`task`] puts something in.
const TASK: &str = r#"<?xml version="1.0" encoding="UTF-16"?>
<Task version="1.2" xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">
  <RegistrationInfo>
    <Description>{description}</Description>
  </RegistrationInfo>
  <Triggers>
    <LogonTrigger>
      <Enabled>true</Enabled>
      <UserId>{user}</UserId>
      <Delay>{delay}</Delay>
      <Repetition>
        <Interval>{every}</Interval>
        <StopAtDurationEnd>false</StopAtDurationEnd>
      </Repetition>
    </LogonTrigger>
  </Triggers>
  <Principals>
    <Principal id="Author">
      <UserId>{user}</UserId>
      <LogonType>InteractiveToken</LogonType>
      <RunLevel>LeastPrivilege</RunLevel>
    </Principal>
  </Principals>
  <Settings>
    <MultipleInstancesPolicy>IgnoreNew</MultipleInstancesPolicy>
    <DisallowStartIfOnBatteries>false</DisallowStartIfOnBatteries>
    <StopIfGoingOnBatteries>false</StopIfGoingOnBatteries>
    <StartWhenAvailable>true</StartWhenAvailable>
    <RunOnlyIfNetworkAvailable>false</RunOnlyIfNetworkAvailable>
    <IdleSettings>
      <StopOnIdleEnd>false</StopOnIdleEnd>
      <RestartOnIdle>false</RestartOnIdle>
    </IdleSettings>
    <AllowStartOnDemand>true</AllowStartOnDemand>
    <Enabled>true</Enabled>
    <Hidden>false</Hidden>
    <RunOnlyIfIdle>false</RunOnlyIfIdle>
    <WakeToRun>false</WakeToRun>
    <ExecutionTimeLimit>{limit}</ExecutionTimeLimit>
  </Settings>
  <Actions Context="Author">
    <Exec>
      <Command>cmd.exe</Command>
      <Arguments>{arguments}</Arguments>
      <WorkingDirectory>{directory}</WorkingDirectory>
    </Exec>
  </Actions>
</Task>
"#;

/// What `cmd.exe` is given: this program and its arguments, with what it says
/// appended to the log.
///
/// `/s` makes cmd take everything between the outer quotes as it is, which is the one
/// form in which quoted paths inside survive.
///
/// # Errors
/// Fails on an argument cmd cannot carry: a double quote ends the quoting early, and a
/// `%` is expanded as a variable even inside quotes.
pub(crate) fn cmd_arguments(exe: &Path, args: &[String], log: &Path) -> anyhow::Result<String> {
    let words: Vec<String> = std::iter::once(exe.display().to_string())
        .chain(args.iter().cloned())
        .collect();
    if let Some(bad) = words.iter().find(|word| word.contains(['"', '%'])) {
        bail!(words!("service-schtasks-cannot-pass", word = bad));
    }
    let line: Vec<String> = words.iter().map(|word| quoted(word)).collect();
    Ok(format!(
        "/d /s /c \"{} >> {} 2>&1\"",
        line.join(" "),
        quoted(&log.display().to_string())
    ))
}

/// One word, quoted the way a Windows program reads its command line.
///
/// Backslashes are only special before a quote, so a path ending in one has it
/// doubled, or the closing quote would be read as part of the path.
fn quoted(word: &str) -> String {
    if !word.is_empty() && !word.contains(|c: char| c.is_whitespace() || "&|<>^(),;=".contains(c)) {
        return word.to_owned();
    }
    let trailing = word.len() - word.trim_end_matches('\\').len();
    format!("\"{word}{}\"", "\\".repeat(trailing))
}

/// Text as XML carries it.
fn escaped(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// The file Task Scheduler reads: the XML in UTF-16, which is what its own XML
/// declaration says and what it writes when it exports a task.
#[must_use]
pub(crate) fn utf16(xml: &str) -> Vec<u8> {
    std::iter::once(0xFEFF)
        .chain(xml.encode_utf16())
        .flat_map(u16::to_le_bytes)
        .collect()
}

/// Create both tasks and start the vigil.
///
/// # Errors
/// Fails if this user cannot be named, the node directory cannot be made, or Task
/// Scheduler will not take a task.
pub(crate) fn install(vigil: &Vigil) -> anyhow::Result<Receipt> {
    let user = match (std::env::var("USERDOMAIN"), std::env::var("USERNAME")) {
        (Ok(domain), Ok(name)) => format!("{domain}\\{name}"),
        _ => bail!(words!("service-schtasks-no-user")),
    };
    // The log is appended to from the first second, and cmd cannot append to a file in
    // a directory that is not there yet.
    std::fs::create_dir_all(&vigil.node)
        .with_context(|| words!("service-creating", path = vigil.node.display().to_string()))?;
    let log = vigil.node.join(LOG);
    let directory = vigil.node.display().to_string();
    let tasks = [
        (
            VIGIL_TASK,
            vigil_task(
                &user,
                &cmd_arguments(&vigil.exe, &vigil.serve, &log)?,
                &directory,
            ),
        ),
        (
            CHECK_TASK,
            check_task(
                &user,
                &cmd_arguments(&vigil.exe, &vigil.check, &log)?,
                &directory,
            ),
        ),
    ];
    for (name, xml) in &tasks {
        let file = std::env::temp_dir().join(format!("{name}.xml"));
        std::fs::write(&file, utf16(xml))
            .with_context(|| words!("service-writing", path = file.display().to_string()))?;
        let created = run_aloud(
            "schtasks",
            &[
                "/Create",
                "/TN",
                name,
                "/XML",
                &file.display().to_string(),
                "/F",
            ],
        );
        let _ = std::fs::remove_file(&file);
        created?;
    }
    run_aloud("schtasks", &["/Run", "/TN", VIGIL_TASK])?;
    say(&words!(
        "service-schtasks-logon",
        log = log.display().to_string()
    ))?;
    Ok(Receipt {
        node: vigil.node.clone(),
        wrote: Vec::new(),
        tasks: tasks.iter().map(|(name, _)| (*name).to_owned()).collect(),
        log: Some(log),
        linger_turned_on: false,
        stopped: false,
        serve: Vec::new(),
    })
}

/// End the vigil and delete both tasks.
///
/// # Errors
/// Never, as written: a task that will not be deleted is said and stepped over, so
/// that the other one still goes.
pub(crate) fn uninstall(_receipt: Option<&Receipt>) -> anyhow::Result<()> {
    for name in [CHECK_TASK, VIGIL_TASK] {
        if outcome("schtasks", &["/Query", "/TN", name]).is_err() {
            continue;
        }
        let _ = outcome("schtasks", &["/End", "/TN", name]);
        if let Err(e) = run_aloud("schtasks", &["/Delete", "/TN", name, "/F"]) {
            say(&words!("service-failed", why = format!("{e:#}")))?;
        }
    }
    Ok(())
}

/// The node's task's state, as Task Scheduler names it, if there is such a task.
fn task_state() -> Option<String> {
    let asked = format!("(Get-ScheduledTask -TaskName '{VIGIL_TASK}' -ErrorAction Stop).State");
    ask(
        "powershell",
        &["-NoProfile", "-NonInteractive", "-Command", &asked],
    )
}

/// Whether the node's task is running now.
#[must_use]
pub(crate) fn active() -> bool {
    task_state().is_some_and(|state| state.trim() == "Running")
}

/// Run the node's task now and at every logon from now on, and the check with it.
///
/// # Errors
/// Fails if Task Scheduler will not enable or run them.
pub(crate) fn resume() -> anyhow::Result<()> {
    for name in [VIGIL_TASK, CHECK_TASK] {
        run_aloud("schtasks", &["/Change", "/TN", name, "/ENABLE"])?;
    }
    run_aloud("schtasks", &["/Run", "/TN", VIGIL_TASK])?;
    Ok(())
}

/// End the node's task and keep both tasks from starting at logon. The tasks stay.
///
/// # Errors
/// Fails if Task Scheduler will not disable one.
pub(crate) fn pause() -> anyhow::Result<()> {
    // A task that is not running is already ended, and says nothing.
    let _ = run_aloud("schtasks", &["/End", "/TN", VIGIL_TASK]);
    for name in [CHECK_TASK, VIGIL_TASK] {
        run_aloud("schtasks", &["/Change", "/TN", name, "/DISABLE"])?;
    }
    Ok(())
}

/// Nothing to follow: Task Scheduler keeps no lines of its own, only the log file.
#[must_use]
pub(crate) const fn follow() -> Option<anyhow::Result<()>> {
    None
}

/// What Task Scheduler says of the vigil, and the last lines of its log.
#[must_use]
pub(crate) fn status(receipt: Option<&Receipt>) -> Seen {
    let state = task_state();
    Seen {
        state: state.map_or_else(
            || words!("service-not-installed"),
            |state| in_words(state.trim()),
        ),
        log: receipt
            .and_then(|receipt| receipt.log.as_deref())
            .map(super::last_lines)
            .unwrap_or_default(),
    }
}

/// A task's state, in words.
fn in_words(state: &str) -> String {
    match state {
        "Running" => words!("service-running"),
        "Ready" => words!("service-schtasks-ready"),
        "Queued" => words!("service-starting"),
        "Disabled" => words!("service-schtasks-disabled"),
        other => other.to_lowercase(),
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
                    words!("service-schtasks-cannot-pass", word = "a%b"),
                    "`a%b` has a \" or a % in it, which cmd.exe cannot pass on as it is",
                ),
                (
                    words!("service-schtasks-no-user"),
                    "Windows did not say who this user is (%USERDOMAIN% and %USERNAME%)",
                ),
                (
                    words!("service-schtasks-logon", log = r"C:\node\vigil.log"),
                    "logon    Windows runs the node while you are logged in, from the moment you log\n\
                     \x20        in. A service run from boot would need an account of its own, and a\n\
                     \x20        node lives in your own directory.\n\
                     \x20        What it says is in C:\\node\\vigil.log.",
                ),
                (in_words("Running"), "running"),
                (
                    in_words("Ready"),
                    "stopped, and started again within 333 seconds",
                ),
                (in_words("Queued"), "starting"),
                (
                    in_words("Disabled"),
                    "disabled: it does not start again by itself",
                ),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

    #[test]
    fn the_vigil_runs_from_logon_for_ever_on_battery_and_again_after_it_stops() {
        let xml = vigil_task(r"HOUSE\someone", "/d /s /c \"333.exe serve\"", r"C:\node");
        for wanted in [
            "<UserId>HOUSE\\someone</UserId>",
            "<LogonTrigger>",
            "<ExecutionTimeLimit>PT0S</ExecutionTimeLimit>",
            "<DisallowStartIfOnBatteries>false</DisallowStartIfOnBatteries>",
            "<StopIfGoingOnBatteries>false</StopIfGoingOnBatteries>",
            "<StartWhenAvailable>true</StartWhenAvailable>",
            "<MultipleInstancesPolicy>IgnoreNew</MultipleInstancesPolicy>",
            "<Interval>PT5M33S</Interval>\n        <StopAtDurationEnd>",
            "<RestartOnFailure>\n      <Interval>PT5M33S</Interval>",
            "<Arguments>/d /s /c &quot;333.exe serve&quot;</Arguments>",
        ] {
            assert!(xml.contains(wanted), "{wanted} is missing from\n{xml}");
        }
    }

    #[test]
    fn the_check_runs_hourly_and_is_not_kept_going() {
        let xml = check_task(r"HOUSE\someone", "check", r"C:\node");
        assert!(xml.contains("<Delay>PT1H</Delay>"));
        assert!(xml.contains("<Interval>PT1H</Interval>"));
        assert!(xml.contains("<ExecutionTimeLimit>PT10M</ExecutionTimeLimit>"));
        assert!(!xml.contains("RestartOnFailure"));
    }

    #[test]
    fn paths_with_spaces_survive_cmd_and_what_it_says_goes_to_the_log() {
        let arguments = cmd_arguments(
            Path::new(r"C:\Users\some one\333.exe"),
            &[
                "--data-dir".to_owned(),
                r"C:\Users\some one\node\".to_owned(),
            ],
            Path::new(r"C:\Users\some one\node\vigil.log"),
        )
        .unwrap();
        assert_eq!(
            arguments,
            r#"/d /s /c ""C:\Users\some one\333.exe" --data-dir "C:\Users\some one\node\\" >> "C:\Users\some one\node\vigil.log" 2>&1""#
        );
    }

    #[test]
    fn an_argument_cmd_would_rewrite_is_refused() {
        let args = ["--bridge".to_owned(), "cert=50%".to_owned()];
        assert!(cmd_arguments(Path::new("333.exe"), &args, Path::new("log")).is_err());
    }

    #[test]
    fn the_file_is_utf16_with_the_mark_task_scheduler_expects() {
        assert_eq!(
            utf16("<a/>"),
            [0xFF, 0xFE, b'<', 0, b'a', 0, b'/', 0, b'>', 0]
        );
    }
}
