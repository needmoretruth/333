//! 333 transport.
//!
//! Layers, each ignorant of the one above it:
//!
//! - [`peer`] reads a peer's address and says which way it can be reached.
//! - [`invite`] is that address written the way one person hands it to another.
//! - [`frame`] moves length-prefixed byte strings over anything that reads and writes.
//! - [`session`] runs one heartbeat exchange over such a stream. It knows nothing
//!   about either transport, which is why the whole exchange is tested in memory.
//! - [`direct`] supplies a stream by opening a socket. This is the ordinary case.
//! - [`tor`] supplies one through an onion service, for a node that needs its address
//!   not to be seen. Present unless the `tor` feature is turned off.
//!
//! TOR IS NOT THE DEFAULT, and the address decides which transport is used rather
//! than a setting: see [`peer`]. Arti is bundled so that any node can reach a hidden
//! peer, but a node that is not hiding never starts it, and never pays the seconds to
//! minutes a bootstrap costs.
//!
//! Nothing here decides what a heartbeat means. That belongs to `n333-core`, and this
//! crate depends on it in one direction only.

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

pub mod direct;
pub mod frame;
pub mod invite;
pub mod peer;
pub mod session;
#[cfg(feature = "tor")]
pub mod tor;

pub use invite::Invite;
pub use peer::{DEFAULT_PORT, PeerAddress};
pub use session::{Exchange, initiate, respond};
