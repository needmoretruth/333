//! Passing on what you have, and taking what the other has.
//!
//! One round, symmetric: each side sends a signed header, then a run of statements,
//! then the empty frame that ends the run. Neither side asks for anything in
//! particular and neither side is obliged to send anything.
//!
//! WHY IT CANNOT GO WRONG. Nothing here is believed. Every statement in a run carries
//! its own signature and is filed by opening it, so a node that sends rubbish is a node
//! that wasted its own bandwidth, and a node that sends nothing has said nothing. There
//! is no state to get out of step, no acknowledgement to lose, and no version of this
//! that ends with two nodes disagreeing about what happened — because nothing happened
//! except that some bytes moved.
//!
//! WHAT IT IS FOR. Mostly addresses. A roll holds keys and the draw is computed from
//! keys, but a node that does not know where a member is cannot put the question to it,
//! and until this existed a node could sign its own whereabouts and hand it to nobody.
//! Admissions travel the same way, and so does a statement about a node that never
//! reached the node it is about.

use futures::{AsyncRead, AsyncWrite};
use n333_core::tidings::{Signed as SignedTidings, Tidings};
use n333_core::{Epoch, Identity, wire};

use crate::frame::{self, AsReceived};

/// Why a round did not finish.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The stream failed mid-frame, or a run ran past its end.
    #[error("frame: {0}")]
    Frame(#[from] frame::Error),
    /// A signed message was malformed, or its signature did not check out.
    #[error("message: {0}")]
    Message(#[from] wire::Error),
    /// What came back where a header was expected is not one.
    #[error("the peer answered with something other than a run of statements")]
    NotTidings,
}

/// Open a round: pass on what this node has, and take back what the peer has.
///
/// # Errors
/// Fails if the stream fails or the peer answers with something else.
pub async fn tell<S>(
    stream: &mut S,
    teller: &Identity,
    now: Epoch,
    mine: &[Vec<u8>],
) -> Result<Vec<Vec<u8>>, Error>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    frame::write_frame(stream, &Tidings::from(teller, now).seal(teller)?).await?;
    frame::write_batch(stream, mine).await?;

    // A peer that hangs up after taking what it was given has been rude and nothing
    // worse: this node keeps what it already had.
    let Ok(header) = frame::read_frame(stream).await else {
        return Ok(Vec::new());
    };
    if header.is_empty() {
        return Ok(Vec::new());
    }
    n333_core::tidings::open(&header).map_err(|_| Error::NotTidings)?;
    Ok(frame::read_batch(stream).await?)
}

/// Answer a round somebody else opened: take what they have, pass on what this has.
///
/// The header has already been read off the wire by whatever decided this was a round
/// of statements; what is left is the run behind it.
///
/// # Errors
/// Fails if the stream fails or a run runs past its end.
pub async fn listen<S>(
    stream: &mut S,
    teller: &Identity,
    now: Epoch,
    _header: &AsReceived<SignedTidings>,
    mine: &[Vec<u8>],
) -> Result<Vec<Vec<u8>>, Error>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let theirs = frame::read_batch(stream).await?;
    frame::write_frame(stream, &Tidings::from(teller, now).seal(teller)?).await?;
    frame::write_batch(stream, mine).await?;
    Ok(theirs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asked::{Asked, take_request};
    use tokio_util::compat::TokioAsyncReadCompatExt as _;

    fn identity(seed: u8) -> Identity {
        Identity::from_seed(&[seed; 32])
    }

    /// One round over a real socket pair, from both ends.
    async fn round(
        opener: Vec<Vec<u8>>,
        answerer: Vec<Vec<u8>>,
    ) -> (Result<Vec<Vec<u8>>, Error>, Result<Vec<Vec<u8>>, Error>) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("binds");
        let address = listener.local_addr().expect("has an address");

        let listening = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accepts");
            let mut stream = stream.compat();
            let Asked::Tidings(header) = take_request(&mut stream).await.expect("a header") else {
                panic!("the peer opened with something else");
            };
            let heard = listen(&mut stream, &identity(2), Epoch(9), &header, &answerer).await;
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            heard
        });

        let mut stream = tokio::net::TcpStream::connect(address)
            .await
            .expect("connects")
            .compat();
        let told = tell(&mut stream, &identity(1), Epoch(9), &opener).await;
        (told, listening.await.expect("the answerer finished"))
    }

    #[tokio::test]
    async fn both_sides_end_the_round_holding_what_the_other_had() {
        let (told, heard) = round(
            vec![b"one".to_vec(), b"two".to_vec()],
            vec![b"three".to_vec()],
        )
        .await;
        assert_eq!(told.expect("the opener finished"), vec![b"three".to_vec()]);
        assert_eq!(
            heard.expect("the answerer finished"),
            vec![b"one".to_vec(), b"two".to_vec()]
        );
    }

    #[tokio::test]
    async fn a_node_with_nothing_to_pass_on_still_completes_the_round() {
        // The ordinary state of a node that has just started, and the reason a run is
        // ended by an empty frame rather than by a count nobody could have sent.
        let (told, heard) = round(Vec::new(), vec![b"something".to_vec()]).await;
        assert_eq!(
            told.expect("the opener finished"),
            vec![b"something".to_vec()]
        );
        assert!(heard.expect("the answerer finished").is_empty());
    }

    #[tokio::test]
    async fn rubbish_travels_and_is_somebody_elses_problem() {
        // Nothing in a run is opened here. Filing is where a statement is judged, and
        // a node that sends nonsense has spent its own bandwidth on it.
        let (told, _) = round(Vec::new(), vec![b"not a statement".to_vec()]).await;
        assert_eq!(
            told.expect("the opener finished"),
            vec![b"not a statement".to_vec()]
        );
    }
}
