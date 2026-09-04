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

use anyhow::Context as _;
use n333_core::extinction::{Remaining, Verdict};
use n333_core::presence::{self, Census, Standing, WINDOW_EPOCHS};
use n333_core::{Epoch, epoch};

use crate::commands::Common;
use crate::node::Node;

/// Show where this node stands.
///
/// # Errors
/// Fails if the node's directory cannot be opened or its own record does not verify.
pub(crate) async fn run(common: &Common) -> anyhow::Result<()> {
    let (node, opened) = Node::open(&common.mistrust(), common.paths.root())?;
    let now = Epoch::now();

    println!("name     {}", node.identity().node_id());
    println!("epoch    {}", now.0);
    crate::commands::report_opening(&opened);
    println!();

    the_count(&node, now).await?;
    println!();
    this_node(&node, now).await?;
    println!();
    the_silence(&node, now).await
}

/// How many of us are answering, first and largest.
///
/// The count that decides everything is the number answering, never the number of
/// names on the roll. A roll can only grow; only the first number can reach zero.
async fn the_count(node: &Node, now: Epoch) -> anyhow::Result<()> {
    let answering = node.answering(now).await?;
    let roll = node.roll().await;
    let members = u64::try_from(roll.len()).unwrap_or(u64::MAX);
    let active = u64::try_from(answering.len()).unwrap_or(u64::MAX);
    let census = Census::of(active, 0, members.saturating_sub(active));

    println!("ANSWERING  {}", census.active());
    println!("silent     {}", census.inactive());
    println!("           ─────");
    println!("roll       {}", census.roll());
    println!();
    println!(
        "That first number is everyone this node holds a signature for in epoch {} or\n\
         {}. It is what this node saw. Somebody else saw something else.",
        now.0.saturating_sub(1),
        now.0
    );
    Ok(())
}

/// What this node's own record says about this node.
async fn this_node(node: &Node, now: Epoch) -> anyhow::Result<()> {
    let Some(joined) = node.joined_in().await else {
        println!("You are on nobody's roll. Nobody has handed you the file, so there is\n\
                  nothing yet for anyone to witness. `333 join` is the whole of it.");
        return Ok(());
    };
    let counted_from = n333_core::enrollment::active_from(joined);
    if now.0 < counted_from.0 {
        println!(
            "Given the file in epoch {}, and counted from epoch {} — {} to go.\n\
             Answer everything asked of you until then. None of it is banked, and all\n\
             of it is watched.",
            joined.0,
            counted_from.0,
            epochs(counted_from.0 - now.0)
        );
        return Ok(());
    }

    let standing = presence::standing_at(now, node.own_record().await?);
    println!("{}", read_standing(&standing));
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
        "You are counted."
    } else {
        "You are not counted. Two of every three is the whole of what is asked."
    };
    format!(
        "Present in {} of the {} epochs you were asked about — {share}. {verdict}\n\
         The window is the last {WINDOW_EPOCHS} epochs and nothing before it exists.\n\
         Ten years of it would read exactly the same, and buy exactly as much.",
        standing.present, standing.counted
    )
}

/// Whether anybody is here, and what is left if nobody is.
async fn the_silence(node: &Node, now: Epoch) -> anyhow::Result<()> {
    let vigil = node.watched(now).await.context("reading the watch")?;
    match vigil.verdict() {
        Verdict::NothingToSay => println!(
            "No one has ever answered this node. That is not evidence of anything: it\n\
             is what a node looks like before it has been anywhere."
        ),
        Verdict::Alive => println!("Somebody is here. Nothing further is owed to the arithmetic."),
        Verdict::Waiting { silent, needed } => println!(
            "Nobody has answered for {}. This node has said nothing about it and will\n\
             not until {}, and only then if it is running for every one of them.",
            epochs(silent),
            epochs(needed)
        ),
        Verdict::Ended { since } => {
            println!(
                "Nobody has answered through {} of unbroken watching. The last of us\n\
                 stopped in epoch {}.",
                epochs(n333_core::extinction::SILENT_EPOCHS_BEFORE_THE_END),
                since.0
            );
            match vigil.remaining_at(epoch::unix_now_seconds()) {
                Some(Remaining { years, days }) => println!(
                    "\nNobody left. {years} years and {days} days until it is gone."
                ),
                None => println!("\nNobody left, and the last of the years has run out."),
            }
        }
    }
    Ok(())
}

/// "1 epoch" or "N epochs", said the way a person would.
fn epochs(count: u64) -> String {
    if count == 1 {
        "1 epoch".to_owned()
    } else {
        format!("{count} epochs")
    }
}
