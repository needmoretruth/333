//! The heartbeat: the one message two nodes exchange when they meet.
//!
//! FROZEN. The field order below **is** the wire format — postcard writes no field
//! names and no header, so swapping two fields is a protocol break that compiles
//! cleanly and shows up only as signatures that will not verify. The signed and
//! unsigned integer types are equally load-bearing: postcard zig-zags signed
//! integers, so changing `u64` to `i64` changes the bytes.
//!
//! Nothing in here is a judgement. A heartbeat says who sent it, which epoch the
//! sender believes it is, and — when it is an answer — which heartbeat it answers.
//! What a node makes of a peer whose epoch differs from its own is that node's own
//! business; this protocol has no authority to appeal to.

use serde::{Deserialize, Serialize};

use crate::epoch::{Epoch, unix_now_millis};
use crate::identity::{Identity, KeyClass, NodeId, parse_public_key};
use crate::wire::{self, DOMAIN_HEARTBEAT};

/// The protocol version this build speaks.
pub const PROTOCOL_VERSION: u16 = 1;

/// A signed statement that a node is present in an epoch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Heartbeat {
    /// Wire protocol version. Bumping it is how a future message shape announces
    /// itself; a node that does not know the version refuses the frame rather than
    /// guessing at its fields.
    pub protocol: u16,
    /// The sender's Ed25519 public key.
    pub sender: [u8; 32],
    /// The epoch the sender believes it is in, by its own wall clock.
    pub epoch: u64,
    /// The sender's wall clock in milliseconds. Reported, never trusted: it exists so
    /// that a node can show a human how far apart two clocks are.
    pub sent_at_ms: u64,
    /// Fresh random bytes. An answer quotes them back, which is what makes a
    /// recorded exchange impossible to replay as a live one.
    pub nonce: [u8; 32],
    /// The nonce this heartbeat answers, if it is an answer.
    pub in_reply_to: Option<[u8; 32]>,
}

impl Heartbeat {
    /// Compose a heartbeat from this node's identity and clock.
    #[must_use]
    pub fn now(identity: &Identity, in_reply_to: Option<[u8; 32]>) -> Self {
        let mut nonce = [0_u8; 32];
        rand_core::RngCore::fill_bytes(&mut rand_core::OsRng, &mut nonce);
        Self {
            protocol: PROTOCOL_VERSION,
            sender: identity.public_key(),
            epoch: Epoch::now().0,
            sent_at_ms: unix_now_millis(),
            nonce,
            in_reply_to,
        }
    }

    /// The epoch this heartbeat claims.
    #[must_use]
    pub const fn epoch(&self) -> Epoch {
        Epoch(self.epoch)
    }

    /// Encode and sign, producing the bytes to put on the wire.
    ///
    /// # Errors
    /// Fails if the encoding is impossible or the frame would exceed the wire limit.
    pub fn seal(&self, identity: &Identity) -> Result<Vec<u8>, wire::Error> {
        let body = postcard::to_stdvec(self).map_err(|e| wire::Error::Decode(e.to_string()))?;
        wire::seal(DOMAIN_HEARTBEAT, &body, identity)
    }
}

/// A heartbeat that arrived, decoded, and carried a signature that checks out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verified {
    /// What the peer said.
    pub heartbeat: Heartbeat,
    /// The name derived from the sender's key.
    pub node_id: NodeId,
    /// What the protocol makes of that name. Carried, not acted on: refusing a
    /// refused prefix is enrolment's job, and showing an ineligible one is the
    /// screen's job.
    pub class: KeyClass,
}

/// Read a frame that arrived from a peer.
///
/// The signature is checked against the body bytes exactly as received. Nothing is
/// re-serialized, so a peer cannot make this node accept one encoding and verify
/// another.
///
/// # Errors
/// Fails if the frame is malformed, the version is unknown, the sender's key is
/// unusable, or the signature does not match.
pub fn open(frame: &[u8]) -> Result<Verified, wire::Error> {
    let (signature, body) = wire::split(frame)?;
    let heartbeat: Heartbeat = wire::decode(body)?;

    if heartbeat.protocol != PROTOCOL_VERSION {
        return Err(wire::Error::Version {
            got: heartbeat.protocol,
            expected: PROTOCOL_VERSION,
        });
    }

    // Parsing the key here rather than inside the signature check keeps the refusal
    // of a non-canonical encoding separate from the refusal of a bad signature: they
    // are different failures and a caller may want to say so.
    parse_public_key(&heartbeat.sender)?;
    wire::check_signature(DOMAIN_HEARTBEAT, body, signature, &heartbeat.sender)?;

    let node_id = NodeId::from_public_key(&heartbeat.sender);
    let class = node_id.class();
    Ok(Verified {
        heartbeat,
        node_id,
        class,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity(seed: u8) -> Identity {
        Identity::from_seed(&[seed; 32])
    }

    #[test]
    fn a_padded_frame_is_refused_even_though_its_signature_verifies() {
        // The attack the refusal exists for: append anything to a valid frame. The
        // signature covers the bytes as received, so it still verifies, and without
        // this check the frame decodes to the same heartbeat. One statement would
        // then have as many valid frames as there are paddings.
        let me = identity(1);
        let mut frame = Heartbeat::now(&me, None).seal(&me).expect("seals");
        assert!(open(&frame).is_ok());

        frame.extend_from_slice(b"junk");
        assert_eq!(open(&frame), Err(wire::Error::TrailingBytes { extra: 4 }));
    }

    #[test]
    fn a_sealed_heartbeat_opens() {
        let me = identity(1);
        let sent = Heartbeat::now(&me, None);
        let frame = sent.seal(&me).expect("seals");
        let got = open(&frame).expect("opens");
        assert_eq!(got.heartbeat, sent);
        assert_eq!(got.node_id, me.node_id());
    }

    #[test]
    fn an_answer_carries_the_nonce_it_answers() {
        let (a, b) = (identity(2), identity(3));
        let question = Heartbeat::now(&a, None);
        let answer = Heartbeat::now(&b, Some(question.nonce));
        let got = open(&answer.seal(&b).expect("seals")).expect("opens");
        assert_eq!(got.heartbeat.in_reply_to, Some(question.nonce));
    }

    #[test]
    fn nonces_differ_between_heartbeats() {
        let me = identity(4);
        let first = Heartbeat::now(&me, None);
        let second = Heartbeat::now(&me, None);
        assert_ne!(first.nonce, second.nonce);
    }

    #[test]
    fn a_frame_signed_by_someone_else_is_refused() {
        let (claimed, actual) = (identity(5), identity(6));
        // A heartbeat naming one key but signed by another: the shape a peer would
        // use to speak in a member's name.
        let mut forged = Heartbeat::now(&actual, None);
        forged.sender = claimed.public_key();
        let frame = forged.seal(&actual).expect("seals");
        assert_eq!(open(&frame), Err(wire::Error::BadSignature));
    }

    #[test]
    fn an_unknown_protocol_version_is_refused_before_anything_else() {
        let me = identity(7);
        let mut future = Heartbeat::now(&me, None);
        future.protocol = PROTOCOL_VERSION + 1;
        let frame = future.seal(&me).expect("seals");
        assert_eq!(
            open(&frame),
            Err(wire::Error::Version {
                got: PROTOCOL_VERSION + 1,
                expected: PROTOCOL_VERSION,
            })
        );
    }

    #[test]
    fn truncated_frames_are_refused() {
        let me = identity(8);
        let frame = Heartbeat::now(&me, None).seal(&me).expect("seals");
        for cut in [0, 1, 32, 63, 64, frame.len() - 1] {
            assert!(open(frame.get(..cut).expect("in range")).is_err());
        }
    }

    /// A heartbeat with every field pinned, so a size or a byte string asserted
    /// against it does not move with the clock.
    fn pinned(in_reply_to: Option<[u8; 32]>) -> Heartbeat {
        Heartbeat {
            protocol: PROTOCOL_VERSION,
            sender: [0xaa; 32],
            epoch: 89_516,
            sent_at_ms: 1_788_000_000_000,
            nonce: [0xbb; 32],
            in_reply_to,
        }
    }

    #[test]
    fn a_heartbeat_frame_is_the_size_the_limit_was_chosen_for() {
        let me = identity(9);
        // At a 2026 clock the epoch is a 3-byte varint and the timestamp a 6-byte
        // one; both gain a byte roughly once a century, so these numbers are the
        // shape of the message and not a permanent promise.
        assert_eq!(pinned(None).seal(&me).expect("seals").len(), 139);
        assert_eq!(pinned(Some([0xcc; 32])).seal(&me).expect("seals").len(), 171);
        assert!(
            Heartbeat::now(&me, None).seal(&me).expect("seals").len() <= 200,
            "a live heartbeat outgrew the size this limit was chosen for"
        );
    }

    #[test]
    fn field_order_is_the_wire_format() {
        // A guard against a silent protocol break: reordering fields, or changing an
        // integer's signedness, changes these bytes while everything still compiles.
        let body = postcard::to_stdvec(&Heartbeat {
            protocol: 1,
            sender: [0xaa; 32],
            epoch: 333,
            sent_at_ms: 1,
            nonce: [0xbb; 32],
            in_reply_to: None,
        })
        .expect("encodes");
        let mut expected = Vec::new();
        expected.extend_from_slice(&[0x01]); // protocol: varint 1
        expected.extend_from_slice(&[0xaa; 32]); // sender
        expected.extend_from_slice(&[0xcd, 0x02]); // epoch: varint 333
        expected.extend_from_slice(&[0x01]); // sent_at_ms: varint 1
        expected.extend_from_slice(&[0xbb; 32]); // nonce
        expected.extend_from_slice(&[0x00]); // in_reply_to: none
        assert_eq!(body, expected);
    }
}
