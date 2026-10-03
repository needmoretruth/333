//! The two lines every way of saying `status` but the JSON begins with: the name, and
//! the epoch with which epoch of this line it is.
//!
//! The epoch is counted from 1970 so that every node works it out from its own clock
//! and trusts nobody for it, which makes it a large number that says nothing about the
//! line this node is in. Beside it goes how long that line has run, counted from the
//! earliest admission this node holds ([`Node::line_epoch`]). A node that holds none
//! has no line to count in, and says the epoch alone.

use n333_core::Epoch;

use crate::node::Node;

/// The name, then the epoch.
pub(super) async fn name_and_epoch(
    out: &mut impl std::io::Write,
    node: &Node,
    now: Epoch,
) -> std::io::Result<()> {
    let name = node.identity().node_id().to_string();
    writeln!(out, "{}", words!("status-name", name = name))?;
    let epoch = match node.line_epoch(now).await {
        Some(nth) => words!("status-epoch-in-line", epoch = now.0, line = the_line(nth)),
        None => words!("status-epoch", epoch = now.0),
    };
    writeln!(out, "{epoch}")
}

/// Which epoch of this line one is, `nth` counted from one: "this line's 3rd".
pub(super) fn the_line(nth: u64) -> String {
    let kind = crate::words::count::ordinal(nth);
    words!("status-the-line", nth = nth, kind = kind)
}
