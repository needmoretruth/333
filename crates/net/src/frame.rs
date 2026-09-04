//! Length-prefixed framing over any byte stream.
//!
//! FROZEN. A frame is a four-byte big-endian length followed by that many bytes.
//!
//! The length is read before the body, so the limit is enforced on the *announced*
//! size: a stranger cannot make this node allocate a buffer it did not agree to.
//! Four bytes in front of a 139- or 171-byte heartbeat frame is under 3% of overhead,
//! which buys room for the larger messages that come later without a second framing
//! to reason about.
//!
//! There is no deadline here. Framing is generic over any byte stream so the exchange
//! can be tested in memory, and a timer inside it would add a runtime dependency, a
//! second responsibility, and per-call deadlines that sum instead of bounding. The
//! deadline belongs to the caller that knows what the whole exchange is worth.

use futures::{AsyncRead, AsyncReadExt as _, AsyncWrite, AsyncWriteExt as _};
use n333_core::MAX_FRAME_LEN;

/// Length of the size prefix.
pub const LENGTH_PREFIX_LEN: usize = 4;

/// Things that can go wrong moving a frame.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The stream failed, or ended in the middle of a frame.
    #[error("stream: {0}")]
    Io(#[from] std::io::Error),
    /// The peer announced a frame larger than this node will read.
    #[error("peer announced a {got}-byte frame, over the {MAX_FRAME_LEN}-byte limit")]
    TooLong {
        /// The announced size.
        got: usize,
    },
}

/// Write one frame.
///
/// The stream is flushed: arti buffers writes, and an unflushed heartbeat is a
/// heartbeat the peer never sees.
///
/// # Errors
/// Fails if the frame is over the limit or the stream fails.
pub async fn write_frame<W: AsyncWrite + Unpin>(
    stream: &mut W,
    frame: &[u8],
) -> Result<(), Error> {
    if frame.len() > MAX_FRAME_LEN {
        return Err(Error::TooLong { got: frame.len() });
    }
    let length = u32::try_from(frame.len()).map_err(|_| Error::TooLong { got: frame.len() })?;
    stream.write_all(&length.to_be_bytes()).await?;
    stream.write_all(frame).await?;
    stream.flush().await?;
    Ok(())
}

/// The announced length, if this node agreed to read that much.
///
/// Pulled out of [`read_frame`] so the bound is a pure function with its own test.
/// Inside an async function that also allocates, the only thing keeping the check
/// ahead of the allocation is the order of two lines.
fn checked_length(prefix: [u8; LENGTH_PREFIX_LEN]) -> Result<usize, Error> {
    let length = usize::try_from(u32::from_be_bytes(prefix)).unwrap_or(usize::MAX);
    if length > MAX_FRAME_LEN {
        return Err(Error::TooLong { got: length });
    }
    Ok(length)
}

/// Read one frame.
///
/// # Errors
/// Fails if the peer announces an oversized frame, or the stream ends early.
pub async fn read_frame<R: AsyncRead + Unpin>(stream: &mut R) -> Result<Vec<u8>, Error> {
    let mut prefix = [0_u8; LENGTH_PREFIX_LEN];
    stream.read_exact(&mut prefix).await?;
    let mut frame = vec![0_u8; checked_length(prefix)?];
    stream.read_exact(&mut frame).await?;
    Ok(frame)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_util::compat::TokioAsyncReadCompatExt as _;

    #[tokio::test]
    async fn a_frame_survives_the_round_trip() {
        let (client, server) = tokio::io::duplex(64 * 1024);
        let (mut client, mut server) = (client.compat(), server.compat());
        let sent = vec![7_u8; 300];
        let expected = sent.clone();
        let writer = tokio::spawn(async move {
            write_frame(&mut client, &sent).await.map_err(|e| e.to_string())
        });
        let got = read_frame(&mut server).await.expect("reads");
        writer.await.expect("task").expect("writes");
        assert_eq!(got, expected);
    }

    #[tokio::test]
    async fn an_empty_frame_round_trips() {
        let (client, server) = tokio::io::duplex(1024);
        let (mut client, mut server) = (client.compat(), server.compat());
        let writer =
            tokio::spawn(async move { write_frame(&mut client, &[]).await.map_err(|e| e.to_string()) });
        assert_eq!(read_frame(&mut server).await.expect("reads"), Vec::<u8>::new());
        writer.await.expect("task").expect("writes");
    }

    #[test]
    fn the_announced_length_is_bounded_before_anything_is_read() {
        assert_eq!(checked_length([0, 0, 0, 0]).expect("zero is a frame"), 0);
        assert_eq!(
            checked_length([0, 0, 0x10, 0]).expect("exactly the limit"),
            MAX_FRAME_LEN
        );
        assert!(matches!(
            checked_length([0, 0, 0x10, 1]),
            Err(Error::TooLong { got }) if got == MAX_FRAME_LEN + 1
        ));
        assert!(matches!(
            checked_length([0xff, 0xff, 0xff, 0xff]),
            Err(Error::TooLong { .. })
        ));
    }

    #[tokio::test]
    async fn a_frame_of_exactly_the_limit_round_trips() {
        // Tested only from the refusing side, an off-by-one would refuse the largest
        // frame the protocol promises to carry.
        let (client, server) = tokio::io::duplex(64 * 1024);
        let (mut client, mut server) = (client.compat(), server.compat());
        let sent = vec![3_u8; MAX_FRAME_LEN];
        let writer = tokio::spawn(async move { write_frame(&mut client, &sent).await });
        let got = read_frame(&mut server).await.expect("reads");
        writer.await.expect("task").expect("writes");
        assert_eq!(got.len(), MAX_FRAME_LEN);
    }

    #[tokio::test]
    async fn an_oversized_announcement_is_refused_before_the_body_is_read() {
        let (client, server) = tokio::io::duplex(1024);
        let (mut client, mut server) = (client.compat(), server.compat());
        // Announce four gigabytes and send nothing. A reader that waited for the
        // body before checking the length would block here forever.
        let announced = u32::MAX;
        let writer = tokio::spawn(async move {
            client.write_all(&announced.to_be_bytes()).await.map_err(|e| e.to_string())?;
            client.flush().await.map_err(|e| e.to_string())
        });
        let err = read_frame(&mut server).await.expect_err("refuses");
        assert!(matches!(err, Error::TooLong { .. }), "got {err:?}");
        writer.await.expect("task").expect("writes");
    }

    #[tokio::test]
    async fn a_stream_that_ends_mid_frame_is_an_error() {
        let (client, server) = tokio::io::duplex(1024);
        let (mut client, mut server) = (client.compat(), server.compat());
        let writer = tokio::spawn(async move {
            client.write_all(&10_u32.to_be_bytes()).await.map_err(|e| e.to_string())?;
            client.write_all(&[1, 2, 3]).await.map_err(|e| e.to_string())?;
            client.flush().await.map_err(|e| e.to_string())
            // then dropped: the promised ten bytes never arrive
        });
        writer.await.expect("task").expect("writes");
        let err = read_frame(&mut server).await.expect_err("refuses");
        assert!(matches!(err, Error::Io(_)), "got {err:?}");
    }

    #[tokio::test]
    async fn writing_over_the_limit_is_this_nodes_own_error() {
        let (client, _server) = tokio::io::duplex(1024);
        let mut client = client.compat();
        let too_big = vec![0_u8; MAX_FRAME_LEN + 1];
        let err = write_frame(&mut client, &too_big).await.expect_err("refuses");
        assert!(matches!(err, Error::TooLong { .. }), "got {err:?}");
    }
}
