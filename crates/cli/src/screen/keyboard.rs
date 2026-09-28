//! Reading the keyboard on a thread of its own, kept apart because it is the one part
//! of the screen that blocks, and the one that has to be stopped by hand.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use ratatui::crossterm::event::{self, Event};
use tokio::sync::mpsc::{UnboundedReceiver, unbounded_channel};

/// How long the key reader waits before looking again at whether it should stop.
const KEY_POLL: Duration = Duration::from_millis(120);

/// What the keyboard gave, or why it gave nothing.
pub(super) enum Typed {
    /// A key.
    Key(event::KeyEvent),
    /// One key could not be read. The next may be.
    Unreadable(String),
    /// The keyboard cannot be read any more.
    Gone(String),
}

/// How many keys in a row may fail to be read before the keyboard is taken to be gone.
///
/// Three: once is a key, twice may be chance, and a reader that went on past that
/// would fill the pane with the same line as fast as it could fail.
const UNREADABLE_IN_A_ROW: usize = 3;

/// Read the keyboard on a thread of its own, because reading it blocks.
///
/// The flag is how it is stopped: a thread left polling a terminal after this program
/// has finished with it eats the keystrokes meant for whatever runs next. A key that
/// could not be read is said rather than skipped, so that a keyboard which has stopped
/// working is not mistaken for a person who has stopped typing.
pub(super) fn read_keys() -> (UnboundedReceiver<Typed>, Arc<AtomicBool>) {
    let (sender, receiver) = unbounded_channel();
    let reading = Arc::new(AtomicBool::new(true));
    let stop = Arc::clone(&reading);
    std::thread::spawn(move || {
        let mut failed = 0;
        while stop.load(Ordering::Relaxed) {
            let typed = match event::poll(KEY_POLL) {
                Ok(true) => match event::read() {
                    Ok(Event::Key(key)) => Typed::Key(key),
                    Ok(_) => continue,
                    Err(e) if failed + 1 < UNREADABLE_IN_A_ROW => {
                        failed += 1;
                        Typed::Unreadable(e.to_string())
                    }
                    Err(e) => Typed::Gone(e.to_string()),
                },
                Ok(false) => continue,
                Err(e) => Typed::Gone(e.to_string()),
            };
            if matches!(typed, Typed::Key(_)) {
                failed = 0;
            }
            let gone = matches!(typed, Typed::Gone(_));
            if sender.send(typed).is_err() || gone {
                return;
            }
        }
    });
    (receiver, reading)
}
