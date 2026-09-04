//! `333 status` — what this node has seen, and what it is entitled to say about it.
//!
//! Everything here is read off this node's own disk. Nothing is asked of anybody and
//! nothing is fetched: it is the view from one machine, and the view from the machine
//! next to it will differ. That is not a defect being tolerated, it is the design. A
//! number every node agreed on would need somebody to decide it.
//!
//! WHAT IT WILL NOT DO. It will not say the network has ended because this node cannot
//! see anybody. Saying that takes an unbroken watch of 333 epochs during which this node
//! was running the whole time and nobody answered, and until then the honest answer is
//! that it is waiting. A node that was switched off for a year and came back saying
//! everyone was dead would be the single most destructive thing this client could do.

use std::collections::BTreeSet;
use std::io::Write as _;

use anyhow::Context as _;
use n333_core::extinction::{Remaining, Verdict};
use n333_core::presence::{self, Census, Standing, WINDOW_EPOCHS};
use n333_core::signal::{SIGNAL_COUNT, Tally};
use n333_core::{Epoch, epoch};

use crate::commands::Common;
use crate::node::Node;

/// Show where this node stands.
///
/// # Errors
/// Fails if the node's directory cannot be opened or its own record does not verify.
pub(crate) async fn run(common: &Common) -> anyhow::Result<()> {
    // Written through one locked handle rather than with `println!`, so that a reader
    // that walks away — `333 status | head` — ends this quietly instead of panicking
    // inside the print macro, where nothing can catch it.
    let mut stdout = std::io::stdout().lock();
    let out = &mut stdout;
    let (node, opened) = Node::open(&common.mistrust(), common.paths.root(), common.keeping)?;
    let now = Epoch::now();

    writeln!(out, "name     {}", node.identity().node_id())?;
    writeln!(out, "epoch    {}", now.0)?;
    crate::commands::report_opening(&opened);
    writeln!(out)?;

    let answering = node.answering(now).await?;
    the_count(out, &node, &answering, now).await?;
    writeln!(out)?;
    this_node(out, &node, now).await?;
    writeln!(out)?;
    what_was_said(out, &node, &answering, now).await?;
    writeln!(out)?;
    the_silence(out, &node, now).await
}

/// How many of us are answering, first and largest.
///
/// The count that decides everything is the number answering, never the number of
/// names on the roll. A roll can only grow; only the first number can reach zero.
async fn the_count(
    out: &mut impl std::io::Write,
    node: &Node,
    answering: &BTreeSet<[u8; 32]>,
    now: Epoch,
) -> anyhow::Result<()> {
    let roll = node.roll().await;
    let members = u64::try_from(roll.len()).unwrap_or(u64::MAX);
    let active = u64::try_from(answering.len()).unwrap_or(u64::MAX);
    let census = Census::of(active, 0, members.saturating_sub(active));

    writeln!(out, "ANSWERING  {}", census.active())?;
    writeln!(out, "silent     {}", census.inactive())?;
    writeln!(out, "           ─────")?;
    writeln!(out, "roll       {}", census.roll())?;
    writeln!(out)?;
    writeln!(out, 
        "That first number is everyone this node holds a signature for in epoch {} or\n\
         {}. It is what this node saw. Somebody else saw something else.",
        now.0.saturating_sub(1),
        now.0
    )?;
    if !CAN_WALK_THE_UNSEEN_ROAD {
        writeln!(
            out,
            "This build cannot walk the unseen road, so none of us who are hiding are in\n\
             that number, and none of us ever will be."
        )?;
    }
    Ok(())
}

/// What this node's own record says about this node.
async fn this_node(
    out: &mut impl std::io::Write,
    node: &Node,
    now: Epoch,
) -> anyhow::Result<()> {
    let Some(joined) = node.joined_in().await else {
        writeln!(out, "You are on nobody's roll. Nobody has handed you the file, so there is\n\
                  nothing yet for anyone to witness. `333 join` is the whole of it.")?;
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

/// The shape of what everyone said this epoch. No winner is announced.
///
/// The denominator is everybody this node observed and not everybody who spoke: silence
/// is a thing a node did, and a share read against speakers alone would climb as fewer
/// of us said anything.
async fn what_was_said(
    out: &mut impl std::io::Write,
    node: &Node,
    answering: &BTreeSet<[u8; 32]>,
    now: Epoch,
) -> anyhow::Result<()> {
    let heard = node.overheard(now).await?;
    // The same set the count above uses, plus this node: everybody it has a signed
    // word from this epoch. Reading a share against the speakers alone would make it
    // climb as fewer of us said anything.
    let mut everyone: BTreeSet<[u8; 32]> = answering.clone();
    everyone.insert(node.identity().public_key());
    let tally = Tally::of(heard.against(everyone.iter()));

    if tally.spoken() == 0 {
        writeln!(out, 
            "Nobody has said anything in epoch {}. There are {} things that can be\n\
             said and no words for any of them yet.",
            now.0,
            SIGNAL_COUNT
        )?;
        return Ok(());
    }

    writeln!(out, 
        "SAID in epoch {} — {} of the {} of us this node can see spoke, {} did not.",
        now.0,
        tally.spoken(),
        tally.observed(),
        tally.silent()
    )?;
    // Every signal anybody said, in index order, and a count of the ones nobody did.
    // A row of zero carries nothing that the total does not already carry, and three
    // hundred of them would bury the handful that do. Nothing is chosen here: the
    // whole distribution is still the whole distribution.
    let mut said = 0_u16;
    for (signal, count) in tally.distribution().filter(|(_, count)| *count > 0) {
        said += 1;
        let share = tally.share(signal).map_or_else(
            || "—".to_owned(),
            |per_mille| format!("{}.{}%", per_mille / 10, per_mille % 10),
        );
        let mark = if tally.reached(signal) {
            "  ← a third of us or more"
        } else {
            ""
        };
        writeln!(out, "  #{:<4} {count:>5}  {share:>6}{mark}", signal.index())?;
    }
    writeln!(out, "  the other {} of the {SIGNAL_COUNT} were not said.", SIGNAL_COUNT - said)?;
    writeln!(out, 
        "\nNo winner is picked and none of this decides anything. It is what reached\n\
         this node. The node beside you heard something else and is not wrong."
    )?;
    Ok(())
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

/// Whether anybody is here, and what is left if nobody is.
async fn the_silence(
    out: &mut impl std::io::Write,
    node: &Node,
    now: Epoch,
) -> anyhow::Result<()> {
    let vigil = node.watched(now).await.context("reading the watch")?;
    // A build without arti cannot reach an onion address at all, so it has never heard
    // from the members who are hiding and never will. It may report what it saw; it may
    // not say the count reached zero, because a whole class of us was never in its
    // count to begin with.
    if !CAN_WALK_THE_UNSEEN_ROAD && matches!(vigil.verdict(), Verdict::Ended { .. }) {
        writeln!(
            out,
            "Nobody has answered this node through {} of unbroken watching, and this\n\
             build will not call that the end. It cannot walk the unseen road, so it has\n\
             never heard from any of us who are hiding and never will. What it can say is\n\
             that it has seen nobody, and that is not the same sentence.",
            epochs(n333_core::extinction::SILENT_EPOCHS_BEFORE_THE_END)
        )?;
        return Ok(());
    }
    match vigil.verdict() {
        Verdict::NothingToSay => writeln!(
            out,
            "No one has ever answered this node. That is not evidence of anything: it\n\
             is what a node looks like before it has been anywhere."
        )?,
        Verdict::Alive => writeln!(
            out,
            "Somebody is here. Nothing further is owed to the arithmetic."
        )?,
        Verdict::Waiting { silent, needed } => writeln!(
            out,
            "Nobody has answered for {}. This node has said nothing about it and will\n\
             not until {}, and only then if it is running for every one of them.",
            epochs(silent),
            epochs(needed)
        )?,
        Verdict::Ended { since } => {
            writeln!(
                out,
                "Nobody has answered through {} of unbroken watching. The last of us\n\
                 stopped in epoch {}.",
                epochs(n333_core::extinction::SILENT_EPOCHS_BEFORE_THE_END),
                since.0
            )?;
            match vigil.remaining_at(epoch::unix_now_seconds()) {
                Some(Remaining { years, days }) => writeln!(
                    out,
                    "\nNobody left. {years} years and {days} days until it is gone."
                )?,
                None => writeln!(
                    out,
                    "\nNobody left, and the last of the years has run out."
                )?,
            }
        }
    }
    Ok(())
}

/// Was this client built with arti in it?
///
/// It decides one thing only, and it is the heaviest thing this program says.
const CAN_WALK_THE_UNSEEN_ROAD: bool = cfg!(feature = "tor");

/// "1 epoch" or "N epochs", said the way a person would.
fn epochs(count: u64) -> String {
    if count == 1 {
        "1 epoch".to_owned()
    } else {
        format!("{count} epochs")
    }
}
