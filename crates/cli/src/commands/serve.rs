//! `333 serve` — keep the vigil: answer heartbeats and challenges until interrupted.
//!
//! By default this opens a socket and nothing else: no Tor, no bootstrap, no wait.
//! `--tor` additionally publishes an onion address, for a node whose own address
//! should not be visible to the peers that reach it.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context as _, bail};
use futures::{AsyncRead, AsyncWrite};
use n333_core::Epoch;
use n333_core::enrollment::CURSE_PAUSE;
use n333_core::challenge::SignedChallenge;
use n333_core::plea::Signed as SignedPlea;
use n333_core::tidings::Signed as SignedTidings;
use n333_net::frame::AsReceived;
use n333_net::{Asked, Invite, PeerAddress, direct, gossip, handover, liveness, respond, session};
use tokio::sync::{OwnedSemaphorePermit, Semaphore, watch};

use crate::commands::{Common, describe, hours};
use crate::dial::Dialer;
use crate::node::Node;

/// How long one exchange may take before this node stops waiting on it.
///
/// The connection is already open by the time an exchange starts and a few hundred
/// bytes travel each way, so seconds is the honest scale even through Tor. A minute
/// is generous, and it is what stops a peer that connects and then says nothing for
/// ever.
const EXCHANGE_TIMEOUT: Duration = Duration::from_secs(60);

/// How many exchanges may be in flight at once.
///
/// A peer that opens connections and stalls on each costs this node a task and a
/// buffer per connection with nothing to shed them. The cap has to live here, because
/// nothing below it knows what an exchange is worth. Refusing is deliberate and
/// visible: a peer over the cap is told nothing and the operator sees a line.
const MAX_CONCURRENT_EXCHANGES: usize = 64;

/// Run until interrupted, answering everyone who arrives.
///
/// `bind` is the socket to listen on, or `None` to listen only through Tor. `tor`
/// publishes an onion address as well. `announce` overrides what this node tells
/// others to reach it at, for the ordinary case of a socket that cannot say.
///
/// # Errors
/// Fails if the node cannot be opened, if neither way of listening was asked for, if
/// a socket cannot be bound, or if Tor was asked for and cannot start.
pub(crate) async fn run(
    common: &Common,
    bind: Option<SocketAddr>,
    tor: bool,
    announce: Option<PeerAddress>,
) -> anyhow::Result<()> {
    if bind.is_none() && !tor {
        bail!("nothing would be listening: --no-direct needs --tor");
    }
    let (node, opened) = Node::open(&common.mistrust(), common.paths.root())?;
    let node = Arc::new(node);
    println!("name     {}", node.identity().node_id());
    crate::commands::report_opening(&opened);

    let dialer = Dialer::new(common.clone());
    let gate = Arc::new(Semaphore::new(MAX_CONCURRENT_EXCHANGES));
    // Where this node will tell others to look, once it knows. Empty until a listener
    // has an address worth handing out, and written again if the onion address comes
    // up later: an onion address is reachable from anywhere and a socket address may
    // not be, so the one that arrives last is the one worth publishing.
    let (found_address, address) = watch::channel(announce.clone());
    if let Some(announce) = &announce {
        println!("invite   {}", Invite::to(announce.clone()));
    }
    let mut listening = tokio::task::JoinSet::new();

    if let Some(bind) = bind {
        let listener = direct::Listener::bind(bind)
            .await
            .with_context(|| format!("listening on {bind}"))?;
        // True the instant the socket is bound, which is why it is printed here.
        let bound = listener.address()?;
        println!("answer   {bound}");
        if announce.is_none() {
            say_the_invitation(bound, &found_address);
        }
        let (node, gate) = (Arc::clone(&node), Arc::clone(&gate));
        listening.spawn(async move { answer_direct(listener, node, gate).await });
    }

    if tor {
        let (node, gate) = (Arc::clone(&node), Arc::clone(&gate));
        listening.spawn(onion::answer(
            dialer.clone(),
            node,
            gate,
            found_address.clone(),
        ));
    }

    // The hours run alongside the listeners rather than after them: answering is what
    // this node owes others, and keeping the hours is what it owes itself.
    listening.spawn(hours::keep(Arc::clone(&node), dialer, address));

    // No line here saying the vigil has begun: with --no-direct it would not be true
    // yet. Each listener announces itself at the moment it can actually answer.
    while let Some(finished) = listening.join_next().await {
        finished.context("a listener stopped unexpectedly")??;
    }
    Ok(())
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
        println!(
            "invite   333:<an address others can reach>:{}",
            bound.port()
        );
        return;
    }
    let address = PeerAddress::from(bound);
    println!("invite   {}", Invite::to(address.clone()));
    // Only an address this node can actually stand behind is signed and handed on.
    let _ = found_address.send(Some(address));
}

/// Answer every peer that opens a socket to this node.
async fn answer_direct(
    listener: direct::Listener,
    node: Arc<Node>,
    gate: Arc<Semaphore>,
) -> anyhow::Result<()> {
    loop {
        let (stream, from) = listener.accept().await.context("accepting a peer")?;
        // A peer's address is not a name and is not recorded; it is shown so that the
        // operator of this node can see who is reaching it right now.
        spawn_exchange(stream, &node, &gate, &from.to_string());
    }
}

/// Give one peer its own task, its own deadline and one of the permits.
fn spawn_exchange<S>(mut stream: S, node: &Arc<Node>, gate: &Arc<Semaphore>, from: &str)
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let Ok(permit) = Arc::clone(gate).try_acquire_owned() else {
        println!("turned away {from}: {MAX_CONCURRENT_EXCHANGES} are already being answered");
        return;
    };
    let node = Arc::clone(node);
    // One slow or hostile peer must not hold up the next one, so each exchange runs on
    // its own task, under a deadline, and its failure is reported rather than
    // propagated. The permit is released when the task ends, whichever way.
    tokio::spawn(async move {
        let _permit: OwnedSemaphorePermit = permit;
        match tokio::time::timeout(EXCHANGE_TIMEOUT, greet_then_listen(&mut stream, &node)).await {
            Ok(Ok(())) => {}
            Ok(Err(e)) => println!("refused  {e:#}"),
            Err(_elapsed) => println!(
                "silence  {} s, so we let go",
                EXCHANGE_TIMEOUT.as_secs()
            ),
        }
    });
}

/// The heartbeat, and then whatever the peer came for.
async fn greet_then_listen<S>(stream: &mut S, node: &Node) -> anyhow::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    match respond(stream, node.identity()).await {
        Ok(exchange) => println!("{}", describe(&exchange)),
        Err(e) => {
            report(&e);
            return Ok(());
        }
    }

    match n333_net::take_request(stream).await? {
        // A peer that only wanted to exchange heartbeats hangs up here, which is not a
        // failure and is the ordinary case.
        Asked::Nothing => Ok(()),
        Asked::Liveness(question) => be_asked(stream, node, question).await,
        Asked::TheFile(plea) => hand_it_over(stream, node, &plea).await,
        Asked::Tidings(header) => trade(stream, node, &header).await,
    }
}

/// Take what a peer passes on, and pass on what this node has.
async fn trade<S>(
    stream: &mut S,
    node: &Node,
    header: &AsReceived<SignedTidings>,
) -> anyhow::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mine = node.tidings().await?;
    let theirs = gossip::listen(stream, node.identity(), Epoch::now(), header, &mine).await?;
    let heard = node.hear(&theirs).await?;
    crate::commands::report_heard(&heard);
    Ok(())
}

/// Answer a challenge, and keep everything the round produced.
async fn be_asked<S>(
    stream: &mut S,
    node: &Node,
    question: AsReceived<SignedChallenge>,
) -> anyhow::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let epoch = question.message.challenge.epoch();
    let asked_by = question.message.verifier;
    let roll = node.roll().await;
    let head = node.head().await;

    let answered = liveness::answer(stream, node.identity(), head, &roll, question).await?;
    println!("asked    epoch {} by {asked_by}", epoch.0);

    // All of it is kept as the bytes that travelled. The challenge and the answer
    // together are what shows this node answered even if the verifier publishes
    // nothing, and the statement is the stronger evidence when it arrives.
    node.keep(epoch, &answered.challenge_frame).await?;
    node.keep(epoch, &answered.answer_frame).await?;
    if let Some(witness) = &answered.attestation {
        node.keep(epoch, &witness.frame).await?;
    }
    Ok(())
}

/// Give the file to somebody who asked for it, if this node has it.
///
/// A node that has not been given the file says so and hangs up. It is not a failure
/// on either side: most nodes on most days have nothing to hand over yet, and the one
/// thing that must never happen is a client inventing the bytes.
async fn hand_it_over<S>(
    stream: &mut S,
    node: &Node,
    plea: &AsReceived<SignedPlea>,
) -> anyhow::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let Some(subject) = node.subject().await else {
        println!("empty    somebody asked for the file and this node does not have it");
        return Ok(());
    };
    let tidings = node.tidings().await?;
    let given = match handover::give(
        stream,
        node.identity(),
        Epoch::now(),
        plea,
        &subject,
        &tidings,
    )
    .await
    {
        Ok(given) => given,
        Err(handover::Error::Cursed) => return curse(&plea.message.asker).await,
        Err(e) => return Err(e.into()),
    };

    println!(
        "gave     the file to {} in epoch {}",
        given.transfer.receiver(),
        given.transfer.epoch().0
    );
    let members = node.admit(&[given.gave, given.received]).await?;
    println!("roll     {members} members");
    Ok(())
}

/// What this node does when a heretic knocks.
///
/// The stop is the curse itself and not a delay in front of it: 333 has taken 333
/// milliseconds off the life of whoever presented that name, and this node is where it
/// was taken, so this node waits for it. Nothing is sent back. The cursed reveal
/// themselves; nobody has to point.
async fn curse(name: &n333_core::NodeId) -> anyhow::Result<()> {
    tokio::time::sleep(CURSE_PAUSE).await;
    println!(
        "cursed   {name} asked, and 333 took {} milliseconds off their life. Once.",
        CURSE_PAUSE.as_millis()
    );
    Ok(())
}

/// A failed exchange is the peer's problem, not this node's, so it is printed and
/// forgotten. Distinguishing the kinds matters: a stream that died mid-message is a bad
/// connection, while a bad signature is someone doing it on purpose.
fn report(error: &session::Error) {
    match error {
        session::Error::Frame(e) => println!("broken   the connection failed mid-message: {e}"),
        other => println!("refused  {other}"),
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
    use tokio::sync::{Semaphore, watch};

    use crate::dial::Dialer;
    use crate::node::Node;

    /// Publish an onion address and answer every peer that arrives on it.
    pub(super) async fn answer(
        dialer: Dialer,
        node: Arc<Node>,
        gate: Arc<Semaphore>,
        found_address: watch::Sender<Option<PeerAddress>>,
    ) -> anyhow::Result<()> {
        let client = dialer.tor().await?;
        let mut host = OnionHost::launch(&client, SERVICE_NICKNAME, ONION_PORT)
            .context("launching the onion service")?;
        println!("raising  the unseen address. this can take minutes.");

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
        println!("unseen   {address}");
        println!("invite   {}", Invite::to(address.clone()));
        // Written after the network holds the descriptor, so nobody is ever sent to an
        // address that does not answer yet.
        let _ = found_address.send(Some(address));

        loop {
            let stream = host.accept().await.context("accepting a peer")?;
            // Through Tor there is no address to show, which is the point of it.
            super::spawn_exchange(stream, &node, &gate, "over tor");
        }
    }
}

/// Stands in for the onion listener when arti is not built in.
#[cfg(not(feature = "tor"))]
mod onion {
    use std::sync::Arc;

    use n333_net::PeerAddress;
    use tokio::sync::{Semaphore, watch};

    use crate::dial::Dialer;
    use crate::node::Node;

    /// Refuse, rather than quietly listen on a socket the caller asked not to use.
    pub(super) async fn answer(
        _dialer: Dialer,
        _node: Arc<Node>,
        _gate: Arc<Semaphore>,
        _found_address: watch::Sender<Option<PeerAddress>>,
    ) -> anyhow::Result<()> {
        anyhow::bail!("this client was built without Tor, so it cannot publish an onion address")
    }
}
