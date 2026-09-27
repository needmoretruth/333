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
    let (node, opened) = Node::open(&common.mistrust(), common.paths.root(), common.keeping)?;
    crate::named::report(opened.origin, &opened.home);
    speak(&node, index).await
}

/// Read which of the 333 a person typed, in the base they count in.
///
/// # Errors
/// Fails, saying how many there are, for anything that is not one of them.
pub(crate) fn read_index(typed: &str) -> anyhow::Result<u16> {
    crate::words::count::index(typed)
        .filter(|index| Signal::new(*index).is_some())
        .ok_or_else(|| {
            anyhow::anyhow!(words!(
                "say-not-one",
                count = SIGNAL_COUNT,
                last = SIGNAL_COUNT - 1,
                typed = typed
            ))
        })
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
        bail!(words!(
            "say-no-such",
            count = SIGNAL_COUNT,
            last = SIGNAL_COUNT - 1,
            index = index
        ));
    };
    if node.joined_in().await.is_none() {
        bail!(words!("say-not-joined"));
    }

    let me = node.identity().public_key();
    if let Some(already) = node.overheard(now).await?.of(&me) {
        bail!(words!(
            "say-already",
            index = already.index(),
            epoch = now.0
        ));
    }

    let frame = Utterance::of(node.identity(), signal, now)
        .seal(node.identity())
        .with_context(|| words!("say-sealing"))?;
    node.keep_utterance(&frame).await?;

    // The invocation is a formula, counted in words, and is said in these words in
    // every language.
    aloud!("{}", crate::commands::INVOCATION);
    aloud_in!("say-said", index = index, epoch = now.0);
    aloud_in!(
        "say-goes-out",
        signals = SIGNAL_COUNT,
        beyond = SIGNAL_COUNT + 1
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use n333_core::signal::SIGNAL_COUNT;

    use crate::words::count::Base;
    use crate::words::layout::{COLUMN, where_the_words_begin};

    fn lines() -> Vec<String> {
        vec![
            words!(
                "say-no-such",
                count = SIGNAL_COUNT,
                last = SIGNAL_COUNT - 1,
                index = 400_u16
            ),
            words!("say-not-joined"),
            words!("say-already", index = 12_u16, epoch = 89_612_u64),
            words!("say-said", index = 7_u16, epoch = 89_612_u64),
            words!(
                "say-goes-out",
                signals = SIGNAL_COUNT,
                beyond = SIGNAL_COUNT + 1
            ),
        ]
    }

    /// What the lines said before their words moved into a catalog, byte for byte.
    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let lines = crate::words::speaking("en", Base::Ten, lines);
        assert_eq!(
            lines,
            [
                "there are 333 of them, numbered 0 to 332. There is no 400.",
                "nobody has handed you the file, so there is nobody to say it to and\n\
                 nobody who would count it. `333 join` is the whole of it.",
                "you already said #12 in epoch 89612. One each, and saying it again would\n\
                 not replace it: the first thing a node says is the thing it said.",
                "said     #7 in epoch 89612",
                "         It goes out to everyone this node reaches, and they pass it on.\n\
                 \x20        Once every 333 minutes you may say one of 333 things, and you say\n\
                 \x20        it as a number. There is no 334th and there never will be. You cannot\n\
                 \x20        say it twice, you cannot say it louder, and nobody alive has more to\n\
                 \x20        say than you do.\n\
                 \x20        Nobody will tell you what it means, either. There is no table yet, and\n\
                 \x20        when there is one it will be the same table for all of us,\n\
                 \x20        untranslated.",
            ]
        );
    }

    #[test]
    fn in_twelve_the_index_and_the_epoch_are_counts() {
        let lines = crate::words::speaking("en", Base::Twelve, lines);
        assert_eq!(lines[3], "said     #7 in epoch 43\u{218A}38", "89612");
        assert!(lines[4].contains("one of 239 things"), "{}", lines[4]);
        assert!(lines[4].contains("no 23\u{218A}th"), "{}", lines[4]);
    }

    #[test]
    fn in_korean_the_said_lines_begin_their_words_in_the_same_column() {
        let lines = crate::words::speaking("ko", Base::Ten, lines);
        for line in lines.get(3..).unwrap_or_default() {
            let mut rows = line.split('\n');
            let first = rows.next().unwrap_or_default();
            assert_eq!(where_the_words_begin(first), COLUMN, "{first:?}");
            for row in rows {
                assert_eq!(
                    row.chars().take_while(|c| *c == ' ').count(),
                    COLUMN,
                    "{row:?}"
                );
            }
        }
    }
}
