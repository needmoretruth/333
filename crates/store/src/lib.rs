//! Where a node keeps what it has to remember.
//!
//! Three shapes and nothing else:
//!
//! - [`log`] is an append-only file of signed frames. A node's own record chain lives
//!   in one of these, kept for ever.
//! - [`once`] is one of those that keeps each record once, for the files a node fills
//!   from what peers pass on — which is mostly what it already holds.
//! - [`window`] holds what other nodes said, one file per epoch, and forgets the
//!   epochs that have fallen out of the 333-epoch window.
//!
//! There is no database. Every record already carries a signature over its own bytes,
//! so damage is caught by a check that has to happen anyway, and a stored record is
//! byte-for-byte the frame that arrived — nothing is re-encoded to be kept, sent, or
//! verified. What a database would add is a format and a dependency that both have to
//! keep working for as long as the network does.
//!
//! This crate knows about `n333-core` and nothing else. It does not know what a frame
//! means, and it never checks a signature: that belongs to whoever reads the bytes.

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

pub mod log;
pub mod once;
pub mod window;

pub use log::{Log, Opened};
pub use once::Once;
pub use window::Window;
