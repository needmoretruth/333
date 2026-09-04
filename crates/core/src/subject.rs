//! The file this whole thing exists to keep alive.
//!
//! FROZEN. The digest below is the identity of the file. The name `333.txt` is not:
//! it is what people happen to call it, and a copy under any other name is the same
//! file.
//!
//! THIS PROGRAM CANNOT PRODUCE THE FILE. It holds the digest and never the content,
//! so the only way to obtain a [`Subject`] is to be handed bytes that hash to it.
//! There is no constructor that takes no arguments and no constant to copy from. A
//! node that has the file has it because another node gave it to them.
//!
//! That is a rule about how this program is built, and it is not a lock. The content
//! is three bytes; anyone can guess it, and it is written in the project's own README.
//! Nothing here claims otherwise, and no part of the protocol treats possession of the
//! file as proof of anything. What gets recorded is the act of handing it on — see
//! [`crate::transfer`] — and that record needs two keys, which is the part that
//! cannot be faked alone.

use sha2::{Digest as _, Sha256};

/// What people call the file. Convention, not identity.
///
/// Two nodes holding the same bytes under different names hold the same file, and
/// nothing in the protocol reads this string.
pub const FILENAME: &str = "333.txt";

/// How many bytes the file is.
///
/// Not a secret, and not a defence: it is here so that a reader receiving a stream
/// can stop before an endless one exhausts them.
pub const LENGTH: usize = 3;

/// SHA-256 of the file's content.
///
/// FROZEN. This is the file's identity and the only description of the content this
/// program carries.
pub const DIGEST: [u8; 32] = [
    0x55, 0x6d, 0x7d, 0xc3, 0xa1, 0x15, 0x35, 0x63, //
    0x50, 0xf1, 0xf9, 0x91, 0x0b, 0x1a, 0xf1, 0xab, //
    0x0e, 0x31, 0x2d, 0x4b, 0x3e, 0x4f, 0xc7, 0x88, //
    0xd2, 0xda, 0x63, 0x66, 0x8f, 0x36, 0xd0, 0x17, //
];

/// The file, once received and recognised.
///
/// Holding one of these is a claim this program can only make about bytes that came
/// from outside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subject {
    /// The content, exactly as received.
    content: [u8; LENGTH],
}

/// Why some bytes are not the file.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum NotTheFile {
    /// The wrong number of bytes arrived. Reported before hashing, since the length
    /// is known and hashing a stream of any size to find out is what an exhausted
    /// node looks like.
    #[error("the file is {LENGTH} bytes; {0} arrived")]
    WrongLength(usize),
    /// The right number of bytes, but not these bytes.
    #[error("those {LENGTH} bytes are not the file")]
    WrongContent,
}

impl Subject {
    /// Recognise received bytes as the file, or say why they are not.
    ///
    /// # Errors
    /// Fails if the length is wrong, or if the content does not hash to [`DIGEST`].
    pub fn recognise(bytes: &[u8]) -> Result<Self, NotTheFile> {
        let content: [u8; LENGTH] = bytes
            .try_into()
            .map_err(|_| NotTheFile::WrongLength(bytes.len()))?;
        if digest_of(&content) == DIGEST {
            Ok(Self { content })
        } else {
            Err(NotTheFile::WrongContent)
        }
    }

    /// The content, to hand on to somebody else.
    #[must_use]
    pub const fn content(&self) -> &[u8; LENGTH] {
        &self.content
    }
}

/// The SHA-256 of some bytes, as this protocol computes it everywhere.
#[must_use]
pub fn digest_of(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The content, written out only here.
    ///
    /// Deliberately confined to the tests: shipped code has no line that could write
    /// the file, and this constant is the fixture that proves the digest above is the
    /// digest of the right thing rather than of itself.
    const CONTENT: &[u8] = b"333";

    #[test]
    fn the_digest_is_the_digest_of_the_file_and_not_of_something_else() {
        assert_eq!(digest_of(CONTENT), DIGEST);
        assert_eq!(
            hex(&DIGEST),
            "556d7dc3a115356350f1f9910b1af1ab0e312d4b3e4fc788d2da63668f36d017"
        );
        assert_eq!(CONTENT.len(), LENGTH);
        assert_eq!(FILENAME, "333.txt");
    }

    #[test]
    fn the_file_is_recognised_when_it_arrives() {
        let subject = Subject::recognise(CONTENT).expect("recognises");
        assert_eq!(subject.content(), b"333");
        // What was recognised is what can be handed on: byte for byte, never rebuilt.
        assert_eq!(
            Subject::recognise(subject.content()).expect("recognises again"),
            subject
        );
    }

    #[test]
    fn anything_else_is_refused_and_says_which_way_it_failed() {
        assert_eq!(Subject::recognise(b""), Err(NotTheFile::WrongLength(0)));
        assert_eq!(Subject::recognise(b"33"), Err(NotTheFile::WrongLength(2)));
        assert_eq!(
            Subject::recognise(b"333\n"),
            Err(NotTheFile::WrongLength(4)),
            "a trailing newline is a different file"
        );
        assert_eq!(Subject::recognise(b"334"), Err(NotTheFile::WrongContent));
        assert_eq!(Subject::recognise(b"666"), Err(NotTheFile::WrongContent));
    }

    #[test]
    fn the_length_is_checked_before_anything_is_hashed() {
        // A peer that answers a request for three bytes with a gigabyte must cost
        // this node a length comparison and nothing more.
        let flood = vec![b'3'; 1_000_000];
        assert_eq!(
            Subject::recognise(&flood),
            Err(NotTheFile::WrongLength(1_000_000))
        );
    }

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }
}
