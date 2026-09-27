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
use n333_net::{Asked, asked, frame, gossip, handover, liveness, respond, session};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

use crate::commands::describe;
use crate::node::Node;

use super::answering::{ask_since_they_came, be_asked, hand_it_over, trade};

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
            Self::Unseen => write!(f, "{}", words!("serve-door-over-tor")),
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
        aloud_in!("serve-door-full", caller = caller.to_string());
        return;
    };
    let node = Arc::clone(node);
    // One slow or hostile peer must not hold up the next one, so each exchange runs on
    // its own task, under a deadline, and its failure is reported rather than
    // propagated. The place is given back when the task ends, whichever way.
    tokio::spawn(async move {
        let _slot: Slot = slot;
        match tokio::time::timeout(
            EXCHANGE_TIMEOUT,
            greet_then_listen(&mut stream, &node, caller),
        )
        .await
        {
            Ok(Ok(())) => {}
            Ok(Err(e)) => aloud!("{}", ended(caller, &e)),
            Err(_elapsed) => aloud_in!(
                "serve-door-silence-exchange",
                seconds = EXCHANGE_TIMEOUT.as_secs()
            ),
        }
    });
}

/// The heartbeat, and then whatever the peer came for.
async fn greet_then_listen<S>(stream: &mut S, node: &Node, caller: Caller) -> anyhow::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let asked = match tokio::time::timeout(GREETING_TIMEOUT, greeting(stream, node, caller)).await {
        Ok(asked) => asked?,
        Err(_elapsed) => {
            aloud_in!(
                "serve-door-silence-greeting",
                seconds = GREETING_TIMEOUT.as_secs()
            );
            return Ok(());
        }
    };

    match asked {
        // A peer that only wanted to exchange heartbeats hangs up here, which is not a
        // failure and is the ordinary case. So is one whose heartbeat did not open.
        None | Some(Asked::Nothing) => Ok(()),
        Some(Asked::Liveness(question)) => be_asked(stream, node, question).await,
        Some(Asked::Presenting(who)) => ask_since_they_came(stream, node, &who).await,
        Some(Asked::TheFile(plea)) => hand_it_over(stream, node, &plea).await,
        Some(Asked::Tidings(header)) => trade(stream, node, &header).await,
    }
}

/// Trade heartbeats and hear what the peer came for, or nothing if it was not a peer.
async fn greeting<S>(stream: &mut S, node: &Node, caller: Caller) -> anyhow::Result<Option<Asked>>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    match respond(stream, node.identity()).await {
        // This node knocking on its own outside address to find out whether the port
        // reaches it. Nobody else can arrive holding this key, so there is no other
        // reading of it. It is said plainly rather than through `describe`, which would
        // put this node's own name where a peer's name goes — and that line is the one
        // on the screen that is supposed to mean somebody else turned up.
        Ok(exchange) if exchange.peer.node_id == node.identity().node_id() => {
            aloud_in!("serve-door-knock");
        }
        Ok(exchange) => aloud!("{}", describe(&exchange)),
        Err(e) => {
            aloud!("{}", unmet(caller, &e));
            return Ok(None);
        }
    }
    Ok(Some(n333_net::take_request(stream).await?))
}

/// A heartbeat that did not happen is the peer's problem, not this node's, so it is said
/// and forgotten. Distinguishing the kinds matters: a stream that stopped is a bad
/// connection or a port scan, while a bad signature or an oversized frame is somebody
/// sending what this node will not take.
fn unmet(caller: Caller, error: &session::Error) -> String {
    match error {
        session::Error::Frame(frame::Error::Io(e)) => words!(
            "serve-door-broken-heartbeat",
            caller = caller.to_string(),
            why = e.to_string()
        ),
        other => words!(
            "serve-door-refused",
            caller = caller.to_string(),
            why = other.to_string()
        ),
    }
}

/// An exchange that ended before it was done, said as whose doing it was.
///
/// "Refused" is only one of three. This node's own record failing to take a statement is
/// this node's failure and says so. A connection that stopped is nobody's decision. What
/// is left is this node declining what it was sent.
fn ended(caller: Caller, error: &anyhow::Error) -> String {
    let (caller, why) = (caller.to_string(), format!("{error:#}"));
    if crate::failed::is_our_own(error) {
        return words!("serve-door-failed-answering", caller = caller, why = why);
    }
    if error.chain().any(stopped) {
        return words!("serve-door-broken-exchange", caller = caller, why = why);
    }
    words!("serve-door-refused", caller = caller, why = why)
}

/// Whether one cause is a connection that stopped, at whichever layer it surfaced.
fn stopped(cause: &(dyn std::error::Error + 'static)) -> bool {
    let io = |frame: &frame::Error| matches!(frame, frame::Error::Io(_));
    cause.downcast_ref::<frame::Error>().is_some_and(io)
        || matches!(cause.downcast_ref(), Some(session::Error::Frame(f)) if io(f))
        || matches!(cause.downcast_ref(), Some(asked::Error::Frame(f)) if io(f))
        || matches!(cause.downcast_ref(), Some(gossip::Error::Frame(f)) if io(f))
        || matches!(cause.downcast_ref(), Some(handover::Error::Frame(f)) if io(f))
        || matches!(cause.downcast_ref(), Some(liveness::Error::Frame(f)) if io(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn caller(address: &str) -> Caller {
        Caller::At(address.parse().expect("an address"))
    }

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let pairs = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            [
                (
                    words!("serve-door-silence-exchange", seconds = 60_u64),
                    "silence  60 s of it, so we let go",
                ),
                (
                    words!("serve-door-silence-greeting", seconds = 20_u64),
                    "silence  20 s and not a word said, so the door is free again",
                ),
                (
                    words!("serve-door-knock"),
                    "knock    this node reached its own front door",
                ),
                (
                    unmet(
                        caller("10.0.0.1:4000"),
                        &session::Error::Frame(frame::Error::from(std::io::Error::from(
                            std::io::ErrorKind::UnexpectedEof,
                        ))),
                    ),
                    "broken   10.0.0.1:4000 stopped before the heartbeat was done: \
                     unexpected end of file",
                ),
                (
                    words!(
                        "serve-door-failed-answering",
                        caller = "10.0.0.1:4000",
                        why = "no room"
                    ),
                    "failed   answering 10.0.0.1:4000: no room",
                ),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

    #[test]
    fn a_full_door_turns_a_caller_away_in_a_keyword_that_fits_the_column() {
        // It was `turned away `, two words and a column too wide for every other line.
        let said = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            words!("serve-door-full", caller = "10.0.0.1:4000")
        });
        assert_eq!(said, "turned   10.0.0.1:4000: this door is full");
    }

    #[cfg(feature = "tor")]
    #[test]
    fn a_caller_over_tor_is_said_as_the_road_it_came_by() {
        let said = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            Caller::Unseen.to_string()
        });
        assert_eq!(said, "over tor");
    }

    #[test]
    fn an_exchange_that_ended_says_whose_doing_it_was() {
        let at = caller("10.0.0.1:4000");
        let eof = || frame::Error::from(std::io::Error::from(std::io::ErrorKind::UnexpectedEof));
        let broke = anyhow::Error::new(handover::Error::from(eof())).context("giving the file");
        assert!(
            ended(at, &broke).starts_with("broken   10.0.0.1:4000 stopped"),
            "{}",
            ended(at, &broke)
        );

        let declined = anyhow::Error::new(handover::Error::NotUs).context("giving the file");
        assert_eq!(
            ended(at, &declined),
            "refused  10.0.0.1:4000: giving the file: the record handed over is about somebody else"
        );

        let disk = n333_store::log::Error::Io {
            path: "/n/window/9".into(),
            source: std::io::Error::from(std::io::ErrorKind::StorageFull),
        };
        let own = anyhow::Error::new(disk).context("keeping a statement about epoch 9");
        assert!(
            ended(at, &own).starts_with("failed   answering 10.0.0.1:4000: keeping"),
            "{}",
            ended(at, &own)
        );
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
        assert_eq!(
            held.len(),
            MAX_FROM_ONE_PLACE,
            "a caller gets its few places"
        );
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
