//! What this node says while it works, and where that goes.
//!
//! Every line the client says while it is keeping the vigil comes through here. With
//! no screen it goes to standard output one line at a time, which is what a machine
//! running this as a service wants and what a pipe can read. With the screen up it
//! goes to the screen's own pane instead: a terminal cannot hold a drawing and a
//! stream of lines at once, and the lines are the more interesting half.
//!
//! WHY THIS IS A GLOBAL. The alternative is threading a sink through every function
//! that has anything to say — the door, the hours, the answering — which puts a
//! parameter about presentation into code that is about the protocol. One place is
//! allowed to know, and this is it.
//!
//! NOTHING IS EVER DROPPED SILENTLY WHILE ANYBODY IS LISTENING. The channel has no
//! bound, because the alternative is a node whose work waits on a screen being drawn.
//! When the screen is gone the send fails and the line is lost, which is correct: the
//! screen is gone.
//!
//! LAID OUT FOR WHERE IT IS READ. A line said to a terminal is laid out again for that
//! terminal's width, so a sentence the catalog broke for its own file does not break
//! mid-sentence on a terminal narrower or wider than that; a line said anywhere else —
//! a pipe, a file, a service manager's log — goes as it was said, because whatever
//! reads it there has no width, and a log is read by searching it.
//!
//! WHOEVER ASKED HEARS THE ANSWER. An order handed over from another terminal is
//! carried out on a task that knows who asked, and every line said on that task goes
//! to them as well as to the vigil. Only those lines: the person who typed `333 say 7`
//! wants to know what became of it, not what the vigil said about everything else in
//! the same second.

use std::future::Future;
use std::sync::OnceLock;

use tokio::sync::mpsc::UnboundedSender;
#[cfg(feature = "screen")]
use tokio::sync::mpsc::{UnboundedReceiver, unbounded_channel};

/// Where the lines go once a screen has asked for them.
static SCREEN: OnceLock<UnboundedSender<String>> = OnceLock::new();

tokio::task_local! {
    /// Whoever handed over the order this task is carrying out.
    static ASKER: UnboundedSender<String>;
}

/// Say one thing out loud.
pub(crate) fn say(line: std::fmt::Arguments<'_>) {
    let line = line.to_string();
    // A send that fails is somebody who stopped waiting. The order goes on; they will
    // not hear how it went, which is what hanging up means.
    let _ = ASKER.try_with(|asker| asker.send(line.clone()));
    match SCREEN.get() {
        // Sent whole, newlines and all: what is said in several lines is one thing
        // said, and the screen is the place that knows how to lay one thing out.
        Some(screen) => {
            let _ = screen.send(line);
        }
        None => printed(&line),
    }
}

/// Write one thing said straight to standard output, laid out for the terminal when
/// it is one.
///
/// For what is said once the screen has given the terminal back, as well as for
/// everything said when there never was a screen.
pub(crate) fn printed(line: &str) {
    let out = std::io::stdout();
    let width = terminal_size::terminal_size_of(&out).map(|(width, _)| usize::from(width.0));
    aloud_to(&mut out.lock(), &laid_for(line, width));
}

/// A line as it is written where it is read: laid out again for a terminal of this
/// width, or as it was said where there is no terminal.
fn laid_for(line: &str, width: Option<usize>) -> std::borrow::Cow<'_, str> {
    match width {
        Some(width) => crate::words::layout::fold(line, width).join("\n").into(),
        None => line.into(),
    }
}

/// Say a line that is already in words: one made by [`words!`](crate::words), or a
/// formula that is the same in every language.
pub(crate) fn line(line: &str) {
    say(format_args!("{line}"));
}

/// Write one line where there is no screen, and drop it if nobody reads there.
///
/// `333 serve --plain | head` closes the vigil's standard output after ten lines, and a
/// vigil is not a reader's to end: it goes on answering, and what it says goes nowhere.
/// `println!` would panic at the next line, inside whichever task said it. Any other
/// failure to write is dropped for the same reason, because there is nowhere left to
/// say it.
fn aloud_to(out: &mut impl std::io::Write, line: &str) {
    let _ = writeln!(out, "{line}");
}

/// Carry out `work` so that everything it says also reaches `asker`.
pub(crate) async fn heard_by<F: Future>(asker: UnboundedSender<String>, work: F) -> F::Output {
    ASKER.scope(asker, work).await
}

/// Whoever asked for what this task is doing, if it was asked from elsewhere.
///
/// For work that goes on in a task of its own and should still be heard by them.
pub(crate) fn the_asker() -> Option<UnboundedSender<String>> {
    ASKER.try_with(Clone::clone).ok()
}

/// Take everything said from here on, instead of printing it.
///
/// Only a build with a screen in it has anywhere else to put them.
///
/// Once, for the life of the process. A second screen would be a second thing to
/// draw on one terminal.
#[cfg(feature = "screen")]
pub(crate) fn into_screen() -> Option<UnboundedReceiver<String>> {
    let (sender, receiver) = unbounded_channel();
    SCREEN.set(sender).ok().map(|()| receiver)
}

/// Say one thing out loud, written the way `println!` is.
///
/// Nothing calls it: every line this client says has its words in a catalog, and
/// `words::checks` fails on a call. It stays so that a line written this way on
/// another branch still builds and is caught by that check, which says what to do.
#[macro_export]
macro_rules! aloud {
    ($($arg:tt)*) => { $crate::aloud::say(format_args!($($arg)*)) };
}

/// Where the libraries under this client say their own lines.
///
/// Not a second stream. A node with a screen up has one terminal, and a warning from
/// deep inside Tor printed straight to it lands in the middle of a drawing — so it
/// goes through the same voice as everything else and lands in the same pane.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Voice;

impl std::io::Write for Voice {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        say(format_args!("{}", String::from_utf8_lossy(buf).trim_end()));
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl tracing_subscriber::fmt::MakeWriter<'_> for Voice {
    type Writer = Self;

    fn make_writer(&self) -> Self::Writer {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Standard output after its reader has gone.
    struct Closed;

    impl std::io::Write for Closed {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::ErrorKind::BrokenPipe.into())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Err(std::io::ErrorKind::BrokenPipe.into())
        }
    }

    #[test]
    fn a_line_said_to_a_terminal_is_laid_out_for_it_and_anywhere_else_is_not() {
        let said = "hand     an invitation names a place, not a person. it swears to nothing;\n\
                    \x20        whoever answers there proves themselves by holding a key.";
        assert_eq!(
            laid_for(said, None),
            said,
            "a log keeps the lines as they were said"
        );
        assert_eq!(
            laid_for(said, Some(48)),
            "hand     an invitation names a place, not a\n\
             \x20        person. it swears to nothing; whoever\n\
             \x20        answers there proves themselves by\n\
             \x20        holding a key."
        );
    }

    #[test]
    fn a_line_nobody_reads_is_dropped_and_the_vigil_goes_on() {
        aloud_to(&mut Closed, "answered 333abc");
        aloud_to(&mut Closed, "and the next one");
    }
}
