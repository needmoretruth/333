//! 333 transport.
//!
//! Layers, each ignorant of the one above it:
//!
//! - [`peer`] reads a peer's address and says which way it can be reached.
//! - [`invite`] is that address written the way one person hands it to another.
//! - [`frame`] moves length-prefixed byte strings over anything that reads and writes.
//! - [`session`] runs one heartbeat exchange over such a stream. It knows nothing
//!   about either transport, which is why the whole exchange is tested in memory.
//! - [`asked`] says which of the three things a peer wants once the heartbeat is done.
//! - [`liveness`] is the round that follows it when one node was drawn to ask another
//!   whether it is awake.
//! - [`handover`] is the second: giving the file to somebody who does not have it,
//!   which is the only way anybody ever becomes a member.
//! - [`gossip`] is the third: trading signed statements, which is how a node learns
//!   where the other members are and therefore how it can ask them anything at all.
//! - [`nearby`] is how two nodes on one network find each other with nobody typing an
//!   address: each announces itself over mDNS and knocks on whoever answers.
//! - [`meeting`] is how two nodes on two networks do it. It is the one fixed address in
//!   a design that wanted none, it is trusted by nothing that reads it, and a node that
//!   has met anybody at all never needs it again.
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

pub mod asked;
pub mod bridges;
pub mod direct;
pub mod frame;
pub mod gossip;
pub mod handover;
pub mod invite;
pub mod liveness;
pub mod meeting;
pub mod nearby;
pub mod peer;
pub mod session;
#[cfg(feature = "tor")]
pub mod tor;

pub use asked::{Asked, take_request};
pub use invite::Invite;
pub use meeting::Meeting;
pub use nearby::Nearby;
pub use peer::{DEFAULT_PORT, PeerAddress};
pub use session::{Exchange, initiate, respond};
