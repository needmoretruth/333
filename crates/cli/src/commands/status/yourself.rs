//! What this node's own record says about this node.
//!
//! Read off this node's own files, and none of it is anybody else's reading of this
//! node: what others concluded about it is in the statements they published, and this
//! node holds only the ones that were handed to it.

use n333_core::Epoch;
use n333_core::presence::{self, Standing, WINDOW_EPOCHS};

use crate::node::Node;

use super::{epochs, share};

/// What this node's own record says about this node.
pub(super) async fn this_node(
    out: &mut impl std::io::Write,
    node: &Node,
    now: Epoch,
) -> anyhow::Result<()> {
    let Some(joined) = node.joined_in().await else {
        let place = crate::commands::THE_PLACE;
        writeln!(
            out,
            "{}",
            words!("status-yourself-on-no-roll", place = place)
        )?;
        return Ok(());
    };
    let counted_from = n333_core::enrollment::active_from(joined);
    if now.0 < counted_from.0 {
        let waiting = words!(
            "status-yourself-not-yet-counted",
            joined = joined.0,
            counted_from = counted_from.0,
            to_go = epochs(counted_from.0 - now.0)
        );
        writeln!(out, "{waiting}")?;
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
        let silent = words!(
            "status-yourself-says-nothing",
            missing = missing,
            window = WINDOW_EPOCHS
        );
        writeln!(out, "\n{silent}")?;
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
        words!("status-yourself-only-your-word")
    } else {
        words!("status-yourself-checkable", witnessed = witnessed)
    };
    words!("status-yourself-record", checkable = checkable)
}

/// The standing sentence: the ratio, and what it means right now.
fn read_standing(standing: &Standing) -> String {
    if standing.counted == 0 {
        return words!("status-yourself-never-asked", window = WINDOW_EPOCHS);
    }
    let verdict = if standing.qualifies() {
        words!("status-yourself-counted")
    } else {
        words!("status-yourself-not-counted")
    };
    words!(
        "status-yourself-standing",
        present = standing.present,
        counted = standing.counted,
        share = share(standing.per_mille()),
        verdict = verdict,
        window = WINDOW_EPOCHS
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::words::count::Base;

    /// What the lines said before their words moved into a catalog, byte for byte,
    /// beside what the catalog says now.
    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let kept = Standing {
            counted: 300,
            present: 250,
        };
        let lapsed = Standing {
            counted: 300,
            present: 100,
        };
        let old = |standing: &Standing| {
            let per_mille = standing.per_mille().unwrap_or_default();
            format!("{}.{}%", per_mille / 10, per_mille % 10)
        };
        let pairs = crate::words::speaking("en", Base::Ten, || {
            [
                (
                    words!("status-yourself-on-no-roll", place = "the333.dev"),
                    "You are on nobody's roll. Nobody has handed you the file, so there is\n\
                     nothing yet for anyone to witness. `333 join` is the whole of it, and it\n\
                     needs an invitation from somebody who already has it.\n\
                     the333.dev says what this is and where the code is. It cannot hand you the file."
                        .to_owned(),
                ),
                (
                    words!(
                        "status-yourself-not-yet-counted",
                        joined = 1_000_u64,
                        counted_from = 1_003_u64,
                        to_go = epochs(2)
                    ),
                    "Given the file in epoch 1000, and counted from epoch 1003 — 2 epochs to go.\n\
                     Answer everything asked of you until then. None of it is banked, and all\n\
                     of it is watched."
                        .to_owned(),
                ),
                (
                    words!(
                        "status-yourself-says-nothing",
                        missing = 30_usize,
                        window = WINDOW_EPOCHS
                    ),
                    "Your record says nothing at all about 30 of those 333 epochs.\n\
                     Nothing here turns that into an absence — a record can only say what a node\n\
                     was there to write. It is also why this ratio is not what anybody else reads\n\
                     you by: what they read is what they were told about you, by whoever was\n\
                     drawn to ask."
                        .to_owned(),
                ),
                (
                    what_the_record_is(5),
                    "Your record is what you wrote down about yourself, in order, signed as you\n\
                     went. Every answer you have ever given named where it stood at that moment, so\n\
                     its length and its order are not yours to change now. What it concludes is\n\
                     still your own word. The part of it anybody else can check is what others signed about you.\n\
                     5 of those are here, kept after the epochs they belong to are gone.\n\
                     \n\
                     Those signatures stay valid for ever. What cannot be recovered is whether the\n\
                     people who made them were of us at the time — by then they are gone, and there\n\
                     is nobody left to ask. A hundred-year-old record proves that somebody holding\n\
                     that key stood behind you, and stops there. 333 does not fix this and does not\n\
                     pretend to. The fix is a register of who was true, kept by somebody, for ever,\n\
                     and that is the one thing this network will not build."
                        .to_owned(),
                ),
                (
                    read_standing(&kept),
                    format!(
                        "Present in 250 of the 300 epochs your record covers — {}. By your own record, you are counted.\n\
                         The window is the last 333 epochs and nothing before it exists.\n\
                         Ten years of it would read exactly the same, and buy exactly as much; a year\n\
                         away and an hour away read exactly the same too, and cost exactly as little.",
                        old(&kept)
                    ),
                ),
                (
                    read_standing(&lapsed),
                    format!(
                        "Present in 100 of the 300 epochs your record covers — {}. By your own record, you are not counted. Two of every three is the whole of\n\
                         what is asked.\n\
                         Nothing here is being served out. The window moves every epoch, and each one\n\
                         you answer pushes an older absence past its edge. Your chain still holds every\n\
                         hour you missed. The count does not reach back for them.\n\
                         The window is the last 333 epochs and nothing before it exists.\n\
                         Ten years of it would read exactly the same, and buy exactly as much; a year\n\
                         away and an hour away read exactly the same too, and cost exactly as little.",
                        old(&lapsed)
                    ),
                ),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }
}
