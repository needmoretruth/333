//! `333 invite` — the line somebody else needs to join through this node, and how they
//! use it.
//!
//! Read off what this node lately signed about where it can be reached, from the file
//! the running node writes whole, so nothing waits for its directory: asking for an
//! invitation must not keep a node from starting, and needs nothing of one that runs.
//! An address is only ever there once this node found one others can reach, so a node
//! that has none says so instead of printing a line that would look like an invitation
//! and work for nobody.

use std::process::ExitCode;

use n333_core::Epoch;
use n333_net::{Invite, PeerAddress};

use crate::commands::Common;

/// The invitation to an address this node signed, if it is one an invitation can name.
#[must_use]
pub(crate) fn to(address: &str) -> Option<String> {
    let address: PeerAddress = address.parse().ok()?;
    Some(Invite::to(address).to_string())
}

/// Say this node's invitations, or why it has none and what gives it one.
///
/// # Errors
/// Fails if there is no node, or standard output is closed.
pub(crate) fn run(common: &Common) -> anyhow::Result<ExitCode> {
    let root = common.paths.root();
    if !crate::identity_file::holds_a_name(root) {
        return Err(crate::commands::start::no_node(root));
    }
    let (sources, _) = crate::node::sources::load(root, Epoch::now());
    let lines: Vec<String> = sources
        .lately_said()
        .iter()
        .filter_map(|address| to(address))
        .collect();
    let Some(first) = lines.first() else {
        aloud_in!("invite-none");
        aloud_in!("invite-none-next");
        return Ok(ExitCode::FAILURE);
    };
    for line in &lines {
        aloud_in!("invite-line", invitation = line);
    }
    aloud_in!("invite-how", invitation = first);
    Ok(ExitCode::SUCCESS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_address_this_node_signed_becomes_the_invitation_join_reads() {
        assert_eq!(to("192.0.2.7:3333").as_deref(), Some("333:192.0.2.7:3333"));
        assert_eq!(
            to("[2001:db8::7]:3333").as_deref(),
            Some("333:[2001:db8::7]:3333")
        );
        assert_eq!(to("not an address"), None);
    }
}
