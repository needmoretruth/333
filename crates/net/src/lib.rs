//! 333 transport.
//!
//! Three layers, each ignorant of the one above it:
//!
//! - [`frame`] moves length-prefixed byte strings over anything that reads and writes.
//! - [`session`] runs one heartbeat exchange over such a stream. It knows nothing
//!   about Tor, which is why the whole exchange is tested in memory.
//! - [`tor`] supplies the streams: a bootstrapped client that dials onion addresses,
//!   and a service that publishes one.
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

pub mod frame;
pub mod session;
pub mod tor;

pub use session::{Exchange, initiate, respond};
