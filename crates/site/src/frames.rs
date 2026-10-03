//! Frames as a node writes them: a four-byte big-endian length, then that many bytes.
//!
//! The board is handed to clients in this shape, and a node's logs are kept in it, so
//! one pair of functions serves both. The store crate has the same reader, but it opens
//! the file to append and cuts a torn tail off; the observer must never write into the
//! node's directory, so it reads the bytes and stops where a whole frame stops.

use n333_store::log::{LENGTH_PREFIX_LEN, MAX_RECORD_LEN};

/// Split bytes into the whole frames at their front.
///
/// Stops, keeping what it has, at a torn tail (a node that died mid-write) and at a
/// length no node writes (damage): the frames before either are still good.
pub(crate) fn split(bytes: &[u8]) -> Vec<Vec<u8>> {
    let mut frames = Vec::new();
    let mut rest = bytes;
    while let Some((prefix, after)) = rest.split_first_chunk::<LENGTH_PREFIX_LEN>() {
        let announced = u32::from_be_bytes(*prefix);
        let Ok(length) = usize::try_from(announced) else {
            break;
        };
        if length > MAX_RECORD_LEN {
            break;
        }
        let Some((frame, next)) = after.split_at_checked(length) else {
            break;
        };
        frames.push(frame.to_vec());
        rest = next;
    }
    frames
}

/// Put frames one after another, each behind its length.
///
/// A frame too long for four bytes to announce is left out rather than announced
/// wrongly. Nothing that reaches here is longer than 512 bytes.
pub(crate) fn join<'a>(frames: impl Iterator<Item = &'a [u8]>) -> Vec<u8> {
    let mut out = Vec::new();
    for frame in frames {
        let Ok(length) = u32::try_from(frame.len()) else {
            continue;
        };
        out.extend_from_slice(&length.to_be_bytes());
        out.extend_from_slice(frame);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frames_survive_the_round_trip_and_a_torn_tail_is_left_behind() {
        let mut bytes = join([b"one".as_slice(), b"two".as_slice()].into_iter());
        bytes.extend_from_slice(&[0, 0, 0, 9, b'x']);
        assert_eq!(split(&bytes), vec![b"one".to_vec(), b"two".to_vec()]);
    }
}
