//! One heartbeat exchange over an already-open byte stream.
//!
//! This module is deliberately ignorant of Tor. It takes anything that reads and
//! writes bytes, which is what lets the whole exchange be tested in memory.
//!
//! WHAT AN EXCHANGE PROVES, AND WHAT IT DOES NOT. The side that speaks first learns
//! something real: its nonce comes back inside a signature it could not have made,
//! so the peer was alive after the nonce was chosen. The side that answers learns
//! much less — the heartbeat it received could have been recorded earlier and replayed
//! by anyone. That asymmetry is not a defect to be patched here; proving liveness in
//! both directions is what the challenge protocol is for. Saying so in the type is
//! better than a comment, so [`Exchange::proves_peer_was_live`] carries it.

use futures::{AsyncRead, AsyncWrite};
use n333_core::epoch::Epoch;
use n333_core::heartbeat::{self, Heartbeat, Verified};
use n333_core::identity::Identity;

use crate::frame;

/// Things that can go wrong during an exchange.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The stream failed, or the peer sent a frame this node will not read.
    #[error("frame: {0}")]
    Frame(#[from] frame::Error),
    /// The bytes arrived but are not a heartbeat this node accepts.
    #[error("message: {0}")]
    Message(#[from] n333_core::WireError),
    /// The peer signed an answer to a nonce this node never sent. Either it is
    /// answering someone else's heartbeat, or it is replaying a recorded one.
    #[error("peer answered a nonce this node did not send")]
    WrongNonce,
    /// The peer sent a heartbeat of its own instead of an answer.
    #[error("peer sent an opening heartbeat where an answer was expected")]
    NotAnAnswer,
}

/// What one exchange produced.
#[derive(Debug, Clone)]
pub struct Exchange {
    /// The peer's heartbeat, decoded and with its signature checked.
    pub peer: Verified,
    /// The epoch this node believed it was in when the exchange happened.
    pub observed_at: Epoch,
    /// Peer's epoch minus ours. Reported for a human to look at; the protocol takes
    /// no action on it, because it has no authority to decide whose clock is right.
    pub epoch_skew: i64,
    /// The peer's own clock minus this node's, in milliseconds, as of the heartbeat.
    ///
    /// The same disagreement as `epoch_skew` at a resolution a person can act on: two
    /// clocks five hours apart and two clocks a second apart both read as one epoch or
    /// none, and only one of them is worth getting up to fix. It includes the trip the
    /// heartbeat made to get here, which is milliseconds directly and can be seconds
    /// through Tor — it is a reading, not a measurement, and nothing is decided on it.
    pub clocks_apart_ms: i64,
    /// True when the peer quoted back a nonce this node had just chosen, which is
    /// the only thing in this exchange that could not have been recorded earlier.
    pub proves_peer_was_live: bool,
}

impl Exchange {
    fn new(peer: Verified, proves_peer_was_live: bool) -> Self {
        let observed_at = Epoch::now();
        let theirs = i64::try_from(peer.heartbeat.sent_at_ms).unwrap_or(i64::MAX);
        let ours = i64::try_from(n333_core::epoch::unix_now_millis()).unwrap_or(i64::MAX);
        Self {
            epoch_skew: peer.heartbeat.epoch().skew_from(observed_at),
            clocks_apart_ms: theirs.saturating_sub(ours),
            peer,
            observed_at,
            proves_peer_was_live,
        }
    }
}

/// Speak first: send a heartbeat, then read the answer.
///
/// # Errors
/// Fails if the stream fails, the answer is malformed or unsigned, or the answer
/// quotes a nonce this node did not send.
pub async fn initiate<S>(stream: &mut S, identity: &Identity) -> Result<Exchange, Error>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let ours = Heartbeat::now(identity, None);
    frame::write_frame(stream, &ours.seal(identity)?).await?;

    let answer = heartbeat::open(&frame::read_frame(stream).await?)?;
    match answer.heartbeat.in_reply_to {
        None => Err(Error::NotAnAnswer),
        Some(quoted) if quoted != ours.nonce => Err(Error::WrongNonce),
        Some(_) => Ok(Exchange::new(answer, true)),
    }
}

/// Answer: read the peer's heartbeat, then send one quoting its nonce.
///
/// # Errors
/// Fails if the stream fails or the peer's heartbeat is malformed or unsigned.
pub async fn respond<S>(stream: &mut S, identity: &Identity) -> Result<Exchange, Error>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let theirs = heartbeat::open(&frame::read_frame(stream).await?)?;
    let ours = Heartbeat::now(identity, Some(theirs.heartbeat.nonce));
    frame::write_frame(stream, &ours.seal(identity)?).await?;
    Ok(Exchange::new(theirs, false))
}

#[cfg(test)]
mod tests {
    use super::*;
    use n333_core::heartbeat::PROTOCOL_VERSION;
    use tokio_util::compat::TokioAsyncReadCompatExt as _;

    fn identity(seed: u8) -> Identity {
        Identity::from_seed(&[seed; 32])
    }

    /// Run both halves of an exchange over an in-memory pipe.
    async fn exchange(
        opener: Identity,
        answerer: Identity,
    ) -> (Result<Exchange, Error>, Result<Exchange, Error>) {
        let (a, b) = tokio::io::duplex(64 * 1024);
        let (mut a, mut b) = (a.compat(), b.compat());
        let answering = tokio::spawn(async move { respond(&mut b, &answerer).await });
        let opened = initiate(&mut a, &opener).await;
        (opened, answering.await.expect("task"))
    }

    #[tokio::test]
    async fn both_sides_see_each_other() {
        let (a, b) = (identity(1), identity(2));
        let (a_id, b_id) = (a.node_id(), b.node_id());
        let (opened, answered) = exchange(a, b).await;

        let opened = opened.expect("opener finishes");
        let answered = answered.expect("answerer finishes");
        assert_eq!(opened.peer.node_id, b_id);
        assert_eq!(answered.peer.node_id, a_id);
        assert_eq!(opened.peer.heartbeat.protocol, PROTOCOL_VERSION);
    }

    #[tokio::test]
    async fn only_the_opener_learns_the_peer_was_live() {
        let (opened, answered) = exchange(identity(3), identity(4)).await;
        assert!(opened.expect("opener finishes").proves_peer_was_live);
        assert!(!answered.expect("answerer finishes").proves_peer_was_live);
    }

    #[tokio::test]
    async fn an_answer_quoting_the_wrong_nonce_is_refused() {
        let (a, b) = (identity(5), identity(6));
        let (mut client, mut server) = {
            let (x, y) = tokio::io::duplex(64 * 1024);
            (x.compat(), y.compat())
        };
        // The peer answers, but quotes a nonce of its own choosing.
        let forger = tokio::spawn(async move {
            let _theirs = frame::read_frame(&mut server).await?;
            let wrong = Heartbeat::now(&b, Some([0xcc; 32]));
            frame::write_frame(&mut server, &wrong.seal(&b)?).await?;
            Ok::<(), Error>(())
        });
        let result = initiate(&mut client, &a).await;
        forger.await.expect("task").expect("peer writes");
        assert!(matches!(result, Err(Error::WrongNonce)), "got {result:?}");
    }

    #[tokio::test]
    async fn an_opening_heartbeat_where_an_answer_belongs_is_refused() {
        let (a, b) = (identity(7), identity(8));
        let (mut client, mut server) = {
            let (x, y) = tokio::io::duplex(64 * 1024);
            (x.compat(), y.compat())
        };
        let peer = tokio::spawn(async move {
            let _theirs = frame::read_frame(&mut server).await?;
            let opening = Heartbeat::now(&b, None);
            frame::write_frame(&mut server, &opening.seal(&b)?).await?;
            Ok::<(), Error>(())
        });
        let result = initiate(&mut client, &a).await;
        peer.await.expect("task").expect("peer writes");
        assert!(matches!(result, Err(Error::NotAnAnswer)), "got {result:?}");
    }

    #[tokio::test]
    async fn a_replayed_exchange_does_not_fool_the_opener() {
        // Record a real answer, then replay it into a fresh exchange. The nonce in the
        // recording belongs to the old exchange, so the opener refuses it.
        let (a, b) = (identity(9), identity(10));
        let first = Heartbeat::now(&a, None);
        let recorded = Heartbeat::now(&b, Some(first.nonce))
            .seal(&b)
            .expect("seals");

        let (mut client, mut server) = {
            let (x, y) = tokio::io::duplex(64 * 1024);
            (x.compat(), y.compat())
        };
        let replayer = tokio::spawn(async move {
            let _fresh = frame::read_frame(&mut server).await?;
            frame::write_frame(&mut server, &recorded).await?;
            Ok::<(), Error>(())
        });
        let result = initiate(&mut client, &a).await;
        replayer.await.expect("task").expect("peer writes");
        assert!(matches!(result, Err(Error::WrongNonce)), "got {result:?}");
    }

    #[tokio::test]
    async fn epoch_skew_is_reported_not_enforced() {
        let (opened, _) = exchange(identity(11), identity(12)).await;
        // Two nodes on one clock agree, and the exchange still reports the number
        // rather than asserting anything about it.
        assert_eq!(opened.expect("opener finishes").epoch_skew, 0);
    }
}
