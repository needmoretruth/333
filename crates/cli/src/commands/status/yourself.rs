//! What this node's own record says about this node.
//!
//! Read off this node's own files, and none of it is anybody else's reading of this
//! node: what others concluded about it is in the statements they published, and this
//! node holds only the ones that were handed to it.

use n333_core::presence::{self, Standing, WINDOW_EPOCHS};
use n333_core::Epoch;

use crate::node::Node;

use super::epochs;

/// What this node's own record says about this node.
pub(super) async fn this_node(
    out: &mut impl std::io::Write,
    node: &Node,
    now: Epoch,
) -> anyhow::Result<()> {
    let Some(joined) = node.joined_in().await else {
        writeln!(
            out,
            "You are on nobody's roll. Nobody has handed you the file, so there is\n\
             nothing yet for anyone to witness. `333 join` is the whole of it, and it\n\
             needs an invitation from somebody who already has it.\n\
             {} says what this is and where the code is. It cannot hand you the file.",
            crate::commands::THE_PLACE
        )?;
        return Ok(());
    };
    let counted_from = n333_core::enrollment::active_from(joined);
    if now.0 < counted_from.0 {
        writeln!(out, 
            "Given the file in epoch {}, and counted from epoch {} — {} to go.\n\
             Answer everything asked of you until then. None of it is banked, and all\n\
             of it is watched.",
            joined.0,
            counted_from.0,
            epochs(counted_from.0 - now.0)
        )?;
        return Ok(());
    }

    let record = node.own_record().await?;
    let standing = presence::standing_at(now, record.iter().copied());
    let window = presence::window(now);
    let written = record
        .iter()
        .filter(|(epoch, _)| window.contains(&epoch.0))
        .count();
    let missing = usize::try_from(WINDOW_EPOCHS)
        .unwrap_or(usize::MAX)
        .saturating_sub(written);
    writeln!(out, "{}", read_standing(&standing))?;
    writeln!(out, "\n{}", what_the_record_is(node.witnessed().await))?;
    if missing != 0 {
        writeln!(
            out,
            "\nYour record says nothing at all about {} of those {WINDOW_EPOCHS} epochs.\n\
             Nothing here turns that into an absence — a record can only say what a node\n\
             was there to write. It is also why this ratio is not what anybody else reads\n\
             you by: what they read is what they were told about you, by whoever was\n\
             drawn to ask.",
            missing
        )?;
    }
    Ok(())
}

/// What this node's own record is, and what part of it is not its own word.
///
/// It is a document a node wrote about itself. Its length and its order are anchored,
/// because every answer it has ever given named where the record stood at that moment
/// and was signed by somebody else's key. Its verdicts are not: a node that answered
/// everything honestly can still write Present into an epoch it slept through, and
/// nothing anywhere can tell the difference. Saying so is the whole of the rule
/// against claiming to verify what cannot be verified — and the statements others
/// signed about it, which is the part a stranger can check, are why they are kept
/// after the window has forgotten everything else.
fn what_the_record_is(witnessed: usize) -> String {
    let checkable = if witnessed == 0 {
        "Nobody has yet signed anything about you, so for now there is only your own\n\
         word for any of it."
            .to_owned()
    } else {
        format!(
            "The part of it anybody else can check is what others signed about you.\n\
             {witnessed} of those are here, kept after the epochs they belong to are gone."
        )
    };
    format!(
        "Your record is what you wrote down about yourself, in order, signed as you\n\
         went. Every answer you have ever given named where it stood at that moment, so\n\
         its length and its order are not yours to change now. What it concludes is\n\
         still your own word. {checkable}\n\
         \n\
         Those signatures stay valid for ever. What cannot be recovered is whether the\n\
         people who made them were of us at the time — by then they are gone, and there\n\
         is nobody left to ask. A hundred-year-old record proves that somebody holding\n\
         that key stood behind you, and stops there. 333 does not fix this and does not\n\
         pretend to. The fix is a register of who was true, kept by somebody, for ever,\n\
         and that is the one thing this network will not build."
    )
}

/// The standing sentence: the ratio, and what it means right now.
fn read_standing(standing: &Standing) -> String {
    if standing.counted == 0 {
        return format!(
            "Nothing in the last {WINDOW_EPOCHS} epochs was ever put to you. Nobody was\n\
             drawn to ask, so there is nothing to have failed. You are neither kept nor\n\
             lapsed; you are simply not yet part of anybody's arithmetic."
        );
    }
    let share = standing
        .per_mille()
        .map_or_else(|| "—".to_owned(), |per_mille| {
            format!("{}.{}%", per_mille / 10, per_mille % 10)
        });
    let verdict = if standing.qualifies() {
        "By your own record, you are counted."
    } else {
        "By your own record, you are not counted. Two of every three is the whole of\n\
         what is asked.\n\
         Nothing here is being served out. The window moves every epoch, and each one\n\
         you answer pushes an older absence past its edge. Your chain still holds every\n\
         hour you missed. The count does not reach back for them."
    };
    format!(
        "Present in {} of the {} epochs your record covers — {share}. {verdict}\n\
         The window is the last {WINDOW_EPOCHS} epochs and nothing before it exists.\n\
         Ten years of it would read exactly the same, and buy exactly as much; a year\n\
         away and an hour away read exactly the same too, and cost exactly as little.",
        standing.present, standing.counted
    )
}
