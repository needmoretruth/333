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

mod json;
mod known;
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
use crate::words::Arg;

/// Which of the three ways of saying it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Show {
    /// Everything, in words.
    Everything,
    /// Every address held and where each came from.
    Sources,
    /// What this node observed, for a program, with no address in it.
    Json,
}

impl Show {
    /// The way asked for by `--sources` and `--json`, which clap keeps apart.
    #[must_use]
    pub(crate) const fn of(sources: bool, json: bool) -> Self {
        if json {
            Self::Json
        } else if sources {
            Self::Sources
        } else {
            Self::Everything
        }
    }

    /// The word that asks for it after `status`, in the screen and over the socket.
    #[must_use]
    pub(crate) const fn word(self) -> &'static str {
        match self {
            Self::Everything => "",
            Self::Sources => "--sources",
            Self::Json => "--json",
        }
    }
}

/// Show where this node stands.
///
/// # Errors
/// Fails if the node's directory cannot be opened or its own record does not verify.
pub(crate) async fn run(common: &Common, show: Show) -> anyhow::Result<()> {
    // Written through one locked handle rather than with `println!`, so that a reader
    // that walks away — `333 status | head` — ends this quietly instead of panicking
    // inside the print macro, where nothing can catch it.
    let mut stdout = std::io::stdout().lock();
    let out = &mut stdout;
    let (node, opened) = Node::open(&common.mistrust(), common.paths.root(), common.keeping)?;
    let now = Epoch::now();
    if show == Show::Json {
        // Nothing but the JSON on standard output, so that it can be piped straight
        // into whatever reads it.
        return json::write(out, &node, now).await;
    }

    name_and_epoch(out, &node, now)?;
    crate::commands::report_opening(&opened);
    writeln!(out)?;
    rest(out, &node, now, show).await
}

/// The two lines every way of saying it but the JSON begins with.
fn name_and_epoch(out: &mut impl std::io::Write, node: &Node, now: Epoch) -> std::io::Result<()> {
    let name = node.identity().node_id().to_string();
    writeln!(out, "{}", words!("status-name", name = name))?;
    writeln!(out, "{}", words!("status-epoch", epoch = now.0))
}

/// Everything after the name and the epoch, in the way asked for.
///
/// # Errors
/// Fails if what is held cannot be read or written out.
async fn rest(
    out: &mut impl std::io::Write,
    node: &Node,
    now: Epoch,
    show: Show,
) -> anyhow::Result<()> {
    known::copies(out, node).await?;
    if show == Show::Sources {
        known::counts(out, node, true).await?;
        writeln!(out)?;
        return known::sources(out, node).await;
    }
    report(out, node, now).await
}

/// All of it, the name and the epoch first, for a node that is already open.
///
/// Shared with the vigil, which answers `333 status` from another terminal with this
/// rather than letting that terminal open files it is writing, so the same flags say
/// the same things whether or not a vigil is running.
///
/// # Errors
/// Fails if what is held cannot be read or written out.
pub(crate) async fn whole(
    out: &mut impl std::io::Write,
    node: &Node,
    now: Epoch,
    show: Show,
) -> anyhow::Result<()> {
    if show == Show::Json {
        return json::write(out, node, now).await;
    }
    name_and_epoch(out, node, now)?;
    writeln!(out)?;
    rest(out, node, now, show).await
}

/// Everything after the name, the epoch and any other copy of this node.
///
/// # Errors
/// Fails if what is held cannot be read.
async fn report(out: &mut impl std::io::Write, node: &Node, now: Epoch) -> anyhow::Result<()> {
    let answering = node.answering(now).await?;
    the_count(out, node, &answering, now).await?;
    writeln!(out)?;
    yourself::this_node(out, node, now).await?;
    writeln!(out)?;
    the_hands(out, node).await?;
    writeln!(out)?;
    known::counts(out, node, false).await?;
    writeln!(out)?;
    what_was_said(out, node, &answering, now).await?;
    writeln!(out)?;
    the_silence(out, node, now).await
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

    // The counts stand in column eleven, where they always have: ANSWERING is nine
    // columns wide and has two spaces after it. Labels are padded by the columns
    // they cover, so a label in wider letters lines up the same.
    let row = |label: String, count: u64| format!("{}{}", padded(&label, 11), counted(count));
    writeln!(out, "{}", row(words!("status-answering"), census.active()))?;
    writeln!(out, "{}", row(words!("status-silent"), census.inactive()))?;
    writeln!(out, "{}─────", padded("", 11))?;
    writeln!(out, "{}", row(words!("status-roll"), census.roll()))?;
    writeln!(out)?;
    let before = now.0.saturating_sub(1);
    writeln!(
        out,
        "{}",
        words!("status-seen", before = before, now = now.0)
    )?;
    if !CAN_WALK_THE_UNSEEN_ROAD {
        writeln!(out, "{}", words!("status-seen-without-tor"))?;
    }
    // The oldest objection to a network like this, answered where the number is rather
    // than in a document nobody opens: a thousand names in one pair of hands is not an
    // attack here, it is a thousand subscriptions.
    writeln!(out, "\n{}", words!("status-how-many-people"))?;
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
    writeln!(out, "{}", words!("status-given-by"))?;
    for (place, member) in hands.iter().enumerate() {
        let name = if place == 0 {
            words!("status-you")
        } else {
            crate::commands::shorten(&NodeId::from_public_key(&member.key).to_string())
        };
        let received = words!("status-received-in", epoch = member.received_in.0);
        writeln!(out, "  {}  {received}", padded(&name, 18))?;
    }
    let furthest = NodeId::from_public_key(&furthest.sponsor).to_string();
    let furthest = padded(&crate::commands::shorten(&furthest), 18);
    writeln!(out, "  {furthest}  {}", words!("status-trail-stops"))?;
    writeln!(out, "\n{}", words!("status-stopped-knowing"))?;
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
        let nothing = words!("status-nothing-said", epoch = now.0);
        writeln!(out, "{nothing}")?;
        return Ok(());
    }

    let said = words!(
        "status-said",
        epoch = now.0,
        spoke = tally.spoken(),
        seen = tally.observed(),
        silent = tally.silent()
    );
    writeln!(out, "{said}")?;
    // Every signal anybody said, in index order, and a count of the ones nobody did.
    // A row of zero carries nothing that the total does not already carry, and three
    // hundred of them would bury the handful that do. Nothing is chosen here: the
    // whole distribution is still the whole distribution.
    let mut said = 0_u16;
    for (signal, count) in tally.distribution().filter(|(_, count)| *count > 0) {
        said += 1;
        let share = share(tally.share(signal));
        let mark = if tally.reached(signal) {
            format!("  {}", words!("status-a-third"))
        } else {
            String::new()
        };
        let (index, count) = (counted(signal.index().into()), counted(count));
        writeln!(out, "  #{index:<4} {count:>5}  {share:>6}{mark}")?;
    }
    let others = words!("status-not-said", others = SIGNAL_COUNT - said);
    writeln!(out, "  {others}")?;
    writeln!(out, "\n{}", words!("status-no-winner"))?;
    Ok(())
}

/// Whether anybody is here, and what is left if nobody is.
async fn the_silence(out: &mut impl std::io::Write, node: &Node, now: Epoch) -> anyhow::Result<()> {
    let vigil = node
        .watched(now)
        .await
        .with_context(|| words!("status-reading-the-watch"))?;
    let remaining = vigil.remaining_at(epoch::unix_now_seconds());
    writeln!(
        out,
        "{}",
        silence(vigil.verdict(), remaining, CAN_WALK_THE_UNSEEN_ROAD)
    )?;
    Ok(())
}

/// What [`the_silence`] says, for a verdict and what is left of the years.
fn silence(verdict: Verdict, remaining: Option<Remaining>, can_walk: bool) -> String {
    use n333_core::extinction::{EXTINCTION_YEARS, SILENT_EPOCHS_BEFORE_THE_END};
    // A build without arti cannot reach an onion address at all, so it has never heard
    // from the members who are hiding and never will. It may report what it saw; it may
    // not say the count reached zero, because a whole class of us was never in its
    // count to begin with.
    if !can_walk && matches!(verdict, Verdict::Ended { .. }) {
        let watched = epochs(SILENT_EPOCHS_BEFORE_THE_END);
        return words!("status-seen-nobody", watched = watched);
    }
    match verdict {
        Verdict::NothingToSay => words!("status-never-answered"),
        Verdict::Alive => words!("status-somebody-is-here"),
        Verdict::Waiting { silent, needed } => words!(
            "status-waiting",
            silent = epochs(silent),
            needed = epochs(needed)
        ),
        Verdict::Ended { since } => {
            let ended = words!(
                "status-nobody-keeping",
                watched = epochs(SILENT_EPOCHS_BEFORE_THE_END),
                since = since.0,
                years = Arg::grouped(EXTINCTION_YEARS)
            );
            let left = match remaining {
                Some(Remaining { years, days }) => {
                    words!("status-remain", years = Arg::grouped(years), days = days)
                }
                None => words!("status-run-out"),
            };
            // The two things a person in front of this screen cannot work out for
            // themselves, and both of them change what it means: when the count
            // started, and that it is not a countdown that can be paused.
            format!("{ended}\n{left}\n\n{}", words!("status-one-answer"))
        }
    }
}

/// Was this client built with arti in it?
///
/// It decides one thing only, and it is the heaviest thing this program says.
const CAN_WALK_THE_UNSEEN_ROAD: bool = cfg!(feature = "tor");

/// "1 epoch" or "N epochs", said the way a person would.
pub(super) fn epochs(count: u64) -> String {
    words!("status-epochs", count = count)
}

/// A share in parts per thousand, to one place: "66.7%", or "—" for no share at all.
///
/// Per hundred and in ten whatever the person counts in, because that is what `%`
/// means: "42.0%" read in twelve is fifty. Cut short rather than rounded, so that no
/// share is ever said to be more than it is.
pub(super) fn share(per_mille: Option<u64>) -> String {
    per_mille.map_or_else(
        || "—".to_owned(),
        |per_mille| {
            words!(
                "status-share",
                whole = Arg::exact(per_mille / 10),
                after = Arg::exact(per_mille % 10)
            )
        },
    )
}

/// A count in the base this client counts in, for a column of them.
fn counted(count: u64) -> String {
    crate::words::count::write(count, false, 0)
}

/// Text padded with spaces to `width` columns of a terminal, for a column of names.
///
/// By what the letters cover rather than how many there are: a label in Korean is
/// half as many letters as columns, and `format!`'s padding counts letters.
pub(super) fn padded(text: &str, width: usize) -> String {
    use unicode_width::UnicodeWidthStr as _;
    let short = width.saturating_sub(text.width());
    format!("{text}{}", " ".repeat(short))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::words::count::Base;
    use n333_core::Identity;
    use n333_core::signal::Signal;
    use n333_core::transfer::{Half, Record};
    use n333_core::utterance::Utterance;
    use n333_core::whereabouts::Whereabouts;

    /// Far enough ahead of any real clock that everything signed in it is later than
    /// the moment the node began keeping count of what it signed.
    const NOW: Epoch = Epoch(100_000);

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("n333-status-test-{name}"));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("creates dir");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700))
                .expect("restricts dir");
        }
        dir
    }

    fn mistrust() -> fs_mistrust::Mistrust {
        fs_mistrust::Mistrust::builder()
            .ignore_prefix(std::env::temp_dir())
            .ignore_environment()
            .build()
            .expect("a buildable Mistrust")
    }

    fn admission(giver: &Identity, taker: &Identity, epoch: u64) -> Vec<Vec<u8>> {
        let epoch = Epoch(epoch);
        let digest = n333_core::subject::DIGEST;
        vec![
            Record::new(giver, taker.public_key(), epoch, digest)
                .seal(Half::Gave, giver)
                .expect("seals"),
            Record::new(taker, giver.public_key(), epoch, digest)
                .seal(Half::Received, taker)
                .expect("seals"),
        ]
    }

    /// A node that heard somebody speak and say where they are, heard a copy of its
    /// own key somewhere else, and was given one address by hand. `joined` puts it on
    /// the roll, two hands down from a key nobody here holds an admission for.
    async fn fixture(name: &str, joined: bool) -> (Node, std::path::PathBuf) {
        let home = scratch(name);
        let (node, _) =
            Node::open(&mistrust(), &home, crate::node::Keeping::TheWindow).expect("opens");
        let someone = Identity::from_seed(&[4; 32]);
        let told = [
            Whereabouts::of(&someone, "198.51.100.77:47113".into(), NOW)
                .seal(&someone)
                .expect("seals"),
            Whereabouts::of(node.identity(), "203.0.113.9:3333".into(), NOW)
                .seal(node.identity())
                .expect("seals"),
            Utterance::of(&someone, Signal::new(7).expect("a signal"), NOW)
                .seal(&someone)
                .expect("seals"),
        ];
        let from = crate::node::sources::Source::Peer {
            name: "333abcdef0123456789".into(),
        };
        node.hear(&told, NOW, &from).await.expect("hears");
        node.given_by_hand("[2001:db8:77::9]:47114", None)
            .await
            .expect("keeps");
        if joined {
            let (first, second) = (Identity::from_seed(&[1; 32]), Identity::from_seed(&[2; 32]));
            node.admit(&admission(&second, &first, 900))
                .await
                .expect("admits");
            node.admit(&admission(&first, node.identity(), 1_000))
                .await
                .expect("admits");
        }
        (node, home)
    }

    /// The whole of `status` for a node, in one language and base, with its name,
    /// which is new every run, as NAME, and the epoch an address was typed in, which
    /// is the real one, as HERE.
    fn shown(node: &Node, show: Show, tag: &str, base: Base) -> String {
        let mut out = Vec::new();
        crate::words::speaking(tag, base, || {
            runtime().block_on(whole(&mut out, node, NOW, show))
        })
        .expect("writes");
        let name = node.identity().node_id().to_string();
        let here = crate::words::count::written_in(base, Epoch::now().0, false, 0);
        String::from_utf8(out)
            .expect("text")
            .replace(&name, "NAME")
            .replace(&format!(", in epoch {here}\n"), ", in epoch HERE\n")
            .replace(&format!(", 에포크 {here}\n"), ", 에포크 HERE\n")
    }

    fn runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_current_thread()
            .build()
            .expect("a runtime")
    }

    /// A fixture, opened once, and the directory it is in removed after.
    fn with_fixture<T>(name: &str, joined: bool, then: impl FnOnce(&Node) -> T) -> T {
        let (node, home) = runtime().block_on(fixture(name, joined));
        let done = then(&node);
        drop(node);
        let _ = std::fs::remove_dir_all(home);
        done
    }

    #[test]
    fn in_english_the_whole_report_says_exactly_what_it_said_before() {
        let text = with_fixture("before", true, |node| {
            shown(node, Show::Everything, "en", Base::Ten)
        });
        assert_eq!(
            text,
            "name     NAME\n\
            epoch    100000\n\
            \n\
            ANOTHER COPY OF THIS NAME\n\
            \n\
            \x20 A statement signed with this node's key, which this node never made, says\n\
            \x20 it is at 203.0.113.9:3333, in epoch 100000.\n\
            \x20 It arrived from 333abcdef0…456789, in epoch 100000.\n\
            \n\
            Either this directory was copied and the copy was started, or somebody else\n\
            has the key. Two nodes on one name contradict each other in every epoch either\n\
            is asked about. Stop one of them; `333 pack` is how a node moves. Nothing here\n\
            stops either copy for you: an old statement can be replayed by anybody, and a\n\
            node that stopped on seeing one could be switched off by whoever holds a copy\n\
            of its key.\n\
            \n\
            ANSWERING  1\n\
            silent     1\n\
            \x20          ─────\n\
            roll       2\n\
            \n\
            That first number is everyone this node holds a signature for in epoch 99999 or\n\
            100000. It is what this node saw. Somebody else saw something else.\n\
            \n\
            How many people that is, this node does not know and cannot find out. What it\n\
            knows is that every one of those names answered in one of those two epochs, and\n\
            will have to answer again in the next, and the one after that, for as long as it\n\
            wants to be counted. If one person is holding a thousand of them, they are\n\
            paying for a thousand of them, hour after hour, and stop being counted the hour\n\
            they stop.\n\
            \n\
            Nothing in the last 333 epochs was ever put to you. Nobody was\n\
            drawn to ask, so there is nothing to have failed. You are neither kept nor\n\
            lapsed; you are simply not yet part of anybody's arithmetic.\n\
            \n\
            Your record is what you wrote down about yourself, in order, signed as you\n\
            went. Every answer you have ever given named where it stood at that moment, so\n\
            its length and its order are not yours to change now. What it concludes is\n\
            still your own word. Nobody has yet signed anything about you, so for now there is only your own\n\
            word for any of it.\n\
            \n\
            Those signatures stay valid for ever. What cannot be recovered is whether the\n\
            people who made them were of us at the time — by then they are gone, and there\n\
            is nobody left to ask. A hundred-year-old record proves that somebody holding\n\
            that key stood behind you, and stops there. 333 does not fix this and does not\n\
            pretend to. The fix is a register of who was true, kept by somebody, for ever,\n\
            and that is the one thing this network will not build.\n\
            \n\
            Your record says nothing at all about 333 of those 333 epochs.\n\
            Nothing here turns that into an absence — a record can only say what a node\n\
            was there to write. It is also why this ratio is not what anybody else reads\n\
            you by: what they read is what they were told about you, by whoever was\n\
            drawn to ask.\n\
            \n\
            GIVEN BY\n\
            \x20 you                 received it in epoch 1000\n\
            \x20 34750f98bd…f3c97e   received it in epoch 900\n\
            \x20 6a3803d5f0…f03827   the trail stops here.\n\
            \n\
            That is where this node stopped knowing, not where it began. The first of us\n\
            was given the file by nobody and has no admission anywhere, and a record this\n\
            node has simply not been handed yet looks exactly the same from here.\n\
            \n\
            KNOWN    2 addresses, by where each was first heard of\n\
            \x20 by hand                1\n\
            \x20 this network           0\n\
            \x20 a meeting point        0\n\
            \x20 from 1 of us           1\n\
            Where each was heard of says nothing about whether anybody answers there.\n\
            `333 status --sources` lists them.\n\
            \n\
            SAID in epoch 100000 — 1 of the 2 of us this node can see spoke, 1 did not.\n\
            \x20 #7        1   50.0%  ← a third of us or more\n\
            \x20 the other 332 of the 333 were not said.\n\
            \n\
            No winner is picked and none of this decides anything. It is what reached\n\
            this node. The node beside you heard something else and is not wrong.\n\
            \n\
            Somebody is here. Nothing further is owed to the arithmetic.\n"
        );
    }

    #[test]
    fn in_english_the_sources_say_what_they_said_before_but_where_to_find_them() {
        let text = with_fixture("sources", true, |node| {
            shown(node, Show::Sources, "en", Base::Ten)
        });
        assert_eq!(
            text,
            "name     NAME\n\
            epoch    100000\n\
            \n\
            ANOTHER COPY OF THIS NAME\n\
            \n\
            \x20 A statement signed with this node's key, which this node never made, says\n\
            \x20 it is at 203.0.113.9:3333, in epoch 100000.\n\
            \x20 It arrived from 333abcdef0…456789, in epoch 100000.\n\
            \n\
            Either this directory was copied and the copy was started, or somebody else\n\
            has the key. Two nodes on one name contradict each other in every epoch either\n\
            is asked about. Stop one of them; `333 pack` is how a node moves. Nothing here\n\
            stops either copy for you: an old statement can be replayed by anybody, and a\n\
            node that stopped on seeing one could be switched off by whoever holds a copy\n\
            of its key.\n\
            \n\
            KNOWN    2 addresses, by where each was first heard of\n\
            \x20 by hand                1\n\
            \x20 this network           0\n\
            \x20 a meeting point        0\n\
            \x20 from 1 of us           1\n\
            Where each was heard of says nothing about whether anybody answers there.\n\
            \n\
            SOURCES\n\
            \n\
            \x20 c5b940ed3f65c391965de8295fc5d25f474fa57b48d36eb10ad363b8539c1b79\n\
            \x20   at     198.51.100.77:47113\n\
            \x20   first  from 333abcdef0…456789, in epoch 100000\n\
            \x20   last   from 333abcdef0…456789, in epoch 100000\n\
            \n\
            \x20 nobody has answered here yet\n\
            \x20   at     [2001:db8:77::9]:47114\n\
            \x20   first  by hand, in epoch HERE\n\
            \x20   last   by hand, in epoch HERE\n"
        );
    }

    /// What the lines no fixture reaches said before their words moved into a
    /// catalog, byte for byte, beside what the catalog says now.
    #[test]
    fn in_english_every_other_moved_line_says_exactly_what_it_said_before() {
        let ended = Verdict::Ended {
            since: Epoch(99_000),
        };
        let left = Some(Remaining {
            years: 19_682,
            days: 7,
        });
        let pairs = crate::words::speaking("en", Base::Ten, || {
            [
                (
                    words!("status-known-nowhere"),
                    "KNOWN    nowhere to knock yet. An invitation given to `333 ping` or\n\
                     \x20        `333 join` is kept, and the vigil knocks there from then on."
                        .to_owned(),
                ),
                (
                    words!("status-known-held", held = 1_usize),
                    "KNOWN    1 address, by where each was first heard of".to_owned(),
                ),
                (epochs(1), "1 epoch".to_owned()),
                (epochs(333), "333 epochs".to_owned()),
                (share(Some(667)), "66.7%".to_owned()),
                (share(None), "—".to_owned()),
                (
                    words!("status-seen-without-tor"),
                    "This build cannot walk the unseen road, so none of us who are hiding are in\n\
                     that number, and none of us ever will be."
                        .to_owned(),
                ),
                (
                    words!("status-nothing-said", epoch = 89_612_u64),
                    "Nobody has said anything in epoch 89612. There are 333 things that can be\n\
                     said and no words for any of them yet."
                        .to_owned(),
                ),
                (
                    silence(Verdict::NothingToSay, None, true),
                    "No one has ever answered this node. That is not evidence of anything: it\n\
                     is what a node looks like before it has been anywhere."
                        .to_owned(),
                ),
                (
                    silence(Verdict::Waiting { silent: 1, needed: 332 }, None, true),
                    "Nobody has answered for 1 epoch. This node has said nothing about it and will\n\
                     not until 332 epochs, and only then if it is running for every one of them."
                        .to_owned(),
                ),
                (
                    silence(ended, None, false),
                    "Nobody has answered this node through 333 epochs of unbroken watching, and this\n\
                     build will not call that the end. It cannot walk the unseen road, so it has\n\
                     never heard from any of us who are hiding and never will. What it can say is\n\
                     that it has seen nobody, and that is not the same sentence."
                        .to_owned(),
                ),
                (silence(ended, left, true), format!("{THE_END}\n19,682 years 7 days remain.\n\n{NO_PAUSE}")),
                (silence(ended, None, true), format!("{THE_END}\nThe last of the years has run out.\n\n{NO_PAUSE}")),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

    /// The end, as it was said before its words moved.
    const THE_END: &str = "NOBODY IS KEEPING 333\n\
         \n\
         You are the only one here. Nobody has answered this node through 333 epochs of\n\
         unbroken watching — seventy-seven days — and the last of us stopped in\n\
         epoch 99000.\n\
         \n\
         333 is not gone. It is going, and the going takes 19,683 years.";

    /// What follows the count of the years, as it was said before its words moved.
    const NO_PAUSE: &str = "The count started when the last of us stopped answering, not when you\n\
         noticed. It has been running the whole time you were watching.\n\
         \n\
         One answer ends it. If anybody, anywhere, answers this node, this goes\n\
         away — and the count is not paused, it is discarded. 333 keeps no record\n\
         of how close it came.";

    #[test]
    fn in_twelve_every_count_changes_and_no_name_address_or_share_does() {
        let text = with_fixture("twelve", true, |node| {
            shown(node, Show::Everything, "en", Base::Twelve)
        });
        // 100000 is 4·12⁴ + 9·12³ + 10·12² + 5·12 + 4.
        assert!(
            text.starts_with("name     NAME\nepoch    49\u{218A}54\n"),
            "{text}"
        );
        assert!(
            text.contains("at 203.0.113.9:3333, in epoch 49\u{218A}54."),
            "{text}"
        );
        assert!(
            text.contains("  the other 238 of the 333 were not said."),
            "{text}"
        );
        assert!(
            text.contains("received it in epoch 6\u{218B}4\n"),
            "1000: {text}"
        );
        assert!(
            text.contains("  #7        1   50.0%  "),
            "a share is per hundred: {text}"
        );
        assert!(!text.contains("100000"), "{text}");
        let (share, ended) = crate::words::speaking("en", Base::Twelve, || {
            let ended = Verdict::Ended { since: Epoch(144) };
            let left = Some(Remaining {
                years: 19_682,
                days: 13,
            });
            (share(Some(667)), silence(ended, left, true))
        });
        assert_eq!(share, "66.7%", "a share is per hundred, in ten");
        assert!(ended.contains("stopped in\nepoch 100.\n"), "{ended}");
        assert!(ended.contains("takes \u{218B},483 years."), "{ended}");
        assert!(
            ended.contains("\n\u{218B},482 years 11 days remain."),
            "{ended}"
        );
    }

    #[test]
    fn in_korean_the_columns_of_the_whole_report_line_up() {
        use unicode_width::UnicodeWidthStr as _;
        let text = with_fixture("korean", true, |node| {
            shown(node, Show::Everything, "ko", Base::Ten)
        });
        let lines: Vec<&str> = text.lines().collect();
        let starting = |prefix: &str| lines.iter().position(|l| l.starts_with(prefix));
        for keyword in ["이름", "에포크"] {
            let at = starting(keyword).unwrap_or_else(|| panic!("{keyword}: {text}"));
            let line = lines[at];
            assert_eq!(
                crate::words::layout::where_the_words_begin(line),
                9,
                "{line}"
            );
        }
        // The count table: every count begins in the twelfth column.
        let answering = starting(&words_in("ko", || words!("status-answering"))).expect("table");
        for line in &lines[answering..answering + 4] {
            let (label, count) =
                line.split_at(line.trim_end_matches(|c: char| !c.is_whitespace()).len());
            assert_eq!(label.width(), 11, "{line:?}");
            assert!(!count.is_empty(), "{line:?}");
        }
        // Every row of the other two tables ends, or begins its words, in one column.
        let rows: Vec<&&str> = lines
            .iter()
            .filter(|l| l.starts_with("  ") && !l.starts_with("   "))
            .collect();
        let given = rows
            .iter()
            .filter(|l| l.contains("에포크 ") && l.ends_with("받음"))
            .count();
        assert_eq!(given, 2, "{text}");
        for row in rows.iter().filter(|l| l.ends_with("받음")) {
            assert_eq!(
                row.find("에포크").map(|at| row[..at].width()),
                Some(22),
                "{row:?}"
            );
        }
        let known: Vec<_> = rows
            .iter()
            .filter(|l| l.ends_with(char::is_numeric))
            .collect();
        assert!(known.len() >= 4, "{text}");
        for row in known {
            assert_eq!(row.width(), 26, "{row:?}");
        }
    }

    fn words_in(tag: &str, say: impl FnOnce() -> String) -> String {
        crate::words::speaking(tag, Base::Ten, say)
    }
}
