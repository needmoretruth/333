//! What a peer says after the heartbeat, and which of the three things it is.
//!
//! A node that has just exchanged heartbeats can want one of exactly three things: to
//! ask whether this node is awake, to be given the file, or to trade what each of them
//! knows. Anything else is refused by name rather than guessed at, because a frame this
//! build does not recognise is not something to improvise on.
//!
//! WHY THIS IS ONE FRAME AND NOT A HANDSHAKE. There is no negotiation, no capability
//! list and no version dance beyond the one number already inside every message. The
//! peer says the one thing it came to say, and this node either understands it or
//! does not.

use futures::AsyncRead;
use n333_core::challenge::{self, SignedChallenge};
use n333_core::plea::{self, Signed as SignedPlea};
use n333_core::tidings::{self, Signed as SignedTidings};

use crate::frame::{self, AsReceived};

/// Why a frame after the heartbeat was not acted on.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The stream failed mid-frame.
    #[error("frame: {0}")]
    Frame(#[from] frame::Error),
    /// The frame opened as none of the three things a peer may ask for.
    #[error("this node was sent something it does not understand")]
    Unrecognised,
}

/// What the peer came for.
#[derive(Debug, Clone)]
pub enum Asked {
    /// It said its piece and hung up, which is the ordinary shape of a node that
    /// only wanted to exchange heartbeats.
    Nothing,
    /// Whether this node is awake, put as a challenge this node must answer.
    Liveness(AsReceived<SignedChallenge>),
    /// The file, which this node can hand over only if it has it.
    TheFile(AsReceived<SignedPlea>),
    /// A trade: the peer will pass on what it has and take what this node has.
    Tidings(AsReceived<SignedTidings>),
}

/// Read whatever the peer says next, and say which of the three it is.
///
/// # Errors
/// Fails if the stream fails mid-frame, or the frame is none of the three.
pub async fn take_request<S>(stream: &mut S) -> Result<Asked, Error>
where
    S: AsyncRead + Unpin,
{
    let frame = match frame::read_frame(stream).await {
        Ok(frame) => frame,
        // A closed stream is a peer that finished, not a peer that failed.
        Err(frame::Error::Io(_)) => return Ok(Asked::Nothing),
        Err(e) => return Err(e.into()),
    };
    // Tried in turn rather than tagged: every message already carries its domain
    // inside the signature, so exactly one of these can succeed on any given frame
    // and a tag in front of it would be a second, weaker copy of that fact.
    if let Ok(message) = challenge::open_challenge(&frame) {
        return Ok(Asked::Liveness(AsReceived { message, frame }));
    }
    if let Ok(message) = plea::open(&frame) {
        return Ok(Asked::TheFile(AsReceived { message, frame }));
    }
    if let Ok(message) = tidings::open(&frame) {
        return Ok(Asked::Tidings(AsReceived { message, frame }));
    }
    Err(Error::Unrecognised)
}

#[cfg(test)]
mod tests {
    use super::*;
    use n333_core::plea::Plea;
    use n333_core::{Challenge, Epoch, Identity};

    fn identity(seed: u8) -> Identity {
        Identity::from_seed(&[seed; 32])
    }

    async fn sent(frame: &[u8]) -> Result<Asked, Error> {
        let mut pipe = Vec::new();
        frame::write_frame(&mut pipe, frame).await.expect("writes");
        take_request(&mut pipe.as_slice()).await
    }

    #[tokio::test]
    async fn a_challenge_is_read_as_a_challenge() {
        let (verifier, prover) = (identity(1), identity(2));
        let frame = Challenge::new(&verifier, prover.public_key(), Epoch(7))
            .seal(&verifier)
            .expect("seals");
        assert!(matches!(sent(&frame).await, Ok(Asked::Liveness(_))));
    }

    #[tokio::test]
    async fn a_plea_is_read_as_a_plea() {
        let asker = identity(3);
        let frame = Plea::of(&asker, Epoch(7)).seal(&asker).expect("seals");
        assert!(matches!(sent(&frame).await, Ok(Asked::TheFile(_))));
    }

    #[tokio::test]
    async fn the_two_do_not_open_as_each_other() {
        // Both are short signed messages, and what keeps them apart is the domain
        // inside the signature rather than anything about their shape.
        let asker = identity(3);
        let frame = Plea::of(&asker, Epoch(7)).seal(&asker).expect("seals");
        assert!(challenge::open_challenge(&frame).is_err());
    }

    #[tokio::test]
    async fn a_run_of_statements_is_read_as_a_run_of_statements() {
        let teller = identity(3);
        let frame = n333_core::Tidings::from(&teller, Epoch(7))
            .seal(&teller)
            .expect("seals");
        assert!(matches!(sent(&frame).await, Ok(Asked::Tidings(_))));
    }

    #[tokio::test]
    async fn anything_else_is_refused_by_name() {
        assert!(matches!(
            sent(b"neither of those things").await,
            Err(Error::Unrecognised)
        ));
    }

    #[tokio::test]
    async fn a_peer_that_hangs_up_asked_for_nothing() {
        let empty: &[u8] = &[];
        assert!(matches!(
            take_request(&mut { empty }).await,
            Ok(Asked::Nothing)
        ));
    }
}
