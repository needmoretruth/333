//! The one place a node with no invitation and no neighbours can look.
//!
//! An invitation needs somebody who is already keeping the file; the local network
//! needs to be the same network. This is what is left: a public rendezvous point in
//! the BitTorrent mainline DHT, where nodes that are willing to be found say so and
//! nodes looking for anyone at all go to ask.
//!
//! IT IS ASKED FOR AND NEVER ASSUMED. Announcing here publishes this node's address to
//! a public table anybody in the world can read, and it cannot be taken back: whoever
//! is watching that table has already written it down. That is not a cost this client
//! decides on somebody's behalf, so nothing here starts unless the person running the
//! node asks for it by name, and a node that is hiding may not ask.
//!
//! IT IS ALSO WHAT MAKES TWO HALVES OF A SPLIT NETWORK ONE AGAIN. Two groups that have
//! never met hold different rolls and neither is wrong; the moment one node of each
//! meets one node of the other, both rolls become the union and stay that way. Nothing
//! has to be reconciled and nobody loses anything by it, which is why it happens by
//! itself. A few of us willing to be found is enough for all of us.
//!
//! WHAT IS PUBLISHED IS AN ADDRESS AND NOTHING ELSE. Not this node's name, not its
//! roll, not a word about who is on it. Whoever is there answers a heartbeat and
//! proves who they are with a key, exactly as at an address from an invitation.

use std::net::SocketAddrV4;

use futures::StreamExt as _;
use mainline::{Dht, Id, async_dht::AsyncDht};

use crate::peer::PeerAddress;

/// What is hashed to find the meeting place. FROZEN.
///
/// The specification says `hash("333")`, which names neither a hash nor what exactly
/// is fed to it, and two clients that read it differently look in two different places
/// and never meet. This is the sentence, hashed with SHA-256, cut to the twenty bytes
/// a DHT key is.
pub const WHERE_WE_MEET: &[u8] = b"333.v1.where.we.meet";

/// The most addresses one look brings back.
///
/// A node needs a way in, not a directory. Past a handful, what it will actually do
/// with them is dial them one after another, and everything after that arrives by
/// gossip from whoever answers first.
const ENOUGH: usize = 111;

/// Things that can go wrong at the meeting place.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The node could not open its own socket for the DHT.
    #[error("{0}")]
    Io(#[from] std::io::Error),
    /// Nobody could be told this node is here.
    #[error("no part of the table would take the announcement: {0}")]
    Unheard(String),
}

/// A node's connection to the public meeting place.
pub struct Rendezvous {
    /// The DHT client, which runs a thread of its own.
    dht: AsyncDht,
    /// The port this node answers on, which is what gets published.
    port: u16,
}

impl Rendezvous {
    /// Join the table, announcing nothing yet.
    ///
    /// # Errors
    /// Fails if the socket the DHT needs cannot be opened.
    pub fn open(port: u16) -> Result<Self, Error> {
        Ok(Self {
            dht: Dht::builder().build()?.as_async(),
            port,
        })
    }

    /// The place all of us look, and the only thing every node here agrees on.
    #[must_use]
    pub fn meeting_place() -> Id {
        let digest = n333_core::subject::digest_of(WHERE_WE_MEET);
        let mut twenty = [0_u8; 20];
        for (slot, byte) in twenty.iter_mut().zip(digest) {
            *slot = byte;
        }
        Id::from(twenty)
    }

    /// Say, publicly, that this node answers at this address.
    ///
    /// # Errors
    /// Fails if no part of the table would take it.
    pub async fn say_we_are_here(&self) -> Result<(), Error> {
        self.dht
            .announce_peer(Self::meeting_place(), Some(self.port))
            .await
            .map(|_| ())
            .map_err(|e| Error::Unheard(e.to_string()))
    }

    /// Everyone the table says is there.
    ///
    /// Claims, not facts: anybody can announce any address, including this one's, and
    /// what comes back is a list of places to knock.
    pub async fn who_else_is_there(&self) -> Vec<PeerAddress> {
        let mut found = Vec::new();
        let mut asking = self.dht.get_peers(Self::meeting_place());
        while let Some(batch) = asking.next().await {
            found.extend(batch.into_iter().map(where_they_said));
            if found.len() >= ENOUGH {
                break;
            }
        }
        found.sort_unstable_by_key(ToString::to_string);
        found.dedup_by_key(|address| address.to_string());
        found
    }
}

/// One address from the table, as this client writes addresses.
fn where_they_said(address: SocketAddrV4) -> PeerAddress {
    PeerAddress::Direct {
        host: address.ip().to_string(),
        port: address.port(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A table of our own, on this machine, that no part of the world can see.
    ///
    /// The real one is a public table anybody can read, and announcing this machine's
    /// address in it to run a test would put it there for good. Two of these bootstrap
    /// off each other and are the whole network for as long as the test runs.
    fn a_table_of_our_own(port: u16, from: Option<u16>) -> Rendezvous {
        let mut builder = Dht::builder();
        builder.server_mode().port(port);
        match from {
            Some(other) => builder.bootstrap(&[format!("127.0.0.1:{other}")]),
            None => builder.no_bootstrap(),
        };
        Rendezvous {
            dht: builder.build().expect("binds").as_async(),
            port,
        }
    }

    #[tokio::test]
    async fn a_node_that_says_it_is_there_is_found_by_one_that_looks() {
        // The whole of what this does: one of us puts an address where anybody can
        // read it, another reads it, and now they know where to knock.
        let first = a_table_of_our_own(36333, None);
        let second = a_table_of_our_own(36334, Some(36333));

        second.say_we_are_here().await.expect("somebody takes it");
        let found = first.who_else_is_there().await;
        assert!(
            found.iter().any(|address| address.port() == 36334),
            "the address that was announced is not in what came back: {found:?}"
        );
    }

    #[test]
    fn the_meeting_place_is_the_same_twenty_bytes_for_everybody() {
        // Frozen: a client that reads this differently looks somewhere else and meets
        // nobody, and there is no way to find out that is what happened.
        let place = Rendezvous::meeting_place();
        assert_eq!(
            place.to_string(),
            "f5b7d7c289211c2b4211a6677227db99bdd6dd8c",
            "the meeting place moved, and every client that does not move with it is alone"
        );
    }
}
