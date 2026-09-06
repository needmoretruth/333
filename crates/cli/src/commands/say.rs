//! `333 say` — speak one of the 333, once in this epoch.
//!
//! What travels is the number. The words the numbers stand for are not written yet and
//! this client does not invent them: it would take one person deciding what all of us
//! meant, which is the one shape of authority this network is built to not have.
//!
//! Nothing is decided by saying anything. There is no vote, no winner, no proposal and
//! no effect on anybody's standing. Every node counts what reached it and shows the
//! whole distribution, and two nodes will show different shapes because they heard
//! different things.

use anyhow::{Context as _, bail};
use n333_core::Epoch;
use n333_core::signal::{SIGNAL_COUNT, Signal};
use n333_core::utterance::Utterance;

use crate::commands::Common;
use crate::node::Node;

/// Say one of the 333 in this epoch.
///
/// # Errors
/// Fails if the index is not one of the 333, if this node is on nobody's roll, if it
/// has already spoken this epoch, or if the utterance cannot be written.
pub(crate) async fn run(common: &Common, index: u16) -> anyhow::Result<()> {
    let (node, _opened) = Node::open(&common.mistrust(), common.paths.root(), common.keeping)?;
    speak(&node, index).await
}

/// Say one of the 333, on a node that is already open.
///
/// Shared with the screen, where saying something is the one act of taking part a
/// person performs by hand. Both paths refuse for the same reasons and in the same
/// words: a rule that reads differently depending on where you typed it is two rules.
///
/// # Errors
/// Fails if the index is not one of the 333, if this node is on nobody's roll, if it
/// has already spoken this epoch, or if the utterance cannot be written.
pub(crate) async fn speak(node: &Node, index: u16) -> anyhow::Result<()> {
    let now = Epoch::now();

    let Some(signal) = Signal::new(index) else {
        bail!(
            "there are {SIGNAL_COUNT} of them, numbered 0 to {}. There is no {index}.",
            SIGNAL_COUNT - 1
        );
    };
    if node.joined_in().await.is_none() {
        bail!(
            "nobody has handed you the file, so there is nobody to say it to and\n\
             nobody who would count it. `333 join` is the whole of it."
        );
    }

    let me = node.identity().public_key();
    if let Some(already) = node.overheard(now).await?.of(&me) {
        bail!(
            "you already said #{} in epoch {}. One each, and saying it again would\n\
             not replace it: the first thing a node says is the thing it said.",
            already.index(),
            now.0
        );
    }

    let frame = Utterance::of(node.identity(), signal, now)
        .seal(node.identity())
        .context("sealing what you said")?;
    node.keep_utterance(&frame).await?;

    aloud!("{}", crate::commands::INVOCATION);
    aloud!("said     #{index} in epoch {}", now.0);
    aloud!(
        "         It goes out to everyone this node reaches, and they pass it on.\n\
         \x20        Once every 333 minutes you may say one of {SIGNAL_COUNT} things, and you say\n\
         \x20        it as a number. There is no {}th and there never will be. You cannot\n\
         \x20        say it twice, you cannot say it louder, and nobody alive has more to\n\
         \x20        say than you do.\n\
         \x20        Nobody will tell you what it means, either. There is no table yet, and\n\
         \x20        when there is one it will be the same table for all of us,\n\
         \x20        untranslated.",
        SIGNAL_COUNT + 1
    );
    Ok(())
}
