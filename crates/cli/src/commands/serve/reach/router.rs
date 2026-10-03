//! Asking the router, keeping what it lent, and giving it back.
//!
//! What is asked and what is answered are both said, because the answer is a change in
//! somebody's house and the words are what they would need to undo it. A mapping made
//! over PCP or NAT-PMP is a lease: it is asked for again before it runs out for as long
//! as the vigil runs, and given back when the vigil ends. A mapping made over UPnP is
//! left as it was made, as it always has been, and is said to be left.

use std::io::Write as _;
use std::net::IpAddr;
use std::time::Duration;

use n333_net::doorway::{self, Asked, Lasts, Lease, Way};
use tokio::sync::oneshot;

use crate::words::Arg;

/// Ask the router to send `port` here, and say what it said.
///
/// Said and not acted on: the knock that follows is what decides, and a router that
/// says yes and is wrong looks exactly like a router that says yes and is right.
pub(super) async fn ask(port: u16) -> Asked {
    let asked = doorway::ask_the_router(port).await;
    match &asked {
        Asked::Forwarded {
            outside,
            port: outside_port,
            by: Way::Upnp,
            lasts,
        } => {
            let (port, on) = (Arg::exact(outside_port), on(*outside));
            match lasts {
                Lasts::For(time) => aloud_in!(
                    "serve-reach-router-opened-upnp-for",
                    port = port,
                    on = on,
                    time = how_long(*time)
                ),
                Lasts::UntilTakenAway | Lasts::WhileKept(_) => {
                    aloud_in!("serve-reach-router-opened-upnp", port = port, on = on);
                }
            }
        }
        Asked::Forwarded {
            lasts: Lasts::WhileKept(lease),
            ..
        } => aloud_in!(
            "serve-reach-router-opened-lease",
            router = lease.router().to_string(),
            way = lease.way().to_string(),
            port = Arg::exact(port),
            asked_for = how_long(doorway::ASK_FOR),
            granted_port = Arg::exact(lease.port()),
            on = on(lease.outside()),
            granted = how_long(lease.granted()),
        ),
        Asked::Forwarded { .. } => {}
        Asked::NobodyAnswered => aloud_in!("serve-reach-router-nobody-answered"),
        Asked::Refused(why) => aloud_in!(
            "serve-reach-router-refused",
            port = Arg::exact(port),
            why = why.to_string()
        ),
    }
    asked
}

/// Keep a lease for as long as the vigil runs, and give it back when `leaving` fires.
///
/// Nothing to keep is nothing to do: a UPnP mapping, or none, waits for the end and
/// returns. The last line is printed rather than said, because by the time it is
/// written the screen has given the terminal back.
pub(super) async fn keep(lease: Option<Lease>, mut leaving: oneshot::Receiver<()>) {
    let Some(mut lease) = lease else {
        return;
    };
    loop {
        let Some(wait) = lease.ask_again_in() else {
            aloud_in!("serve-reach-router-let-go", port = Arg::exact(lease.port()));
            return;
        };
        tokio::select! {
            () = tokio::time::sleep(wait) => renew(&mut lease).await,
            _ = &mut leaving => break,
        }
    }
    let (port, way) = (lease.port(), lease.way());
    let left = lease.left();
    let said = match lease.give_back().await {
        Ok(()) => words!(
            "serve-reach-router-given-back",
            port = Arg::exact(port),
            way = way.to_string()
        ),
        Err(why) => words!(
            "serve-reach-router-not-taken-back",
            port = Arg::exact(port),
            why = why.to_string(),
            time = how_long(left)
        ),
    };
    // Dropped if nobody reads standard output any more, as every other line is.
    let _ = writeln!(std::io::stdout().lock(), "{said}");
}

/// Ask once more, and say so only if something changed or went wrong.
///
/// A renewal that gives the same port on the same address is the lease continuing,
/// which is not news; a line every hour saying so would bury the lines that are.
async fn renew(lease: &mut Lease) {
    let before = (lease.port(), lease.outside());
    match lease.renew().await {
        Ok(()) if (lease.port(), lease.outside()) == before => {}
        Ok(()) => aloud_in!(
            "serve-reach-router-moved",
            port = Arg::exact(lease.port()),
            on = on(lease.outside()),
            before_port = Arg::exact(before.0),
            before_on = on(before.1),
        ),
        Err(why) => aloud_in!(
            "serve-reach-router-not-kept",
            port = Arg::exact(lease.port()),
            why = why.to_string(),
            time = how_long(lease.left()),
        ),
    }
}

/// Where the port is, in the words the router gave or the ones it did not.
fn on(outside: Option<IpAddr>) -> String {
    outside.map_or_else(
        || words!("serve-reach-router-on-its-outside-address"),
        |address| words!("serve-reach-router-on", address = address.to_string()),
    )
}

/// A length of time the way a person says it, never shorter than it is.
///
/// Rounded up, to the minute from a minute on and to the hour where that is whole,
/// because every length said here is a promise about when something will have gone:
/// "within 59 minutes" for a mapping with 59 and a half left would be a small lie.
pub(super) fn how_long(time: Duration) -> String {
    let seconds = time.as_secs() + u64::from(time.subsec_nanos() > 0);
    let minutes = seconds.div_ceil(60);
    match seconds {
        1 => words!("serve-reach-router-one-second"),
        2 => words!("serve-reach-router-two-seconds"),
        s if s < 60 => words!("serve-reach-router-seconds", count = s),
        _ if minutes.is_multiple_of(60) => match minutes / 60 {
            1 => words!("serve-reach-router-one-hour"),
            2 => words!("serve-reach-router-two-hours"),
            hours => words!("serve-reach-router-hours", count = hours),
        },
        _ => match minutes {
            1 => words!("serve-reach-router-one-minute"),
            2 => words!("serve-reach-router-two-minutes"),
            minutes => words!("serve-reach-router-minutes", count = minutes),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let on = || "on 203.0.113.7".to_owned();
        let pairs = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            [
                (
                    words!(
                        "serve-reach-router-opened-upnp",
                        port = Arg::exact(3333),
                        on = on()
                    ),
                    "opened   the router says port 3333 on 203.0.113.7 now comes to this machine. It\n\
                     \x20        is listed there as `333` if you want to take it away again. Whether\n\
                     \x20        anything arrives is the next line.",
                ),
                (
                    words!(
                        "serve-reach-router-opened-upnp-for",
                        port = Arg::exact(3333),
                        on = on(),
                        time = "two hours"
                    ),
                    "opened   the router says port 3333 on 203.0.113.7 now comes to this machine, \
                     for two hours. It\n\
                     \x20        is listed there as `333` if you want to take it away again. Whether\n\
                     \x20        anything arrives is the next line.",
                ),
                (
                    words!(
                        "serve-reach-router-opened-lease",
                        router = "192.168.1.1",
                        way = "PCP",
                        port = Arg::exact(3333),
                        asked_for = "two hours",
                        granted_port = Arg::exact(3334),
                        on = on(),
                        granted = "one hour",
                    ),
                    "opened   asked the router at 192.168.1.1 over PCP for port 3333 for two hours. It says port\n\
                     \x20        3334 on 203.0.113.7 now comes here, for one hour. This node asks again before that\n\
                     \x20        runs out and gives it back when the node stops; if the node is\n\
                     \x20        killed instead, the router drops it when the time is up. Whether\n\
                     \x20        anything arrives is the next line.",
                ),
                (
                    words!("serve-reach-router-nobody-answered"),
                    "closed   no router here answered a request to open a port, over UPnP-IGD, PCP\n\
                     \x20        or NAT-PMP. That is ordinary: plenty have all three turned off, and a\n\
                     \x20        machine with an address of its own has nothing to ask. `--no-router`\n\
                     \x20        stops this node asking at all.",
                ),
                (
                    words!(
                        "serve-reach-router-refused",
                        port = Arg::exact(3333),
                        why = "no"
                    ),
                    "closed   the router would not open port 3333: no",
                ),
                (
                    words!("serve-reach-router-let-go", port = Arg::exact(3333)),
                    "closed   the router let port 3333 go: it was not asked again in time, so nobody\n\
                     \x20        outside can reach this node on it now. Restarting the node asks\n\
                     \x20        again.",
                ),
                (
                    words!(
                        "serve-reach-router-given-back",
                        port = Arg::exact(3333),
                        way = "NAT-PMP"
                    ),
                    "closed   port 3333 is given back to the router over NAT-PMP; it no longer\n\
                     \x20        comes to this machine.",
                ),
                (
                    words!(
                        "serve-reach-router-not-taken-back",
                        port = Arg::exact(3333),
                        why = "timed out",
                        time = "90 minutes"
                    ),
                    "closed   the router did not take port 3333 back (timed out). It drops it by\n\
                     \x20        itself within 90 minutes.",
                ),
                (
                    words!(
                        "serve-reach-router-moved",
                        port = Arg::exact(3334),
                        on = on(),
                        before_port = Arg::exact(3333),
                        before_on = "on its outside address",
                    ),
                    "opened   the router moved this node: port 3334 on 203.0.113.7 now comes here instead of\n\
                     \x20        port 3333 on its outside address. An invitation naming the old one no longer arrives.",
                ),
                (
                    words!(
                        "serve-reach-router-not-kept",
                        port = Arg::exact(3333),
                        why = "timed out",
                        time = "45 seconds",
                    ),
                    "waiting  the router did not keep port 3333 when asked (timed out). It still has it\n\
                     \x20        for 45 seconds, and is asked again before then.",
                ),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

    #[test]
    fn a_lease_is_said_in_the_largest_unit_it_fills() {
        let said = [7_200, 3_600, 5_400, 86_400, 45, 1].map(|s| how_long(Duration::from_secs(s)));
        assert_eq!(
            said,
            [
                "two hours",
                "one hour",
                "90 minutes",
                "24 hours",
                "45 seconds",
                "one second"
            ]
        );
    }

    #[test]
    fn what_is_left_of_a_lease_is_never_said_as_less_than_it_is() {
        let nearly_an_hour = Duration::from_millis(3_599_700);
        assert_eq!(how_long(nearly_an_hour), "one hour");
        assert_eq!(how_long(Duration::from_secs(61)), "two minutes");
    }

    #[test]
    fn a_router_that_would_not_say_where_it_is_is_not_given_an_address() {
        assert_eq!(on(None), "on its outside address");
        assert_eq!(on(Some(IpAddr::from([203, 0, 113, 7]))), "on 203.0.113.7");
    }
}
