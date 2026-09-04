//! The bytes on the wire.
//!
//! FROZEN. Everything in this module is part of the protocol's identity after 3.3.3.
//! A frame is
//!
//! ```text
//! frame = signature (64 bytes) || body (postcard)
//! ```
//!
//! and what gets signed is
//!
//! ```text
//! signed = domain (16 bytes) || body
//! ```
//!
//! Two decisions here are worth their reasons.
//!
//! The signature sits **outside** the signed value, not in a field of it. Putting it
//! inside forces the sender to serialize with the field zeroed, sign, then overwrite
//! it — and forces the verifier to decode, blank the field and re-serialize. Every
//! signature check would then depend on the encoder round-tripping byte for byte,
//! which quietly breaks on any future encoder change. Here the verifier signs and
//! checks the bytes it actually received, and never re-serializes anything.
//!
//! The domain is a fixed 16-byte string prepended before signing. Without it, a
//! signature collected in one part of the protocol can be replayed as a valid
//! signature in another. All domains are exactly 16 bytes so that no domain can be a
//! prefix of another.

use crate::identity::Identity;

/// Length of a domain-separation tag. Every domain is exactly this long.
pub const DOMAIN_LEN: usize = 16;

/// Length of an Ed25519 signature.
pub const SIGNATURE_LEN: usize = 64;

/// Largest frame this protocol will read from a peer.
///
/// A heartbeat is under 150 bytes; the rest is room for the message types that come
/// after it. Bulk transfers — a node handing over its attendance log — do not travel
/// as one frame and are not bounded by this number; when they arrive they carry their
/// own limit, because a stranger must never be able to make this node buffer more
/// than it agreed to.
pub const MAX_FRAME_LEN: usize = 4096;

/// The domain for heartbeats.
pub const DOMAIN_HEARTBEAT: &[u8; DOMAIN_LEN] = b"333.v1.heartbeat";

/// Things that can be wrong with bytes claiming to be a frame.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    /// Shorter than a signature, so there is no body at all.
    #[error("frame is {got} bytes, shorter than the {SIGNATURE_LEN}-byte signature")]
    TooShort {
        /// How many bytes arrived.
        got: usize,
    },
    /// Longer than [`MAX_FRAME_LEN`].
    #[error("frame is {got} bytes, over the {MAX_FRAME_LEN}-byte limit")]
    TooLong {
        /// How many bytes were announced or arrived.
        got: usize,
    },
    /// The body is not a valid encoding of the expected message.
    #[error("body does not decode: {0}")]
    Decode(String),
    /// The signature does not match the body under the sender's key.
    #[error("signature does not verify")]
    BadSignature,
    /// The sender's public key is unusable.
    #[error("sender key: {0}")]
    SenderKey(#[from] crate::identity::PublicKeyError),
    /// The message announces a protocol version this build does not speak.
    #[error("protocol version {got}, this build speaks {expected}")]
    Version {
        /// What the peer announced.
        got: u16,
        /// What this build speaks.
        expected: u16,
    },
}

/// Build the exact byte string that gets signed.
#[must_use]
pub fn signing_input(domain: &[u8; DOMAIN_LEN], body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(DOMAIN_LEN + body.len());
    out.extend_from_slice(domain);
    out.extend_from_slice(body);
    out
}

/// Sign `body` under `domain` and lay out the frame.
///
/// # Errors
/// Fails if the resulting frame would exceed [`MAX_FRAME_LEN`]. That is this node's
/// own bug, not a peer's, so it is worth failing loudly rather than sending a frame
/// the other side is required to refuse.
pub fn seal(
    domain: &[u8; DOMAIN_LEN],
    body: &[u8],
    identity: &Identity,
) -> Result<Vec<u8>, Error> {
    let total = SIGNATURE_LEN.saturating_add(body.len());
    if total > MAX_FRAME_LEN {
        return Err(Error::TooLong { got: total });
    }
    let signature = identity.sign(&signing_input(domain, body));
    let mut frame = Vec::with_capacity(total);
    frame.extend_from_slice(&signature);
    frame.extend_from_slice(body);
    Ok(frame)
}

/// Split a frame into its signature and its body, without verifying anything.
///
/// The body has to be decoded before the signature can be checked, because the key
/// that signed it is named inside the body. The caller decodes, then calls
/// [`check_signature`] with the same body slice it decoded — never with a
/// re-serialization of the decoded value.
///
/// # Errors
/// Fails if the frame is shorter than a signature or longer than [`MAX_FRAME_LEN`].
pub fn split(frame: &[u8]) -> Result<(&[u8; SIGNATURE_LEN], &[u8]), Error> {
    if frame.len() > MAX_FRAME_LEN {
        return Err(Error::TooLong { got: frame.len() });
    }
    let (signature, body) = frame
        .split_at_checked(SIGNATURE_LEN)
        .ok_or(Error::TooShort { got: frame.len() })?;
    let signature: &[u8; SIGNATURE_LEN] = signature
        .try_into()
        .map_err(|_| Error::TooShort { got: frame.len() })?;
    Ok((signature, body))
}

/// Check a signature over `body` under `domain`, made by `public_key`.
///
/// # Errors
/// Fails if the key is unusable or the signature does not match.
pub fn check_signature(
    domain: &[u8; DOMAIN_LEN],
    body: &[u8],
    signature: &[u8; SIGNATURE_LEN],
    public_key: &[u8; 32],
) -> Result<(), Error> {
    let signed = signing_input(domain, body);
    if crate::identity::verify(public_key, &signed, signature)? {
        Ok(())
    } else {
        Err(Error::BadSignature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_domain_is_the_same_length() {
        assert_eq!(DOMAIN_HEARTBEAT.len(), DOMAIN_LEN);
    }

    #[test]
    fn seal_then_split_gives_the_body_back() {
        let identity = Identity::from_seed(&[1_u8; 32]);
        let frame = seal(DOMAIN_HEARTBEAT, b"hello", &identity).expect("small body");
        let (signature, body) = split(&frame).expect("well-formed frame");
        assert_eq!(body, b"hello");
        check_signature(DOMAIN_HEARTBEAT, body, signature, &identity.public_key())
            .expect("signature is ours");
    }

    #[test]
    fn a_flipped_body_byte_fails() {
        let identity = Identity::from_seed(&[2_u8; 32]);
        let mut frame = seal(DOMAIN_HEARTBEAT, b"hello", &identity).expect("small body");
        let last = frame.len() - 1;
        frame[last] ^= 0x01;
        let (signature, body) = split(&frame).expect("still well-formed");
        assert_eq!(
            check_signature(DOMAIN_HEARTBEAT, body, signature, &identity.public_key()),
            Err(Error::BadSignature)
        );
    }

    #[test]
    fn a_signature_from_another_domain_fails() {
        let identity = Identity::from_seed(&[3_u8; 32]);
        let other_domain = b"333.v1.otherdom_";
        assert_eq!(other_domain.len(), DOMAIN_LEN);
        let frame = seal(other_domain, b"hello", &identity).expect("small body");
        let (signature, body) = split(&frame).expect("well-formed frame");
        assert_eq!(
            check_signature(DOMAIN_HEARTBEAT, body, signature, &identity.public_key()),
            Err(Error::BadSignature)
        );
    }

    #[test]
    fn frames_shorter_than_a_signature_are_refused() {
        assert_eq!(split(&[0_u8; 10]), Err(Error::TooShort { got: 10 }));
        assert!(split(&[0_u8; SIGNATURE_LEN]).is_ok());
    }

    #[test]
    fn frames_over_the_limit_are_refused_at_both_ends() {
        let identity = Identity::from_seed(&[4_u8; 32]);
        let body = vec![0_u8; MAX_FRAME_LEN];
        assert!(matches!(
            seal(DOMAIN_HEARTBEAT, &body, &identity),
            Err(Error::TooLong { .. })
        ));
        let frame = vec![0_u8; MAX_FRAME_LEN + 1];
        assert_eq!(
            split(&frame),
            Err(Error::TooLong {
                got: MAX_FRAME_LEN + 1
            })
        );
    }
}
