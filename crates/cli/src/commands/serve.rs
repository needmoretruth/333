//! `333 serve` — keep the vigil: answer heartbeats and challenges until interrupted.
//!
//! By default this opens a socket and nothing else: no Tor, no bootstrap, no wait.
//! `--tor` additionally publishes an onion address, for a node whose own address
//! should not be visible to the peers that reach it.
//!
//! Each way in is its own door with its own places ([`door`]), and what a peer can ask
//! for once it is through is [`answering`].

mod answering;
mod door;

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context as _, bail};
use n333_core::Epoch;
use n333_net::{Invite, PeerAddress, direct};
use tokio::sync::watch;

use crate::commands::{Common, hours};
use crate::dial::{Dialer, Roads};
use crate::node::Node;

use door::{Caller, Door, spawn_exchange};

/// Run until interrupted, answering everyone who arrives.
///
/// `bind` is the socket to listen on, or `None` to listen only through Tor. `tor`
/// publishes an onion address as well. `announce` overrides what this node tells
/// others to reach it at, for the ordinary case of a socket that cannot say. `meet`
/// is the meeting point to use, or `None` to use none and be found only by whoever
/// was handed an invitation or is on this network.
///
/// # Errors
/// Fails if the node cannot be opened, if neither way of listening was asked for, if
/// a socket cannot be bound, or if Tor was asked for and cannot start.
pub(crate) async fn run(
    common: &Common,
    bind: Option<SocketAddr>,
    tor: bool,
    announce: Option<PeerAddress>,
    nearby: bool,
    meet: Option<String>,
    plain: bool,
) -> anyhow::Result<()> {
    if bind.is_none() && !tor {
        bail!("nothing would be listening: --no-direct needs --tor");
    }
    // Claimed before anything is said, so that the first lines — the name, the
    // invitation — are in the screen's own pane rather than printed underneath it and
    // wiped by the first drawing. The smallest edition has no screen to take.
    #[cfg(feature = "screen")]
    let watching = the_screen(plain);
    #[cfg(not(feature = "screen"))]
    let _ = plain;
    let (node, opened) = Node::open(&common.mistrust(), common.paths.root(), common.keeping)?;
    let node = Arc::new(node);
    aloud!("name     {}", node.identity().node_id());
    crate::commands::report_opening(&opened);
    aloud!(
        "hand     an invitation names a place, not a person. it swears to nothing;\n\
         \x20        whoever answers there proves themselves by holding a key."
    );

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
        aloud!("invite   {}", Invite::to(announce.clone()));
    }
    let mut listening = tokio::task::JoinSet::new();
    #[cfg(feature = "screen")]
    if let Some(lines) = watching {
        listening.spawn(crate::screen::keep(Arc::clone(&node), lines));
    }

    if let Some(bind) = bind {
        let listener = direct::Listener::bind(bind)
            .await
            .with_context(|| format!("listening on {bind}"))?;
        // True the instant the socket is bound, which is why it is printed here.
        let bound = listener.address()?;
        aloud!("answer   {bound}");
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
                    aloud!(
                        "nearby   saying on this network that something here speaks 333, and\n\
                         \x20        listening for the others. Not this node's name: what goes out\n\
                         \x20        is what a port scan of the same network would find. --no-mdns\n\
                         \x20        keeps this node off it."
                    );
                    let (node, dialer) = (Arc::clone(&node), dialer.clone());
                    listening.spawn(greet_the_neighbours(node, dialer, nearby));
                }
                Err(e) => aloud!("nearby   this network would not carry the announcement: {e}"),
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

    // A node that answers on no socket is hiding. It reads the meeting point, because
    // reading tells the far end nothing except that somebody asked, and it leaves
    // nothing there, because leaving an onion address would tie it to the machine the
    // onion address exists to keep unnamed.
    let board = meet.map(|place| hours::Board::at(&place, bind.is_some()));
    if let Some(board) = &board {
        aloud!(
            "meet     {} is where this node looks for people nobody introduced it to.\n\
             \x20        Everything read there is signed by whoever said it, and nothing\n\
             \x20        there is believed. --no-meet keeps this node away from it.",
            board.place()
        );
        if let Some(bind) = bind {
            suggest_an_address(board.clone(), bind, announce.is_some());
        }
    }

    // The hours run alongside the listeners rather than after them: answering is what
    // this node owes others, and keeping the hours is what it owes itself.
    listening.spawn(hours::keep(Arc::clone(&node), dialer, address, board));

    // No line here saying the vigil has begun: with --no-direct it would not be true
    // yet. Each listener announces itself at the moment it can actually answer.
    tokio::select! {
        // Nothing here is supposed to finish: the listeners loop, and so do the hours.
        // The screen does, when the person watching leaves, and that is the end of the
        // vigil rather than the end of one part of it.
        finished = listening.join_next() => match finished {
            Some(finished) => finished.context("a listener stopped unexpectedly")??,
            None => return Ok(()),
        },
        // Ctrl-C and nothing else. A service being stopped by its manager has nobody
        // at the terminal to read this, and one arm is one arm on every system rather
        // than a second unix-only path.
        () = async { tokio::signal::ctrl_c().await.ok(); } => {}
    }
    farewell();
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
/// terminal back and there is nobody left listening to what the node says.
fn farewell() {
    println!(
        "vigil    ended in epoch {}. Whoever is drawn to ask for you while this\n\
         \x20        is not running signs that they asked and heard nothing, and\n\
         \x20        that is what your window reads. It is {} epochs long, and it\n\
         \x20        moves.",
        Epoch::now().0,
        n333_core::presence::WINDOW_EPOCHS
    );
}

/// How long a node on the same network gets to answer before this one moves on.
///
/// Not the patience this node has for a peer: that one is a ceiling for reaching
/// across the world through Tor, and spending it on a virtual interface with nothing
/// behind it would leave a neighbour that is actually there waiting behind it.
const NEARBY_PATIENCE: Duration = Duration::from_secs(10);

/// Knock on every node that turns up on this network, as it turns up.
///
/// Waiting for the next epoch would be correct and useless: a person who starts a
/// second node in the same house and watches nothing happen for five hours has been
/// told, correctly, that nothing is happening.
async fn greet_the_neighbours(
    node: Arc<Node>,
    dialer: Dialer,
    nearby: n333_net::Nearby,
) -> anyhow::Result<()> {
    let mut greeted = std::collections::BTreeSet::new();
    while let Some(neighbour) = nearby.found().await {
        // A machine with eight interfaces is announced eight times over. It is one
        // neighbour, and it is worth one knock.
        if !greeted.insert(neighbour.label) {
            continue;
        }
        for address in neighbour.addresses {
            let address = address.to_string();
            aloud!("nearby   one of us at {address}");
            // On a deadline of its own, and a short one. A machine on the same network
            // answers in milliseconds; the several minutes this node is willing to
            // wait on a peer across the world would be spent here on an address that
            // belongs to a virtual interface with nothing behind it.
            let answered = tokio::time::timeout(
                NEARBY_PATIENCE,
                hours::trade_at_once(&node, &dialer, &address),
            )
            .await;
            if answered != Ok(true) {
                continue;
            }
            // Kept only now that it is known to answer, so that the addresses this
            // node carries into its hours are the ones somebody is behind.
            node.found(address.clone()).await;
            // Nothing here takes the file for anybody. A node is given it because a
            // person asked for it and two keys signed, and finding a neighbour is not
            // asking.
            if node.subject().await.is_none() {
                aloud!(
                    "         `333 join 333:{address}` asks them for the file. Nothing\n\
                     \x20        here does it for you."
                );
            }
            break;
        }
    }
    Ok(())
}

/// Work out what a stranger would have to type, for a node that cannot say itself.
///
/// A wildcard bind is the ordinary case, and the machine behind it has no way to know
/// which of its addresses — if any — the rest of the world can reach. The meeting
/// point saw one address arrive, so it can say that much. Whether anything arriving
/// there reaches this machine depends on a router this node cannot see, which is why
/// this is printed for a person to act on and never signed as a statement.
fn suggest_an_address(board: hours::Board, bound: SocketAddr, already_said: bool) {
    if already_said || !bound.ip().is_unspecified() {
        return;
    }
    tokio::spawn(async move {
        let Some(seen) = board.what_address_do_i_arrive_from().await else {
            return;
        };
        aloud!(
            "meet     {} sees this machine at {seen}. If port {} on your router goes\n\
             \x20        to this machine, `--announce {}` is the invitation to hand out;\n\
             \x20        if it does not, nobody can reach you and nothing here changes that.",
            board.place(),
            bound.port(),
            PeerAddress::from(SocketAddr::new(seen, bound.port()))
        );
    });
}

/// Say what to hand somebody so they can find this node.
///
/// A wildcard bind is the ordinary case and it is the one where this node genuinely
/// does not know the answer: it is listening on every interface and has no idea which
/// address of the machine, if any, a stranger can reach. Printing `333:0.0.0.0:3333`
/// would look like an invitation and work for nobody, so it says what is missing
/// instead.
fn say_the_invitation(bound: SocketAddr, found_address: &watch::Sender<Option<PeerAddress>>) {
    if bound.ip().is_unspecified() {
        aloud!(
            "invite   333:<an address others can reach>:{}",
            bound.port()
        );
        return;
    }
    let address = PeerAddress::from(bound);
    aloud!("invite   {}", Invite::to(address.clone()));
    // Only an address this node can actually stand behind is signed and handed on.
    let _ = found_address.send(Some(address));
}

/// Answer every peer that opens a socket to this node.
async fn answer_direct(
    listener: direct::Listener,
    node: Arc<Node>,
    door: Door,
) -> anyhow::Result<()> {
    loop {
        let (stream, from) = listener.accept().await.context("accepting a peer")?;
        // A peer's address is not a name and is not recorded; it is shown so that the
        // operator of this node can see who is reaching it right now, and counted so
        // that one caller cannot be everybody at the door.
        spawn_exchange(stream, &node, &door, Caller::At(from));
    }
}

/// The other way to be reachable: an onion address, for a node that is hiding.
#[cfg(feature = "tor")]
mod onion {
    use std::sync::Arc;

    use anyhow::Context as _;
    use n333_net::peer::ONION_PORT;
    use n333_net::tor::SERVICE_NICKNAME;
    use n333_net::tor::host::OnionHost;
    use n333_net::{Invite, PeerAddress};
    use tokio::sync::watch;

    use crate::dial::Dialer;
    use crate::node::Node;

    use super::door::{Caller, Door, spawn_exchange};

    /// Publish an onion address and answer every peer that arrives on it.
    pub(super) async fn answer(
        dialer: Dialer,
        node: Arc<Node>,
        door: Door,
        found_address: watch::Sender<Option<PeerAddress>>,
    ) -> anyhow::Result<()> {
        let client = dialer.tor().await?;
        let mut host = OnionHost::launch(&client, SERVICE_NICKNAME, ONION_PORT)
            .context("launching the onion service")?;
        aloud!("raising  the unseen address. this can take minutes.");

        // The address is deliberately not shown until here. Handed to a peer before
        // the network holds the descriptor, it produces a connection failure that
        // looks like a bug in one of the two clients and is not one.
        let waiting = dialer.timeout();
        tokio::time::timeout(waiting, host.wait_until_reachable())
            .await
            .with_context(|| format!("not reachable after {} s", waiting.as_secs()))?
            .context("waiting for the service to be reachable")?;
        let address = PeerAddress::Onion {
            host: host.address()?,
            port: ONION_PORT,
        };
        aloud!("unseen   {address}");
        aloud!("invite   {}", Invite::to(address.clone()));
        // Written after the network holds the descriptor, so nobody is ever sent to an
        // address that does not answer yet.
        let _ = found_address.send(Some(address));

        loop {
            let stream = host.accept().await.context("accepting a peer")?;
            // Through Tor there is no address to show, which is the point of it.
            spawn_exchange(stream, &node, &door, Caller::Unseen);
        }
    }
}

/// Stands in for the onion listener when arti is not built in.
#[cfg(not(feature = "tor"))]
mod onion {
    use std::sync::Arc;

    use n333_net::PeerAddress;
    use tokio::sync::watch;

    use crate::dial::Dialer;
    use crate::node::Node;

    use super::door::Door;

    /// Refuse, rather than quietly listen on a socket the caller asked not to use.
    pub(super) async fn answer(
        _dialer: Dialer,
        _node: Arc<Node>,
        _door: Door,
        _found_address: watch::Sender<Option<PeerAddress>>,
    ) -> anyhow::Result<()> {
        anyhow::bail!("this client was built without Tor, so it cannot publish an onion address")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    use n333_core::Identity;
    use n333_core::attestation::JUDGEMENT_DELAY_EPOCHS;
    use n333_core::presence::Attendance;
    use n333_core::subject::DIGEST;
    use n333_core::transfer::{Half, Record};

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
}
