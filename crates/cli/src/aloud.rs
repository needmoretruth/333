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

use std::sync::OnceLock;

#[cfg(feature = "screen")]
use tokio::sync::mpsc::{UnboundedReceiver, unbounded_channel};
use tokio::sync::mpsc::UnboundedSender;

/// Where the lines go once a screen has asked for them.
static SCREEN: OnceLock<UnboundedSender<String>> = OnceLock::new();

/// Say one thing out loud.
pub(crate) fn say(line: std::fmt::Arguments<'_>) {
    match SCREEN.get() {
        // Sent whole, newlines and all: what is said in several lines is one thing
        // said, and the screen is the place that knows how to lay one thing out.
        Some(screen) => {
            let _ = screen.send(line.to_string());
        }
        None => println!("{line}"),
    }
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
/// It exists so that the shape of the call at the hundred places that have something
/// to say is the shape everybody already knows.
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
