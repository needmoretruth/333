//! Length-prefixed framing over any byte stream.
//!
//! FROZEN. A frame is a four-byte big-endian length followed by that many bytes.
//!
//! The length is read before the body, so the limit is enforced on the *announced*
//! size: a stranger cannot make this node allocate a buffer it did not agree to.
//! Four bytes for a 150-byte heartbeat is 2.7% of overhead, which buys room for the
//! larger messages that come later without a second framing to reason about.

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

/// Read one frame.
///
/// # Errors
/// Fails if the peer announces an oversized frame, or the stream ends early.
pub async fn read_frame<R: AsyncRead + Unpin>(stream: &mut R) -> Result<Vec<u8>, Error> {
    let mut length_bytes = [0_u8; LENGTH_PREFIX_LEN];
    stream.read_exact(&mut length_bytes).await?;
    let length = usize::try_from(u32::from_be_bytes(length_bytes)).unwrap_or(usize::MAX);
    if length > MAX_FRAME_LEN {
        return Err(Error::TooLong { got: length });
    }
    let mut frame = vec![0_u8; length];
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

    #[tokio::test]
    async fn an_oversized_announcement_is_refused_without_allocating() {
        let (client, server) = tokio::io::duplex(1024);
        let (mut client, mut server) = (client.compat(), server.compat());
        // Announce four gigabytes and send nothing. A reader that allocated first
        // would be holding the buffer before it noticed.
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
