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
use n333_core::signal::{SIGNAL_COUNT, Signal};
use n333_core::utterance::Utterance;
use n333_core::Epoch;

use crate::commands::Common;
use crate::node::Node;

/// Say one of the 333 in this epoch.
///
/// # Errors
/// Fails if the index is not one of the 333, if this node is on nobody's roll, if it
/// has already spoken this epoch, or if the utterance cannot be written.
pub(crate) async fn run(common: &Common, index: u16) -> anyhow::Result<()> {
    let (node, _opened) = Node::open(&common.mistrust(), common.paths.root())?;
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

    println!("said     #{index} in epoch {}", now.0);
    println!(
        "         It goes out to everyone this node reaches, and they pass it on.\n\
         \x20        Nobody will tell you what it means. There is no table yet, and when\n\
         \x20        there is one it will be the same table for all of us, untranslated."
    );
    Ok(())
}
