//! Nodes finding nodes with no meeting point: each is handed one address, and the
//! rounds do the rest.
//!
//! Real sockets on this machine, real rounds, and nothing shortened but the wait
//! between epochs. Within an epoch the nodes take their rounds one at a time in an
//! order shuffled by a fixed seed — a stand-in for clocks a few seconds apart, and
//! the thing that keeps every door here below the three callers it lets in from one
//! address, since every caller in these tests is the same address.

use std::hash::{Hash as _, Hasher as _};
use std::sync::Arc;

use n333_core::{Epoch, NodeId};
use n333_net::{PeerAddress, direct};

use super::door::Door;
use super::socket::answer_direct;
use crate::commands::{Common, hours};
use crate::dial::Dialer;
use crate::node::sources::Source;
use crate::node::{Keeping, Node};
use crate::paths::NodePaths;

/// One node answering on a socket of its own.
struct Answering {
    node: Arc<Node>,
    dialer: Dialer,
    address: PeerAddress,
    listening: tokio::task::JoinHandle<anyhow::Result<()>>,
    home: std::path::PathBuf,
}

impl Drop for Answering {
    fn drop(&mut self) {
        self.listening.abort();
        let _ = std::fs::remove_dir_all(&self.home);
    }
}

impl Answering {
    async fn start(test: &str, n: usize) -> Self {
        let home =
            std::env::temp_dir().join(format!("n333-learning-{test}-{n}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&home);
        std::fs::create_dir_all(&home).expect("creates dir");
        let common = Common {
            bridges: Arc::new(std::sync::Mutex::new(n333_net::bridges::Bridges::none())),
            paths: NodePaths::at(home.clone()),
            timeout: std::time::Duration::from_secs(10),
            keeping: Keeping::TheWindow,
            trust_directory_permissions: true,
        };
        let (node, _) = Node::open(&common.mistrust(), &home, Keeping::TheWindow).expect("opens");
        let node = Arc::new(node);
        let listener = direct::Listener::bind("127.0.0.1:0".parse().expect("an address"))
            .await
            .expect("binds");
        let address = PeerAddress::from(listener.address().expect("bound"));
        let answering = Arc::clone(&node);
        let listening =
            tokio::spawn(async move { answer_direct(listener, answering, Door::new()).await });
        Self {
            node,
            dialer: Dialer::new(common),
            address,
            listening,
            home,
        }
    }

    /// Stop answering, the way a machine that has been switched off stops.
    fn switch_off(&self) {
        self.listening.abort();
    }

    fn name(&self) -> NodeId {
        self.node.identity().node_id()
    }

    async fn knows(&self, other: &Self) -> bool {
        let key = other.node.identity().public_key();
        self.node.address_of(&key).await.is_some()
    }

    /// Where this node's note says it last heard of `other`'s address.
    async fn last_heard_of(&self, other: &Self) -> Option<Source> {
        let at = other.address.to_string();
        self.node
            .sources()
            .await
            .into_iter()
            .find(|(_, address, _)| *address == at)
            .and_then(|(_, _, learned)| learned)
            .map(|learned| learned.last.from)
    }

    async fn first_heard_of(&self, other: &Self) -> Option<Source> {
        let at = other.address.to_string();
        self.node
            .sources()
            .await
            .into_iter()
            .find(|(_, address, _)| *address == at)
            .and_then(|(_, _, learned)| learned)
            .map(|learned| learned.first.from)
    }
}

/// A number from a seed and some context, the same on every run.
fn scatter(seed: u64, what: impl std::hash::Hash) -> u64 {
    let mut hasher = std::hash::DefaultHasher::new();
    (seed, what).hash(&mut hasher);
    hasher.finish()
}

/// One epoch: every node's round, one at a time, in a shuffled order.
async fn an_epoch(nodes: &[Answering], seed: u64, epoch: Epoch) {
    let mut order: Vec<usize> = (0..nodes.len()).collect();
    order.sort_by_key(|n| scatter(seed, (epoch.0, *n)));
    for n in order {
        let Some(one) = nodes.get(n) else { continue };
        hours::one_round(
            &one.node,
            &one.dialer,
            Some(one.address.clone()),
            None,
            epoch,
        )
        .await;
    }
}

/// Does every node know where every other one is?
async fn all_know_all(nodes: &[Answering]) -> bool {
    for one in nodes {
        for other in nodes {
            if one.name() != other.name() && !one.knows(other).await {
                return false;
            }
        }
    }
    true
}

fn from(node: &Answering) -> Source {
    Source::Peer {
        name: node.name().to_string(),
    }
}

#[tokio::test]
async fn a_chain_where_each_knows_only_the_next_ends_with_everybody_trading_with_everybody() {
    const LENGTH: usize = 5;
    let mut nodes = Vec::new();
    for n in 0..LENGTH {
        nodes.push(Answering::start("chain", n).await);
    }
    for pair in nodes.windows(2) {
        if let [this, next] = pair {
            let typed = next.address.to_string();
            this.node.given_by_hand(&typed, None).await.expect("keeps");
        }
    }
    let (head, tail) = (&nodes[0], &nodes[LENGTH - 1]);
    let start = Epoch::now();
    let mut epochs = 0;
    while !all_know_all(&nodes).await {
        assert!(epochs < 10, "still not everybody after {epochs} epochs");
        an_epoch(&nodes, 5, Epoch(start.0 + epochs)).await;
        epochs += 1;
    }
    println!("measured: a chain of {LENGTH} knew each other after {epochs} epochs");
    assert!(epochs <= 3, "{epochs} epochs for a chain of {LENGTH}");

    for never_met in &nodes[2..] {
        assert!(
            matches!(
                head.first_heard_of(never_met).await,
                Some(Source::Peer { .. })
            ),
            "the head was only ever given the second node's address"
        );
    }
    // Everybody between the two ends goes away, and the two ends — never introduced
    // to each other — take one more epoch. What each hears of the other now can only
    // have come from the other.
    for between in &nodes[1..LENGTH - 1] {
        between.switch_off();
    }
    let later = Epoch(start.0 + epochs);
    an_epoch(std::slice::from_ref(head), 5, later).await;
    an_epoch(std::slice::from_ref(tail), 5, later).await;
    assert_eq!(tail.last_heard_of(head).await, Some(from(head)));
    assert_eq!(head.last_heard_of(tail).await, Some(from(tail)));
}

#[tokio::test]
async fn a_week_of_epochs_on_a_sparse_graph_leaves_nobody_unknown() {
    const NODES: usize = 20;
    const WEEK: u64 = 30;
    const LATE: u64 = 24;
    const SEED: u64 = 333;
    let mut nodes = Vec::new();
    for n in 0..NODES {
        nodes.push(Answering::start("week", n).await);
    }
    // Every node is handed one address: somebody who arrived before it, chosen by the
    // seed. The first is handed the last. Nobody is handed more than that.
    for n in 0..NODES {
        let given = if n == 0 {
            NODES - 1
        } else {
            usize::try_from(scatter(SEED, n) % n as u64).expect("small")
        };
        let typed = nodes[given].address.to_string();
        nodes[n]
            .node
            .given_by_hand(&typed, None)
            .await
            .expect("keeps");
    }

    let start = Epoch::now();
    let (mut everybody_by, mut the_late_one_by) = (None, None);
    for epoch in 0..WEEK {
        if epoch == LATE {
            // Somebody new, late in the week, handed one address and known to nobody.
            let late = Answering::start("week", NODES).await;
            let typed = nodes[3].address.to_string();
            late.node.given_by_hand(&typed, None).await.expect("keeps");
            nodes.push(late);
        }
        an_epoch(&nodes, SEED, Epoch(start.0 + epoch)).await;
        let all = all_know_all(&nodes).await;
        if everybody_by.is_none() && all {
            everybody_by = Some(epoch + 1);
        }
        if the_late_one_by.is_none() && epoch >= LATE && all {
            the_late_one_by = Some(epoch + 1 - LATE);
        }
    }
    println!(
        "measured: {NODES} nodes each handed one address all knew each other after \
         {everybody_by:?} epochs; the one who came in epoch {LATE} was known to all \
         after {the_late_one_by:?}"
    );
    assert!(everybody_by.is_some_and(|epochs| epochs <= LATE));
    assert!(
        all_know_all(&nodes).await,
        "and the one who arrived in epoch {LATE} is known to all {NODES} by the end of the week"
    );
}
