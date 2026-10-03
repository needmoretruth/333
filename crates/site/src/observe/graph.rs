//! The nodes and lines the observation draws, built only from records that verify.
//!
//! A LINE IS A SIGNED RELATION, NOT A CONNECTION. `handover` is an admission both sides
//! signed; `answered` and `silent` are a verifier's signed attestation about a prover.
//! Nothing here says two machines are connected now, and nothing unsigned can add a
//! node or a line: a record that does not open is passed over.
//!
//! The founder is whoever handed the file on and was never handed it: a sponsor with no
//! admission of its own. The roll leaves the founder out on purpose (a member who can
//! never leave is a count that can never reach zero); the picture puts it back, marked.

use std::collections::{BTreeMap, BTreeSet};

use n333_core::whereabouts::Directory;
use n333_core::{Epoch, NodeId, Roll, attestation, chain, heartbeat, utterance};
use serde::Serialize;

use super::given::Given;
use super::node_files::{Held, RECENT_EPOCHS};
use crate::place::host_of;

/// How a node said it can be reached. Never the address, except the site node's own,
/// which is the invitation the site gives out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Reach {
    /// An ordinary address.
    Direct,
    /// An onion address.
    Onion,
}

/// What a line is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Kind {
    /// `from` handed the file to `to`.
    Handover,
    /// `from` asked `to` and published `to`'s signed answer.
    Answered,
    /// `from` asked `to` and swore nothing came back.
    Silent,
}

/// One node in the picture.
#[derive(Debug, Default, PartialEq, Eq, Serialize)]
pub(crate) struct Node {
    /// The node's name.
    pub(crate) id: String,
    /// Handed the file on and never handed it.
    pub(crate) founder: bool,
    /// The node this site runs.
    pub(crate) site: bool,
    /// The epoch it was admitted in.
    pub(crate) admitted: Option<u64>,
    /// Who admitted it.
    pub(crate) sponsor: Option<String>,
    /// The first epoch its record counts.
    pub(crate) counts_from: Option<u64>,
    /// The newest epoch of a heartbeat held from it.
    pub(crate) last_heartbeat: Option<u64>,
    /// The newest epoch of a positive attestation held about it, as the prover.
    pub(crate) last_answered: Option<u64>,
    /// A positive attestation or a heartbeat in this epoch or the one before.
    pub(crate) answered_now: bool,
    /// What it said this epoch.
    pub(crate) signal: Option<u16>,
    /// How it said it can be reached.
    pub(crate) reach: Option<Reach>,
}

/// One line in the picture.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub(crate) struct Edge {
    /// The giver, or the verifier.
    pub(crate) from: String,
    /// The receiver, or the prover.
    pub(crate) to: String,
    /// What it is.
    pub(crate) kind: Kind,
    /// When.
    pub(crate) epoch: u64,
}

/// The picture.
#[derive(Debug, Default)]
pub(crate) struct Graph {
    /// The site node's name, as told, or as its chain says.
    pub(crate) site_node: Option<String>,
    /// As told, or `333:` and the site node's own newest address, if it published one.
    pub(crate) invitation: Option<String>,
    /// Every node, by name.
    pub(crate) nodes: Vec<Node>,
    /// Every line, sorted.
    pub(crate) edges: Vec<Edge>,
}

/// Draw the picture from what the node holds, as of `now`. What the operator was
/// `given` wins over what the files suggest.
pub(crate) fn build(held: &Held, now: Epoch, given: &Given) -> Graph {
    let mut drawing = Drawing {
        now,
        nodes: BTreeMap::new(),
        edges: BTreeSet::new(),
    };
    let site = given
        .site_node
        .clone()
        .or_else(|| site_key(&held.chain).map(|key| NodeId::from_public_key(&key).to_string()));
    drawing.roll(&held.admissions);
    let (directory, _) = Directory::from_frames(&held.whereabouts);
    drawing.addresses(&directory);
    drawing.statements(&held.recent);
    drawing.statements(&held.witnessed);
    if let Some(name) = &site {
        drawing.named(name).site = true;
    }
    let invitation = given.invitation.clone().or_else(|| {
        let name = site.as_deref()?;
        let (_, address) = directory
            .entries()
            .find(|(key, _)| NodeId::from_public_key(key).to_string() == name)?;
        Some(format!("333:{address}"))
    });
    Graph {
        site_node: site,
        invitation,
        nodes: drawing.nodes.into_values().collect(),
        edges: drawing.edges.into_iter().collect(),
    }
}

/// The node's own key: the author of the first entry of its chain that verifies.
fn site_key(chain: &[Vec<u8>]) -> Option<[u8; 32]> {
    chain
        .iter()
        .find_map(|frame| chain::open(frame).ok())
        .map(|signed| signed.entry.author)
}

/// The picture while it is drawn.
struct Drawing {
    /// The epoch it is drawn in.
    now: Epoch,
    /// Nodes by name.
    nodes: BTreeMap<String, Node>,
    /// Lines, each once.
    edges: BTreeSet<Edge>,
}

impl Drawing {
    /// The node behind a key, added if it is new.
    fn node(&mut self, key: &[u8; 32]) -> &mut Node {
        self.named(&NodeId::from_public_key(key).to_string())
    }

    /// The node with this name, added if it is new.
    fn named(&mut self, name: &str) -> &mut Node {
        self.nodes.entry(name.to_owned()).or_insert_with(|| Node {
            id: name.to_owned(),
            ..Node::default()
        })
    }

    /// A line between two keys.
    fn edge(&mut self, from: &[u8; 32], to: &[u8; 32], kind: Kind, epoch: u64) {
        self.edges.insert(Edge {
            from: NodeId::from_public_key(from).to_string(),
            to: NodeId::from_public_key(to).to_string(),
            kind,
            epoch,
        });
    }

    /// Members, their sponsors, the handovers between them, and the founders.
    fn roll(&mut self, admissions: &[Vec<u8>]) {
        let (roll, _) = Roll::from_halves(admissions);
        for member in roll.members() {
            let sponsor = NodeId::from_public_key(&member.sponsor).to_string();
            let node = self.node(&member.key);
            node.admitted = Some(member.received_in.0);
            node.counts_from = Some(member.counts_from().0);
            node.sponsor = Some(sponsor);
            self.edge(
                &member.sponsor,
                &member.key,
                Kind::Handover,
                member.received_in.0,
            );
            if roll.member(&member.sponsor).is_none() {
                self.node(&member.sponsor).founder = true;
            }
        }
    }

    /// How each node said it can be reached.
    fn addresses(&mut self, directory: &Directory) {
        for (key, address) in directory.entries() {
            let onion = host_of(address).ends_with(".onion");
            self.node(key).reach = Some(if onion { Reach::Onion } else { Reach::Direct });
        }
    }

    /// Heartbeats, utterances and attestations, whichever each frame opens as.
    fn statements(&mut self, frames: &[Vec<u8>]) {
        let now = self.now.0;
        // This epoch or the one before.
        let recent = |epoch: u64| epoch.saturating_add(1) >= now;
        for frame in frames {
            if let Ok(beat) = heartbeat::open(frame) {
                let epoch = beat.heartbeat.epoch;
                let node = self.node(&beat.heartbeat.sender);
                node.last_heartbeat = node.last_heartbeat.max(Some(epoch));
                node.answered_now |= recent(epoch);
            } else if let Ok(said) = utterance::open(frame) {
                let node = self.node(&said.utterance.speaker);
                if said.utterance.epoch == now && node.signal.is_none() {
                    node.signal = Some(said.signal.index());
                }
            } else if let Ok(seen) = attestation::open(frame) {
                self.attested(&seen.attestation, seen.is_positive());
            }
        }
    }

    /// One verified attestation: the prover's last answer, and a line if it is recent.
    fn attested(&mut self, attested: &attestation::Attestation, positive: bool) {
        let now = self.now.0;
        // Nothing claiming to be from the future counts.
        if attested.epoch > now {
            return;
        }
        let prover = self.node(&attested.prover);
        if positive {
            prover.last_answered = prover.last_answered.max(Some(attested.epoch));
            // This epoch or the one before.
            prover.answered_now |= attested.epoch.saturating_add(1) >= now;
        }
        self.node(&attested.verifier);
        // Lines only for the last RECENT_EPOCHS epochs.
        if attested.epoch.saturating_add(RECENT_EPOCHS) > now {
            let kind = if positive {
                Kind::Answered
            } else {
                Kind::Silent
            };
            self.edge(&attested.verifier, &attested.prover, kind, attested.epoch);
        }
    }
}

#[cfg(test)]
mod tests {
    use n333_core::Identity;
    use n333_core::subject::DIGEST;
    use n333_core::transfer::{Half, Record};

    use super::*;

    #[test]
    fn a_two_record_roll_draws_the_founder_and_the_handover() {
        let (founder, member) = (Identity::from_seed(&[1; 32]), Identity::from_seed(&[2; 32]));
        let gave = Record::new(&founder, member.public_key(), Epoch(900), DIGEST)
            .seal(Half::Gave, &founder)
            .unwrap();
        let received = Record::new(&member, founder.public_key(), Epoch(900), DIGEST)
            .seal(Half::Received, &member)
            .unwrap();
        let held = Held {
            admissions: vec![gave, received],
            ..Held::default()
        };

        let graph = build(&held, Epoch(905), &Given::default());
        let (founder_id, member_id) = (founder.node_id().to_string(), member.node_id().to_string());
        let first = graph
            .nodes
            .iter()
            .find(|node| node.id == founder_id)
            .unwrap();
        let second = graph
            .nodes
            .iter()
            .find(|node| node.id == member_id)
            .unwrap();
        assert!(first.founder && first.admitted.is_none() && first.sponsor.is_none());
        assert!(!second.founder);
        assert_eq!(
            (second.admitted, second.sponsor.as_deref()),
            (Some(900), Some(founder_id.as_str()))
        );
        assert_eq!(
            graph.edges,
            vec![Edge {
                from: founder_id,
                to: member_id,
                kind: Kind::Handover,
                epoch: 900
            }]
        );
    }

    #[test]
    fn a_positive_attestation_is_the_provers_last_answer_even_past_the_lines() {
        use n333_core::challenge::{Answer, Challenge};
        let (verifier, prover) = (Identity::from_seed(&[3; 32]), Identity::from_seed(&[4; 32]));
        let asked = Challenge::new(&verifier, prover.public_key(), Epoch(890));
        let answer = Answer::to(&asked, &prover, [0; 32], 0)
            .seal(&prover)
            .unwrap();
        let positive = attestation::Attestation::answered(
            &verifier,
            prover.public_key(),
            Epoch(890),
            asked.nonce,
            answer,
        )
        .seal(&verifier)
        .unwrap();
        let held = Held {
            witnessed: vec![positive],
            ..Held::default()
        };

        let graph = build(&held, Epoch(905), &Given::default());
        let id = prover.node_id().to_string();
        let node = graph.nodes.iter().find(|node| node.id == id).unwrap();
        assert_eq!(node.last_answered, Some(890));
        assert!(!node.answered_now);
        assert!(
            graph.edges.is_empty(),
            "lines are kept for three epochs only"
        );
    }
}
