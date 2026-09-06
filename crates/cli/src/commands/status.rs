//! `333 status` — what this node has seen, and what it is entitled to say about it.
//!
//! Two halves: what this node saw of everybody else, which is here, and what its own
//! record says about itself, which is [`yourself`]. They are read off the same disk in
//! one pass and printed in that order, because the first question a person has is
//! whether anybody is out there and the second is where they stand.
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

mod yourself;

use std::collections::BTreeSet;
use std::io::Write as _;

use anyhow::Context as _;
use n333_core::extinction::{Remaining, Verdict};
use n333_core::presence::Census;
use n333_core::signal::{SIGNAL_COUNT, Tally};
use n333_core::{Epoch, NodeId, epoch};

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
    yourself::this_node(out, &node, now).await?;
    writeln!(out)?;
    the_hands(out, &node).await?;
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
    writeln!(
        out,
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
    // The oldest objection to a network like this, answered where the number is rather
    // than in a document nobody opens: a thousand names in one pair of hands is not an
    // attack here, it is a thousand subscriptions.
    writeln!(
        out,
        "\nHow many people that is, this node does not know and cannot find out. What it\n\
         knows is that every one of those names answered in one of those two epochs, and\n\
         will have to answer again in the next, and the one after that, for as long as it\n\
         wants to be counted. If one person is holding a thousand of them, they are\n\
         paying for a thousand of them, hour after hour, and stop being counted the hour\n\
         they stop."
    )?;
    Ok(())
}

/// The hands this node's copy came through.
///
/// The only history this network has, and it is not kept anywhere as one: it falls out
/// of admissions already on this disk, each naming who handed the file to whom, read
/// backwards.
async fn the_hands(out: &mut impl std::io::Write, node: &Node) -> anyhow::Result<()> {
    let hands = node.lineage().await;
    let Some(furthest) = hands.last() else {
        return Ok(());
    };
    writeln!(out, "GIVEN BY")?;
    for (place, member) in hands.iter().enumerate() {
        let name = if place == 0 {
            "you".to_owned()
        } else {
            crate::commands::shorten(&NodeId::from_public_key(&member.key).to_string())
        };
        writeln!(
            out,
            "  {name:<18}  received it in epoch {}",
            member.received_in.0
        )?;
    }
    writeln!(
        out,
        "  {:<18}  the trail stops here.",
        crate::commands::shorten(&NodeId::from_public_key(&furthest.sponsor).to_string())
    )?;
    writeln!(
        out,
        "\nThat is where this node stopped knowing, not where it began. The first of us\n\
         was given the file by nobody and has no admission anywhere, and a record this\n\
         node has simply not been handed yet looks exactly the same from here."
    )?;
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
        writeln!(
            out,
            "Nobody has said anything in epoch {}. There are {} things that can be\n\
             said and no words for any of them yet.",
            now.0, SIGNAL_COUNT
        )?;
        return Ok(());
    }

    writeln!(
        out,
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
    writeln!(
        out,
        "  the other {} of the {SIGNAL_COUNT} were not said.",
        SIGNAL_COUNT - said
    )?;
    writeln!(
        out,
        "\nNo winner is picked and none of this decides anything. It is what reached\n\
         this node. The node beside you heard something else and is not wrong."
    )?;
    Ok(())
}

/// Whether anybody is here, and what is left if nobody is.
async fn the_silence(out: &mut impl std::io::Write, node: &Node, now: Epoch) -> anyhow::Result<()> {
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
                "NOBODY IS KEEPING 333\n\
                 \n\
                 You are the only one here. Nobody has answered this node through {} of\n\
                 unbroken watching — seventy-seven days — and the last of us stopped in\n\
                 epoch {}.\n\
                 \n\
                 333 is not gone. It is going, and the going takes {} years.",
                epochs(n333_core::extinction::SILENT_EPOCHS_BEFORE_THE_END),
                since.0,
                crate::commands::in_threes(n333_core::extinction::EXTINCTION_YEARS),
            )?;
            match vigil.remaining_at(epoch::unix_now_seconds()) {
                Some(Remaining { years, days }) => writeln!(
                    out,
                    "{} years {days} days remain.",
                    crate::commands::in_threes(years)
                )?,
                None => writeln!(out, "The last of the years has run out.")?,
            }
            // The two things a person in front of this screen cannot work out for
            // themselves, and both of them change what it means: when the count
            // started, and that it is not a countdown that can be paused.
            writeln!(
                out,
                "\nThe count started when the last of us stopped answering, not when you\n\
                 noticed. It has been running the whole time you were watching.\n\
                 \n\
                 One answer ends it. If anybody, anywhere, answers this node, this goes\n\
                 away — and the count is not paused, it is discarded. 333 keeps no record\n\
                 of how close it came."
            )?;
        }
    }
    Ok(())
}

/// Was this client built with arti in it?
///
/// It decides one thing only, and it is the heaviest thing this program says.
const CAN_WALK_THE_UNSEEN_ROAD: bool = cfg!(feature = "tor");

/// "1 epoch" or "N epochs", said the way a person would.
pub(super) fn epochs(count: u64) -> String {
    if count == 1 {
        "1 epoch".to_owned()
    } else {
        format!("{count} epochs")
    }
}
