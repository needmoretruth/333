//! The door: who gets in, how long they may stand there, and who is turned away.
//!
//! A slot at this door is the scarce thing a serving node has. It costs a peer one
//! connection and costs this node a task, a buffer and one of a fixed number of places
//! until the peer is finished or the deadline runs out.
//!
//! SO THE DEADLINE IS SPLIT, AND THE PART BEFORE A PEER HAS SAID ANYTHING IS SHORT.
//! Sixty-four sockets that connect and say nothing used to hold every place for a
//! minute each; renewed as they expired, that is a node that answers nobody for as
//! long as somebody cares to keep it up — no newcomer given the file, no challenge
//! answered, no trade taken in. A peer that has said what it came for is worth the
//! minute. A peer that has said nothing is worth seconds.
//!
//! AND ONE PLACE MAY NOT HOLD THE WHOLE DOOR. Counting the callers from each address
//! is the only bound here that does not simply raise the price: a deadline shortens
//! how long one socket holds a place, and a cap says how many places one caller may
//! hold at once. It is deliberately loose — a few nodes behind one household's
//! address is ordinary, and being turned away is not a judgement about anybody.
//!
//! Through Tor there is no address to count, which is the point of Tor, so that door
//! has a deadline and a cap of its own and no per-caller bound. Each door has its own
//! places for the same reason: a stalled circuit must not be able to shut the socket.

use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures::{AsyncRead, AsyncWrite};
use n333_net::{Asked, respond, session};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

use crate::commands::describe;
use crate::node::Node;

use super::answering::{be_asked, hand_it_over, trade};

/// How long one exchange may take before this node stops waiting on it.
///
/// The connection is already open by the time an exchange starts and a few hundred
/// bytes travel each way, so seconds is the honest scale even through Tor. A minute
/// is generous, and it is what stops a peer that has asked for something and then
/// goes quiet from holding a place for ever.
const EXCHANGE_TIMEOUT: Duration = Duration::from_secs(60);

/// How long a peer has to say who it is and what it came for.
///
/// One small frame each way for the heartbeat and one more for the request, on a
/// connection that is already open and, over Tor, a circuit that is already built.
/// Twenty seconds is generous for that. It is the only part of an exchange a peer
/// reaches without having proved anything, so it is the part that is cheap to hold.
const GREETING_TIMEOUT: Duration = Duration::from_secs(20);

/// How many exchanges one door may have in flight at once.
///
/// The cap has to live here, because nothing below it knows what an exchange is worth.
/// Refusing is deliberate and visible: a peer over the cap is told nothing and the
/// operator sees a line.
const MAX_CONCURRENT_EXCHANGES: usize = 64;

/// How many of those one address may hold at once.
///
/// A node with something to say opens one connection and uses it, so this is already
/// several times what taking part requires. It exists so that one machine cannot be
/// every caller at the door.
const MAX_FROM_ONE_PLACE: usize = 3;

/// Who is at the door, as far as this node can tell.
#[derive(Debug, Clone, Copy)]
pub(super) enum Caller {
    /// A socket, which has an address. It is shown to the operator and counted, and
    /// it is not written down anywhere: an address is not a name.
    At(SocketAddr),
    /// Somebody who came the unseen road, which has no address to show or count.
    ///
    /// A build with no arti in it has no unseen road, so in that build there is
    /// nobody this could be.
    #[cfg(feature = "tor")]
    Unseen,
}

impl std::fmt::Display for Caller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::At(address) => write!(f, "{address}"),
            #[cfg(feature = "tor")]
            Self::Unseen => write!(f, "over tor"),
        }
    }
}

/// One way in, with its own places.
#[derive(Debug, Clone)]
pub(super) struct Door {
    /// The places at this door.
    room: Arc<Semaphore>,
    /// How many are held by each address, so that no address holds them all.
    from_each: Arc<Mutex<HashMap<IpAddr, usize>>>,
}

impl Door {
    /// A door with nobody at it.
    pub(super) fn new() -> Self {
        Self {
            room: Arc::new(Semaphore::new(MAX_CONCURRENT_EXCHANGES)),
            from_each: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Take a place for this caller, if there is one they may have.
    fn place_for(&self, caller: Caller) -> Option<Slot> {
        let room = Arc::clone(&self.room).try_acquire_owned().ok()?;
        let place = match caller {
            Caller::At(address) => Some(address.ip()),
            #[cfg(feature = "tor")]
            Caller::Unseen => None,
        };
        if let Some(place) = place {
            let mut counts = self.counts();
            let held = counts.entry(place).or_insert(0);
            if *held >= MAX_FROM_ONE_PLACE {
                return None;
            }
            *held += 1;
        }
        Some(Slot {
            _room: room,
            place,
            from_each: Arc::clone(&self.from_each),
        })
    }

    /// The count of who is here.
    ///
    /// A thread that panicked while holding this lock poisoned a map that is only ever
    /// incremented and decremented, so what is in it is still the count. Refusing every
    /// caller from then on would be the only harm.
    fn counts(&self) -> std::sync::MutexGuard<'_, HashMap<IpAddr, usize>> {
        self.from_each
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

/// A place at the door, given back when the exchange ends however it ends.
struct Slot {
    /// One of the door's places.
    _room: OwnedSemaphorePermit,
    /// The address it was counted against, if the caller had one.
    place: Option<IpAddr>,
    /// Where that count lives.
    from_each: Arc<Mutex<HashMap<IpAddr, usize>>>,
}

impl Drop for Slot {
    fn drop(&mut self) {
        let Some(place) = self.place else { return };
        let mut counts = self
            .from_each
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(held) = counts.get_mut(&place) {
            *held = held.saturating_sub(1);
            if *held == 0 {
                counts.remove(&place);
            }
        }
    }
}

/// Give one peer its own task, its own deadline and one of the door's places.
pub(super) fn spawn_exchange<S>(mut stream: S, node: &Arc<Node>, door: &Door, caller: Caller)
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let Some(slot) = door.place_for(caller) else {
        println!("turned away {caller}: this door is full");
        return;
    };
    let node = Arc::clone(node);
    // One slow or hostile peer must not hold up the next one, so each exchange runs on
    // its own task, under a deadline, and its failure is reported rather than
    // propagated. The place is given back when the task ends, whichever way.
    tokio::spawn(async move {
        let _slot: Slot = slot;
        match tokio::time::timeout(EXCHANGE_TIMEOUT, greet_then_listen(&mut stream, &node)).await {
            Ok(Ok(())) => {}
            Ok(Err(e)) => println!("refused  {e:#}"),
            Err(_elapsed) => println!(
                "silence  {} s of it, so we let go",
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
    let asked = match tokio::time::timeout(GREETING_TIMEOUT, greeting(stream, node)).await {
        Ok(asked) => asked?,
        Err(_elapsed) => {
            println!(
                "silence  {} s and not a word said, so the door is free again",
                GREETING_TIMEOUT.as_secs()
            );
            return Ok(());
        }
    };

    match asked {
        // A peer that only wanted to exchange heartbeats hangs up here, which is not a
        // failure and is the ordinary case. So is one whose heartbeat did not open.
        None | Some(Asked::Nothing) => Ok(()),
        Some(Asked::Liveness(question)) => be_asked(stream, node, question).await,
        Some(Asked::TheFile(plea)) => hand_it_over(stream, node, &plea).await,
        Some(Asked::Tidings(header)) => trade(stream, node, &header).await,
    }
}

/// Trade heartbeats and hear what the peer came for, or nothing if it was not a peer.
async fn greeting<S>(stream: &mut S, node: &Node) -> anyhow::Result<Option<Asked>>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    match respond(stream, node.identity()).await {
        Ok(exchange) => println!("{}", describe(&exchange)),
        Err(e) => {
            report(&e);
            return Ok(None);
        }
    }
    Ok(Some(n333_net::take_request(stream).await?))
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

#[cfg(test)]
mod tests {
    use super::*;

    fn caller(address: &str) -> Caller {
        Caller::At(address.parse().expect("an address"))
    }

    #[test]
    fn one_place_cannot_hold_the_whole_door() {
        // The attack this is against costs one TCP connection per place and nothing
        // else: connect, say nothing, hold. What it must not be able to buy is every
        // place at once, because a door held shut answers no challenge and hands the
        // file to nobody.
        let door = Door::new();
        let held: Vec<Slot> = (0..MAX_FROM_ONE_PLACE)
            .filter_map(|n| door.place_for(caller(&format!("10.0.0.1:{}", 4000 + n))))
            .collect();
        assert_eq!(held.len(), MAX_FROM_ONE_PLACE, "a caller gets its few places");
        assert!(
            door.place_for(caller("10.0.0.1:4999")).is_none(),
            "and not one more, however many sockets it opens"
        );
        assert!(
            door.place_for(caller("10.0.0.2:4000")).is_some(),
            "while everybody else is answered as usual"
        );

        // What is given back is taken again.
        drop(held);
        assert!(door.place_for(caller("10.0.0.1:5000")).is_some());
    }

    #[test]
    fn a_door_is_full_when_its_places_are_taken() {
        let door = Door::new();
        let held: Vec<Slot> = (0..MAX_CONCURRENT_EXCHANGES)
            .filter_map(|n| {
                door.place_for(caller(&format!(
                    "10.{}.{}.1:4000",
                    n / MAX_FROM_ONE_PLACE / 256,
                    n / MAX_FROM_ONE_PLACE % 256
                )))
            })
            .collect();
        assert_eq!(held.len(), MAX_CONCURRENT_EXCHANGES);
        assert!(door.place_for(caller("10.9.9.9:4000")).is_none());
    }
}
