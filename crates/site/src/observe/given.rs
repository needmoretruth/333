//! What the operator tells the observer outright: the site node's name and invitation.
//!
//! Both can be guessed (the name from the chain's author, the invitation from that
//! node's own whereabouts), but a founder's chain is empty until its first verdict, so
//! the guess has nothing to go on exactly when the site is new. Told, they win.
//!
//! Both are checked strictly, because both are printed on public pages. An empty value
//! (an unset line in the environment file) counts as not told.

use std::net::{Ipv4Addr, Ipv6Addr};

use anyhow::{bail, ensure};

/// The longest address a node will sign, and so the longest worth inviting to.
const LONGEST_ADDRESS: usize = n333_core::whereabouts::MAX_ADDRESS_LEN;

/// What the operator said, checked.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct Given {
    /// The site node's name, 64 lower-case hex digits beginning with `333`.
    pub(crate) site_node: Option<String>,
    /// `333:host:port`.
    pub(crate) invitation: Option<String>,
}

impl Given {
    /// Check both values; an empty or absent one is `None`.
    ///
    /// # Errors
    /// Fails, naming the value, if either is present and malformed.
    pub(crate) fn checked(
        site_node: Option<&str>,
        invitation: Option<&str>,
    ) -> anyhow::Result<Self> {
        fn told(value: Option<&str>) -> Option<&str> {
            value.map(str::trim).filter(|value| !value.is_empty())
        }
        let site_node = told(site_node).map(site_node_checked).transpose()?;
        let invitation = told(invitation).map(invitation_checked).transpose()?;
        Ok(Self {
            site_node,
            invitation,
        })
    }
}

/// A node name the protocol counts.
fn site_node_checked(name: &str) -> anyhow::Result<String> {
    let hex = name.len() == 64
        && name
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'));
    ensure!(
        hex && name.starts_with("333"),
        "--site-node is 64 lower-case hex digits beginning with 333, not {name:?}"
    );
    Ok(name.to_owned())
}

/// `333:` and an address a node could dial.
fn invitation_checked(invitation: &str) -> anyhow::Result<String> {
    let wrong = || format!("--invitation is 333:host:port, not {invitation:?}");
    let Some(address) = invitation.strip_prefix("333:") else {
        bail!(wrong());
    };
    let Some((host, port)) = address.rsplit_once(':') else {
        bail!(wrong());
    };
    let port_ok = !port.is_empty()
        && port.bytes().all(|byte| byte.is_ascii_digit())
        && port.parse::<u16>().is_ok_and(|port| port != 0);
    ensure!(
        port_ok && address.len() <= LONGEST_ADDRESS && host_ok(host),
        wrong()
    );
    Ok(invitation.to_owned())
}

/// An IPv4 address, a bracketed IPv6 address, or a DNS (or onion) name.
fn host_ok(host: &str) -> bool {
    if let Some(inner) = host
        .strip_prefix('[')
        .and_then(|rest| rest.strip_suffix(']'))
    {
        return inner.parse::<Ipv6Addr>().is_ok();
    }
    host.parse::<Ipv4Addr>().is_ok() || name_ok(host)
}

/// A DNS name with at least two labels, whose last label is not a number (so a
/// malformed IPv4 address is not taken for a name).
fn name_ok(host: &str) -> bool {
    let labels: Vec<&str> = host.split('.').collect();
    let label_ok = |label: &&str| {
        (1..=63).contains(&label.len())
            && label
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
            && !label.starts_with('-')
            && !label.ends_with('-')
    };
    let numeric_tail = labels
        .last()
        .is_some_and(|last| last.bytes().all(|byte| byte.is_ascii_digit()));
    host.len() <= 253 && labels.len() >= 2 && labels.iter().all(label_ok) && !numeric_tail
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_well_formed_values_are_taken() {
        let name = format!("333{}", "a".repeat(61));
        let given = Given::checked(Some(&name), Some("333:158.69.112.10:3333")).unwrap();
        assert_eq!(given.site_node.as_deref(), Some(name.as_str()));
        assert_eq!(
            Given::checked(Some(""), Some(" ")).unwrap(),
            Given::default()
        );
        for good in [
            "333:[2001:db8::1]:3333",
            "333:the333.dev:1",
            "333:abcdefghijklmnopqrstuvwxyz234567abcdefghijklmnopqrstuvwx.onion:3333",
        ] {
            assert!(Given::checked(None, Some(good)).is_ok(), "{good}");
        }
        for bad in [
            "158.69.112.10:3333",
            "333:1.2.3.4:0",
            "333:1.2.3.4:65536",
            "333:1.2.3.999:3333",
            "333:2001:db8::1:3333",
            "333:<b>.dev:3333",
            "333:host:3333",
            "333:a.dev:+1",
        ] {
            assert!(Given::checked(None, Some(bad)).is_err(), "{bad}");
        }
        assert!(Given::checked(Some(&"a".repeat(64)), None).is_err());
        assert!(Given::checked(Some(&name.to_uppercase()), None).is_err());
    }
}
