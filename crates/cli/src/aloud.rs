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
        None => println!("{line}"),
    }
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
