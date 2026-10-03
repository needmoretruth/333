//! `333 run` (once `333 serve`) — answer heartbeats and challenges until stopped.
//!
//! By default this opens a socket and nothing else: no Tor, no bootstrap, no wait.
//! A socket only answers strangers if the router in front of it was told to send the
//! port here, which on most home networks nobody has done. [`reach`] knocks on this
//! node's own outside address and says whether that is the case, rather than leaving
//! the person to guess.
//!
//! `--tor` publishes an onion address instead. It needs no port opened and no router
//! touched, because the node builds its own way in from the inside out, and it is
//! therefore the way in that works from an ordinary home connection.
//!
//! Each way in is its own door with its own places ([`door`]), and what a peer can ask
//! for once it is through is [`answering`].

pub(crate) mod answering;
mod carrying;
mod door;
mod ending;
mod invitation;
#[cfg(test)]
mod learning;
mod neighbours;
mod onion;
mod reach;
mod socket;
mod told;

use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{Context as _, bail};
use n333_core::Epoch;
use n333_net::{Invite, PeerAddress};
use tokio::sync::watch;

use crate::commands::{Common, hours};
use crate::dial::{Dialer, Roads};
use crate::node::Node;

use door::Door;
use invitation::say_the_invitation;
use neighbours::greet_the_neighbours;
use socket::{answer_direct, listen};

/// Everything about how one vigil is kept.
///
/// One struct rather than seven parameters: they are read together, they are written
/// down together where the command line is read, and seven of anything in a row is a
/// place where two of them get swapped and it still compiles.
pub(crate) struct Vigil {
    /// The socket to listen on, or `None` to listen only through Tor.
    pub(crate) bind: Option<SocketAddr>,
    /// Whether to publish an onion address as well.
    pub(crate) tor: bool,
    /// What this node tells others to reach it at, when it cannot work that out.
    pub(crate) announce: Option<PeerAddress>,
    /// Whether to say on the local network that something here speaks 333.
    pub(crate) nearby: bool,
    /// The meeting point, or `None` to use none.
    pub(crate) meet: Option<String>,
    /// Whether to say the lines rather than draw the screen.
    pub(crate) plain: bool,
    /// Whether to ask the router to send the port to this machine.
    pub(crate) ask_the_router: bool,
}

/// Run until interrupted, answering everyone who arrives.
///
/// # Errors
/// Fails if the node cannot be opened, if neither way of listening was asked for, if
/// a socket cannot be bound, or if Tor was asked for and cannot start.
pub(crate) async fn run(common: &Common, how: Vigil) -> anyhow::Result<()> {
    let Vigil {
        bind,
        tor,
        announce,
        nearby,
        meet,
        plain,
        ask_the_router,
    } = how;
    if bind.is_none() && !tor {
        bail!(words!("serve-nothing-listening"));
    }
    // Claimed before anything is said, so that the first lines — the name, the
    // invitation — are in the screen's own pane rather than printed underneath it and
    // wiped by the first drawing. The smallest edition has no screen to take.
    #[cfg(feature = "screen")]
    let watching = the_screen(plain);
    #[cfg(feature = "screen")]
    let drawing = watching.is_some();
    #[cfg(not(feature = "screen"))]
    let _ = plain;
    let (node, opened) = Node::open(&common.mistrust(), common.paths.root(), common.keeping)?;
    let node = Arc::new(node);
    aloud_in!("serve-name", name = node.identity().node_id().to_string());
    crate::commands::report_opening(&opened);
    // The one thing a person who has just downloaded this needs to be told, in the one
    // place they will be sitting when they wonder. `status` says it; `serve` is where
    // they wait, and it was saying everything except this.
    if !opened.has_the_file {
        aloud_in!("serve-waiting-for-the-file");
    }
    aloud_in!("serve-hand");

    // A node that answers on no socket is hiding, and a hiding node that dials
    // clearnet peers has shown its address itself, at the far end, where it can be
    // written down.
    let dialer = Dialer::travelling(
        common.clone(),
        if bind.is_none() {
            Roads::OnlyUnseen
        } else {
            Roads::Whichever
        },
    );
    // Where this node will tell others to look, once it knows. Empty until a listener
    // has an address worth handing out, and written again if the onion address comes
    // up later: an onion address is reachable from anywhere and a socket address may
    // not be, so the one that arrives last is the one worth publishing.
    let (found_address, address) = watch::channel(announce.clone());
    if let Some(announce) = &announce {
        aloud_in!(
            "serve-invite",
            invitation = Invite::to(announce.clone()).to_string()
        );
    }
    // The address the socket actually got, which is not the one that was asked for
    // when the port was left to the system to choose.
    let mut bound_at: Option<SocketAddr> = None;
    let mut listening = tokio::task::JoinSet::new();
    // What the person types into the screen, carried out where the dialler is. The
    // smallest edition has no screen, and its sender is dropped at once.
    let (asked, orders) = tokio::sync::mpsc::unbounded_channel();
    #[cfg(feature = "screen")]
    if let Some(lines) = watching {
        listening.spawn(crate::screen::keep(Arc::clone(&node), lines, asked.clone()));
    }
    drop(asked);
    let (order_node, order_common, order_dialer) =
        (Arc::clone(&node), common.clone(), dialer.clone());

    if let Some(bind) = bind {
        let listener = listen(bind).await?;
        // True the instant the socket is bound, which is why it is printed here.
        let bound = listener.address()?;
        bound_at = Some(bound);
        aloud_in!("serve-answer", bound = bound.to_string());
        if announce.is_none() {
            say_the_invitation(bound, &found_address);
        }
        // Only a node that answers on a socket says anything on the local network.
        // Browsing is not the quiet half of it — asking the whole network out loud
        // whether anybody here speaks 333 is the same disclosure as answering it — so
        // a hiding node does neither.
        if nearby {
            match n333_net::Nearby::start(bound.port()) {
                Ok(nearby) => {
                    aloud_in!("serve-nearby");
                    let (node, dialer) = (Arc::clone(&node), dialer.clone());
                    listening.spawn(greet_the_neighbours(node, dialer, nearby));
                }
                // What fails here is this machine's responder starting, not the network
                // declining to carry anything.
                Err(e) => aloud_in!("serve-nearby-failed", why = e.to_string()),
            }
        }
        let node = Arc::clone(&node);
        listening.spawn(async move { answer_direct(listener, node, Door::new()).await });
    }

    if tor {
        // Its own door, so that a stalled circuit cannot shut the socket and a full
        // socket cannot shut the unseen road.
        let node = Arc::clone(&node);
        listening.spawn(onion::answer(
            dialer.clone(),
            node,
            Door::new(),
            found_address.clone(),
        ));
    }

    // Where a node that nobody introduced goes looking. It reads what others left and
    // leaves whatever address it has that a stranger could actually dial — which for a
    // node behind a router is the onion address and nothing else, since that is the
    // only one of the two that answers when somebody knocks.
    let board = meet.map(|place| hours::Board::at(&place));
    if let Some(board) = &board {
        aloud_in!("serve-meet", place = board.place());
    }
    let lent = bound_at.map(|bound| {
        let (board, dialer, node) = (board.clone(), dialer.clone(), Arc::clone(&node));
        reach::tell_them(
            board,
            dialer,
            node,
            bound,
            found_address.clone(),
            ask_the_router,
        )
    });

    // What the screen asked for, and what other terminals hand over, done where the
    // dialler is. Before the hours, so that a person who types at once is answered
    // rather than queued behind a round. Not one of the tasks the vigil waits on: with
    // no screen and no socket it has nobody to listen to and ends, and the vigil must
    // not end with it.
    let (tell, told) = tokio::sync::mpsc::unbounded_channel();
    let _taking_orders = told::listen(common.paths.root(), tell);
    let asked_to_stop = Arc::new(tokio::sync::Notify::new());
    tokio::spawn(carrying::until_nobody_asks(
        orders,
        told,
        carrying::Carrier {
            node: order_node,
            common: order_common,
            dialer: order_dialer,
            found_address,
            unseen: std::sync::Mutex::new(None),
            ending: Arc::clone(&asked_to_stop),
        },
    ));

    // The hours run alongside the listeners rather than after them: answering is what
    // this node owes others, and keeping the hours is what it owes itself.
    listening.spawn(hours::keep(Arc::clone(&node), dialer, address, board));

    // No line here saying the vigil has begun: with --no-direct it would not be true
    // yet. Each listener announces itself at the moment it can actually answer.
    let asked_to_end = tokio::select! {
        // Nothing here is supposed to finish: the listeners loop, and so do the hours.
        // The screen does, when the person watching leaves, and that is the end of the
        // vigil rather than the end of one part of it.
        finished = listening.join_next() => match finished {
            Some(finished) => {
                finished.with_context(|| words!("serve-listener-stopped"))??;
                false
            }
            None => return Ok(()),
        },
        // Ctrl-C, a service manager stopping it, a terminal closing under it: one
        // ending, however it was asked for.
        () = ending::asked() => true,
        // `333 stop` from another terminal, through the socket.
        () = asked_to_stop.notified() => true,
    };
    // A screen still drawing when the vigil was asked to end from outside is stopped
    // first and the terminal given back, so that what follows is printed on a terminal
    // rather than into a drawing nobody will clear.
    #[cfg(feature = "screen")]
    if asked_to_end && drawing {
        listening.shutdown().await;
        let _ = ratatui::try_restore();
    }
    #[cfg(not(feature = "screen"))]
    let _ = asked_to_end;
    if let Some(lent) = lent {
        lent.give_back().await;
    }
    farewell(node.joined_in().await.is_some());
    Ok(())
}

/// Take the terminal for a screen, if there is a terminal and it was not refused.
///
/// Everything said from here on goes to the screen instead of to standard output. A
/// build without the screen in it has nothing to decide.
#[cfg(feature = "screen")]
fn the_screen(plain: bool) -> Option<tokio::sync::mpsc::UnboundedReceiver<String>> {
    if plain || !crate::screen::wanted() {
        return None;
    }
    crate::aloud::into_screen()
}

/// What is true the moment this node stops answering.
///
/// Printed rather than said, because by the time this runs the screen has given the
/// terminal back and there is nobody left listening to what the node says; laid out
/// for the terminal as a line said to it would be. Dropped if nobody reads standard
/// output any more: a vigil piped into `head` has nobody to tell.
fn farewell(on_a_roll: bool) {
    crate::aloud::printed(&said_at_the_end(Epoch::now(), on_a_roll));
}

/// The farewell's words, for the epoch it is said in.
///
/// Whoever is drawn goes out to ask the members on their roll and nobody else, so for
/// a node on nobody's roll — one not yet handed the file, or the one given it by
/// nobody — there is nobody to sign that it was not there.
fn said_at_the_end(now: Epoch, on_a_roll: bool) -> String {
    if !on_a_roll {
        return words!("serve-farewell-on-no-roll", epoch = now.0);
    }
    words!(
        "serve-farewell",
        epoch = now.0,
        window = n333_core::presence::WINDOW_EPOCHS
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;
    use std::time::Duration;

    use n333_core::Identity;
    use n333_core::attestation::JUDGEMENT_DELAY_EPOCHS;
    use n333_core::presence::Attendance;
    use n333_core::subject::DIGEST;
    use n333_core::transfer::{Half, Record};
    use n333_net::direct;

    use crate::node::Keeping;
    use crate::paths::NodePaths;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("n333-round-test-{name}"));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("creates dir");
        dir
    }

    fn common(root: PathBuf) -> Common {
        Common {
            bridges: std::sync::Arc::new(std::sync::Mutex::new(n333_net::bridges::Bridges::none())),
            paths: NodePaths::at(root),
            timeout: Duration::from_secs(10),
            keeping: Keeping::TheWindow,
            trust_directory_permissions: true,
        }
    }

    /// Both halves of "somebody handed the file to this node", as they would be held.
    ///
    /// Signed rather than staged: the roll is built out of two keys agreeing, and a
    /// test that put a member on a roll any other way would be testing something this
    /// protocol does not do.
    fn admitted(founder: &Identity, node: &Node, epoch: Epoch) -> Vec<Vec<u8>> {
        vec![
            Record::new(founder, node.identity().public_key(), epoch, DIGEST)
                .seal(Half::Gave, founder)
                .expect("seals"),
            Record::new(node.identity(), founder.public_key(), epoch, DIGEST)
                .seal(Half::Received, node.identity())
                .expect("seals"),
        ]
    }

    /// Open a node, answer on a socket, and say where that socket is.
    async fn a_node_answering(name: &str) -> (Arc<Node>, Common, PeerAddress) {
        let home = scratch(name);
        let common = common(home.clone());
        let (node, _) = Node::open(&common.mistrust(), &home, Keeping::TheWindow).expect("opens");
        let node = Arc::new(node);
        let listener = direct::Listener::bind("127.0.0.1:0".parse().expect("an address"))
            .await
            .expect("binds");
        let where_it_is = PeerAddress::from(listener.address().expect("bound"));
        let answering = Arc::clone(&node);
        tokio::spawn(async move { answer_direct(listener, answering, Door::new()).await });
        (node, common, where_it_is)
    }

    #[tokio::test]
    async fn one_of_us_is_asked_answers_and_is_written_down_as_present() {
        // The loop the whole protocol rests on, end to end: two of us on a roll, one
        // drawn to ask the other, the answer given, the statement published and handed
        // back, and three epochs later a verdict written into a record that is never
        // revisited. It takes sixteen hours to watch happen and it has to work the
        // first time, for ever.
        let founder = Identity::from_seed(&[1; 32]);
        let now = Epoch::now();
        // Two epochs ago, so this epoch is the first their records cover: what came
        // before it is not theirs to answer for and nothing is written about it.
        let joined = Epoch(now.0.saturating_sub(2));

        let (asked, asked_common, where_asked_is) = a_node_answering("asked").await;
        let (asker, asker_common, where_asker_is) = a_node_answering("asker").await;

        // Both hold both admissions, so both rolls are the same two of us. The founder
        // is on neither: nobody handed the file to whoever had it first.
        let mut halves = admitted(&founder, &asked, joined);
        halves.extend(admitted(&founder, &asker, joined));
        for node in [&asked, &asker] {
            assert_eq!(node.admit(&halves).await.expect("admits"), 2);
        }

        // Each knows where to knock, the way a node does after an invitation or after
        // hearing a neighbour on its own network: an address and nothing else.
        asked.found(where_asker_is.to_string()).await;
        asker.found(where_asked_is.to_string()).await;

        // One round each. The first says where it is and hands that to the second; the
        // second, now knowing WHO is at that address, is drawn to ask it.
        let rounds = [
            (&asked, &asked_common, where_asked_is),
            (&asker, &asker_common, where_asker_is),
        ];
        for (node, common, where_it_is) in rounds {
            let dialer = Dialer::new(common.clone());
            hours::one_round(node, &dialer, Some(where_it_is), None, now).await;
        }

        // The verifier's round ends when it has published; the node it asked is still
        // writing down what the round produced. In an epoch that gap is nothing and
        // three epochs pass before any of it is read. Here it is the test running
        // faster than a disk.
        let mut waited = Duration::ZERO;
        while asked.witnessed().await == 0 && waited < Duration::from_secs(5) {
            tokio::time::sleep(Duration::from_millis(10)).await;
            waited += Duration::from_millis(10);
        }

        // Nothing is written about an epoch until it is too old to change.
        assert!(
            asked.own_record().await.expect("reads").is_empty(),
            "an epoch that can still be spoken about is not judged"
        );

        hours::judge_what_is_ready(&asked, Epoch(now.0 + JUDGEMENT_DELAY_EPOCHS)).await;
        assert_eq!(
            asked.own_record().await.expect("reads"),
            vec![(now, Attendance::Present)],
            "asked, answered, and witnessed by the one drawn to ask"
        );
        assert_eq!(
            asked.witnessed().await,
            1,
            "and the statement that says so is kept, in somebody else's hand"
        );
    }

    #[tokio::test]
    async fn one_of_us_nobody_can_reach_goes_and_is_written_down_as_present() {
        // The same loop for the node behind a router. Nobody can open a connection to
        // it, so the verifier drawn to ask it never gets to; left there it would be
        // absent from every epoch for ever while answering everything it was asked.
        // Instead it works out who was drawn — the draw is the epoch and two keys and
        // nothing else — goes to them, and the question comes back down the connection
        // it opened.
        let founder = Identity::from_seed(&[1; 32]);
        let now = Epoch::now();
        let joined = Epoch(now.0.saturating_sub(2));

        // One answers on a socket. The other answers on nothing at all.
        let (asker, asker_common, where_asker_is) = a_node_answering("goes-asker").await;
        let hidden_home = scratch("goes-hidden");
        let hidden_common = common(hidden_home.clone());
        let (hidden, _) =
            Node::open(&hidden_common.mistrust(), &hidden_home, Keeping::TheWindow).expect("opens");
        let hidden = Arc::new(hidden);

        let mut halves = admitted(&founder, &hidden, joined);
        halves.extend(admitted(&founder, &asker, joined));
        for node in [&hidden, &asker] {
            assert_eq!(node.admit(&halves).await.expect("admits"), 2);
        }

        // Only one direction is known, which is the whole point: the hidden node has
        // somewhere to knock and nobody has anywhere to knock for it.
        hidden.found(where_asker_is.to_string()).await;

        let asker_dialer = Dialer::new(asker_common.clone());
        hours::one_round(&asker, &asker_dialer, Some(where_asker_is), None, now).await;
        assert_eq!(
            hidden.witnessed().await,
            0,
            "the one drawn to ask has no address to ask at, so nothing has been said"
        );

        // The hidden node's round: it trades first, which is how it learns which key is
        // at the address it was told about, and then it goes.
        let hidden_dialer = Dialer::new(hidden_common.clone());
        hours::one_round(&hidden, &hidden_dialer, None, None, now).await;

        let mut waited = Duration::ZERO;
        while hidden.witnessed().await == 0 && waited < Duration::from_secs(5) {
            tokio::time::sleep(Duration::from_millis(10)).await;
            waited += Duration::from_millis(10);
        }

        hours::judge_what_is_ready(&hidden, Epoch(now.0 + JUDGEMENT_DELAY_EPOCHS)).await;
        assert_eq!(
            hidden.own_record().await.expect("reads"),
            vec![(now, Attendance::Present)],
            "present, on a connection nobody could have opened to it"
        );
        assert_eq!(
            hidden.witnessed().await,
            1,
            "and the verifier's statement says so, signed by the verifier"
        );
    }

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let pairs = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            [
                (
                    words!("serve-nothing-listening"),
                    "nothing would be listening: --no-direct needs --tor",
                ),
                (words!("serve-name", name = "333abc"), "name     333abc"),
                (
                    words!("serve-waiting-for-the-file"),
                    "waiting  this node has not been given the file, so nothing is counted for it\n\
                     \x20        yet and there is nothing yet for anybody to witness. It cannot make\n\
                     \x20        one. It only ever arrives from somebody who already holds it, and\n\
                     \x20        the two of you sign for the handover. Ask for an\n\
                     \x20        invitation, then `333 join 333:their.address:3333`. Answering in the\n\
                     \x20        meantime costs nothing and is how people find you.",
                ),
                (
                    words!("serve-hand"),
                    "trust    an invitation names a place, not a person. Whoever answers there\n\
                     \x20        proves who they are by holding their key.",
                ),
                (
                    words!("serve-invite", invitation = "333:192.0.2.7:3333"),
                    "invite   333:192.0.2.7:3333",
                ),
                (
                    words!("serve-answer", bound = "0.0.0.0:3333"),
                    "answer   0.0.0.0:3333",
                ),
                (
                    words!("serve-nearby"),
                    "nearby   saying on this network that something here speaks 333, and\n\
                     \x20        listening for the others. Not this node's name: what goes out\n\
                     \x20        is what a port scan of the same network would find. --no-mdns\n\
                     \x20        keeps this node off it.",
                ),
                (
                    words!("serve-nearby-failed", why = "no interface"),
                    "nearby   could not start saying on this network that this node is here: \
                     no interface",
                ),
                (
                    words!("serve-meet", place = "the333.dev"),
                    "meet     the333.dev is where this node looks for people nobody introduced it to.\n\
                     \x20        Everything read there is signed by whoever said it, and nothing\n\
                     \x20        there is believed. --no-meet keeps this node away from it.",
                ),
                (
                    words!("serve-listener-stopped"),
                    "a listener stopped unexpectedly",
                ),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

    /// Every keyword in the Korean catalogs whose names begin with `stem`, as written.
    fn korean_keywords(stem: &str) -> Vec<(String, String)> {
        crate::words::catalog::built_in("ko")
            .into_iter()
            .filter(|source| {
                std::path::Path::new(&source.name)
                    .file_stem()
                    .is_some_and(|name| name.to_string_lossy().starts_with(stem))
            })
            .flat_map(|source| {
                let name = source.name.clone();
                source
                    .text
                    .lines()
                    .filter_map(|line| line.trim_start().strip_prefix(".keyword = "))
                    .map(|keyword| (name.clone(), keyword.replace("{\"\"}", "")))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    #[test]
    fn in_korean_every_line_of_the_vigil_and_its_hours_begins_its_words_in_the_ninth_column() {
        use crate::words::layout::{COLUMN, where_the_words_begin};
        use unicode_width::UnicodeWidthStr as _;
        let mut keywords = korean_keywords("serve");
        keywords.extend(korean_keywords("hours"));
        assert!(keywords.len() > 60, "{keywords:?}");
        for (file, keyword) in keywords {
            assert!(keyword.width() < COLUMN, "{file}: {keyword}");
        }
        let lines = crate::words::speaking("ko", crate::words::count::Base::Ten, || {
            [
                words!("serve-waiting-for-the-file"),
                words!("serve-name", name = "333abc"),
                said_at_the_end(Epoch(89_612), true),
                said_at_the_end(Epoch(89_612), false),
            ]
        });
        for line in &lines {
            for (at, one) in line.lines().enumerate() {
                let begins = if at == 0 {
                    where_the_words_begin(one)
                } else {
                    one.len() - one.trim_start().len()
                };
                assert_eq!(begins, COLUMN, "{one:?}");
            }
        }
    }

    #[test]
    fn the_farewell_says_in_english_what_it_said_before_its_words_moved() {
        let said = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            said_at_the_end(Epoch(89_612), true)
        });
        assert_eq!(
            said,
            "node     ended in epoch 89612. Whoever is drawn to ask for you while this\n\
             \x20        is not running signs that they asked and heard nothing, and\n\
             \x20        that is what your window reads. It is 333 epochs long, and it\n\
             \x20        moves."
        );
    }

    #[test]
    fn a_node_on_nobodys_roll_is_not_told_that_anybody_signs_it_absent() {
        // Nobody goes out to ask a node that is on no roll: not a newcomer that has not
        // been handed the file, and not the one that was handed it by nobody.
        let said = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            said_at_the_end(Epoch(89_612), false)
        });
        assert_eq!(
            said,
            "node     ended in epoch 89612. You are on nobody's roll, so nobody goes out\n\
             \x20        to ask for you, and nothing is signed about you while this is not\n\
             \x20        running."
        );
    }
}
