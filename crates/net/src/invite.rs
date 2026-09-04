//! An invitation: the shortest thing one node can hand another so it knows where to
//! look.
//!
//! FROZEN. The prefix and the canonical form below are the format.
//!
//! ```text
//! 333:node.example:3333
//! 333:abcdefghij…xyz.onion:333
//! ```
//!
//! AN INVITATION NAMES A PLACE, NOT A PERSON. It carries no signature and no claim
//! about who is there. Anyone can mint one for any address, exactly as anyone can
//! write down a street address. That is not a weakness that a signature would fix: a
//! signature is only meaningful against a key the reader already holds, and a code
//! carrying both the signature and the key it was made with is a self-signed
//! assertion anybody can mint. Including one would claim a check the reader cannot
//! perform. Whether the node at the far end is worth anything is settled by talking
//! to it, where it has to prove it holds a key.
//!
//! ONE PEER, ONE STRING. `node.example` and `node.example:3333` name the same peer,
//! so exactly one of them is an invitation and the other is refused with the correct
//! form attached. Without that rule the same peer appears under two byte strings, and
//! everything that counts peers, deduplicates them or signs a list of them has to
//! decide which one it meant — usually silently, and differently on each side. This
//! is the same rule the protocol already applies to public keys, where a
//! non-canonical encoding is refused rather than normalised.
//!
//! There is no checksum. The payload is a hostname or an address a person can read,
//! so a mangled one is visible as mangled; an onion address carries a checksum of its
//! own inside the format Tor already defines. A second checksum over the top would be
//! a second thing that can disagree with the first, and it would cost a dependency
//! that the smallest edition would have to carry to read a string.

use std::fmt;
use std::str::FromStr;

use crate::peer::{AddressError, PeerAddress};

/// What every invitation starts with. FROZEN.
///
/// A tag rather than bare address text so that a code pasted into a chat window is
/// recognisable as one, and so a reader can refuse a string that was never meant to
/// be an invitation instead of trying to dial it.
pub const PREFIX: &str = "333:";

/// The longest invitation this will read.
///
/// A hostname is at most 253 characters, and the longest thing here is an onion
/// address at 62 plus a port. The limit exists so that reading an invitation out of
/// something a stranger sent cannot be made expensive.
pub const MAX_LEN: usize = 300;

/// Why a string is not an invitation.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum InviteError {
    /// It does not start with the tag.
    #[error("an invitation starts with `{PREFIX}`")]
    NotAnInvitation,
    /// Longer than [`MAX_LEN`].
    #[error("an invitation is at most {MAX_LEN} characters; this one is {0}")]
    TooLong(usize),
    /// The address inside it could not be read.
    #[error("{0}")]
    Address(#[from] AddressError),
    /// The address is readable but written a second way.
    ///
    /// Carries the one spelling that is an invitation, so the refusal tells the
    /// reader what to use rather than only that they were wrong.
    #[error("one of us is one place, spelled one way; the invitation is `{canonical}`")]
    NotCanonical {
        /// The invitation this address should have been written as.
        canonical: String,
    },
}

/// An invitation, already checked.
///
/// Holding one means the string was well formed and canonical. It does not mean
/// anybody is there.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Invite {
    /// Where to look.
    address: PeerAddress,
}

impl Invite {
    /// Mint an invitation to a peer at this address.
    #[must_use]
    pub const fn to(address: PeerAddress) -> Self {
        Self { address }
    }

    /// Where the invitation points.
    #[must_use]
    pub const fn address(&self) -> &PeerAddress {
        &self.address
    }

    /// Take the address out.
    #[must_use]
    pub fn into_address(self) -> PeerAddress {
        self.address
    }
}

impl FromStr for Invite {
    type Err = InviteError;

    /// Read an invitation, refusing anything that is not exactly one.
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let text = text.trim();
        if text.len() > MAX_LEN {
            return Err(InviteError::TooLong(text.len()));
        }
        let rest = text
            .strip_prefix(PREFIX)
            .ok_or(InviteError::NotAnInvitation)?;
        let address: PeerAddress = rest.parse()?;
        // The canonicality rule, and the reason this type exists at all. Comparing
        // against the re-rendered address is what makes one peer exactly one string.
        if address.to_string() != rest {
            return Err(InviteError::NotCanonical {
                canonical: Self::to(address).to_string(),
            });
        }
        Ok(Self::to(address))
    }
}

impl fmt::Display for Invite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{PREFIX}{}", self.address)
    }
}

/// Read either an invitation or a plain address typed by hand.
///
/// The canonical form is required of invitations and not of typing. The rule exists
/// so that a peer appears as one byte string everywhere it is stored, counted or
/// signed for — refusing `node.example` from someone at a keyboard would buy nothing
/// and cost them a puzzle.
///
/// # Errors
/// Fails if a string tagged as an invitation is not a canonical one, or if what
/// remains is not an address at all.
pub fn address_or_invite(text: &str) -> Result<PeerAddress, InviteError> {
    if text.trim().starts_with(PREFIX) {
        return text.parse::<Invite>().map(Invite::into_address);
    }
    Ok(text.trim().parse::<PeerAddress>()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::peer::{DEFAULT_PORT, ONION_PORT};

    const ONION: &str = "qprbghv6box5b5hx5ud7hcuzorklavqfdug3xacgnrm3c2bvjhnei3id.onion";

    fn read(text: &str) -> Result<Invite, InviteError> {
        text.parse()
    }

    #[test]
    fn the_frozen_parts_are_the_agreed_ones() {
        assert_eq!(PREFIX, "333:");
        assert_eq!(MAX_LEN, 300);
    }

    #[test]
    fn an_invitation_is_the_tag_and_a_canonical_address() {
        let invite = read("333:node.example:3333").expect("reads");
        assert_eq!(invite.address().host(), "node.example");
        assert_eq!(invite.address().port(), DEFAULT_PORT);
        assert_eq!(invite.to_string(), "333:node.example:3333");
    }

    #[test]
    fn an_onion_invitation_reads_the_same_way() {
        let invite = read(&format!("333:{ONION}:333")).expect("reads");
        assert!(invite.address().needs_tor());
        assert_eq!(invite.address().port(), ONION_PORT);
    }

    #[test]
    fn minting_and_reading_are_inverses() {
        for text in [
            "333:node.example:3333",
            "333:1.2.3.4:9",
            "333:[::1]:3333",
            "333:[2001:db8::1]:4444",
        ] {
            let invite = read(text).expect("reads");
            assert_eq!(invite.to_string(), text);
            assert_eq!(read(&invite.to_string()), Ok(invite));
        }
    }

    #[test]
    fn the_second_spelling_of_an_address_is_refused_with_the_first_attached() {
        // The whole point of the rule. Both of these name the same peer; only one of
        // them is an invitation, and the refusal says which.
        assert_eq!(
            read("333:node.example"),
            Err(InviteError::NotCanonical {
                canonical: "333:node.example:3333".into()
            })
        );
        assert_eq!(
            read(&format!("333:{ONION}")),
            Err(InviteError::NotCanonical {
                canonical: format!("333:{ONION}:333")
            })
        );
        // An unbracketed IPv6 literal renders bracketed, so it is the other spelling.
        assert_eq!(
            read("333:2001:db8::1"),
            Err(InviteError::NotCanonical {
                canonical: "333:[2001:db8::1]:3333".into()
            })
        );
    }

    #[test]
    fn what_the_refusal_names_is_itself_an_invitation() {
        // A refusal that hands back something also refused would be a loop for
        // whoever is trying to fix it.
        for wrong in ["333:node.example", "333:2001:db8::1", "333:[::1]"] {
            let Err(InviteError::NotCanonical { canonical }) = read(wrong) else {
                panic!("{wrong} should be refused as non-canonical");
            };
            assert!(read(&canonical).is_ok(), "{canonical} must itself read");
        }
    }

    #[test]
    fn case_is_folded_before_canonicality_is_judged() {
        // Addresses are case-insensitive and render lowercase, so a capitalised one
        // is the other spelling rather than a different peer.
        assert_eq!(
            read("333:Node.Example:3333"),
            Err(InviteError::NotCanonical {
                canonical: "333:node.example:3333".into()
            })
        );
    }

    #[test]
    fn a_string_that_is_not_an_invitation_is_refused_as_one() {
        assert_eq!(read("node.example:3333"), Err(InviteError::NotAnInvitation));
        assert_eq!(read("http://node.example"), Err(InviteError::NotAnInvitation));
        assert_eq!(read(""), Err(InviteError::NotAnInvitation));
        assert_eq!(read("333:"), Err(AddressError::Empty.into()));
        assert_eq!(
            read("333:node.example:0"),
            Err(AddressError::BadPort("0".into()).into())
        );
    }

    #[test]
    fn an_overlong_string_is_refused_on_its_length_alone() {
        let long = format!("333:{}:3333", "a".repeat(MAX_LEN));
        assert_eq!(read(&long), Err(InviteError::TooLong(long.len())));
    }

    #[test]
    fn surrounding_whitespace_survives_a_paste() {
        assert!(read("  333:node.example:3333\n").is_ok());
    }

    #[test]
    fn typing_an_address_is_allowed_where_writing_an_invitation_is_not() {
        // Strictness is for what gets stored and signed, not for a person at a
        // keyboard. Both of these dial; only one of them is an invitation.
        assert_eq!(
            address_or_invite("node.example").expect("dials").to_string(),
            "node.example:3333"
        );
        assert_eq!(
            address_or_invite("333:node.example:3333")
                .expect("dials")
                .to_string(),
            "node.example:3333"
        );
        // But a string that claims to be an invitation is held to the rule.
        assert_eq!(
            address_or_invite("333:node.example"),
            Err(InviteError::NotCanonical {
                canonical: "333:node.example:3333".into()
            })
        );
    }
}
