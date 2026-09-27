//! `333 status --json` — what this node observed, for a program to read.
//!
//! THE FIELDS. Every name below is fixed for as long as `format` is 1; a field is
//! only ever added, and a change to what one means is a new `format`.
//!
//! - `format` — 1.
//! - `epoch` — the epoch this was read in.
//! - `attendance` — this node's own record over the window: `window_epochs`,
//!   `given_in` and `counted_from` (null for a node nobody has handed the file),
//!   `recorded` (epochs of the window the record says anything about), `counted`,
//!   `present`, `absent`, `per_mille` (null when nothing was asked) and `qualifies`.
//! - `answering` — how many of us this node holds a signed word from, this epoch or
//!   the last. `roll` — how many are on its roll.
//! - `admissions` — one entry per member of the roll: `given_in`, `counted_from`.
//!   Admission epochs are signed and passed to everybody; names are left out anyway.
//! - `signals` — this epoch's: `observed`, `spoken`, `silent`, and `said`, a list of
//!   `signal` and `count` for every signal anybody said.
//! - `diversity` — counts over the addresses this node holds: `ipv4_24` and
//!   `ipv6_48`, how many different /24 and /48 networks they are in; `hostnames` and
//!   `onion`, which are counted and cannot be placed in any network from here.
//! - `provenance` — how many of those addresses were first heard of each way:
//!   `by_hand`, `this_network`, `meeting_point`, `from_peers`, `peers` (how many
//!   different nodes), `unrecorded`.
//! - `other_copies` — statements under this node's key it did not make, in the window.
//! - `unseen` — whether it holds the file, has kept a record for a few epochs among
//!   others, and nobody has signed anything about it: what `serve` says as `unseen`.
//!
//! WHAT IS NEVER IN IT. No address, onion address or port, of this node or anybody
//! else. The output of a command is the kind of thing people paste into a chat to ask
//! what it means, and a count of networks answers every question a count can answer.

use std::collections::BTreeSet;
use std::net::IpAddr;

use n333_core::presence::{self, WINDOW_EPOCHS};
use n333_core::signal::Tally;
use n333_core::{Epoch, enrollment};
use n333_net::PeerAddress;
use serde::Serialize;

use crate::node::Node;
use crate::node::sources::Counts;

/// Which shape of output this is.
pub(super) const FORMAT: u32 = 1;

/// Everything, in the order the fields are documented.
#[derive(Serialize)]
struct Observed {
    format: u32,
    epoch: u64,
    attendance: Attendance,
    answering: usize,
    roll: usize,
    admissions: Vec<Admission>,
    signals: Signals,
    diversity: Diversity,
    provenance: Counts,
    other_copies: usize,
    unseen: bool,
}

/// This node's own record over the window.
#[derive(Serialize, Default)]
struct Attendance {
    window_epochs: u64,
    given_in: Option<u64>,
    counted_from: Option<u64>,
    recorded: u64,
    counted: u64,
    present: u64,
    absent: u64,
    per_mille: Option<u64>,
    qualifies: bool,
}

/// One member of the roll, by when.
#[derive(Serialize)]
struct Admission {
    given_in: u64,
    counted_from: u64,
}

/// What everybody said this epoch.
#[derive(Serialize)]
struct Signals {
    observed: u64,
    spoken: u64,
    silent: u64,
    said: Vec<Said>,
}

/// One signal and how many said it.
#[derive(Serialize)]
struct Said {
    signal: u16,
    count: u64,
}

/// How many networks the addresses held are spread over.
#[derive(Serialize, Default, PartialEq, Eq, Debug)]
struct Diversity {
    ipv4_24: usize,
    ipv6_48: usize,
    hostnames: usize,
    onion: usize,
}

/// Write what this node observed.
///
/// # Errors
/// Fails if this node's files cannot be read or the output cannot be written.
pub(super) async fn write(
    out: &mut impl std::io::Write,
    node: &Node,
    now: Epoch,
) -> anyhow::Result<()> {
    let answering = node.answering(now).await?;
    let mut everyone: BTreeSet<[u8; 32]> = answering.clone();
    everyone.insert(node.identity().public_key());
    let tally = Tally::of(node.overheard(now).await?.against(everyone.iter()));
    let mut admissions: Vec<Admission> = node
        .members()
        .await
        .iter()
        .map(|member| Admission {
            given_in: member.received_in.0,
            counted_from: member.counts_from().0,
        })
        .collect();
    admissions.sort_by_key(|admission| admission.given_in);

    let observed = Observed {
        format: FORMAT,
        epoch: now.0,
        attendance: attendance(node, now).await?,
        answering: answering.len(),
        roll: node.roll().await.len(),
        admissions,
        signals: Signals {
            observed: tally.observed(),
            spoken: tally.spoken(),
            silent: tally.silent(),
            said: tally
                .distribution()
                .filter(|(_, count)| *count > 0)
                .map(|(signal, count)| Said {
                    signal: signal.index(),
                    count,
                })
                .collect(),
        },
        diversity: spread(&node.held().await),
        provenance: node.known().await,
        other_copies: node.copies().await.len(),
        unseen: crate::commands::unseen_now(node).await,
    };
    serde_json::to_writer_pretty(&mut *out, &observed)?;
    writeln!(out)?;
    Ok(())
}

/// This node's own record, in numbers.
async fn attendance(node: &Node, now: Epoch) -> anyhow::Result<Attendance> {
    let Some(joined) = node.joined_in().await else {
        return Ok(Attendance {
            window_epochs: WINDOW_EPOCHS,
            ..Attendance::default()
        });
    };
    let record = node.own_record().await?;
    let standing = presence::standing_at(now, record.iter().copied());
    let window = presence::window(now);
    Ok(Attendance {
        window_epochs: WINDOW_EPOCHS,
        given_in: Some(joined.0),
        counted_from: Some(enrollment::active_from(joined).0),
        recorded: record
            .iter()
            .filter(|(epoch, _)| window.contains(&epoch.0))
            .count() as u64,
        counted: standing.counted,
        present: standing.present,
        absent: standing.absent(),
        per_mille: standing.per_mille(),
        qualifies: standing.qualifies(),
    })
}

/// How many different networks a set of addresses is in, without keeping any of them.
fn spread(addresses: &[String]) -> Diversity {
    let (mut v4, mut v6) = (BTreeSet::new(), BTreeSet::new());
    let mut diversity = Diversity::default();
    for address in addresses {
        match address.parse::<PeerAddress>() {
            Ok(PeerAddress::Onion { .. }) => diversity.onion += 1,
            Ok(PeerAddress::Direct { host, .. }) => match host.parse::<IpAddr>() {
                Ok(IpAddr::V4(ip)) => {
                    let [a, b, c, _] = ip.octets();
                    v4.insert([a, b, c]);
                }
                Ok(IpAddr::V6(ip)) => {
                    let [a, b, c, ..] = ip.segments();
                    v6.insert([a, b, c]);
                }
                Err(_) => diversity.hostnames += 1,
            },
            // Not an address this client can dial, so not in any network it could reach.
            Err(_) => {}
        }
    }
    diversity.ipv4_24 = v4.len();
    diversity.ipv6_48 = v6.len();
    diversity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn nothing_that_could_be_dialled_is_in_it() {
        let home = std::env::temp_dir().join(format!("n333-json-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&home);
        std::fs::create_dir_all(&home).expect("creates dir");
        let mistrust = fs_mistrust::Mistrust::new_dangerously_trust_everyone();
        let (node, _) =
            Node::open(&mistrust, &home, crate::node::Keeping::TheWindow).expect("opens");
        let someone = n333_core::Identity::from_seed(&[4; 32]);
        let statement = n333_core::whereabouts::Whereabouts::of(
            &someone,
            "198.51.100.77:47113".into(),
            Epoch::now(),
        )
        .seal(&someone)
        .expect("seals");
        let from = crate::node::sources::Source::Peer {
            name: "333ab".into(),
        };
        node.hear(&[statement], Epoch::now(), &from)
            .await
            .expect("hears");
        node.given_by_hand("[2001:db8:77::9]:47114", None)
            .await
            .expect("keeps");
        node.given_by_hand(
            "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz2345.onion:47115",
            None,
        )
        .await
        .expect("keeps");

        let mut out = Vec::new();
        write(&mut out, &node, Epoch::now()).await.expect("writes");
        let text = String::from_utf8(out).expect("text");
        for fragment in [
            "198.51", "2001", "db8", ".onion", "abcdefgh", "4711", "333ab",
        ] {
            assert!(!text.contains(fragment), "{fragment} is in {text}");
        }
        let read: serde_json::Value = serde_json::from_str(&text).expect("json");
        assert_eq!(read["format"], FORMAT);
        assert_eq!(read["diversity"]["ipv4_24"], 1);
        assert_eq!(read["diversity"]["ipv6_48"], 1);
        assert_eq!(read["diversity"]["onion"], 1);
        assert_eq!(read["provenance"]["by_hand"], 2);
        assert_eq!(read["provenance"]["from_peers"], 1);
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn the_format_is_one() {
        assert_eq!(FORMAT, 1);
    }

    #[test]
    fn networks_are_counted_and_the_addresses_are_not_kept() {
        let addresses = [
            "192.0.2.1:3333",
            "192.0.2.200:4444",
            "198.51.100.7:3333",
            "[2001:db8:1::1]:3333",
            "[2001:db8:1:ffff::2]:3333",
            "[2001:db8:2::1]:3333",
            "node.example:3333",
            "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz2345.onion:3333",
        ]
        .map(str::to_owned);
        assert_eq!(
            spread(&addresses),
            Diversity {
                ipv4_24: 2,
                ipv6_48: 2,
                hostnames: 1,
                onion: 1,
            }
        );
    }
}
