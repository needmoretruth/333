//! The Standard edition's screen: one terminal that shows what this node is doing.
//!
//! A node's work happens on a 333-minute rhythm, which means a client that only prints
//! lines is silent for hours at a time and gives a person nothing to look at while
//! their machine is taking part. This is what the Standard edition is for. It shows
//! the same numbers `333 status` prints and the same lines the vigil says, at once,
//! and it keeps showing them.
//!
//! IT IS NOT A SECOND PROGRAM. It runs inside the node it is drawing, because the
//! files it would otherwise read are being written by that node — one writer, and a
//! second process opening the same append-only files would repair a tail the first one
//! was in the middle of writing.
//!
//! THE SMALLEST EDITION DOES NOT HAVE IT. A machine with no terminal to look at wants
//! the lines, not a drawing, and the Light build is compiled without any of this.

mod draw;
mod keyboard;
mod watch;

use std::io::IsTerminal as _;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use ratatui::crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use tokio::sync::mpsc::UnboundedReceiver;

use n333_core::Epoch;
use n333_core::signal::SIGNAL_COUNT;

use crate::node::Node;
use crate::orders::Order;
use crate::words::Arg;
use keyboard::{Typed, read_keys};
use watch::Watch;

/// How often everything is read off the disk again.
///
/// Not every second. Reading the window is hundreds of files, and a node that spent
/// its time answering its own screen would be a node that answers nothing else.
const READ_AGAIN: Duration = Duration::from_secs(10);

/// How many things the vigil said are kept to scroll back through.
const REMEMBERED: usize = 500;

/// What the person at the keyboard is in the middle of.
pub(super) enum Saying {
    /// Nothing. Watching.
    Nothing,
    /// Typing which of the 333 to say.
    Which(Entry),
    /// Typing anything the terminal can be told, in the terminal's own words.
    Typing(Entry),
    /// Reading every key and every order word, until any key is pressed.
    Keys,
}

/// One thing the vigil said, and the time of day it arrived.
///
/// Kept whole rather than as the lines it came in: how it is broken into lines is the
/// pane's to decide, at whatever width the pane is when it is drawn.
pub(super) struct Heard {
    /// When it arrived, as a clock shows it.
    pub(super) at: String,
    /// What was said, every line of it.
    pub(super) said: String,
}

/// A line being typed, and why the last try at it was refused, if it was.
///
/// The refusal stays beside what was typed, with the typing, so that a person who
/// mistyped one letter mends that letter instead of typing the whole line again.
#[derive(Default)]
pub(super) struct Entry {
    /// What has been typed so far.
    pub(super) typed: String,
    /// Why pressing enter on it did nothing, until the next key changes it.
    pub(super) refused: Option<String>,
}

impl Entry {
    /// One more letter, which makes the old refusal about a different line.
    fn push(&mut self, letter: char) {
        self.typed.push(letter);
        self.refused = None;
    }

    /// One letter fewer.
    fn pop(&mut self) {
        self.typed.pop();
        self.refused = None;
    }
}

/// Is there a terminal here that wants a screen?
///
/// A pipe, a service manager's log and a redirect to a file all want the lines, and
/// drawing a screen into any of them produces a file full of escape codes.
pub(crate) fn wanted() -> bool {
    std::io::stdout().is_terminal()
}

/// Draw until the person leaves, and put the terminal back as it was.
///
/// # Errors
/// Fails if the terminal cannot be taken over or put back.
pub(crate) async fn keep(
    node: Arc<Node>,
    lines: UnboundedReceiver<String>,
    orders: tokio::sync::mpsc::UnboundedSender<Order>,
) -> anyhow::Result<()> {
    let mut terminal = ratatui::try_init()?;
    let watching = draw_until_they_leave(&mut terminal, &node, lines, &orders).await;
    ratatui::try_restore()?;
    watching
}

/// The loop: draw, wait for whichever of the three things happens first, draw again.
async fn draw_until_they_leave(
    terminal: &mut ratatui::DefaultTerminal,
    node: &Arc<Node>,
    mut lines: UnboundedReceiver<String>,
    orders: &tokio::sync::mpsc::UnboundedSender<Order>,
) -> anyhow::Result<()> {
    let (mut keys, reading) = read_keys();
    let mut log: Vec<Heard> = Vec::new();
    let mut saying = Saying::Nothing;
    let mut watch = Watch::of(node, Epoch::now()).await?;
    let mut read_at = Instant::now();

    loop {
        terminal.draw(|frame| draw::everything(frame, &watch, &log, &saying))?;

        tokio::select! {
            line = lines.recv() => match line {
                Some(line) => remember(&mut log, &line),
                // Nothing can say anything any more, which happens only when the node
                // itself has stopped. Leaving the screen up would be drawing a vigil
                // that is not being kept.
                None => break,
            },
            key = keys.recv() => {
                let key = match key {
                    Some(Typed::Key(key)) => key,
                    Some(Typed::Unreadable(why)) => {
                        remember(&mut log, &words!("screen-key-unreadable", why = why));
                        continue;
                    }
                    // Said after the terminal is given back, where it can still be read:
                    // a line in the pane would be gone with the pane.
                    Some(Typed::Gone(why)) => {
                        reading.store(false, Ordering::Relaxed);
                        anyhow::bail!(words!("screen-keyboard-gone", why = why));
                    }
                    None => break,
                };
                if key.kind != KeyEventKind::Release {
                    match pressed(node, &mut saying, orders, key.code, key.modifiers).await {
                        Pressed::Carry => {}
                        // Read again at once: a person who has just said something is
                        // looking for it to appear, and ten seconds of it not being
                        // there reads as it not having worked.
                        Pressed::Said(line) => {
                            remember(&mut log, &line);
                            watch = Watch::of(node, Epoch::now()).await?;
                            read_at = Instant::now();
                        }
                        Pressed::Leave => break,
                    }
                }
            }
            // The countdown in the header moves whether or not anything happens.
            () = tokio::time::sleep(Duration::from_secs(1)) => {}
        }

        if read_at.elapsed() >= READ_AGAIN {
            watch = Watch::of(node, Epoch::now()).await?;
            read_at = Instant::now();
        }
    }

    reading.store(false, Ordering::Relaxed);
    Ok(())
}

/// What a keypress did.
enum Pressed {
    /// Nothing that needs saying.
    Carry,
    /// Something worth a line in the vigil.
    Said(String),
    /// The person is leaving.
    Leave,
}

/// Act on one key.
async fn pressed(
    node: &Arc<Node>,
    saying: &mut Saying,
    orders: &tokio::sync::mpsc::UnboundedSender<Order>,
    key: KeyCode,
    with: KeyModifiers,
) -> Pressed {
    if with.contains(KeyModifiers::CONTROL) && matches!(key, KeyCode::Char('c' | 'C')) {
        return Pressed::Leave;
    }
    match saying {
        Saying::Nothing => match key {
            KeyCode::Char('q' | 'Q') | KeyCode::Esc => Pressed::Leave,
            KeyCode::Char('s' | 'S') => {
                *saying = Saying::Which(Entry::default());
                Pressed::Carry
            }
            // Everything the terminal can be told, told here. The node holding these
            // files is this one, so the same words typed in another terminal are handed
            // to this process too, rather than opening files it is writing.
            KeyCode::Char(':') => {
                *saying = Saying::Typing(Entry::default());
                Pressed::Carry
            }
            KeyCode::Char('?') => {
                *saying = Saying::Keys;
                Pressed::Carry
            }
            _ => Pressed::Carry,
        },
        // Any key at all: a list that wants one particular key to close it is one more
        // thing to learn before the list is any use.
        Saying::Keys => {
            *saying = Saying::Nothing;
            Pressed::Carry
        }
        Saying::Typing(entry) => match key {
            KeyCode::Esc => {
                *saying = Saying::Nothing;
                Pressed::Carry
            }
            KeyCode::Char(letter) => {
                entry.push(letter);
                Pressed::Carry
            }
            KeyCode::Backspace => {
                entry.pop();
                Pressed::Carry
            }
            KeyCode::Enter => match Order::read(&entry.typed) {
                Ok(Order::Leave) => Pressed::Leave,
                // Carried out where the dialler and the listeners are, which is not
                // here: this is a drawing, and a drawing that opened connections would
                // be a second node inside the first.
                Ok(order) => {
                    // Echoed as it was typed rather than as it was understood. A person
                    // who mistyped wants to see what they typed, and the shape of the
                    // thing this program turned it into is not something they asked to
                    // be shown.
                    let said = words!("screen-asked", typed = entry.typed.trim());
                    *saying = Saying::Nothing;
                    if orders.send(order).is_err() {
                        return Pressed::Said(words!("screen-unheard"));
                    }
                    Pressed::Said(said)
                }
                Err(why) => {
                    let why = why.to_string();
                    let said = words!("screen-unread", why = why.as_str());
                    entry.refused = Some(why);
                    Pressed::Said(said)
                }
            },
            _ => Pressed::Carry,
        },
        Saying::Which(entry) => match key {
            KeyCode::Esc => {
                *saying = Saying::Nothing;
                Pressed::Carry
            }
            // Three digits is all there is, in ten or in twelve: the largest of them is
            // 332, or 238. A digit is whatever reads as one in the base this person
            // counts in, so ten and eleven can be typed as well as shown.
            KeyCode::Char(digit)
                if entry.typed.chars().count() < 3
                    && !digit.is_whitespace()
                    && crate::words::count::read(&format!("{}{digit}", entry.typed)).is_some() =>
            {
                entry.push(digit);
                Pressed::Carry
            }
            KeyCode::Backspace => {
                entry.pop();
                Pressed::Carry
            }
            KeyCode::Enter => match crate::words::count::index(&entry.typed)
                .filter(|index| usize::from(*index) < usize::from(SIGNAL_COUNT))
            {
                Some(index) => {
                    let said = say_it(node, index).await;
                    *saying = Saying::Nothing;
                    Pressed::Said(said)
                }
                None => {
                    let why = words!(
                        "say-not-one",
                        count = SIGNAL_COUNT,
                        last = SIGNAL_COUNT - 1,
                        typed = entry.typed.as_str()
                    );
                    let said = words!("screen-refused", why = why.as_str());
                    entry.refused = Some(why);
                    Pressed::Said(said)
                }
            },
            _ => Pressed::Carry,
        },
    }
}

/// Say one of the 333, and say what happened either way.
async fn say_it(node: &Arc<Node>, index: u16) -> String {
    match crate::commands::say::speak(node, index).await {
        // Saying it says its own lines; there is nothing to add here.
        Ok(()) => String::new(),
        Err(e) => crate::failed::not_said(&e),
    }
}

/// Keep what was said, stamped with the time it arrived, and forget the oldest.
///
/// One stamp for all of it. Everything said in more than one line is one thing said,
/// and stamping each line of it would make one sentence look like four things
/// happening at once.
fn remember(log: &mut Vec<Heard>, said: &str) {
    if said.trim().is_empty() {
        return;
    }
    let at = stamp(n333_core::epoch::unix_now_seconds());
    log.push(Heard {
        at,
        said: said.to_owned(),
    });
    if log.len() > REMEMBERED {
        log.drain(..log.len() - REMEMBERED);
    }
}

/// The time of day a line arrived, in UTC, as a clock shows it.
///
/// Written in ten whatever the person counts in: it is the reading of a clock, which
/// names a moment rather than counting anything, and a clock in twelve would read as a
/// wrong time rather than as the same one. The pane's title says it is UTC.
fn stamp(seconds: u64) -> String {
    let clock = |value: u64| Arg::exact(format!("{value:02}"));
    words!(
        "screen-at",
        hours = clock(seconds / 3600 % 24),
        minutes = clock(seconds / 60 % 60),
        seconds = clock(seconds % 60)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::words::count::Base;

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let pairs = crate::words::speaking("en", Base::Ten, || {
            [
                (words!("screen-asked", typed = "ping x"), "asked    ping x"),
                (
                    words!("screen-unheard"),
                    "unheard  nothing is carrying orders out any more",
                ),
                (
                    words!("screen-unread", why = "nothing typed"),
                    "unread   nothing typed",
                ),
                (
                    words!(
                        "screen-refused",
                        why = words!(
                            "say-not-one",
                            count = SIGNAL_COUNT,
                            last = SIGNAL_COUNT - 1,
                            typed = "400"
                        )
                    ),
                    "refused  there are 333 of them, numbered 0 to 332. \"400\" is not one.",
                ),
                (
                    words!("screen-key-unreadable", why = "no terminal"),
                    "keyboard a key could not be read: no terminal. The keys after it may be.",
                ),
                (
                    words!("screen-keyboard-gone", why = "no terminal"),
                    "the keyboard could not be read (no terminal), so the screen has closed and\n\
                     the node with it. `333 run --plain` runs the node with no keyboard.",
                ),
                (stamp(7 * 3600 + 5 * 60), "07:05:00"),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

    #[test]
    fn a_time_of_day_is_the_clock_s_whatever_the_base() {
        let at = 16 * 3600 + 29 * 60 + 51;
        let twelve = crate::words::speaking("ko", Base::Twelve, || stamp(at));
        assert_eq!(twelve, "16:29:51");
    }
}
