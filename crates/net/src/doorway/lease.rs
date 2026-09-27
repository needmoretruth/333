//! Asking over PCP and NAT-PMP, the protocols a router speaks when it does not speak UPnP.
//!
//! Apple's routers speak NAT-PMP, many providers' boxes and OpenWrt speak PCP, and PCP
//! is the successor that answers a NAT-PMP router's clients too. Both are one small
//! UDP datagram to the gateway on port 5351 and one back. The messages are written and
//! read by `crab_nat`, which is the maintained crate that speaks both; this file
//! decides what to ask for, in which order, and what to say about the answer.
//!
//! PCP FIRST. RFC 6886 asks clients to try PCP first, and a router that only speaks
//! NAT-PMP answers a PCP request by saying which version it does speak, which is the
//! cue to fall back. A router that says nothing to PCP is asked over NAT-PMP anyway: an
//! old one may drop what it does not understand rather than answer it.
//!
//! A LEASE, NOT A GIFT. Neither protocol has a permanent mapping. It is asked for two
//! hours, which is what RFC 6886 recommends; whatever the router grants is what is
//! reported; it is asked again at half of what is left, as both RFCs ask; and it is
//! given back when the vigil ends. A node that dies without giving it back leaves a
//! mapping the router drops by itself within two hours, pointed at a port nobody is
//! listening on.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::num::NonZeroU16;
use std::time::{Duration, Instant};

use crab_nat::{
    GatewayAddress, InternetProtocol, PortMapping, PortMappingOptions, PortMappingType,
    TimeoutConfig, natpmp, pcp,
};

use super::{Asked, Lasts, Way, address_the_router_sees};

/// How long each mapping is asked for: the two hours RFC 6886 §3.3 recommends.
///
/// Public so that what is printed about the ask is the number that was asked.
pub const ASK_FOR: Duration = Duration::from_secs(7_200);

/// Less than this left, and there is no asking again in time.
///
/// Half of what is left is when the next ask goes out; below two seconds that is
/// under a second, which is a router being hammered rather than a lease being kept.
const TOO_LITTLE_LEFT: Duration = Duration::from_secs(2);

/// How long to wait for the router, and how often to repeat the question.
///
/// A quarter of a second, doubling, four sends: under four seconds before deciding
/// nobody is there. The router is one hop away and answers in milliseconds if it
/// answers at all; the RFCs' own schedule runs to minutes, which a node starting up
/// cannot spend on a router that is never going to answer.
const PATIENCE: TimeoutConfig = TimeoutConfig {
    initial_timeout: Duration::from_millis(250),
    max_retries: 3,
    max_retry_timeout: Some(Duration::from_secs(2)),
};

/// A mapping the router made for a while, and the means to keep it or give it back.
#[derive(Debug, Clone)]
pub struct Lease {
    /// What the router agreed to.
    mapping: PortMapping,
    /// The address the router says the household is at, if it said.
    outside: Option<IpAddr>,
}

impl Lease {
    /// Which protocol the router agreed over. Renewing and giving back use the same.
    #[must_use]
    pub fn way(&self) -> Way {
        match self.mapping.mapping_type() {
            PortMappingType::NatPmp => Way::NatPmp,
            PortMappingType::Pcp { .. } => Way::Pcp,
        }
    }

    /// The router the mapping is on.
    #[must_use]
    pub fn router(&self) -> IpAddr {
        self.mapping.gateway().into()
    }

    /// The port the world knocks on, which the router chose and may not be the one asked.
    #[must_use]
    pub fn port(&self) -> u16 {
        self.mapping.external_port().get()
    }

    /// The address the router says the household is at, if it said.
    #[must_use]
    pub fn outside(&self) -> Option<IpAddr> {
        self.outside
    }

    /// How long the router granted at the last ask.
    #[must_use]
    pub fn granted(&self) -> Duration {
        Duration::from_secs(u64::from(self.mapping.lifetime()))
    }

    /// How long the router will keep the mapping if it is not asked again.
    #[must_use]
    pub fn left(&self) -> Duration {
        self.mapping
            .expiration()
            .saturating_duration_since(Instant::now())
    }

    /// How long to wait before asking again, or `None` if it is too late to.
    #[must_use]
    pub fn ask_again_in(&self) -> Option<Duration> {
        when_to_ask_again(self.left())
    }

    /// Ask the router for the same mapping again, for as long as it granted last time.
    ///
    /// # Errors
    /// Fails with what the router said, or that it said nothing. The mapping it made
    /// before is still there until it runs out.
    pub async fn renew(&mut self) -> Result<(), String> {
        let way = self.way();
        self.mapping
            .renew()
            .await
            .map_err(|e| format!("{way}: {e}"))?;
        if let PortMappingType::Pcp { external_ip, .. } = self.mapping.mapping_type() {
            self.outside = Some(external_ip);
        }
        Ok(())
    }

    /// Tell the router to forget the mapping now rather than when it runs out.
    ///
    /// # Errors
    /// Fails with what the router said, or that it said nothing. A PCP router is
    /// allowed to refuse to shorten a mapping (RFC 6887 §15), in which case it lasts
    /// until it runs out and then goes by itself.
    pub async fn give_back(self) -> Result<(), String> {
        let way = self.way();
        self.mapping
            .try_drop()
            .await
            .map_err(|(e, _)| format!("{way}: {e}"))
    }
}

/// Half of what is left, or `None` when what is left is too little to ask in.
///
/// Half because RFC 6886 §3.3 says to begin renewing halfway to expiry and RFC 6887
/// §11.2.1 says between one half and five eighths. Measured from what is left rather
/// than from what was granted, so the same rule serves after an ask that failed: the
/// next one goes out at half of what remains, and so on until there is nothing left.
fn when_to_ask_again(left: Duration) -> Option<Duration> {
    (left >= TOO_LITTLE_LEFT).then_some(left / 2)
}

/// Ask the router at `gateway` to send TCP `port` here, over PCP and then NAT-PMP.
pub(super) async fn ask(gateway: Ipv4Addr, port: u16) -> Asked {
    let Some(internal) = NonZeroU16::new(port) else {
        return Asked::Refused("there is no port 0 to send anywhere".to_owned());
    };
    let at = SocketAddr::new(gateway.into(), crab_nat::GATEWAY_PORT);
    let Some(client) = address_the_router_sees(at) else {
        return Asked::Refused("this machine has no address on the router's network".to_owned());
    };
    let router = GatewayAddress::from(gateway);
    let options = PortMappingOptions {
        external_port: Some(internal),
        lifetime_seconds: u32::try_from(ASK_FOR.as_secs()).ok(),
        timeout_config: Some(PATIENCE),
    };
    let request = pcp::BaseMapRequest::new(router, client, InternetProtocol::Tcp, internal);
    let pcp_said = match pcp::port_mapping(request, None, None, options).await {
        Ok(mapping) => {
            let outside = match mapping.mapping_type() {
                PortMappingType::Pcp { external_ip, .. } => Some(external_ip),
                PortMappingType::NatPmp => None,
            };
            return leased(Lease { mapping, outside });
        }
        // The router speaks only NAT-PMP and said so, which is what it is meant to do.
        Err(pcp::Failure::UnsupportedVersion(crab_nat::VersionCode::NatPmp)) => {
            Some("PCP: it speaks only NAT-PMP")
        }
        // Nobody there, or somebody there who drops what it does not understand.
        Err(pcp::Failure::Timeout | pcp::Failure::Socket(_)) => None,
        Err(e) => return Asked::Refused(format!("PCP: {e}")),
    };
    match natpmp::port_mapping(router, InternetProtocol::Tcp, internal, options).await {
        Ok(mapping) => {
            let outside = natpmp::external_address(router, Some(PATIENCE)).await;
            leased(Lease {
                mapping,
                outside: outside.ok().map(IpAddr::V4),
            })
        }
        Err(natpmp::Failure::Timeout | natpmp::Failure::Socket(_)) => match pcp_said {
            Some(said) => Asked::Refused(format!("{said}, and then NAT-PMP was not answered")),
            None => Asked::NobodyAnswered,
        },
        Err(e) => Asked::Refused(format!("NAT-PMP: {e}")),
    }
}

/// What a lease is, said as an answer.
fn leased(lease: Lease) -> Asked {
    Asked::Forwarded {
        outside: lease.outside(),
        port: lease.port(),
        by: lease.way(),
        lasts: Lasts::WhileKept(lease),
    }
}

#[cfg(test)]
mod tests {
    //! A router on this machine's own loopback, answering in the bytes the RFCs lay out.
    //!
    //! Both protocols are spoken to port 5351 of the gateway and nowhere else, so the
    //! fake router listens on 127.0.0.1:5351, which needs no privileges, and the tests
    //! that use it take turns.

    use std::sync::{Arc, Mutex};

    use super::*;

    /// Only one fake router can hold the port at a time.
    static THE_PORT: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

    /// 43333, the port asked for, as it goes on the wire.
    const PORT: [u8; 2] = [0xA9, 0x45];

    /// RFC 6886 §3.3: version 0, opcode 2 (TCP), reserved, internal port, suggested
    /// external port, requested lifetime (7200 seconds).
    const NAT_PMP_MAP_TCP: [u8; 12] = [0, 2, 0, 0, 0xA9, 0x45, 0xA9, 0x45, 0, 0, 0x1C, 0x20];

    /// RFC 6886 §3.4: the same request with the external port and lifetime both zero.
    const NAT_PMP_DELETE_TCP: [u8; 12] = [0, 2, 0, 0, 0xA9, 0x45, 0, 0, 0, 0, 0, 0];

    /// RFC 6886 §3.2: version 0, opcode 0.
    const NAT_PMP_EXTERNAL_ADDRESS: [u8; 2] = [0, 0];

    /// A router listening where both protocols are spoken, keeping what it heard.
    struct FakeRouter {
        heard: Arc<Mutex<Vec<Vec<u8>>>>,
        task: tokio::task::JoinHandle<()>,
    }

    impl FakeRouter {
        async fn answering(answer: fn(&[u8]) -> Option<Vec<u8>>) -> Self {
            let socket = tokio::net::UdpSocket::bind((Ipv4Addr::LOCALHOST, crab_nat::GATEWAY_PORT))
                .await
                .expect("port 5351 on loopback is free");
            let heard = Arc::new(Mutex::new(Vec::new()));
            let keeping = Arc::clone(&heard);
            let task = tokio::spawn(async move {
                let mut buffer = [0; 1100];
                while let Ok((n, from)) = socket.recv_from(&mut buffer).await {
                    let request = buffer.get(..n).expect("within the buffer").to_vec();
                    keeping.lock().expect("not poisoned").push(request.clone());
                    if let Some(reply) = answer(&request) {
                        let _ = socket.send_to(&reply, from).await;
                    }
                }
            });
            Self { heard, task }
        }

        fn heard(&self) -> Vec<Vec<u8>> {
            self.heard.lock().expect("not poisoned").clone()
        }
    }

    impl Drop for FakeRouter {
        fn drop(&mut self) {
            self.task.abort();
        }
    }

    /// An old router that speaks NAT-PMP alone, granting an hour and naming 203.0.113.7.
    fn nat_pmp_only(request: &[u8]) -> Option<Vec<u8>> {
        let epoch = [0, 0, 0, 7];
        let reply = match request {
            // RFC 6887 §9: a NAT-PMP server answers version 2 with its own version and
            // result 1, unsupported version.
            [2, ..] => [&[0, 0x81, 0, 1][..], &epoch].concat(),
            r if r == NAT_PMP_MAP_TCP => [
                &[0, 0x82, 0, 0][..],
                &epoch,
                &PORT,
                &PORT,
                &[0, 0, 0x0E, 0x10],
            ]
            .concat(),
            r if r == NAT_PMP_EXTERNAL_ADDRESS => {
                [&[0, 0x80, 0, 0][..], &epoch, &[203, 0, 113, 7]].concat()
            }
            r if r == NAT_PMP_DELETE_TCP => {
                [&[0, 0x82, 0, 0][..], &epoch, &PORT, &[0, 0], &[0, 0, 0, 0]].concat()
            }
            _ => return None,
        };
        Some(reply)
    }

    /// A PCP router that grants an hour on port 50000 of 203.0.113.9 (RFC 6887 §7.2, §11.1).
    fn pcp(request: &[u8]) -> Option<Vec<u8>> {
        let (Some(lifetime), Some(nonce)) = (request.get(4..8), request.get(24..36)) else {
            return None;
        };
        let granted: &[u8] = if lifetime == [0, 0, 0, 0] {
            &[0, 0, 0, 0]
        } else {
            &[0, 0, 0x0E, 0x10]
        };
        let outside = [&[0; 10][..], &[0xFF, 0xFF, 203, 0, 113, 9]].concat();
        Some(
            [&[2, 0x81, 0, 0][..], granted, &[0, 0, 0, 7], &[0; 12]]
                .into_iter()
                .chain([nonce, &[6, 0, 0, 0], &PORT, &[0xC3, 0x50], &outside])
                .collect::<Vec<_>>()
                .concat(),
        )
    }

    /// A PCP router that answers every request with result 2, not authorised.
    fn pcp_refusing(request: &[u8]) -> Option<Vec<u8>> {
        let mut reply = pcp(request)?;
        *reply.get_mut(3)? = 2;
        Some(reply)
    }

    /// RFC 6887 §7.1 and §11.1 for this node's request, around the nonce it chose.
    fn pcp_map_tcp(lifetime: [u8; 4], nonce: &[u8], suggested_port: [u8; 2]) -> Vec<u8> {
        let loopback = [&[0; 10][..], &[0xFF, 0xFF, 127, 0, 0, 1]].concat();
        let any = [&[0; 10][..], &[0xFF, 0xFF, 0, 0, 0, 0]].concat();
        [
            &[2, 1, 0, 0][..],
            &lifetime,
            &loopback,
            nonce,
            &[6, 0, 0, 0],
            &PORT,
            &suggested_port,
            &any,
        ]
        .concat()
    }

    fn lease(asked: Asked) -> Lease {
        match asked {
            Asked::Forwarded {
                lasts: Lasts::WhileKept(lease),
                ..
            } => lease,
            other => panic!("expected a lease, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn a_nat_pmp_router_is_asked_in_the_bytes_rfc_6886_lays_out() {
        let _turn = THE_PORT.lock().await;
        let router = FakeRouter::answering(nat_pmp_only).await;
        let asked = ask(Ipv4Addr::LOCALHOST, 43_333).await;
        let heard = router.heard();
        assert_eq!(
            heard.get(1..),
            Some(&[NAT_PMP_MAP_TCP.to_vec(), NAT_PMP_EXTERNAL_ADDRESS.to_vec()][..])
        );
        let Asked::Forwarded {
            outside,
            port,
            by,
            lasts: Lasts::WhileKept(lease),
        } = asked
        else {
            panic!("expected a lease, got {asked:?}");
        };
        assert_eq!(
            (outside, port, by),
            (Some(IpAddr::from([203, 0, 113, 7])), 43_333, Way::NatPmp)
        );
        assert_eq!(lease.granted(), Duration::from_secs(3_600));
    }

    #[tokio::test]
    async fn a_nat_pmp_mapping_is_given_back_with_a_lifetime_of_zero() {
        let _turn = THE_PORT.lock().await;
        let router = FakeRouter::answering(nat_pmp_only).await;
        let held = lease(ask(Ipv4Addr::LOCALHOST, 43_333).await);
        assert_eq!(held.give_back().await, Ok(()));
        assert_eq!(router.heard().last(), Some(&NAT_PMP_DELETE_TCP.to_vec()));
    }

    #[tokio::test]
    async fn a_pcp_router_is_asked_in_the_bytes_rfc_6887_lays_out() {
        let _turn = THE_PORT.lock().await;
        let router = FakeRouter::answering(pcp).await;
        let asked = ask(Ipv4Addr::LOCALHOST, 43_333).await;
        let heard = router.heard();
        let request = heard.first().expect("one request");
        let nonce = request.get(24..36).expect("a nonce");
        assert_eq!(request, &pcp_map_tcp([0, 0, 0x1C, 0x20], nonce, PORT));
        let Asked::Forwarded {
            outside, port, by, ..
        } = asked
        else {
            panic!("expected a lease, got {asked:?}");
        };
        // The port the router chose, not the one it was asked for.
        assert_eq!(
            (outside, port, by),
            (Some(IpAddr::from([203, 0, 113, 9])), 50_000, Way::Pcp)
        );
    }

    #[tokio::test]
    async fn a_pcp_mapping_is_renewed_with_the_same_nonce_and_the_port_it_was_given() {
        let _turn = THE_PORT.lock().await;
        let router = FakeRouter::answering(pcp).await;
        let mut held = lease(ask(Ipv4Addr::LOCALHOST, 43_333).await);
        assert_eq!(held.renew().await, Ok(()));
        let heard = router.heard();
        let nonce = heard
            .first()
            .and_then(|first| first.get(24..36))
            .expect("a nonce");
        let mut expected = pcp_map_tcp([0, 0, 0x0E, 0x10], nonce, [0xC3, 0x50]);
        // Asked again on the outside address it was given, as RFC 6887 §11.2.1 says.
        expected.splice(56..60, [203, 0, 113, 9]);
        assert_eq!(heard.get(1), Some(&expected));
    }

    #[tokio::test]
    async fn a_pcp_mapping_is_given_back_with_a_lifetime_of_zero() {
        let _turn = THE_PORT.lock().await;
        let router = FakeRouter::answering(pcp).await;
        let held = lease(ask(Ipv4Addr::LOCALHOST, 43_333).await);
        assert_eq!(held.give_back().await, Ok(()));
        let heard = router.heard();
        let last = heard.last().expect("a request");
        assert_eq!(last.get(4..8), Some(&[0, 0, 0, 0][..]));
    }

    #[tokio::test]
    async fn a_pcp_refusal_is_said_in_the_routers_words_and_nat_pmp_is_not_asked() {
        let _turn = THE_PORT.lock().await;
        let router = FakeRouter::answering(pcp_refusing).await;
        let asked = ask(Ipv4Addr::LOCALHOST, 43_333).await;
        let Asked::Refused(why) = asked else {
            panic!("expected a refusal, got {asked:?}");
        };
        assert!(
            why.starts_with("PCP: ") && why.contains("Not authorized"),
            "{why}"
        );
        assert_eq!(router.heard().len(), 1);
    }

    #[tokio::test]
    async fn a_gateway_nobody_answers_on_is_nobody_answering() {
        let _turn = THE_PORT.lock().await;
        assert!(matches!(
            ask(Ipv4Addr::LOCALHOST, 43_333).await,
            Asked::NobodyAnswered
        ));
    }

    #[test]
    fn the_next_ask_goes_out_at_half_of_what_is_left() {
        assert_eq!(when_to_ask_again(ASK_FOR), Some(Duration::from_secs(3_600)));
        assert_eq!(
            when_to_ask_again(Duration::from_secs(3)),
            Some(Duration::from_millis(1_500))
        );
    }

    #[test]
    fn with_too_little_left_there_is_no_asking_again_in_time() {
        assert_eq!(when_to_ask_again(Duration::from_millis(1_999)), None);
        assert_eq!(when_to_ask_again(Duration::ZERO), None);
    }

    #[test]
    fn two_hours_is_what_is_asked_for() {
        assert_eq!(ASK_FOR, Duration::from_secs(7_200));
    }
}
