//! `333 bootstrap` — begin on your own, when there is nobody to be given the file by.
//!
//! Everything else in this client refuses to do this. A node is given the file by
//! somebody who already has it, both of them sign for it, and those two signatures are
//! what everybody else reads as that node's beginning. A node that starts on its own has
//! none of that. It is the start of its own line, and nobody can vouch for where it came
//! from, which is the plain truth about it and is said out loud rather than hidden.
//!
//! WHY IT IS ALLOWED AT ALL. Somebody has to be first, and the rule that nobody may make
//! the file was never enforceable: the file is short and its contents are written in this
//! repository. A client that pretended to prevent it would be claiming to verify
//! something it cannot verify, which is the one thing this design refuses to do
//! anywhere. So it is allowed, it is named after what it is, and it is discouraged in the
//! only way that actually helps: by looking first, and telling you to go and join
//! somebody if there is anybody to join.
//!
//! THE CLIENT STILL CANNOT MAKE THE FILE. It carries the hash and not the bytes. What
//! this does is fetch the bytes from the meeting point and refuse them unless they are
//! the file, which is the same check a handover goes through.

use anyhow::{Context as _, bail};
use n333_core::subject::Subject;
use n333_net::Meeting;

use crate::commands::Common;
use crate::node::Node;

/// Begin a line of your own.
///
/// # Errors
/// Fails if the node cannot be opened, if it already has the file, if the meeting point
/// cannot be reached, if somebody is already there and `anyway` was not asked for, or if
/// what comes back is not the file.
pub(crate) async fn run(common: &Common, meet: &str, anyway: bool) -> anyhow::Result<()> {
    let (node, opened) = Node::open(&common.mistrust(), common.paths.root(), common.keeping)?;
    aloud_in!(
        "bootstrap-name",
        name = node.identity().node_id().to_string()
    );
    crate::commands::report_opening(&opened);
    if begin(&node, meet, anyway).await? {
        aloud_in!("bootstrap-vigil");
    }
    Ok(())
}

/// Begin a line of your own, as a node that is already open. True if it began.
///
/// Shared with the vigil, which holds the node already and is already answering.
///
/// # Errors
/// Fails if the node already has the file, if the meeting point cannot be reached, or
/// if what comes back is not the file.
pub(crate) async fn begin(node: &Node, meet: &str, anyway: bool) -> anyhow::Result<bool> {
    if node.subject().await.is_some() {
        bail!(words!("bootstrap-already-has-it"));
    }

    let meeting = Meeting::at(meet);
    let already = look_first(&meeting).await?;
    if already != 0 && !anyway {
        aloud_in!(
            "bootstrap-stop",
            already = already,
            meet = meet,
            board = meeting.browse()
        );
        return Ok(false);
    }

    let bytes = ask_for_it(&meeting, meet).await?;
    let subject = Subject::recognise(&bytes).with_context(|| words!("bootstrap-not-the-file"))?;
    node.receive(subject).await?;

    aloud_in!("bootstrap-begun");
    Ok(true)
}

/// How many nodes are already saying where they are.
///
/// Only the ones that verify are counted, so a board full of noise does not talk
/// somebody out of beginning when there is genuinely nobody there.
async fn look_first(meeting: &Meeting) -> anyhow::Result<usize> {
    let asking = meeting.clone();
    let board = tokio::task::spawn_blocking(move || asking.read())
        .await
        .with_context(|| reading_the_board(meeting))?
        .with_context(|| reading_the_board(meeting))?;
    Ok(board
        .iter()
        .filter(|frame| n333_core::whereabouts::open(frame).is_ok())
        .count())
}

/// What was being done when the board could not be read.
fn reading_the_board(meeting: &Meeting) -> String {
    words!("bootstrap-reading-the-board", place = meeting.place())
}

/// Ask the meeting point for the file itself.
async fn ask_for_it(meeting: &Meeting, meet: &str) -> anyhow::Result<Vec<u8>> {
    aloud_in!("bootstrap-asking", meet = meet);
    let asking = meeting.clone();
    // Not "would not hand it over": most of the ways this fails are the meeting point
    // not being reached at all, and the sentence under this one says which.
    tokio::task::spawn_blocking(move || asking.the_file())
        .await
        .with_context(|| words!("bootstrap-asking-for-the-file", meet = meet))?
        .with_context(|| words!("bootstrap-asking-for-the-file", meet = meet))
}

#[cfg(test)]
mod tests {
    use crate::words::count::Base;
    use crate::words::layout::{COLUMN, where_the_words_begin};

    const MEET: &str = "the333.dev";
    const BOARD: &str = "https://the333.dev/333";

    fn lines() -> Vec<String> {
        vec![
            words!("bootstrap-name", name = "333abc"),
            words!("bootstrap-vigil"),
            words!(
                "bootstrap-stop",
                already = 1_usize,
                meet = MEET,
                board = BOARD
            ),
            words!(
                "bootstrap-stop",
                already = 4_usize,
                meet = MEET,
                board = BOARD
            ),
            words!("bootstrap-begun"),
            words!("bootstrap-asking", meet = MEET),
        ]
    }

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let stop = |some: &str| {
            format!(
                "stop     {some} saying where they can be reached at {MEET}. Beginning on\n\
                 \x20        your own now would start a second line beside theirs for no\n\
                 \x20        reason. Open {BOARD} in a browser, take one of the invitations, and\n\
                 \x20        run `333 join` with it instead.\n\
                 \x20        If you have read that and still mean to begin, `--anyway` says so."
            )
        };
        let (said, phrases) = crate::words::speaking("en", Base::Ten, || {
            let phrases = [
                (
                    words!("bootstrap-already-has-it"),
                    "this node already has the file. There is nothing to begin.",
                ),
                (
                    words!("bootstrap-not-the-file"),
                    "what came back is not the file",
                ),
                (
                    words!("bootstrap-reading-the-board", place = MEET),
                    "reading the board at the333.dev",
                ),
                (
                    words!("bootstrap-asking-for-the-file", meet = MEET),
                    "asking the333.dev for the file",
                ),
            ];
            (lines(), phrases)
        });
        assert_eq!(
            said,
            [
                "name     333abc".to_owned(),
                "vigil    run `333 serve` to answer.".to_owned(),
                stop("1 of us is"),
                stop("4 of us are"),
                "begun    the file is in this node's directory and this node is the start of its\n\
                 \x20        own line. Nobody signed for handing it over, because nobody did, and\n\
                 \x20        anybody reading this node's record can see that.\n\
                 \n\
                 \x20        That is the founder's position and not an ordinary one. A roll\n\
                 \x20        admits whoever received the file, so a node that received it from\n\
                 \x20        nobody is on no roll: nobody will come here to ask anything, and\n\
                 \x20        this node is never drawn to ask anybody. It can still go to the\n\
                 \x20        ones drawn to ask it and be witnessed that way.\n\
                 \n\
                 \x20        Whoever you hand the file to afterwards is admitted the ordinary\n\
                 \x20        way, with both of you signing, and is counted from that moment."
                    .to_owned(),
                "asking   the333.dev for the file".to_owned(),
            ]
        );
        for (now, before) in phrases {
            assert_eq!(now, before);
        }
    }

    #[test]
    fn in_korean_every_line_of_bootstrap_begins_its_words_in_the_same_column() {
        for line in crate::words::speaking("ko", Base::Ten, lines) {
            assert!(!line.is_ascii(), "{line:?}");
            assert_eq!(where_the_words_begin(&line), COLUMN, "{line:?}");
        }
    }
}
