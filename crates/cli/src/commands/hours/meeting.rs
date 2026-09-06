//! The third way a node meets anybody, and the only one that needs somewhere fixed.
//!
//! An invitation needs nobody. The local network needs nobody. Two machines on two
//! networks that have never heard of each other need somewhere both of them already
//! know to look, and there is no arrangement between the two of them that produces
//! one. So: one address, a board of signed statements, and a node that stops needing
//! it the moment it has met somebody.
//!
//! WHAT IS BELIEVED FROM IT: nothing. Every statement read here goes through the same
//! door as a statement a peer handed over — it opens under its own signature or it is
//! dropped — and the meeting point is never told, and never learns, whether any of it
//! was true.
//!
//! WHO LEAVES A STATEMENT AND WHO ONLY READS: a node that answers on a socket has an
//! address the world can already see, and leaving it here costs that node nothing it
//! has not already spent. A node that answers only through Tor is hiding, and leaving
//! an onion address here would hand whoever runs the meeting point the one fact the
//! onion address exists to withhold: which machine is behind it. That node reads and
//! says nothing, because reading gives away nothing about the reader.

use std::sync::Arc;

use n333_net::Meeting;

use crate::node::Node;

/// One node's dealings with one meeting point.
#[derive(Clone)]
pub(crate) struct Board {
    /// Where it is, and the connection to it.
    place: Arc<Meeting>,
    /// Whether this node leaves its own address there, or only reads.
    speaks: bool,
}

impl Board {
    /// Deal with the meeting point at `place`, leaving a statement only if `speaks`.
    pub(crate) fn at(place: &str, speaks: bool) -> Self {
        Self {
            place: Arc::new(Meeting::at(place)),
            speaks,
        }
    }

    /// Where this node is dealing.
    pub(crate) fn place(&self) -> &str {
        self.place.place()
    }

    /// Ask what address this node appears to arrive from.
    ///
    /// Used once, at the start, for a node that is listening on every interface and
    /// therefore cannot say which of its addresses a stranger could reach. The answer
    /// is a suggestion to a person, never a statement this node signs: an address that
    /// arrives at a router is not an address that reaches this machine, and only
    /// whoever set the router up knows whether it does.
    pub(crate) async fn what_address_do_i_arrive_from(&self) -> Option<std::net::IpAddr> {
        let place = Arc::clone(&self.place);
        tokio::task::spawn_blocking(move || place.what_address_do_i_arrive_from())
            .await
            .ok()?
            .ok()
    }

    /// Leave this node's address if it has one to leave, and read everyone else's.
    ///
    /// Nothing here is allowed to stop an epoch. A meeting point that is down, slow,
    /// blocked by somebody's firewall or gone for good costs this node the addresses
    /// it did not already have, and nothing else.
    pub(crate) async fn visit(&self, node: &Node, mine: Option<Vec<u8>>) {
        if let Some(statement) = mine.filter(|_| self.speaks) {
            self.say(node, statement).await;
        }
        self.listen(node).await;
    }

    /// Put this node's own address on the board.
    async fn say(&self, node: &Node, statement: Vec<u8>) {
        let who = node.identity().node_id();
        let place = Arc::clone(&self.place);
        let said = tokio::task::spawn_blocking(move || place.say(&who, &statement)).await;
        match said {
            Ok(Ok(())) => aloud!("meet     left this node's address at {}", self.place()),
            Ok(Err(e)) => aloud!(
                "meet     {} did not take this node's address: {e}",
                self.place()
            ),
            Err(e) => aloud!("meet     could not reach the meeting point: {e}"),
        }
    }

    /// Read every address left on the board and keep the ones that are newer.
    async fn listen(&self, node: &Node) {
        let place = Arc::clone(&self.place);
        let board = match tokio::task::spawn_blocking(move || place.read()).await {
            Ok(Ok(board)) => board,
            Ok(Err(e)) => {
                aloud!("meet     {} could not be read: {e}", self.place());
                return;
            }
            Err(e) => {
                aloud!("meet     could not reach the meeting point: {e}");
                return;
            }
        };
        let mut fresh = 0_usize;
        for statement in &board {
            // Only addresses. The board is one thing and gossip is another, and a
            // meeting point that could hand out admissions would be a meeting point
            // whose operator could decide who this node hears about being admitted.
            if node.note_address(statement).await.unwrap_or(false) {
                fresh = fresh.saturating_add(1);
            }
        }
        say_what_was_there(self.place(), board.len(), fresh);
    }
}

/// Report a visit, without a line about nothing having happened.
///
/// A board that has not changed since the last epoch is the ordinary case for a node
/// that has been running a while, and a line every 333 minutes saying so is a line
/// that teaches a person to stop reading.
fn say_what_was_there(place: &str, saying: usize, fresh: usize) {
    match (saying, fresh) {
        (0, _) => aloud!("meet     nobody is saying where they are at {place}"),
        (_, 0) => {}
        (_, fresh) => aloud!(
            "meet     {fresh} newer {} at {place}",
            if fresh == 1 { "address" } else { "addresses" }
        ),
    }
}
