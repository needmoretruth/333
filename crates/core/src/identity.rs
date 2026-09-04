//! Identities.
//!
//! A node's identity is an Ed25519 key pair. Its published name is the SHA-256 hash
//! of the public key, and the only condition the protocol can check is the one the
//! specification names: does that hash, written in hexadecimal, begin with `333`?
//!
//! FROZEN. The derivation below — SHA-256 over the 32 raw public-key bytes, read as
//! lowercase hexadecimal, first three digits — is what every node compares against.
//! Changing the hash, the encoding, or the number of digits renames every identity
//! that exists.
//!
//! What is deliberately absent: any claim that a key was "honestly" mined. A key
//! found in a loop and a key found by a warehouse of machines are the same 32 bytes.
//! There is nothing here to verify, so nothing here claims to verify it.

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use sha2::{Digest, Sha256};
use zeroize::Zeroizing;

/// Number of leading hexadecimal digits that decide a key's class.
pub const PREFIX_DIGITS: usize = 3;

/// The hexadecimal prefix an identity must have.
pub const ELIGIBLE_PREFIX: [u8; PREFIX_DIGITS] = [3, 3, 3];

/// Prefixes that are refused outright.
///
/// The client never offers these as a choice: there is no flag, no prompt and no
/// menu for them. A key that lands on one during a search is discarded without a
/// word, and this constant exists only so that a key arriving from elsewhere can be
/// recognised and refused.
pub const REJECTED_PREFIXES: [[u8; PREFIX_DIGITS]; 2] = [[6, 6, 6], [1, 1, 1]];

/// 2^255 - 19, little-endian: the field modulus of Curve25519.
const FIELD_MODULUS_LE: [u8; 32] = [
    0xed, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f,
];

/// What the protocol makes of a public key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyClass {
    /// The hash begins with `333`. This key may enrol.
    Eligible,
    /// The hash begins with `666` or `111`. This key is refused.
    Rejected,
    /// Any other prefix: not an identity, and nothing is said about it.
    Ineligible,
}

/// The published name of a node: SHA-256 of its public key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId([u8; 32]);

impl NodeId {
    /// Derive the name of the node holding this public key.
    #[must_use]
    pub fn from_public_key(public_key: &[u8; 32]) -> Self {
        Self(Sha256::digest(public_key).into())
    }

    /// The raw hash bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// The first [`PREFIX_DIGITS`] hexadecimal digits, as values 0..=15.
    #[must_use]
    pub fn hex_prefix(&self) -> [u8; PREFIX_DIGITS] {
        let mut out = [0_u8; PREFIX_DIGITS];
        for (i, slot) in out.iter_mut().enumerate() {
            let byte = self.0.get(i / 2).copied().unwrap_or(0);
            *slot = if i % 2 == 0 { byte >> 4 } else { byte & 0x0f };
        }
        out
    }

    /// What the protocol makes of this name.
    #[must_use]
    pub fn class(&self) -> KeyClass {
        let prefix = self.hex_prefix();
        if prefix == ELIGIBLE_PREFIX {
            KeyClass::Eligible
        } else if REJECTED_PREFIXES.contains(&prefix) {
            KeyClass::Rejected
        } else {
            KeyClass::Ineligible
        }
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

/// Reasons a public key arriving from elsewhere is not usable.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PublicKeyError {
    /// The bytes are not a point on the curve, or are otherwise unusable.
    #[error("not a valid Ed25519 public key")]
    NotAPoint,
    /// The bytes decode to a point, but are not that point's canonical encoding.
    #[error("public key is not in canonical encoding")]
    NotCanonical,
}

/// Parse a public key that arrived from another node.
///
/// Two different byte strings can decode to the same curve point, because the
/// y-coordinate is reduced modulo 2^255 - 19 during decompression. Left alone, that
/// would give one node two names. RFC 8032 permits refusing such encodings, and this
/// protocol does: the name is derived from the bytes, so the bytes must be the only
/// ones that mean this key.
pub fn parse_public_key(bytes: &[u8; 32]) -> Result<VerifyingKey, PublicKeyError> {
    if !is_canonical_encoding(bytes) {
        return Err(PublicKeyError::NotCanonical);
    }
    VerifyingKey::from_bytes(bytes).map_err(|_| PublicKeyError::NotAPoint)
}

/// Is `bytes` the canonical encoding of its point — that is, is the y-coordinate
/// already reduced below the field modulus?
fn is_canonical_encoding(bytes: &[u8; 32]) -> bool {
    let mut y = *bytes;
    // The top bit carries the sign of x and is not part of y.
    if let Some(last) = y.last_mut() {
        *last &= 0x7f;
    }
    for (mine, modulus) in y.iter().rev().zip(FIELD_MODULUS_LE.iter().rev()) {
        if mine < modulus {
            return true;
        }
        if mine > modulus {
            return false;
        }
    }
    // Exactly equal to the modulus is the encoding of zero, written the long way.
    false
}

/// Why a signature was not accepted.
///
/// The two cases are kept apart because a caller may want to say which happened,
/// but they are one type on purpose: a function returning `Result<bool, _>` invites
/// `if verify(..).is_ok()`, which reads like English, compiles, and accepts every
/// forgery whose key merely parses.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum VerifyError {
    /// The sender's public key is unusable.
    #[error(transparent)]
    Key(#[from] PublicKeyError),
    /// The key is fine; the signature does not match this message.
    #[error("signature does not match")]
    BadSignature,
}

/// Verify a signature made by `public_key` over `message`.
///
/// Uses strict verification, which refuses low-order public keys and low-order
/// signature components. The permissive check accepts a key for which a single
/// signature validates against almost any message — and every key here arrives
/// over the network from a stranger.
///
/// # Errors
/// Fails if the key is unusable or the signature does not match. A returned `Ok`
/// means verified, with nothing further to inspect.
pub fn verify(
    public_key: &[u8; 32],
    message: &[u8],
    signature: &[u8; 64],
) -> Result<(), VerifyError> {
    let key = parse_public_key(public_key)?;
    key.verify_strict(message, &Signature::from_bytes(signature))
        .map_err(|_| VerifyError::BadSignature)
}

/// This node's key pair.
pub struct Identity {
    signing: SigningKey,
}

impl std::fmt::Debug for Identity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Never print the secret half, not even as a length.
        f.debug_struct("Identity")
            .field("node_id", &self.node_id().to_string())
            .finish_non_exhaustive()
    }
}

impl Identity {
    /// Rebuild an identity from its 32-byte seed.
    #[must_use]
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        Self {
            signing: SigningKey::from_bytes(seed),
        }
    }

    /// The 32-byte seed, wiped from memory when dropped.
    ///
    /// The only caller that should want this is the one writing the key to disk.
    #[must_use]
    pub fn seed(&self) -> Zeroizing<[u8; 32]> {
        Zeroizing::new(self.signing.to_bytes())
    }

    /// The public half, as it travels on the wire.
    #[must_use]
    pub fn public_key(&self) -> [u8; 32] {
        self.signing.verifying_key().to_bytes()
    }

    /// This node's published name.
    #[must_use]
    pub fn node_id(&self) -> NodeId {
        NodeId::from_public_key(&self.public_key())
    }

    /// What the protocol makes of this key.
    #[must_use]
    pub fn class(&self) -> KeyClass {
        self.node_id().class()
    }

    /// Sign a message.
    #[must_use]
    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        self.signing.sign(message).to_bytes()
    }

    /// Generate one key pair from the operating system's randomness.
    #[must_use]
    pub fn generate() -> Self {
        Self {
            signing: SigningKey::generate(&mut rand_core::OsRng),
        }
    }

    /// Search for a key whose name begins with `333`, reporting how many keys it took.
    ///
    /// One core manages roughly twenty thousand key pairs a second and one attempt in
    /// 4,096 succeeds, so this returns in well under a second. It is not a defence and
    /// it is not a cost: it is the shape of the door.
    ///
    /// Keys landing on a refused prefix are dropped here without comment. There is no
    /// way to ask this function for one.
    #[must_use]
    pub fn mine() -> (Self, u64) {
        let mut attempts = 0_u64;
        loop {
            attempts = attempts.saturating_add(1);
            let candidate = Self::generate();
            if candidate.class() == KeyClass::Eligible {
                return (candidate, attempts);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_prefix_reads_the_first_three_digits() {
        let mut hash = [0_u8; 32];
        hash[0] = 0x33;
        hash[1] = 0x3f;
        let id = NodeId(hash);
        assert_eq!(id.hex_prefix(), [3, 3, 3]);
        assert_eq!(id.class(), KeyClass::Eligible);
        assert!(id.to_string().starts_with("333f"));
    }

    #[test]
    fn refused_prefixes_are_recognised() {
        let mut six = [0_u8; 32];
        six[0] = 0x66;
        six[1] = 0x60;
        assert_eq!(NodeId(six).class(), KeyClass::Rejected);

        let mut one = [0_u8; 32];
        one[0] = 0x11;
        one[1] = 0x1a;
        assert_eq!(NodeId(one).class(), KeyClass::Rejected);
    }

    #[test]
    fn everything_else_is_ineligible() {
        let mut other = [0_u8; 32];
        other[0] = 0x33;
        other[1] = 0x40;
        assert_eq!(NodeId(other).class(), KeyClass::Ineligible);
    }

    #[test]
    fn node_id_is_sha256_of_the_public_key() {
        let identity = Identity::from_seed(&[7_u8; 32]);
        let expected: [u8; 32] = Sha256::digest(identity.public_key()).into();
        assert_eq!(identity.node_id().as_bytes(), &expected);
    }

    #[test]
    fn mining_lands_on_an_eligible_key() {
        let (identity, attempts) = Identity::mine();
        assert_eq!(identity.class(), KeyClass::Eligible);
        assert!(identity.node_id().to_string().starts_with("333"));
        assert!(attempts >= 1);
    }

    #[test]
    fn signatures_round_trip() {
        let identity = Identity::from_seed(&[9_u8; 32]);
        let signature = identity.sign(b"333");
        assert_eq!(verify(&identity.public_key(), b"333", &signature), Ok(()));
        assert_eq!(
            verify(&identity.public_key(), b"334", &signature),
            Err(VerifyError::BadSignature)
        );
    }

    #[test]
    fn a_small_order_key_never_verifies() {
        // The identity point, written as y = 1 with the sign bit clear. It is a
        // canonical encoding and it decompresses, so it reaches verification. Paired
        // with R = the same point and s = 0, the permissive check succeeds against
        // EVERY message — which is the forgery strict verification exists to refuse.
        // Replacing verify_strict with verify makes this test fail.
        let mut point = [0_u8; 32];
        point[0] = 1;
        let mut signature = [0_u8; 64];
        signature[0] = 1;
        assert!(
            parse_public_key(&point).is_ok(),
            "the key has to reach verification for this to test anything"
        );
        for message in [b"333".as_slice(), b"a message it never saw".as_slice()] {
            assert_eq!(
                verify(&point, message, &signature),
                Err(VerifyError::BadSignature),
                "a small-order key verified"
            );
        }
    }

    #[test]
    fn a_seed_reproduces_the_same_identity() {
        let seed = [42_u8; 32];
        let first = Identity::from_seed(&seed);
        let second = Identity::from_seed(&first.seed());
        assert_eq!(first.public_key(), second.public_key());
        assert_eq!(*first.seed(), seed);
    }

    #[test]
    fn non_canonical_encodings_are_refused() {
        // y = p is the long way of writing zero; y = p + 1 reduces to 1.
        assert!(!is_canonical_encoding(&FIELD_MODULUS_LE));
        let mut over = FIELD_MODULUS_LE;
        over[0] = 0xee;
        assert!(!is_canonical_encoding(&over));
        assert_eq!(parse_public_key(&over), Err(PublicKeyError::NotCanonical));

        // The sign bit is not part of y, so setting it must not change the verdict.
        let mut signed = FIELD_MODULUS_LE;
        signed[0] = 0xec;
        signed[31] |= 0x80;
        assert!(is_canonical_encoding(&signed));
    }

    #[test]
    fn real_public_keys_are_canonical() {
        for seed in 0_u8..8 {
            let identity = Identity::from_seed(&[seed; 32]);
            assert!(is_canonical_encoding(&identity.public_key()));
            assert!(parse_public_key(&identity.public_key()).is_ok());
        }
    }

    #[test]
    fn debug_does_not_print_the_secret() {
        let identity = Identity::from_seed(&[3_u8; 32]);
        let shown = format!("{identity:?}");
        let seed_hex: String = identity.seed().iter().map(|b| format!("{b:02x}")).collect();
        assert!(!shown.contains(&seed_hex));
        assert!(shown.contains(&identity.node_id().to_string()));
    }
}
