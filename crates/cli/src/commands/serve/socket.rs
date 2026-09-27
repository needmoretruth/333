//! The socket listener, kept apart from the onion one so that each way in can be read
//! and changed without the other's feature flags in the way.

use std::io::ErrorKind;
use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Context as _;
use n333_net::direct;

use crate::node::Node;
use crate::words::Arg;

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
        direct::Error::Io { cause } if cause.kind() == ErrorKind::AddrInUse => Some(words!(
            "serve-socket-port-taken",
            port = Arg::exact(bind.port())
        )),
        direct::Error::Io { cause } if cause.kind() == ErrorKind::AddrNotAvailable => Some(words!(
            "serve-socket-not-here",
            ip = bind.ip().to_string(),
            port = Arg::exact(bind.port())
        )),
        _ => None,
    };
    let failed = anyhow::Error::new(failed)
        .context(words!("serve-socket-listening-on", bind = bind.to_string()));
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
        let (stream, from) = listener
            .accept()
            .await
            .with_context(|| words!("serve-socket-accepting"))?;
        // A peer's address is not a name and is not recorded; it is shown so that the
        // operator of this node can see who is reaching it right now, and counted so
        // that one caller cannot be everybody at the door.
        spawn_exchange(stream, &node, &door, Caller::At(from));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_english_every_moved_line_says_what_it_said_before() {
        let pairs = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            [
                (
                    words!("serve-socket-listening-on", bind = "0.0.0.0:3333"),
                    "listening on 0.0.0.0:3333",
                ),
                (words!("serve-socket-accepting"), "accepting a peer"),
                // Both were one line nine columns wider than a terminal leaves once the
                // step is indented under `failed`, and are broken earlier now.
                (
                    words!("serve-socket-port-taken", port = Arg::exact(3333)),
                    "Something on this machine already listens on port 3333: another\n\
                     node, or another program. Stop it, or pass --bind with another port.",
                ),
                (
                    words!(
                        "serve-socket-not-here",
                        ip = "192.0.2.7",
                        port = Arg::exact(3333)
                    ),
                    "192.0.2.7 is not an address of this machine. --bind 0.0.0.0:3333\n\
                     listens on all of them.",
                ),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

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
