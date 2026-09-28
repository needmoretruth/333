//! Where the others are, and how this node came to know it.
//!
//! Three things arrive as addresses: statements nodes signed about themselves, which
//! go into the directory; addresses somebody typed, which are kept until whoever
//! answers there has signed one of their own; and addresses announced on this
//! network, which are knocked on while the node runs and never trusted further than
//! that. Every one of them is also written into [`sources`](super::sources), so the
//! person running this node can see which came from where.

use std::collections::BTreeSet;

use anyhow::Context as _;
use n333_core::whereabouts;
use n333_core::{Epoch, NodeId};

use super::Node;
use super::sources::{self, Counts, Heard, Learned, Mine, Sighting, Source};

impl Node {
    /// Keep a node's statement about where it is, if it is newer than what is held,
    /// and note where it came from either way.
    ///
    /// A statement under this node's own key that it did not make is not kept and is
    /// not passed on: filed, it would take the place of this node's own entry and
    /// every peer would be handed the other copy's address as this node's. It is said
    /// out loud instead, and written down for `status`.
    ///
    /// WHY THIS DOES NOT STOP THE NODE. A statement is proof that somebody holding the
    /// key signed it once, not that they are running now: anybody can replay an old
    /// one. If seeing one stopped a node, whoever holds a copy of the key could switch
    /// the other off at will by handing it that statement, and the two copies could
    /// each switch the other off in turn. Only the person who runs them knows which
    /// one is meant to live.
    ///
    /// # Errors
    /// Fails if the frame is not a statement of where somebody is, or the file cannot
    /// be written.
    pub(crate) async fn note_address(
        &self,
        frame: &[u8],
        from: &Source,
        now: Epoch,
    ) -> anyhow::Result<bool> {
        let signed = whereabouts::open(frame).with_context(|| words!("node-addresses-reading"))?;
        self.note_signed(signed, frame, from, now).await
    }

    /// [`Self::note_address`], for a statement already opened.
    ///
    /// Opening is checking a signature, and a trade hands over every address a peer
    /// holds every round: checking each one twice is half of what a trade costs.
    ///
    /// # Errors
    /// Fails if the file cannot be written.
    pub(crate) async fn note_signed(
        &self,
        signed: whereabouts::Signed,
        frame: &[u8],
        from: &Source,
        now: Epoch,
    ) -> anyhow::Result<bool> {
        let said = &signed.whereabouts;
        let heard = Heard {
            from: from.clone(),
            epoch: now.0,
        };
        let mut state = self.state.lock().await;
        if said.node == self.identity.public_key() {
            if state.sources.is_mine(&said.address, said.epoch()) != Mine::No {
                return Ok(false);
            }
            let sighting = Sighting {
                address: said.address.clone(),
                said_in: said.epoch,
                heard,
            };
            let fresh = state.sources.sighted(sighting.clone());
            drop(state);
            if fresh {
                say_there_is_another_copy(&sighting, &self.identity.node_id().to_string());
                self.write_down_sources(now).await?;
            }
            return Ok(false);
        }
        let name = signed.node.to_string();
        state.sources.heard(&said.address, Some(name), heard);
        if !state.directory.note(signed, frame.to_vec()) {
            return Ok(false);
        }
        state
            .whereabouts
            .append(frame)
            .with_context(|| words!("node-addresses-keeping"))?;
        Ok(true)
    }

    /// Keep a statement this node has just signed about where it is.
    ///
    /// Written down as this node's own before it goes anywhere, so that when it comes
    /// back from a peer — and it will, every round — it is recognised as this node's
    /// and not taken for another copy's.
    ///
    /// # Errors
    /// Fails if a file cannot be written.
    pub(crate) async fn note_own_address(&self, frame: &[u8], now: Epoch) -> anyhow::Result<()> {
        let signed =
            whereabouts::open(frame).with_context(|| words!("node-addresses-reading-own"))?;
        {
            let mut state = self.state.lock().await;
            state
                .sources
                .said(&signed.whereabouts.address, signed.whereabouts.epoch());
            if state.directory.note(signed, frame.to_vec()) {
                state
                    .whereabouts
                    .append(frame)
                    .with_context(|| words!("node-addresses-keeping-own"))?;
            }
        }
        self.write_down_sources(now).await
    }

    /// Where a node last said it could be found.
    pub(crate) async fn address_of(&self, node: &[u8; 32]) -> Option<String> {
        self.state
            .lock()
            .await
            .directory
            .address_of(node)
            .map(ToOwned::to_owned)
    }

    /// Everywhere this node could knock: what nodes signed, what somebody typed, and
    /// what it overheard.
    ///
    /// The three are not distinguished here on purpose. An address is somewhere to
    /// knock; whoever answers proves who they are by holding a key, and an address
    /// that came from a broadcast on this network is worth exactly as much and no
    /// more than one that came from an invitation.
    ///
    /// A typed address stops being knocked on once whoever answered there has signed
    /// where they are: from then on their own word is the better one, and a person
    /// who moved house should not be looked for at the old one for the rest of the
    /// window. Except when what answered there was this node's own name somewhere
    /// this node is not. That is another copy, and knocking is how each of the two
    /// hands the other the statement that shows it.
    pub(crate) async fn where_others_are(&self) -> Vec<String> {
        let me = self.identity.public_key();
        let mut everywhere = BTreeSet::new();
        {
            let state = self.state.lock().await;
            let mine = state.directory.address_of(&me);
            let my_name = self.identity.node_id().to_string();
            let signed: BTreeSet<String> = state
                .directory
                .entries()
                .map(|(key, _)| NodeId::from_public_key(key).to_string())
                .filter(|name| *name != my_name)
                .collect();
            everywhere.extend(
                state
                    .directory
                    .entries()
                    .filter(|(key, _)| **key != me)
                    .map(|(_, address)| address.to_owned()),
            );
            everywhere.extend(
                state
                    .sources
                    .given_by_hand()
                    .filter(|(address, learned)| {
                        !learned
                            .name
                            .as_ref()
                            .is_some_and(|name| signed.contains(name))
                            && Some(*address) != mine
                    })
                    .map(|(address, _)| address.to_owned()),
            );
        }
        everywhere.extend(self.found.lock().await.iter().cloned());
        everywhere.into_iter().collect()
    }

    /// Keep an address overheard on this network, and say whether it is new.
    ///
    /// The cap is what stops a machine on the same network from filling this node's
    /// memory by announcing a new address every second. Past it, the ones already
    /// here are kept: a node that has heard of 64 neighbours has enough to be going
    /// on with, and the ones it already trades with tell it about the rest.
    pub(crate) async fn found(&self, address: String) -> bool {
        const ENOUGH_NEIGHBOURS: usize = 64;
        let mut found = self.found.lock().await;
        if found.len() >= ENOUGH_NEIGHBOURS && !found.contains(&address) {
            return false;
        }
        let heard = Heard {
            from: Source::ThisNetwork,
            epoch: Epoch::now().0,
        };
        self.state.lock().await.sources.heard(&address, None, heard);
        found.insert(address)
    }

    /// Keep an address somebody typed, to be knocked on from now on.
    ///
    /// # Errors
    /// Fails if the record cannot be written.
    pub(crate) async fn given_by_hand(
        &self,
        address: &str,
        name: Option<NodeId>,
    ) -> anyhow::Result<()> {
        let now = Epoch::now();
        let heard = Heard {
            from: Source::ByHand,
            epoch: now.0,
        };
        let name = name.map(|name| name.to_string());
        self.state.lock().await.sources.heard(address, name, heard);
        self.write_down_sources(now).await
    }

    /// Put a name to an address, because whoever answered there holds that key.
    pub(crate) async fn answered_at(&self, address: &str, name: NodeId) {
        let mut state = self.state.lock().await;
        state.sources.answered(address, name.to_string());
    }

    /// Write down where things came from.
    ///
    /// # Errors
    /// Fails if the file cannot be written.
    pub(crate) async fn write_down_sources(&self, now: Epoch) -> anyhow::Result<()> {
        let mut state = self.state.lock().await;
        sources::save(&self.home, &mut state.sources, now)
    }

    /// Every address this node holds: everywhere it would knock, and what this network
    /// announced within the window.
    ///
    /// The second is here because a command run beside the vigil has not heard the
    /// announcements the vigil heard. The record has, and an address this node was
    /// told of is an address it holds, whether or not it will knock there again.
    pub(crate) async fn held(&self) -> Vec<String> {
        let mut held: BTreeSet<String> = self.where_others_are().await.into_iter().collect();
        let state = self.state.lock().await;
        held.extend(state.sources.heard_on_this_network().map(ToOwned::to_owned));
        held.into_iter().collect()
    }

    /// Every address this node holds, with where it came from if that was noted.
    ///
    /// The name is the one that signed for it, or answered at it; `None` for an
    /// address nobody has answered at yet.
    pub(crate) async fn sources(&self) -> Vec<(Option<String>, String, Option<Learned>)> {
        let held = self.held().await;
        let state = self.state.lock().await;
        let names: std::collections::BTreeMap<&str, String> = state
            .directory
            .entries()
            .map(|(key, address)| (address, NodeId::from_public_key(key).to_string()))
            .collect();
        held.into_iter()
            .map(|address| {
                let learned = state.sources.of(&address).cloned();
                let name = names
                    .get(address.as_str())
                    .cloned()
                    .or_else(|| learned.as_ref().and_then(|l| l.name.clone()));
                (name, address, learned)
            })
            .collect()
    }

    /// How many of the addresses this node holds came each way.
    pub(crate) async fn known(&self) -> Counts {
        let held = self.held().await;
        let state = self.state.lock().await;
        Counts::of(&state.sources, held.iter().map(String::as_str))
    }

    /// Every statement under this node's key, within the window, that it did not make.
    pub(crate) async fn copies(&self) -> Vec<Sighting> {
        self.state.lock().await.sources.sightings().to_vec()
    }
}

/// Say, as loudly as a line can, that this node's name is being used somewhere else.
fn say_there_is_another_copy(sighting: &Sighting, me: &str) {
    aloud_in!(
        "node-addresses-another-copy",
        address = &sighting.address,
        said_in = sighting.said_in,
        from = sighting.arrived(me)
    );
}

/// What opening the record of where addresses came from found, when it is worth a line.
pub(crate) fn say_what_the_sources_held(opened: &super::Opened) {
    if opened.sources == sources::Loaded::Unreadable {
        aloud_in!("node-addresses-unread");
    }
    if opened.copies != 0 {
        aloud_in!("node-addresses-copies", copies = opened.copies);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::Keeping;
    use n333_core::whereabouts::Whereabouts;

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let pairs = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            [
                (
                    words!("node-addresses-reading"),
                    "reading an address".to_owned(),
                ),
                (
                    words!("node-addresses-keeping"),
                    "keeping an address".to_owned(),
                ),
                (
                    words!("node-addresses-reading-own"),
                    "reading this node's address".to_owned(),
                ),
                (
                    words!("node-addresses-keeping-own"),
                    "keeping this node's address".to_owned(),
                ),
                (
                    words!(
                        "node-addresses-another-copy",
                        address = "there.example:3333",
                        said_in = 89_612_u64,
                        from = "from 333ab"
                    ),
                    format!(
                        "another  copy of this node's name is out there. A statement signed with this\n\
                         \x20        node's key, which this node never made, says it is at\n\
                         \x20        {}, in epoch {}.\n\
                         \x20        It arrived {from}. Either this directory was copied and the copy was\n\
                         \x20        started, or somebody else has the key. Two nodes on one name\n\
                         \x20        contradict each other in every epoch either is asked about. Stop\n\
                         \x20        one of them; `333 pack` is how a node moves. This one keeps\n\
                         \x20        running until you decide which.",
                        "there.example:3333",
                        89_612,
                        from = "from 333ab"
                    ),
                ),
                (
                    words!("node-addresses-unread"),
                    "unread   the note of where each address came from could not be read, so a\n\
                     \x20        new one begins. Nothing this node decides reads it."
                        .to_owned(),
                ),
                (
                    words!("node-addresses-copies", copies = 1_usize),
                    "another  a statement signed with this node's key, which it did not make,\n\
                     \x20        reached it in the window. Another copy of this name has been\n\
                     \x20        running. `333 status` says where it said it was."
                        .to_owned(),
                ),
                (
                    words!("node-addresses-copies", copies = 3_usize),
                    format!(
                        "another  {statements} signed with this node's key, which it did not make,\n\
                         \x20        reached it in the window. Another copy of this name has been\n\
                         \x20        running. `333 status` says where it said it was.",
                        statements = "3 statements"
                    ),
                ),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

    /// A directory for one test, gone when the test is.
    struct Scratch(std::path::PathBuf);

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn scratch(name: &str) -> Scratch {
        let dir =
            std::env::temp_dir().join(format!("n333-addresses-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("creates dir");
        Scratch(dir)
    }

    fn open(home: &std::path::Path) -> Node {
        let mistrust = fs_mistrust::Mistrust::new_dangerously_trust_everyone();
        Node::open(&mistrust, home, Keeping::TheWindow)
            .expect("opens")
            .0
    }

    /// This node, and a copy of its directory taken before either said anything.
    fn a_node_and_its_copy(name: &str) -> (Node, Node, [Scratch; 2]) {
        let (here, there) = (
            scratch(&format!("{name}-here")),
            scratch(&format!("{name}-there")),
        );
        let node = open(&here.0);
        std::fs::copy(here.0.join("identity.key"), there.0.join("identity.key")).expect("copies");
        let copy = open(&there.0);
        (node, copy, [here, there])
    }

    fn from_a_peer() -> Source {
        Source::Peer {
            name: "333ab".into(),
        }
    }

    #[tokio::test]
    async fn a_statement_under_this_key_that_this_node_did_not_make_is_seen_and_not_kept() {
        let (node, copy, _dirs) = a_node_and_its_copy("seen");
        let now = Epoch::now();
        let mine = Whereabouts::of(
            node.identity(),
            "here.example:3333".into(),
            Epoch(now.0 + 1),
        )
        .seal(node.identity())
        .expect("seals");
        node.note_own_address(&mine, now).await.expect("keeps");
        let theirs = Whereabouts::of(
            copy.identity(),
            "there.example:3333".into(),
            Epoch(now.0 + 2),
        )
        .seal(copy.identity())
        .expect("seals");

        let heard = node
            .hear(std::slice::from_ref(&theirs), now, &from_a_peer())
            .await
            .expect("hears");
        assert_eq!(heard.addresses, 0, "it is not taken as news");
        let copies = node.copies().await;
        assert_eq!(copies.len(), 1);
        assert_eq!(copies[0].address, "there.example:3333");
        assert_eq!(copies[0].heard.from, from_a_peer());
        assert_eq!(
            node.address_of(&node.identity().public_key())
                .await
                .as_deref(),
            Some("here.example:3333"),
            "and it does not take this node's place in its own directory"
        );
        let passed_on = node.tidings(now).await.expect("gathers").frames;
        assert!(!passed_on.contains(&theirs), "nor is it handed to anybody");

        // Written down at once, so that `status` in another terminal can say it.
        assert_eq!(open(&node.home).copies().await.len(), 1);
    }

    #[tokio::test]
    async fn this_nodes_own_statement_coming_back_is_not_a_copy() {
        // It comes back every round, from every peer that was handed it.
        let (node, _, _dirs) = a_node_and_its_copy("echo");
        let now = Epoch::now();
        let mine = Whereabouts::of(
            node.identity(),
            "here.example:3333".into(),
            Epoch(now.0 + 1),
        )
        .seal(node.identity())
        .expect("seals");
        node.note_own_address(&mine, now).await.expect("keeps");
        node.hear(&[mine], now, &from_a_peer())
            .await
            .expect("hears");
        assert!(node.copies().await.is_empty());
        let reopened = open(&node.home);
        let again = Whereabouts::of(
            node.identity(),
            "here.example:3333".into(),
            Epoch(now.0 + 1),
        )
        .seal(node.identity())
        .expect("seals");
        reopened
            .hear(&[again], now, &from_a_peer())
            .await
            .expect("hears");
        assert!(reopened.copies().await.is_empty(), "nor after a restart");
    }

    #[tokio::test]
    async fn where_this_node_said_it_is_is_not_counted_among_the_others() {
        // The opening line said "where 2 of us said to look" of a node that knew
        // one other, while the screen and status said 1.
        let (node, _, _dirs) = a_node_and_its_copy("counted");
        let now = Epoch::now();
        let mine = Whereabouts::of(node.identity(), "here.example:3333".into(), now)
            .seal(node.identity())
            .expect("seals");
        node.note_own_address(&mine, now).await.expect("keeps");
        let someone = n333_core::Identity::from_seed(&[9; 32]);
        let theirs = Whereabouts::of(&someone, "there.example:3333".into(), now)
            .seal(&someone)
            .expect("seals");
        node.hear(&[theirs], now, &from_a_peer())
            .await
            .expect("hears");
        let mistrust = fs_mistrust::Mistrust::new_dangerously_trust_everyone();
        let (_, opened) = Node::open(&mistrust, &node.home, Keeping::TheWindow).expect("opens");
        assert_eq!(opened.addresses, 1);
    }

    #[tokio::test]
    async fn a_copy_given_the_other_copys_address_keeps_knocking_there() {
        // The only way either of two copies ever finds out about the other is by one
        // of them handing the other its statement.
        let (node, _, _dirs) = a_node_and_its_copy("knocks");
        let now = Epoch::now();
        let mine = Whereabouts::of(node.identity(), "here.example:3333".into(), now)
            .seal(node.identity())
            .expect("seals");
        node.note_own_address(&mine, now).await.expect("keeps");
        node.given_by_hand("there.example:3333", Some(node.identity().node_id()))
            .await
            .expect("keeps");
        assert_eq!(node.where_others_are().await, vec!["there.example:3333"]);
    }

    #[tokio::test]
    async fn what_this_network_announced_is_still_counted_by_a_command_run_beside_the_vigil() {
        let (node, _, _dirs) = a_node_and_its_copy("nearby");
        assert!(node.found("192.0.2.5:3333".into()).await);
        node.write_down_sources(Epoch::now()).await.expect("writes");
        let beside = open(&node.home);
        assert!(beside.where_others_are().await.is_empty(), "not knocked on");
        assert_eq!(beside.known().await.this_network, 1, "and still held");
    }

    #[tokio::test]
    async fn a_typed_address_is_knocked_on_until_its_owner_says_where_they_are() {
        let (node, _, _dirs) = a_node_and_its_copy("typed");
        let someone = n333_core::Identity::from_seed(&[9; 32]);
        node.given_by_hand("old.example:3333", None)
            .await
            .expect("keeps");
        assert_eq!(node.where_others_are().await, vec!["old.example:3333"]);

        node.answered_at("old.example:3333", someone.node_id())
            .await;
        let moved = Whereabouts::of(&someone, "new.example:3333".into(), Epoch::now())
            .seal(&someone)
            .expect("seals");
        node.hear(&[moved], Epoch::now(), &from_a_peer())
            .await
            .expect("hears");
        assert_eq!(
            node.where_others_are().await,
            vec!["new.example:3333"],
            "their own word replaces what was typed"
        );
    }
}
