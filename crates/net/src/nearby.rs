//! Finding the nodes on the same network as this one, with nobody handing anything over.
//!
//! An invitation is the ordinary way in and it needs a person: somebody who is already
//! keeping the file tells you where to knock. On one local network that is a strange
//! thing to require — two machines in the same house, or the same room, can hear each
//! other say so. This is that, and it is the only way a node ever learns of a peer
//! without being told by another node.
//!
//! WHAT IT ANNOUNCES, EXACTLY. That something here speaks 333, and on which port.
//! Not this node's name: a name is what every statement it signs is stamped with, and
//! putting it on a broadcast would tell whoever is listening on the network — the
//! router, the other machines, whoever runs the place — which machine belongs to which
//! name in the record. What is announced instead is what a port scan of the same
//! network would find anyway.
//!
//! WHAT IS DONE WITH WHAT IT FINDS. Nothing is believed. An address heard this way is
//! somewhere to knock, and nothing else: the node that answers there proves who it is
//! by holding a key, exactly as it would at an address from an invitation. Nothing
//! found here is written down, because nobody signed it — it lives in memory until
//! the peer says where it is in a statement of its own.
//!
//! A NODE THAT IS HIDING DOES NOT DO THIS. An onion address exists so that nobody
//! learns which machine is behind it, and shouting on the local network that this
//! machine speaks 333 hands that back to everyone on it.

use std::net::{IpAddr, Ipv6Addr};

use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};

use crate::peer::PeerAddress;

/// What this network is called on a local network. FROZEN: it is the name two clients
/// have to agree on to hear each other at all, and a client that is never updated
/// again is still listening for exactly this.
///
/// Not `_333`: a service name has to hold at least one letter, and a responder that
/// follows the rule refuses to say a name that does not.
pub const SERVICE: &str = "_n333._tcp.local.";

/// Things that can go wrong looking for nodes nearby.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The responder could not be started, or the network refused it.
    #[error("{0}")]
    Mdns(#[from] mdns_sd::Error),
}

/// Listening for other nodes on this network, and saying this one is here.
pub struct Nearby {
    /// The responder, which owns a thread of its own.
    daemon: ServiceDaemon,
    /// What this node announced itself as, if it announced anything.
    announced: Option<String>,
    /// Everything the responder has heard.
    heard: mdns_sd::Receiver<ServiceEvent>,
}

impl Nearby {
    /// Announce this node at `port`, and listen for the others.
    ///
    /// Both or neither. Listening is not the quiet half of this: browsing asks the
    /// whole network, out loud, whether anybody here speaks 333, which is the same
    /// disclosure as answering it.
    ///
    /// # Errors
    /// Fails if the responder cannot be started or the network refuses it.
    pub fn start(port: u16) -> Result<Self, Error> {
        let daemon = ServiceDaemon::new()?;
        let heard = daemon.browse(SERVICE)?;
        let mut nearby = Self {
            daemon,
            announced: None,
            heard,
        };
        nearby.announce(port)?;
        Ok(nearby)
    }

    /// Say that this node is here, on `port`.
    fn announce(&mut self, port: u16) -> Result<(), Error> {
        // Enough to tell two nodes on one machine apart, and nothing anybody can carry
        // back to a name in the record. The clock is in it because two clients started
        // in the same second on the same port would otherwise claim the same label.
        let label = format!("n333-{port}-{:x}", n333_core::epoch::unix_now_millis());
        let service = ServiceInfo::new(
            SERVICE,
            &label,
            &format!("{label}.local."),
            (),
            port,
            None,
        )?
        // The addresses of this machine, filled in by the responder and kept right as
        // interfaces come and go. A node bound to every interface does not know which
        // of its addresses a stranger can reach, and here it does not have to.
        .enable_addr_auto();
        self.daemon.register(service)?;
        self.announced = Some(format!("{label}.{SERVICE}"));
        Ok(())
    }

    /// Wait for the next node to turn up.
    ///
    /// Ends when the responder stops.
    pub async fn found(&self) -> Option<Neighbour> {
        loop {
            let event = self.heard.recv_async().await.ok()?;
            let ServiceEvent::ServiceResolved(service) = event else {
                continue;
            };
            if self.announced.as_deref() == Some(service.fullname.as_str()) {
                continue;
            }
            let addresses: Vec<PeerAddress> = service
                .addresses
                .iter()
                .filter_map(|scoped| match scoped {
                    mdns_sd::ScopedIp::V4(v4) => Some(IpAddr::V4(*v4.addr())),
                    mdns_sd::ScopedIp::V6(v6) => reachable_without_a_scope(*v6.addr()),
                    // A kind of address this build has never heard of is not a kind
                    // of address it can knock on.
                    _ => None,
                })
                .map(|address| PeerAddress::Direct {
                    host: address.to_string(),
                    port: service.port,
                })
                .collect();
            if !addresses.is_empty() {
                return Some(Neighbour {
                    label: service.fullname.clone(),
                    addresses,
                });
            }
        }
    }
}

/// One node heard of on this network.
#[derive(Debug, Clone)]
pub struct Neighbour {
    /// What it called itself in the announcement.
    ///
    /// Not a name in the sense the rest of this protocol means one — nothing is signed
    /// and anybody can claim it. It is here for one purpose: a machine with eight
    /// interfaces is announced eight times, and this is what says those are one
    /// neighbour rather than eight.
    pub label: String,
    /// Everywhere it said it could be reached, in no particular order.
    pub addresses: Vec<PeerAddress>,
}

/// An address that means the same thing to whoever is handed it.
///
/// A link-local address is only reachable with the interface it was heard on attached
/// to it, and this client's addresses are host and port and nothing else. Dropping
/// them loses nothing: a machine with a link-local address has another one on the same
/// network, and it announces that too.
fn reachable_without_a_scope(address: Ipv6Addr) -> Option<IpAddr> {
    let link_local = address.segments().first().is_some_and(|first| first & 0xffc0 == 0xfe80);
    (!link_local).then_some(IpAddr::V6(address))
}

impl Drop for Nearby {
    fn drop(&mut self) {
        // Told rather than left to expire. A record that outlives the node sends the
        // next person who hears it to a socket nobody is answering.
        if let Some(announced) = &self.announced {
            let _ = self.daemon.unregister(announced);
        }
        let _ = self.daemon.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_name_this_network_answers_to_is_one_a_responder_will_say() {
        // The rule that bit: a service name has to hold at least one letter, so
        // `_333._tcp.local.` — the obvious name, and the one the specification writes
        // — is refused by the responder at registration and the node is never heard.
        let (name, rest) = SERVICE.split_at(SERVICE.find('.').unwrap_or_default());
        assert_eq!(rest, "._tcp.local.");
        let name = name.strip_prefix('_').expect("a service name starts with _");
        assert!((1..=15).contains(&name.len()), "{name} is not 1 to 15 characters");
        assert!(name.chars().any(|c| c.is_ascii_alphabetic()), "{name} has no letter");
        assert!(
            name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'),
            "{name} has something other than letters, digits and hyphens"
        );
    }

    #[test]
    fn an_address_that_only_means_something_on_one_interface_is_not_carried() {
        // A link-local address is reachable only with the interface it was heard on
        // attached to it, and an address in this protocol is a host and a port. Handed
        // on as it is, it would send whoever receives it somewhere else entirely.
        let link_local = "fe80::1".parse().expect("an address");
        assert!(reachable_without_a_scope(link_local).is_none());

        for anywhere in ["2001:db8::1", "::1"] {
            let address = anywhere.parse().expect("an address");
            assert!(
                reachable_without_a_scope(address).is_some(),
                "{anywhere} means the same thing wherever it is read"
            );
        }
    }
}
