//! What the client can be asked to do.
//!
//! One file per command. Each one owns its own output text, because the words a
//! person reads are part of the command, not a detail of it.

pub(crate) mod bootstrap;
pub(crate) mod elsewhere;
pub(crate) mod hours;
pub(crate) mod id;
pub(crate) mod join;
pub(crate) mod languages;
pub(crate) mod moved;
pub(crate) mod pack;
pub(crate) mod ping;
pub(crate) mod say;
pub(crate) mod serve;
pub(crate) mod service;
pub(crate) mod status;
pub(crate) mod unpack;
mod unseen;

use std::time::Duration;

pub(crate) use unseen::{unseen, unseen_now};

use n333_net::Exchange;

use crate::words::Arg;

/// Options every command shares.
#[derive(Debug, Clone)]
pub(crate) struct Common {
    /// Where this node keeps its files.
    pub(crate) paths: crate::paths::NodePaths,
    /// How long to wait for a step that talks to the Tor network.
    pub(crate) timeout: Duration,
    /// How much of the past this node holds on to.
    pub(crate) keeping: crate::node::Keeping,
    /// How to get into Tor where the ordinary way in is blocked. Empty is ordinary.
    ///
    /// Here rather than on the one command that starts Tor, because every command
    /// that can reach an onion address can need it, and a person on a network that
    /// blocks Tor needs it on all of them or on none.
    /// Behind a lock because a running vigil can be told to add one, from its screen
    /// or from another terminal, and the next Tor start is the one that reads it. Once
    /// Tor is up, adding is a thing that has no effect, and the vigil says so rather
    /// than pretending.
    // Read where Tor is started, which is a build with arti in it. Every shipped
    // build has one; the bare build with no features is checked and not released, and
    // in that one this is carried and never opened.
    #[cfg_attr(not(feature = "tor"), allow(dead_code))]
    pub(crate) bridges: std::sync::Arc<std::sync::Mutex<n333_net::bridges::Bridges>>,
    /// Whether to accept state directories other users can read.
    ///
    /// One bool, not two policies: arti and this client have to agree about whether a
    /// directory is private, or the client would write a seed into a directory arti
    /// then refuses to start in.
    pub(crate) trust_directory_permissions: bool,
}

/// Say so when this machine's clock makes the node unusable.
///
/// A clock that reads before 1970 gives epoch 0, which is honest — there is no
/// authority here to appeal to about what time it is — and leaves a node that is
/// refused by everybody with nothing to go on. Every handover it attempts is turned
/// away for being in the wrong hour, and the word it is given for that is "refused".
pub(crate) fn check_the_clock(now: n333_core::Epoch) {
    if now.0 != 0 {
        return;
    }
    aloud_in!("commands-clock-at-zero", epoch = now.0);
}

/// A name, short enough to sit in a column and long enough to be that name.
///
/// Ten from the front, six from the back. The front is where the `333` is and where
/// two names differ if they differ at all; the back is what a person checks when they
/// already know which name they are looking for.
pub(crate) fn shorten(name: &str) -> String {
    let (head, tail) = name.split_at(name.len().min(10));
    match tail.len() {
        0..=6 => name.to_owned(),
        _ => format!("{head}…{}", tail.get(tail.len() - 6..).unwrap_or_default()),
    }
}

/// The one address written into this client.
///
/// Not a node and not a way in: it hands over no file, joins no roll, and issues no
/// invitation. It is a page that says what this is and where the code is, for somebody
/// who has heard the name and has nobody to ask. The specification refuses hardcoded
/// addresses because a hardcoded NODE makes the network depend on one machine staying
/// up and on one person being able to make new entry points. Neither is true of a page:
/// take it away and every node carries on exactly as it was.
pub(crate) const THE_PLACE: &str = "the333.dev";

/// What is said before speaking of 333, or speaking one of the 333.
///
/// The Recommendations ask for it at the front of every prayer and every telling. A
/// client that speaks on somebody's behalf says it too.
pub(crate) const INVOCATION: &str = "To 333 I offer 333, and I speak 333.";

impl Common {
    /// How strictly to judge the permissions on this node's directory.
    ///
    /// The default consults `$FS_MISTRUST_DISABLE_PERMISSIONS_CHECKS`, which is what
    /// arti does with the same setting, so one variable governs both.
    #[must_use]
    pub(crate) fn mistrust(&self) -> fs_mistrust::Mistrust {
        if self.trust_directory_permissions {
            fs_mistrust::Mistrust::new_dangerously_trust_everyone()
        } else {
            fs_mistrust::Mistrust::new()
        }
    }
}

/// Start a Tor client, giving up after the shared timeout.
///
/// Only reached when an address asks for Tor, or when `serve --tor` was used. A node
/// that is not hiding never calls this and never pays for it.
///
/// Arti retries bootstrap 128 times by default, so without a deadline a broken
/// network hangs instead of failing.
///
/// # Errors
/// Fails if the timeout elapses or arti cannot start.
#[cfg(feature = "tor")]
pub(crate) async fn bootstrap(common: &Common) -> anyhow::Result<n333_net::tor::Client> {
    use anyhow::Context as _;
    // Copied out under the lock and not held across the wait: starting Tor is minutes,
    // and a lock held for minutes is a screen that stops answering keys.
    let bridges = common
        .bridges
        .lock()
        .map_or_else(|held| held.into_inner().clone(), |held| held.clone());
    if bridges.is_empty() {
        aloud_in!("commands-waking");
    } else {
        aloud_in!("commands-waking-through", bridges = bridges.lines.len());
    }
    tokio::time::timeout(
        common.timeout,
        n333_net::tor::bootstrap(
            &common.paths.tor(),
            common.trust_directory_permissions,
            &bridges,
        ),
    )
    .await
    // The deadline's own words are "deadline has elapsed", which says it twice.
    .map_err(|_| {
        let seconds = common.timeout.as_secs();
        anyhow::anyhow!(words!("commands-no-tor", seconds = seconds))
    })?
    .with_context(|| words!("commands-starting-tor"))
}

/// What opening a node found, said once at the start.
///
/// Only the lines that are true of this node right now. A fresh node has no record
/// and no members, and saying "0 members" every start would train the operator to
/// ignore the line that matters when it is not zero.
pub(crate) fn report_opening(opened: &crate::node::Opened) {
    crate::named::report(opened.origin, &opened.home);
    if opened.chain_truncated != 0 {
        // A byte count, which names a size rather than counting anything a person
        // tallies, and is the number a file manager would show for the same bytes.
        let bytes = Arg::exact(opened.chain_truncated);
        aloud_in!("commands-torn", bytes = bytes);
    }
    if opened.chain_length != 0 {
        aloud_in!("commands-record", epochs = opened.chain_length);
    }
    if opened.witnessed != 0 {
        aloud_in!("commands-witnessed", statements = opened.witnessed);
    }
    // Being on the roll and having nothing signed about you is the one failure this
    // client can see from the inside and a person cannot see at all. Everything looks
    // right: the node starts, it dials out, it trades statements, it says nothing is
    // wrong. What is wrong is that nobody can dial back, so nobody asks, so nothing is
    // ever witnessed, so the window fills with epochs that do not count. Said only
    // after a few of them, and only when there is somebody who could have asked.
    if unseen(opened) {
        aloud_in!("commands-unseen");
    }
    if opened.members != 0 {
        if opened.members == 1 {
            aloud_in!("commands-roll-alone");
        } else {
            aloud_in!("commands-roll", members = opened.members);
        }
    }
    if opened.addresses != 0 {
        aloud_in!("commands-known", addresses = opened.addresses);
    }
    crate::node::say_what_the_sources_held(opened);
    if opened.has_the_file {
        aloud_in!("commands-holding");
    }
    if opened.keeping == crate::node::Keeping::Everything {
        aloud_in!("commands-keeping");
    }
    if opened.read.unreadable != 0 {
        aloud_in!("commands-ignored", admissions = opened.read.unreadable);
    }
}

/// What a trade of statements changed, when it changed anything.
///
/// Silent when it changed nothing, which is the ordinary case once a node has settled:
/// a line every time would be a line every 333 minutes per neighbour saying nothing
/// happened.
pub(crate) fn report_heard(heard: &crate::node::Heard) {
    if heard.addresses != 0 {
        aloud_in!("commands-learned-where", addresses = heard.addresses);
    }
    if heard.members != 0 {
        if heard.were != 0 && heard.members >= heard.were {
            // One meeting brought more of us than this node had ever held. Two halves
            // of a network that had not spoken look exactly like this from one side.
            aloud_in!(
                "commands-rejoined",
                members = heard.members,
                were = heard.were
            );
        } else {
            aloud_in!("commands-learned-names", members = heard.members);
        }
    }
    if heard.said != 0 {
        aloud_in!("commands-heard", speakers = heard.said);
    }
    if heard.witnessed != 0 {
        aloud_in!("commands-carried", statements = heard.witnessed);
    }
}

/// The two sentences a handover actually puts a signature under, read back.
///
/// They are not a summary of the record. They are the record: those two lines, in two
/// hands, are the whole of what an admission is, and printing anything else in their
/// place would be printing a paraphrase of the only thing either node signed.
///
/// The closing line is deliberately identical at both ends. It is the one formula both
/// sides of the act speak, which is what a pair is.
#[must_use]
pub(crate) fn what_was_signed(transfer: &n333_core::Transfer, ours_was_the_giving: bool) -> String {
    let epoch = transfer.epoch().0;
    // Two messages rather than two pronouns handed in: which of the two said which
    // sentence changes the whole sentence in most languages, not one word of it.
    if ours_was_the_giving {
        words!("commands-signed-giving", epoch = epoch)
    } else {
        words!("commands-signed-taking", epoch = epoch)
    }
}

/// The line that says a name was found, said as the naming it is.
///
/// The number is how many keys were made and passed over. The one that was called is
/// not among them, which is the whole difference between reading a loop and reading
/// what happened.
#[must_use]
pub(crate) fn naming(not_called: u64) -> String {
    match not_called {
        0 => words!("commands-called-first"),
        not_called => words!("commands-called", not_called = not_called),
    }
}

/// Say when a node had more to pass on than would fit in one run.
///
/// A cap that nothing reports is a cap that reads as "everything was sent" right up
/// until somebody wonders why the roll stopped growing.
pub(crate) fn report_left_behind(tidings: &crate::node::Tidings) {
    if tidings.left_behind != 0 {
        aloud_in!("commands-brimming", statements = tidings.left_behind);
    }
}

/// One line describing what a completed exchange showed.
///
/// The parenthesis is the part that matters and the part most easily overstated: one
/// of these two exchanges proves the peer was awake and the other does not.
#[must_use]
pub(crate) fn describe(exchange: &Exchange) -> String {
    let liveness = if exchange.proves_peer_was_live {
        words!("commands-answered-the-challenge")
    } else {
        words!("commands-spoke-first")
    };
    words!(
        "commands-exchange",
        node = exchange.peer.node_id.to_string(),
        epoch = exchange.peer.heartbeat.epoch,
        clocks = clocks(exchange.clocks_apart_ms),
        liveness = liveness
    )
}

/// How far apart two clocks are, said at a scale somebody can act on.
///
/// A minute is nothing to this protocol and everything to the person reading it: it is
/// the difference between a machine that is fine and a machine whose owner is about to
/// stop being witnessed by everybody whose clock agrees with everybody else's. Under a
/// few seconds is the trip the message made and is not worth a word.
fn clocks(apart_ms: i64) -> String {
    const NOT_WORTH_SAYING: i64 = 5_000;
    if apart_ms.abs() < NOT_WORTH_SAYING {
        return words!("commands-clocks-together");
    }
    let seconds = apart_ms.unsigned_abs() / 1_000;
    let (hours, minutes) = (seconds / 3_600, (seconds % 3_600) / 60);
    let apart = if hours != 0 {
        words!(
            "commands-hours-and-minutes",
            hours = hours,
            minutes = Arg::padded(minutes, 2)
        )
    } else if minutes != 0 {
        words!(
            "commands-minutes-and-seconds",
            minutes = minutes,
            seconds = Arg::padded(seconds % 60, 2)
        )
    } else {
        words!("commands-seconds", seconds = seconds)
    };
    if apart_ms > 0 {
        words!("commands-clocks-ahead", apart = apart)
    } else {
        words!("commands-clocks-behind", apart = apart)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What the lines said before their words moved into a catalog, byte for byte,
    /// beside what the catalog says now.
    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        use crate::words::count::Base;
        let pairs = crate::words::speaking("en", Base::Ten, || {
            [
                (naming(0), "called   the first key made was called."),
                (
                    naming(1),
                    "called   1 key was made and not called. this one was.",
                ),
                (
                    naming(4_021),
                    "called   4021 keys were made and not called. this one was.",
                ),
                (
                    words!("commands-clock-at-zero", epoch = 0_u64),
                    "epoch    0. This machine's clock says it is 1970, so this node believes it is\n\
                     \x20        at the beginning of time. Nobody will hand it anything and nobody will\n\
                     \x20        witness it until the clock is set.",
                ),
                (
                    words!("commands-torn", bytes = Arg::exact(17)),
                    "torn     17 bytes of an unfinished entry were dropped from the record",
                ),
                (
                    words!("commands-record", epochs = 1_u64),
                    "record   1 epoch already answered for, none of them open to revision",
                ),
                (
                    words!("commands-record", epochs = 40_u64),
                    "record   40 epochs already answered for, none of them open to revision",
                ),
                (
                    words!("commands-witnessed", statements = 12_usize),
                    "witness  12 statements other keys signed about this node. They are kept\n\
                     \x20        after the epochs they belong to are gone, because nothing else of\n\
                     \x20        them survives the window.",
                ),
                (
                    words!("commands-unseen"),
                    "unseen   nothing has been signed about this node, in any epoch. Reaching out\n\
                     \x20        works and being reached does not, and only the second one is counted:\n\
                     \x20        whoever is drawn to ask has to arrive. Two things do this. A router\n\
                     \x20        that does not send port 3333 to this machine, and an address nobody\n\
                     \x20        was given. `serve --tor` needs neither — an onion address is reachable\n\
                     \x20        from behind any router, and this client already carries Tor.",
                ),
                (
                    words!("commands-roll-alone"),
                    "roll     1 of us, which is this node",
                ),
                (
                    words!("commands-roll", members = 7_usize),
                    "roll     7 of us",
                ),
                (
                    words!("commands-known", addresses = 3_usize),
                    "known    where 3 of us said to look",
                ),
                (
                    words!("commands-holding"),
                    "holding  the file, and able to pass it on",
                ),
                (
                    words!("commands-keeping"),
                    "keeping  everything, for ever. It buys this node nothing: every statement\n\
                     \x20        carries its own signature and verifies the same wherever it was\n\
                     \x20        kept. There is no archive of record and there is no archivist.",
                ),
                (
                    words!("commands-ignored", admissions = 2_usize),
                    "ignored  2 admissions that could not be read",
                ),
                (
                    words!("commands-learned-where", addresses = 5_usize),
                    "learned  where 5 more of us are",
                ),
                (
                    words!("commands-rejoined", members = 30_usize, were = 9_usize),
                    "rejoined 30 more of us by name, from a node that knew 9. There were\n\
                     \x20        two of us and now the counting is one count.",
                ),
                (
                    words!("commands-learned-names", members = 4_usize),
                    "learned  4 more of us by name",
                ),
                (
                    words!("commands-heard", speakers = 6_usize),
                    "heard    6 of us speak",
                ),
                (
                    words!("commands-carried", statements = 8_usize),
                    "carried  8 statements about epochs still open",
                ),
                (
                    words!(
                        "commands-exchange",
                        node = "333abc",
                        epoch = 89_612_u64,
                        clocks = clocks(0),
                        liveness = words!("commands-answered-the-challenge")
                    ),
                    "witness  333abc  epoch 89612  clocks together  (answered the challenge we chose)",
                ),
                (
                    words!("commands-spoke-first"),
                    "spoke first, which proves only that it spoke",
                ),
                (
                    words!("commands-waking"),
                    "waking   Tor. the unseen road takes a while to open.",
                ),
                (
                    words!("commands-waking-through", bridges = 1_usize),
                    "waking   Tor, through 1 bridge. the unseen road takes a while to open.",
                ),
                (
                    words!("commands-waking-through", bridges = 3_usize),
                    "waking   Tor, through 3 bridges. the unseen road takes a while to open.",
                ),
                (
                    words!("commands-no-tor", seconds = 120_u64),
                    "no Tor connection after 120 s",
                ),
                (words!("commands-starting-tor"), "starting the Tor client"),
                (
                    words!("commands-signed-giving", epoch = 89_612_u64),
                    "signed   you said: I handed the file to you in epoch 89612.\n\
                     \x20        they said: I received the file from you in epoch 89612.\n\
                     \x20        it is written in two hands, and neither hand can take it back.",
                ),
                (
                    words!("commands-signed-taking", epoch = 89_612_u64),
                    "signed   they said: I handed the file to you in epoch 89612.\n\
                     \x20        you said: I received the file from you in epoch 89612.\n\
                     \x20        it is written in two hands, and neither hand can take it back.",
                ),
                (
                    words!("commands-brimming", statements = 40_usize),
                    "brimming 40 statements would not fit in one run and wait for the next",
                ),
                (clocks(4_999), "clocks together"),
                (clocks(-42_000), "their clock 42s behind ours"),
                (clocks(185_000), "their clock 3m 05s ahead of ours"),
                (clocks(-7_380_000), "their clock 2h 03m behind ours"),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

    #[test]
    fn in_twelve_the_counts_change_and_the_names_do_not() {
        use crate::words::count::Base;
        let (called, apart, exchange) = crate::words::speaking("en", Base::Twelve, || {
            let exchange = words!(
                "commands-exchange",
                node = "333abc",
                epoch = 144_u64,
                clocks = clocks(0),
                liveness = words!("commands-spoke-first")
            );
            (naming(333), clocks(7_140_000), exchange)
        });
        assert_eq!(
            called,
            "called   239 keys were made and not called. this one was."
        );
        assert_eq!(
            apart, "their clock 1h 4\u{218B}m ahead of ours",
            "59 minutes"
        );
        assert!(
            exchange.starts_with("witness  333abc  epoch 100  "),
            "{exchange}"
        );
    }

    #[test]
    fn in_korean_every_line_this_file_says_begins_its_words_in_the_ninth_column() {
        use crate::words::count::Base;
        use crate::words::layout::{COLUMN, where_the_words_begin};
        let lines = crate::words::speaking("ko", Base::Ten, || {
            [
                words!("commands-waking"),
                words!("commands-waking-through", bridges = 2_usize),
                words!("commands-signed-giving", epoch = 89_612_u64),
                words!("commands-signed-taking", epoch = 89_612_u64),
                words!("commands-brimming", statements = 40_usize),
                naming(3),
                words!("commands-unseen"),
                words!("commands-rejoined", members = 30_usize, were = 9_usize),
            ]
        });
        for line in &lines {
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

    #[test]
    fn a_shortened_name_keeps_both_ends_and_is_left_alone_when_it_is_short() {
        let name = "333ac0bdd148d7f9a783194ee6a9102c2e53b1227a8a41e7621f133fdba16cd4";
        assert_eq!(shorten(name), "333ac0bdd1…a16cd4");
        assert!(
            name.starts_with("333ac0bdd1"),
            "the front is where the 333 is"
        );
        assert!(name.ends_with("a16cd4"), "the back is what a person checks");
        assert_eq!(shorten("333"), "333", "a short name is already itself");
    }
}
