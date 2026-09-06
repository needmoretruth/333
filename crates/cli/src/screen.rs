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
mod watch;

use std::io::IsTerminal as _;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use tokio::sync::mpsc::{UnboundedReceiver, unbounded_channel};

use n333_core::Epoch;
use n333_core::signal::SIGNAL_COUNT;

use crate::node::Node;
use watch::Watch;

/// How often everything is read off the disk again.
///
/// Not every second. Reading the window is hundreds of files, and a node that spent
/// its time answering its own screen would be a node that answers nothing else.
const READ_AGAIN: Duration = Duration::from_secs(10);

/// How many lines of the vigil are kept to scroll back through.
const REMEMBERED: usize = 500;

/// How long the key reader waits before looking again at whether it should stop.
const KEY_POLL: Duration = Duration::from_millis(120);

/// What the person at the keyboard is in the middle of.
pub(super) enum Saying {
    /// Nothing. Watching.
    Nothing,
    /// Typing which of the 333 to say.
    Which(String),
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
pub(crate) async fn keep(node: Arc<Node>, lines: UnboundedReceiver<String>) -> anyhow::Result<()> {
    let mut terminal = ratatui::try_init()?;
    let watching = draw_until_they_leave(&mut terminal, &node, lines).await;
    ratatui::try_restore()?;
    watching
}

/// The loop: draw, wait for whichever of the three things happens first, draw again.
async fn draw_until_they_leave(
    terminal: &mut ratatui::DefaultTerminal,
    node: &Arc<Node>,
    mut lines: UnboundedReceiver<String>,
) -> anyhow::Result<()> {
    let (mut keys, reading) = read_keys();
    let mut log: Vec<String> = Vec::new();
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
                let Some(key) = key else { break };
                if key.kind != KeyEventKind::Release {
                    match pressed(node, &mut saying, key.code, key.modifiers).await {
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
                *saying = Saying::Which(String::new());
                Pressed::Carry
            }
            _ => Pressed::Carry,
        },
        Saying::Which(typed) => match key {
            KeyCode::Esc => {
                *saying = Saying::Nothing;
                Pressed::Carry
            }
            // Three digits is all there is: the largest of them is 332.
            KeyCode::Char(digit @ '0'..='9') if typed.len() < 3 => {
                typed.push(digit);
                Pressed::Carry
            }
            KeyCode::Backspace => {
                typed.pop();
                Pressed::Carry
            }
            KeyCode::Enter => {
                let said = say_it(node, typed).await;
                *saying = Saying::Nothing;
                Pressed::Said(said)
            }
            _ => Pressed::Carry,
        },
    }
}

/// Say one of the 333, and say what happened either way.
async fn say_it(node: &Arc<Node>, typed: &str) -> String {
    let Ok(index) = typed.parse::<u16>() else {
        return format!("refused  there are {SIGNAL_COUNT} of them, numbered 0 to {}. \"{typed}\" is not one.", SIGNAL_COUNT - 1);
    };
    match crate::commands::say::speak(node, index).await {
        // Saying it says its own lines; there is nothing to add here.
        Ok(()) => String::new(),
        Err(e) => format!("refused  {e:#}"),
    }
}

/// Keep what was said, stamped with the hour it arrived, and forget the oldest.
///
/// The hour is on the first line only. Everything said in more than one line is one
/// thing said, and stamping each line of it would make one sentence look like four
/// things happening at once.
fn remember(log: &mut Vec<String>, said: &str) {
    let mut lines = said.split('\n').filter(|line| !line.trim().is_empty());
    let Some(first) = lines.next() else {
        return;
    };
    let seconds = n333_core::epoch::unix_now_seconds();
    log.push(format!(
        "{:02}:{:02}:{:02}  {first}",
        seconds / 3600 % 24,
        seconds / 60 % 60,
        seconds % 60
    ));
    log.extend(lines.map(|line| format!("          {}", line.trim_start())));
    if log.len() > REMEMBERED {
        log.drain(..log.len() - REMEMBERED);
    }
}

/// Read the keyboard on a thread of its own, because reading it blocks.
///
/// The flag is how it is stopped: a thread left polling a terminal after this program
/// has finished with it eats the keystrokes meant for whatever runs next.
fn read_keys() -> (
    UnboundedReceiver<event::KeyEvent>,
    Arc<AtomicBool>,
) {
    let (sender, receiver) = unbounded_channel();
    let reading = Arc::new(AtomicBool::new(true));
    let stop = Arc::clone(&reading);
    std::thread::spawn(move || {
        while stop.load(Ordering::Relaxed) {
            match event::poll(KEY_POLL) {
                Ok(true) => {
                    if let Ok(Event::Key(key)) = event::read()
                        && sender.send(key).is_err()
                    {
                        return;
                    }
                }
                Ok(false) => {}
                Err(_) => return,
            }
        }
    });
    (receiver, reading)
}
