//! `333 ping` — reach another node and exchange one heartbeat.
//!
//! The address decides how. A plain host opens a socket and the exchange is done in
//! milliseconds; a `.onion` address starts Tor first, which costs seconds to minutes
//! before the first byte moves. Nothing else differs between the two, and neither
//! this file nor anything above it knows which one happened.

use std::time::Duration;

use anyhow::Context as _;
use n333_core::{Identity, NodeId};
use n333_net::{PeerAddress, initiate};

use crate::commands::{Common, describe};
use crate::dial::Dialer;
use crate::identity_file;
use crate::node::sources;

/// Exchange one heartbeat with the node at `address`.
///
/// # Errors
/// Fails if the identity cannot be read, the peer cannot be reached, or the answer
/// does not check out.
pub(crate) async fn run(common: &Common, address: &PeerAddress) -> anyhow::Result<()> {
    let (identity, origin) =
        identity_file::load_or_create(&common.mistrust(), common.paths.root())?;
    aloud_in!("ping-name", name = identity.node_id().to_string());
    crate::named::report(origin, common.paths.root());
    // Written down before the knock, and again with a name once somebody answers: it
    // is an address this node was given, and the vigil goes on knocking there.
    let home = common.paths.root();
    let typed = address.to_string();
    sources::typed(home, &typed, None).with_context(|| words!("ping-writing-the-address"))?;
    let answered = knock(
        &identity,
        &Dialer::new(common.clone()),
        common.timeout,
        address,
    )
    .await?;
    sources::typed(home, &typed, Some(answered))
        .with_context(|| words!("ping-writing-who-answered"))
}

/// Exchange one heartbeat, as a node that is already running.
///
/// Shared with the vigil, which has the key and a dialler of its own already and must
/// not start a second of either.
///
/// The name that answered, for the caller to write down where it keeps addresses.
///
/// # Errors
/// Fails if the peer cannot be reached or the answer does not check out.
pub(crate) async fn knock(
    identity: &Identity,
    dialer: &Dialer,
    timeout: Duration,
    address: &PeerAddress,
) -> anyhow::Result<NodeId> {
    let typed = address.to_string();
    aloud_in!("ping-knocking", address = &typed);

    let mut stream = dialer
        .dial(address)
        .await
        .with_context(|| words!("ping-knocking-on", address = &typed))?;
    let exchange = tokio::time::timeout(timeout, initiate(&mut stream, identity))
        .await
        .map_err(|_| {
            anyhow::anyhow!(words!(
                "ping-unfinished",
                address = &typed,
                seconds = timeout.as_secs()
            ))
        })?
        .with_context(|| words!("ping-exchanging"))?;

    crate::aloud::line(&describe(&exchange));
    Ok(exchange.peer.node_id)
}

#[cfg(test)]
mod tests {
    use crate::words::count::Base;
    use crate::words::layout::{COLUMN, where_the_words_begin};

    const AT: &str = "127.0.0.1:3333";

    fn lines() -> Vec<String> {
        vec![
            words!("ping-name", name = "333abc"),
            words!("ping-knocking", address = AT),
        ]
    }

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let (said, phrases) = crate::words::speaking("en", Base::Ten, || {
            let phrases = [
                (
                    words!("ping-writing-the-address"),
                    "writing down the address",
                ),
                (
                    words!("ping-writing-who-answered"),
                    "writing down who answered",
                ),
                (
                    words!("ping-knocking-on", address = AT),
                    "knocking on 127.0.0.1:3333",
                ),
                (
                    words!("ping-unfinished", address = AT, seconds = 30_u64),
                    "127.0.0.1:3333 took the connection and did not finish the exchange \
                     within 30 s",
                ),
                (words!("ping-exchanging"), "exchanging heartbeats"),
            ];
            (lines(), phrases)
        });
        assert_eq!(said, ["name     333abc", "knocking 127.0.0.1:3333"]);
        for (now, before) in phrases {
            assert_eq!(now, before);
        }
    }

    #[test]
    fn in_korean_every_line_of_ping_begins_its_words_in_the_same_column() {
        for line in crate::words::speaking("ko", Base::Ten, lines) {
            assert!(!line.is_ascii(), "{line:?}");
            assert_eq!(where_the_words_begin(&line), COLUMN, "{line:?}");
        }
    }
}
