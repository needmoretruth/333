//! Running the system's own programs, and saying so.
//!
//! Installing a service is a handful of commands a person could have typed. Every one
//! of them that changes something is said out loud as it is run, in the form they
//! would type it, so that what was done to their system is on their screen and can be
//! done again, or undone, by hand.

use std::process::Command;

use anyhow::bail;

/// Run a program that changes something, say that it ran, and hand back what it said.
///
/// # Errors
/// Fails, with the first thing it said on its error stream, if it could not be run or
/// did not succeed.
pub(crate) fn run_aloud(program: &str, args: &[&str]) -> anyhow::Result<String> {
    let typed = typed(program, args);
    match outcome(program, args) {
        Ok(said) => {
            crate::commands::service::say(&words!("service-programs-ran", command = &typed))?;
            Ok(said)
        }
        Err(why) => bail!(words!(
            "service-programs-did-not-succeed",
            command = typed,
            why = why
        )),
    }
}

/// Run a program that only reports, and hand back what it said if it succeeded.
///
/// Nothing is said out loud: asking a service manager what it is doing changes
/// nothing, and a line for it would be a line for nothing.
#[must_use]
pub(crate) fn ask(program: &str, args: &[&str]) -> Option<String> {
    outcome(program, args).ok()
}

/// Run a program quietly: what it printed if it succeeded, or why it did not.
///
/// # Errors
/// The first line it wrote on its error stream, or why it could not be started.
pub(crate) fn outcome(program: &str, args: &[&str]) -> Result<String, String> {
    match Command::new(program).args(args).output() {
        Ok(output) if output.status.success() => {
            Ok(String::from_utf8_lossy(&output.stdout).into_owned())
        }
        Ok(output) => {
            let said = String::from_utf8_lossy(&output.stderr);
            let first = said.lines().find(|line| !line.trim().is_empty());
            Err(first.map_or_else(
                || {
                    words!(
                        "service-programs-ended-with",
                        status = output.status.to_string()
                    )
                },
                |line| line.trim().to_owned(),
            ))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Err(words!("service-programs-no-such", program = program))
        }
        Err(e) => Err(words!(
            "service-programs-not-started",
            program = program,
            why = e.to_string()
        )),
    }
}

/// A command as a person would type it, quoting only what needs quoting.
#[must_use]
pub(crate) fn typed(program: &str, args: &[&str]) -> String {
    let mut typed = program.to_owned();
    for arg in args {
        typed.push(' ');
        if arg.is_empty() || arg.contains(|c: char| c.is_whitespace() || "'\"\\$`".contains(c)) {
            typed.push('\'');
            typed.push_str(&arg.replace('\'', r"'\''"));
            typed.push('\'');
        } else {
            typed.push_str(arg);
        }
    }
    typed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let pairs = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            [
                (
                    words!("service-programs-ran", command = "loginctl enable-linger"),
                    "ran      loginctl enable-linger",
                ),
                (
                    words!(
                        "service-programs-did-not-succeed",
                        command = "loginctl enable-linger",
                        why = "no"
                    ),
                    "`loginctl enable-linger` did not succeed: no",
                ),
                (
                    words!("service-programs-ended-with", status = "exit status: 1"),
                    "it ended with exit status: 1",
                ),
                (
                    words!(
                        "service-programs-not-started",
                        program = "id",
                        why = "denied"
                    ),
                    "id could not be started: denied",
                ),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

    #[test]
    fn a_command_is_shown_the_way_it_would_be_typed() {
        assert_eq!(
            typed("systemctl", &["--user", "enable", "333.service"]),
            "systemctl --user enable 333.service"
        );
        assert_eq!(
            typed("schtasks", &["/TN", "333 vigil"]),
            "schtasks /TN '333 vigil'"
        );
    }

    #[test]
    fn a_program_that_is_not_there_is_said_to_be_not_there() {
        let why = outcome("333-there-is-no-such-program", &[]).unwrap_err();
        assert_eq!(
            why,
            "there is no 333-there-is-no-such-program on this system"
        );
    }
}
