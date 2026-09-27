//! The socket listener, kept apart from the onion one so that each way in can be read
//! and changed without the other's feature flags in the way.

use std::io::ErrorKind;
use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Context as _;
use n333_net::direct;

use crate::node::Node;

use super::door::{Caller, Door, spawn_exchange};

/// Open the socket, or say what stood in the way and the one thing that moves it.
///
/// The two failures a person meets are both about the address they gave, and the
/// operating system's words for them — "address already in use", "cannot assign
/// requested address" — do not say what to change.
pub(super) async fn listen(bind: SocketAddr) -> anyhow::Result<direct::Listener> {
    let failed = match direct::Listener::bind(bind).await {
        Ok(listener) => return Ok(listener),
        Err(failed) => failed,
    };
    let step = match &failed {
        direct::Error::Io { cause } if cause.kind() == ErrorKind::AddrInUse => Some(format!(
            "Something on this machine already listens on port {}: another node, or another\n\
             program. Stop it, or pass --bind with another port.",
            bind.port()
        )),
        direct::Error::Io { cause } if cause.kind() == ErrorKind::AddrNotAvailable => {
            Some(format!(
                "{} is not an address of this machine. --bind 0.0.0.0:{} listens on all of them.",
                bind.ip(),
                bind.port()
            ))
        }
        _ => None,
    };
    let failed = anyhow::Error::new(failed).context(format!("listening on {bind}"));
    Err(match step {
        Some(step) => crate::failed::next_step(failed, step),
        None => failed,
    })
}

/// Answer every peer that opens a socket to this node.
pub(super) async fn answer_direct(
    listener: direct::Listener,
    node: Arc<Node>,
    door: Door,
) -> anyhow::Result<()> {
    loop {
        let (stream, from) = listener.accept().await.context("accepting a peer")?;
        // A peer's address is not a name and is not recorded; it is shown so that the
        // operator of this node can see who is reaching it right now, and counted so
        // that one caller cannot be everybody at the door.
        spawn_exchange(stream, &node, &door, Caller::At(from));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn a_port_already_taken_says_what_to_change() {
        let taken = direct::Listener::bind("127.0.0.1:0".parse().expect("an address"))
            .await
            .expect("binds");
        let bind = taken.address().expect("has an address");
        let Err(refused) = listen(bind).await else {
            panic!("a port held by another socket was bound twice");
        };
        let said = crate::failed::said(&refused);
        assert!(
            said.starts_with(&format!("failed   listening on {bind}: ")),
            "{said}"
        );
        assert!(said.contains("pass --bind with another port."), "{said}");
    }
}
