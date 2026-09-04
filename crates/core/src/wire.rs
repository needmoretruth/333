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
/// A heartbeat frame is 139 bytes when it opens an exchange and 171 when it answers
/// one, measured at a 2026 clock. Both grow: the epoch and the timestamp are
/// variable-length integers that each gain a byte roughly once a century — the
/// timestamp in 2109, the epoch in 3298. The rest of this limit is room for the
/// message types that come after.
///
/// Bulk transfers — a node handing over its attendance log — do not travel as one
/// frame and are not bounded by this number; when they arrive they carry their own
/// limit, because a stranger must never be able to make this node buffer more than
/// it agreed to.
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
    crate::identity::verify(public_key, &signed, signature).map_err(|e| match e {
        crate::identity::VerifyError::Key(key) => Error::SenderKey(key),
        crate::identity::VerifyError::BadSignature => Error::BadSignature,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_frozen_values_are_the_agreed_ones() {
        // Recomputed from the constants, these would be tautologies. Written as
        // literals, they are what stands between a one-character edit and a network
        // split into builds that cannot verify each other.
        assert_eq!(DOMAIN_HEARTBEAT, b"333.v1.heartbeat");
        assert_eq!(MAX_FRAME_LEN, 4096);
        assert_eq!(DOMAIN_LEN, 16);
        assert_eq!(SIGNATURE_LEN, 64);
    }

    #[test]
    fn the_signed_bytes_are_the_domain_then_the_body() {
        // Signer and verifier both call this, so testing them against each other
        // proves nothing about the order. This asserts the bytes.
        assert_eq!(
            signing_input(DOMAIN_HEARTBEAT, b"hello"),
            b"333.v1.heartbeathello".to_vec()
        );
    }

    #[test]
    fn a_frame_is_the_signature_then_the_body() {
        let identity = Identity::from_seed(&[1_u8; 32]);
        let frame = seal(DOMAIN_HEARTBEAT, b"hello", &identity).expect("small body");
        assert_eq!(
            &frame[..SIGNATURE_LEN],
            &identity.sign(&signing_input(DOMAIN_HEARTBEAT, b"hello"))
        );
        assert_eq!(&frame[SIGNATURE_LEN..], b"hello");

        // ...and the reader takes them from the same ends. Moving the signature to
        // the tail on both sides would pass a round-trip test; it fails this one.
        let mut hand_built = vec![0_u8; SIGNATURE_LEN];
        hand_built.extend_from_slice(b"body");
        let (signature, body) = split(&hand_built).expect("well-formed");
        assert_eq!(signature, &[0_u8; SIGNATURE_LEN]);
        assert_eq!(body, b"body");
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
    fn the_frame_limit_is_inclusive() {
        // Tested only from the refusing side, an off-by-one here would refuse the
        // largest frame the protocol promises to carry — on both sides at once, so
        // neither could fix it alone.
        let identity = Identity::from_seed(&[4_u8; 32]);
        let exactly = vec![0_u8; MAX_FRAME_LEN - SIGNATURE_LEN];
        let frame = seal(DOMAIN_HEARTBEAT, &exactly, &identity).expect("exactly at the limit");
        assert_eq!(frame.len(), MAX_FRAME_LEN);
        assert!(split(&frame).is_ok());

        let one_more = vec![0_u8; MAX_FRAME_LEN - SIGNATURE_LEN + 1];
        assert!(matches!(
            seal(DOMAIN_HEARTBEAT, &one_more, &identity),
            Err(Error::TooLong { .. })
        ));
        assert_eq!(
            split(&vec![0_u8; MAX_FRAME_LEN + 1]),
            Err(Error::TooLong {
                got: MAX_FRAME_LEN + 1
            })
        );
    }
}
