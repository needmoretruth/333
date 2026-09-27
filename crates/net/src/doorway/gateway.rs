//! Which address the router is at.
//!
//! UPnP finds its router by shouting onto the network and seeing who answers. NAT-PMP
//! and PCP do not: they are spoken to the machine this one sends everything through,
//! and nothing else. So that machine has to be found first, and the operating system
//! already knows it — it is the gateway of the default route.
//!
//! WHY THE TABLE IS READ AND NOT ASKED THROUGH A LIBRARY. The crates that do this on
//! every system (`netdev`, `getifs`) enumerate every interface to do it, hardware
//! addresses included, and bring a netlink stack, Apple's configuration framework and
//! the Windows API bindings with them. One address is wanted. Linux writes the table to
//! a file; macOS and the BSDs print it with `route`; Windows prints it with `netsh`,
//! which, unlike `route print`, does not list the interfaces' hardware addresses on the
//! way. Each is read for the one line that matters and nothing else.
//!
//! IPv4 ONLY. NAT-PMP has no IPv6 at all, and a household behind IPv6 has no
//! translation to ask anybody about.

use std::net::Ipv4Addr;

/// The gateway of this machine's default IPv4 route, if it has one.
///
/// `None` is ordinary: a machine with an address of its own, a machine on a network
/// that routes everything through a tunnel, a system this does not know how to read.
/// Each of those has nothing to ask.
#[must_use]
pub fn default_gateway() -> Option<Ipv4Addr> {
    from_the_system()
}

/// Linux writes the table to a file, so nothing is run to read it.
#[cfg(any(target_os = "linux", target_os = "android"))]
fn from_the_system() -> Option<Ipv4Addr> {
    let table = std::fs::read_to_string("/proc/net/route").ok()?;
    from_proc_net_route(&table)
}

/// macOS and the BSDs answer the question directly when asked for one route.
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly"
))]
fn from_the_system() -> Option<Ipv4Addr> {
    from_route_get(&printed("/sbin/route", &["-n", "get", "default"])?)
}

/// Windows prints its table with `netsh`, from its own system directory.
#[cfg(windows)]
fn from_the_system() -> Option<Ipv4Addr> {
    let root = std::env::var_os("SystemRoot")?;
    let netsh = std::path::Path::new(&root)
        .join("System32")
        .join("netsh.exe");
    from_netsh(&printed(netsh, &["interface", "ipv4", "show", "route"])?)
}

/// A system this does not know how to read has nothing to ask.
#[cfg(not(any(
    windows,
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly"
)))]
fn from_the_system() -> Option<Ipv4Addr> {
    None
}

/// What a system tool printed, if it ran and said it had succeeded.
///
/// Named by full path, so that whatever is first on somebody's `PATH` is not what gets
/// run in its place.
#[cfg(any(
    windows,
    target_os = "macos",
    target_os = "ios",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly"
))]
fn printed(program: impl AsRef<std::ffi::OsStr>, args: &[&str]) -> Option<String> {
    let out = std::process::Command::new(program)
        .args(args)
        .stdin(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).into_owned())
}

/// The default route's gateway, read from Linux's `/proc/net/route`.
///
/// Each line after the heading is one route: interface, destination, gateway, flags,
/// three counters, metric, mask. Addresses are eight hexadecimal digits in the
/// machine's own byte order, which on every machine Linux runs a router client on is
/// little-endian — the kernel writes the four bytes of the address as one number.
/// The default route is the one whose destination and mask are both zero and which is
/// up and goes through a gateway; if there are several, the lowest metric wins, which
/// is the one the kernel itself uses.
#[cfg(any(target_os = "linux", target_os = "android", test))]
fn from_proc_net_route(table: &str) -> Option<Ipv4Addr> {
    /// The route is in use.
    const UP: u32 = 0x1;
    /// The route goes through a gateway rather than straight onto the wire.
    const GATEWAY: u32 = 0x2;
    let hex = |field: &str| u32::from_str_radix(field, 16).ok();
    table
        .lines()
        .skip(1)
        .filter_map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            let [_, destination, gateway, flags, _, _, metric, mask, ..] = fields.as_slice() else {
                return None;
            };
            let flags = hex(flags)?;
            let default = hex(destination)? == 0 && hex(mask)? == 0;
            let usable = flags & UP != 0 && flags & GATEWAY != 0;
            let metric: u32 = metric.parse().ok()?;
            let gateway = Ipv4Addr::from(hex(gateway)?.to_le_bytes());
            (default && usable && !gateway.is_unspecified()).then_some((metric, gateway))
        })
        .min_by_key(|(metric, _)| *metric)
        .map(|(_, gateway)| gateway)
}

/// The gateway line of `route -n get default`, as macOS and the BSDs print it.
///
/// The line is `gateway: 192.168.1.1`. A default route through a tunnel names an
/// interface there instead (`link#12`), which is not an address and is not asked.
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly",
    test
))]
fn from_route_get(printed: &str) -> Option<Ipv4Addr> {
    printed
        .lines()
        .find_map(|line| line.trim().strip_prefix("gateway:"))
        .and_then(|gateway| gateway.trim().parse().ok())
}

/// The default route's gateway in `netsh interface ipv4 show route`.
///
/// The headings are translated into the system's language and the words in the first
/// two columns are too, so neither is read. What is read is the row whose prefix is
/// `0.0.0.0/0`: the number before it is the metric, the one after it the interface,
/// and the one after that the gateway. The lowest metric wins, as it does for Windows.
#[cfg(any(windows, test))]
fn from_netsh(printed: &str) -> Option<Ipv4Addr> {
    printed
        .lines()
        .filter_map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            let at = fields.iter().position(|field| *field == "0.0.0.0/0")?;
            let metric: u32 = fields.get(at.checked_sub(1)?)?.parse().ok()?;
            let gateway: Ipv4Addr = fields.get(at + 2)?.parse().ok()?;
            (!gateway.is_unspecified()).then_some((metric, gateway))
        })
        .min_by_key(|(metric, _)| *metric)
        .map(|(_, gateway)| gateway)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A laptop on a home network with the wired route preferred over the wireless.
    const PROC_NET_ROUTE: &str = "\
Iface\tDestination\tGateway \tFlags\tRefCnt\tUse\tMetric\tMask\t\tMTU\tWindow\tIRTT
wlan0\t00000000\t0101A8C0\t0003\t0\t0\t600\t00000000\t0\t0\t0
eth0\t00000000\t0137A8C0\t0003\t0\t0\t100\t00000000\t0\t0\t0
eth0\t0037A8C0\t00000000\t0001\t0\t0\t100\t00FFFFFF\t0\t0\t0
";

    #[test]
    fn linux_names_the_gateway_of_the_cheapest_default_route() {
        assert_eq!(
            from_proc_net_route(PROC_NET_ROUTE),
            Some(Ipv4Addr::new(192, 168, 55, 1))
        );
    }

    #[test]
    fn a_linux_table_with_only_the_local_network_has_no_gateway() {
        let table = "Iface\tDestination\tGateway\tFlags\tRefCnt\tUse\tMetric\tMask\n\
                     eth0\t0037A8C0\t00000000\t0001\t0\t0\t100\t00FFFFFF\t0\t0\t0\n";
        assert_eq!(from_proc_net_route(table), None);
    }

    #[test]
    fn a_linux_default_route_that_is_down_is_not_used() {
        let table = "Iface\tDestination\tGateway\tFlags\tRefCnt\tUse\tMetric\tMask\n\
                     eth0\t00000000\t0137A8C0\t0002\t0\t0\t100\t00000000\t0\t0\t0\n";
        assert_eq!(from_proc_net_route(table), None);
    }

    #[test]
    fn macos_names_the_gateway_route_prints() {
        let printed = "   route to: default\n\
                       destination: default\n       mask: default\n\
                       \x20   gateway: 10.0.1.1\n  interface: en0\n\
                       \x20     flags: <UP,GATEWAY,DONE,STATIC,PRCLONING,GLOBAL>\n";
        assert_eq!(from_route_get(printed), Some(Ipv4Addr::new(10, 0, 1, 1)));
    }

    #[test]
    fn a_macos_default_route_through_a_tunnel_has_no_address_to_ask() {
        let printed = "   route to: default\n    gateway: link#12\n  interface: utun3\n";
        assert_eq!(from_route_get(printed), None);
    }

    #[test]
    fn windows_names_the_gateway_of_the_cheapest_default_route_in_any_language() {
        // German headings and column words, which is the point: none of them is read.
        let printed = "\n\
            Veröff.   Typ       Met   Präfix                    Idx  Gateway/Schnittstelle\n\
            -------  --------  ---  ------------------------  ---  ------------------------\n\
            Nein     Manuell   35   0.0.0.0/0                  17  192.168.178.1\n\
            Nein     Manuell   25   0.0.0.0/0                  12  192.168.1.1\n\
            Nein     System    256  127.0.0.0/8                 1  Loopback Pseudo-Interface 1\n";
        assert_eq!(from_netsh(printed), Some(Ipv4Addr::new(192, 168, 1, 1)));
    }

    #[test]
    fn a_windows_table_with_no_default_route_has_no_gateway() {
        let printed = "Nein     System    256  127.0.0.0/8   1  Loopback Pseudo-Interface 1\n";
        assert_eq!(from_netsh(printed), None);
    }

    #[test]
    fn this_machine_answers_the_question_without_failing() {
        // Whatever it is on, a build machine either has a default route or does not,
        // and asking is never an error.
        let _ = default_gateway();
    }
}
