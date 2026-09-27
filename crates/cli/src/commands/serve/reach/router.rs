//! Asking the router, keeping what it lent, and giving it back.
//!
//! What is asked and what is answered are both said, because the answer is a change in
//! somebody's house and the words are what they would need to undo it. A mapping made
//! over PCP or NAT-PMP is a lease: it is asked for again before it runs out for as long
//! as the vigil runs, and given back when the vigil ends. A mapping made over UPnP is
//! left as it was made, as it always has been, and is said to be left.

use std::net::IpAddr;
use std::time::Duration;

use n333_net::doorway::{self, Asked, Lasts, Lease, Way};
use tokio::sync::oneshot;

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
        } => aloud!(
            "opened   the router says port {outside_port} {} now comes to this machine{}. It\n\
             \x20        is listed there as `333` if you want to take it away again. Whether\n\
             \x20        anything arrives is the next line.",
            on(*outside),
            match lasts {
                Lasts::For(time) => format!(", for {}", how_long(*time)),
                Lasts::UntilTakenAway | Lasts::WhileKept(_) => String::new(),
            }
        ),
        Asked::Forwarded {
            lasts: Lasts::WhileKept(lease),
            ..
        } => aloud!(
            "opened   asked the router at {} over {} for port {port} for {}. It says port\n\
             \x20        {} {} now comes here, for {}. This node asks again before that\n\
             \x20        runs out and gives it back when the vigil ends; stopped any other\n\
             \x20        way, the router drops it by itself when the time is up. Whether\n\
             \x20        anything arrives is the next line.",
            lease.router(),
            lease.way(),
            how_long(doorway::ASK_FOR),
            lease.port(),
            on(lease.outside()),
            how_long(lease.granted()),
        ),
        Asked::Forwarded { .. } => {}
        Asked::NobodyAnswered => aloud!(
            "closed   no router here answered a request to open a port, over UPnP-IGD, PCP\n\
             \x20        or NAT-PMP. That is ordinary: plenty have all three turned off, and a\n\
             \x20        machine with an address of its own has nothing to ask. `--no-router`\n\
             \x20        stops this node asking at all."
        ),
        Asked::Refused(why) => aloud!("closed   the router would not open port {port}: {why}"),
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
            aloud!(
                "closed   the router let port {} go: it was not asked again in time, so nobody\n\
                 \x20        outside can reach this node on it now. Starting the vigil again\n\
                 \x20        asks again.",
                lease.port()
            );
            return;
        };
        tokio::select! {
            () = tokio::time::sleep(wait) => renew(&mut lease).await,
            _ = &mut leaving => break,
        }
    }
    let (port, way) = (lease.port(), lease.way());
    let left = lease.left();
    match lease.give_back().await {
        Ok(()) => println!(
            "closed   port {port} is given back to the router over {way}; it no longer\n\
             \x20        comes to this machine."
        ),
        Err(why) => println!(
            "closed   the router did not take port {port} back ({why}). It drops it by\n\
             \x20        itself within {}.",
            how_long(left)
        ),
    }
}

/// Ask once more, and say so only if something changed or went wrong.
///
/// A renewal that gives the same port on the same address is the lease continuing,
/// which is not news; a line every hour saying so would bury the lines that are.
async fn renew(lease: &mut Lease) {
    let before = (lease.port(), lease.outside());
    match lease.renew().await {
        Ok(()) if (lease.port(), lease.outside()) == before => {}
        Ok(()) => aloud!(
            "opened   the router moved this node: port {} {} now comes here instead of\n\
             \x20        port {} {}. An invitation naming the old one no longer arrives.",
            lease.port(),
            on(lease.outside()),
            before.0,
            on(before.1),
        ),
        Err(why) => aloud!(
            "waiting  the router did not keep port {} when asked ({why}). It still has it\n\
             \x20        for {}, and is asked again before then.",
            lease.port(),
            how_long(lease.left()),
        ),
    }
}

/// Where the port is, in the words the router gave or the ones it did not.
fn on(outside: Option<IpAddr>) -> String {
    outside.map_or_else(
        || "on its outside address".to_owned(),
        |address| format!("on {address}"),
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
    let (count, unit) = match seconds {
        s if s < 60 => (s, "second"),
        _ if minutes.is_multiple_of(60) => (minutes / 60, "hour"),
        _ => (minutes, "minute"),
    };
    match count {
        1 => format!("one {unit}"),
        2 => format!("two {unit}s"),
        n => format!("{n} {unit}s"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
