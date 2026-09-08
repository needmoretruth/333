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
    aloud!("name     {}", node.identity().node_id());
    crate::commands::report_opening(&opened);

    if node.subject().await.is_some() {
        bail!("this node already has the file. There is nothing to begin.");
    }

    let meeting = Meeting::at(meet);
    let already = look_first(&meeting).await?;
    if already != 0 && !anyway {
        let some = if already == 1 {
            "1 of us is".to_owned()
        } else {
            format!("{already} of us are")
        };
        aloud!(
            "stop     {some} saying where they can be reached at {meet}. Beginning on\n\
             \x20        your own now would start a second line beside theirs for no\n\
             \x20        reason. Open {} in a browser, take one of the invitations, and\n\
             \x20        run `333 join` with it instead.\n\
             \x20        If you have read that and still mean to begin, `--anyway` says so.",
            meeting.browse()
        );
        return Ok(());
    }

    let bytes = ask_for_it(&meeting, meet).await?;
    let subject = Subject::recognise(&bytes).context("what came back is not the file")?;
    node.receive(subject).await?;

    aloud!(
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
         \x20        Run `333 serve` to answer. Whoever you hand the file to afterwards\n\
         \x20        is admitted the ordinary way, with both of you signing, and is\n\
         \x20        counted from that moment."
    );
    Ok(())
}

/// How many nodes are already saying where they are.
///
/// Only the ones that verify are counted, so a board full of noise does not talk
/// somebody out of beginning when there is genuinely nobody there.
async fn look_first(meeting: &Meeting) -> anyhow::Result<usize> {
    let asking = meeting.clone();
    let board = tokio::task::spawn_blocking(move || asking.read())
        .await
        .context("looking at the meeting point")?
        .context("reading the meeting point")?;
    Ok(board
        .iter()
        .filter(|frame| n333_core::whereabouts::open(frame).is_ok())
        .count())
}

/// Ask the meeting point for the file itself.
async fn ask_for_it(meeting: &Meeting, meet: &str) -> anyhow::Result<Vec<u8>> {
    aloud!("asking   {meet} for the file");
    let asking = meeting.clone();
    tokio::task::spawn_blocking(move || asking.the_file())
        .await
        .context("asking for the file")?
        .context("the meeting point would not hand over the file")
}
