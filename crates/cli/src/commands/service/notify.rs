//! Saying something on this machine's desktop, through whatever the system itself uses.
//!
//! Nothing is bundled and nothing is installed for it: the system's own notifier is
//! run as a program, if there is one, and if there is none, or it fails, nothing
//! happens here. The same line always goes to the service's log as well, which is
//! the place that does not depend on anybody's desktop.

use std::process::{Command, Stdio};

/// The title every notification carries.
const TITLE: &str = "333";

/// Raise `line` as a desktop notification, if this system has a way to.
pub(crate) fn raise(line: &str) {
    let text = plain(line);
    for mut attempt in notifiers(&text) {
        let shown = attempt
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok_and(|status| status.success());
        if shown {
            return;
        }
    }
}

/// A printed line as one sentence: the keyword column and the line breaks are for a
/// terminal, and a notification is not one.
fn plain(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The ways this system raises a notification, in the order to try them.
#[cfg(target_os = "macos")]
fn notifiers(text: &str) -> Vec<Command> {
    // The text is handed over as an argument rather than written into the script, so
    // that nothing in it can be read as AppleScript.
    let mut osascript = Command::new("osascript");
    osascript.args([
        "-e",
        "on run argv",
        "-e",
        &format!("display notification (item 1 of argv) with title \"{TITLE}\""),
        "-e",
        "end run",
        text,
    ]);
    vec![osascript]
}

/// The ways this system raises a notification, in the order to try them.
#[cfg(windows)]
fn notifiers(text: &str) -> Vec<Command> {
    // The text travels in an environment variable rather than in the script, so that
    // nothing in it can be read as PowerShell. The application id is PowerShell's own:
    // Windows shows a toast only for an application it knows.
    let script = "[Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime] > $null; \
                  $toast = [Windows.UI.Notifications.ToastNotificationManager]::GetTemplateContent([Windows.UI.Notifications.ToastTemplateType]::ToastText02); \
                  $lines = $toast.GetElementsByTagName('text'); \
                  $lines.Item(0).AppendChild($toast.CreateTextNode($env:N333_TITLE)) > $null; \
                  $lines.Item(1).AppendChild($toast.CreateTextNode($env:N333_LINE)) > $null; \
                  [Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier('{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe').Show([Windows.UI.Notifications.ToastNotification]::new($toast))";
    let mut toast = Command::new("powershell");
    toast
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .env("N333_TITLE", TITLE)
        .env("N333_LINE", text);
    let mut msg = Command::new("msg");
    msg.args([
        std::env::var("USERNAME").unwrap_or_else(|_| "*".to_owned()),
        format!("{TITLE}: {text}"),
    ]);
    vec![toast, msg]
}

/// The ways this system raises a notification, in the order to try them.
#[cfg(not(any(target_os = "macos", windows)))]
fn notifiers(text: &str) -> Vec<Command> {
    let mut send = Command::new("notify-send");
    // `--` so that a line beginning with a dash is a line and not an option.
    send.args(["--app-name", TITLE, "--", TITLE, text]);
    vec![send]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_notification_is_the_line_as_one_sentence() {
        assert_eq!(
            plain("vigil    not kept since 2026-09-22T03:10:00Z,\n         12 epochs ago."),
            "vigil not kept since 2026-09-22T03:10:00Z, 12 epochs ago."
        );
    }
}
