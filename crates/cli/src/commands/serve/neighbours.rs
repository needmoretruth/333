//! Greeting the nodes this one finds on its own network, kept apart from the vigil
//! because it is the one listener with its own deadline and its own idea of done.

use std::sync::Arc;
use std::time::Duration;

use crate::commands::hours;
use crate::dial::Dialer;
use crate::node::Node;

/// How long a node on the same network gets to answer before this one moves on.
///
/// Not the patience this node has for a peer: that one is a ceiling for reaching
/// across the world through Tor, and spending it on a virtual interface with nothing
/// behind it would leave a neighbour that is actually there waiting behind it.
const NEARBY_PATIENCE: Duration = Duration::from_secs(10);

/// Knock on every node that turns up on this network, as it turns up.
///
/// Waiting for the next epoch would be correct and useless: a person who starts a
/// second node in the same house and watches nothing happen for five hours has been
/// told, correctly, that nothing is happening.
pub(super) async fn greet_the_neighbours(
    node: Arc<Node>,
    dialer: Dialer,
    nearby: n333_net::Nearby,
) -> anyhow::Result<()> {
    let mut greeted = std::collections::BTreeSet::new();
    while let Some(neighbour) = nearby.found().await {
        // A machine with eight interfaces is announced eight times over. It is one
        // neighbour, and it is worth one knock.
        if !greeted.insert(neighbour.label) {
            continue;
        }
        for address in neighbour.addresses {
            let address = address.to_string();
            aloud!("nearby   one of us at {address}");
            // On a deadline of its own, and a short one. A machine on the same network
            // answers in milliseconds; the several minutes this node is willing to
            // wait on a peer across the world would be spent here on an address that
            // belongs to a virtual interface with nothing behind it.
            let answered = tokio::time::timeout(
                NEARBY_PATIENCE,
                hours::trade_at_once(&node, &dialer, &address),
            )
            .await;
            if answered != Ok(true) {
                continue;
            }
            // Kept only now that it is known to answer, so that the addresses this
            // node carries into its hours are the ones somebody is behind.
            node.found(address.clone()).await;
            // Nothing here takes the file for anybody. A node is given it because a
            // person asked for it and two keys signed, and finding a neighbour is not
            // asking.
            if node.subject().await.is_none() {
                aloud!(
                    "         `333 join 333:{address}` asks them for the file. Nothing\n\
                     \x20        here does it for you."
                );
            }
            break;
        }
    }
    Ok(())
}
