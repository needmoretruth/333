//! 333 protocol core.
//!
//! This crate holds the parts of the protocol that must never disagree between two
//! nodes: how time is divided into epochs, what makes a key an eligible identity,
//! and the exact bytes that get signed. It performs no I/O and knows nothing about
//! Tor, so every rule in it can be tested without a network.
//!
//! Two properties here are frozen once the network is running, and are called out
//! at their definitions: the byte layout of a signed message, and the way an
//! identity is derived from a public key.

// Tests assert by panicking, so the lints that forbid panicking in shipped code are
// off inside them. Nothing else in the workspace gets this exemption.
#![cfg_attr(
    test,
    allow(
        clippy::expect_used,
        clippy::unwrap_used,
        clippy::panic,
        clippy::indexing_slicing
    )
)]

pub mod attestation;
pub mod chain;
pub mod challenge;
pub mod draw;
pub mod enrollment;
pub mod epoch;
pub mod extinction;
pub mod heartbeat;
pub mod identity;
pub mod plea;
pub mod presence;
pub mod ratio;
pub mod roll;
pub mod signal;
pub mod subject;
pub mod tidings;
pub mod transfer;
pub mod utterance;
pub mod whereabouts;
pub mod wire;

pub use attestation::{Attestation, Testimony, judge};
pub use chain::{Entry, Head};
pub use challenge::{Answer, Challenge, RESPONSE_WINDOW_SECONDS};
pub use draw::{VERIFIERS_PER_EPOCH, verifiers_for};
pub use enrollment::{ACTIVATION_EPOCHS, Progress};
pub use epoch::{EPOCH_SECONDS, Epoch};
pub use extinction::{EXTINCTION_YEARS, Remaining, Verdict, Vigil, Watched};
pub use heartbeat::{Heartbeat, PROTOCOL_VERSION};
pub use identity::{Identity, KeyClass, NodeId};
pub use plea::Plea;
pub use presence::{Attendance, Census, Standing, WINDOW_EPOCHS};
pub use ratio::{Fraction, PER_MILLE};
pub use roll::{Member, Roll};
pub use signal::{SIGNAL_COUNT, Signal, Tally};
pub use subject::Subject;
pub use tidings::Tidings;
pub use transfer::Transfer;
pub use utterance::Utterance;
pub use whereabouts::{Directory, Whereabouts};
pub use wire::{Error as WireError, MAX_FRAME_LEN};
