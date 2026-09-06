//! Bridges: getting into Tor where the ordinary way in is blocked.
//!
//! A bridge is a relay that is not in the public directory. Somewhere that blocks Tor
//! blocks it by taking the public list and refusing everything on it, and a relay that
//! was never on the list is not on the list to refuse. Bridge addresses are handed out
//! a few at a time, by people, deliberately slowly, so that the same list cannot simply
//! be collected and blocked in turn.
//!
//! A plain bridge is enough where the block is a list. Where the block is a machine
//! reading the shape of the traffic, the bridge has to speak something that does not
//! look like Tor, and that is what an obfuscated bridge line asks for.
//!
//! THE PROGRAM THAT DOES THE OBFUSCATING IS NOT IN HERE. It is a separate executable,
//! it is a moving target because the thing it is hiding from moves, and a copy frozen
//! inside this client would be the wrong copy within a year while looking like the
//! right one. So the configuration names it and the person installs it. On Debian and
//! its relatives that is `apt install lyrebird`; on Fedora `dnf install lyrebird`; on
//! Arch it is in the AUR. Older systems call the same thing `obfs4proxy`.
//!
//! NOTHING HERE IS ON BY DEFAULT. A node with no bridge lines behaves exactly as it did
//! before this existed, which is the only sensible default: bridges are slower, they
//! are scarce, and using them where they are not needed spends somebody else's.

/// The program named when a bridge line asks for an obfuscated transport and the
/// person has not said which program speaks it.
///
/// `lyrebird` is what the obfs4 implementation is called now. A system that still
/// ships it under the older name wants that name given instead.
pub const USUAL_HELPER: &str = "lyrebird";

/// How this node gets into Tor, where getting in is the problem.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Bridges {
    /// Bridge lines, exactly as they were handed out.
    ///
    /// Kept as text rather than parsed here, because the format belongs to Tor and
    /// the people handing them out, and a copy of that parser in this crate would be
    /// a second opinion about what a bridge line means.
    pub lines: Vec<String>,
    /// The program that speaks the obfuscated protocols, by name or by path.
    pub helper: Option<String>,
}

impl Bridges {
    /// No bridges: the ordinary way in.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            lines: Vec::new(),
            helper: None,
        }
    }

    /// Whether any bridge was given.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// The program to run for the obfuscated transports, whether or not one was named.
    #[must_use]
    pub fn helper(&self) -> &str {
        self.helper.as_deref().unwrap_or(USUAL_HELPER)
    }

    /// The transport names the lines ask for, first seen first, without repeats.
    ///
    /// A bridge line is an optional `Bridge` word, then either an address — a plain
    /// bridge, needing no help — or the name of a transport. Only the second kind
    /// needs a program, and only the names actually asked for are configured, so a
    /// person handed one obfs4 line does not end up starting anything else.
    #[must_use]
    pub fn transports(&self) -> Vec<String> {
        let mut named: Vec<String> = Vec::new();
        for line in &self.lines {
            if let Some(name) = transport_of(line)
                && !named.iter().any(|seen| seen == name)
            {
                named.push(name.to_owned());
            }
        }
        named
    }
}

/// The transport a bridge line asks for, or `None` where it asks for none.
fn transport_of(line: &str) -> Option<&str> {
    let mut words = line.split_whitespace();
    let first = words.next()?;
    let candidate = if first.eq_ignore_ascii_case("bridge") {
        words.next()?
    } else {
        first
    };
    // An address is a host and a port, so it has a colon in it and a transport name
    // does not. That is the whole of the difference and it is the one Tor's own
    // parser makes.
    if candidate.contains(':') {
        None
    } else {
        Some(candidate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_plain_bridge_needs_no_program() {
        let bridges = Bridges {
            lines: vec!["Bridge 198.51.100.25:443 7DD62766BF2052432051D7B7E08A22F7E34A4543".into()],
            helper: None,
        };
        assert!(bridges.transports().is_empty());
        assert!(!bridges.is_empty());
    }

    #[test]
    fn an_obfuscated_bridge_names_what_speaks_it() {
        let bridges = Bridges {
            lines: vec![
                "obfs4 192.0.2.55:38114 316E643333645F6D79216558614D3931657A5F5F cert=x".into(),
                "Bridge obfs4 192.0.2.56:38114 316E643333645F6D79216558614D3931657A5F5F cert=y"
                    .into(),
                "Bridge snowflake 192.0.2.3:1 0000000000000000000000000000000000000000".into(),
            ],
            helper: None,
        };
        // Twice named, once configured.
        assert_eq!(bridges.transports(), vec!["obfs4", "snowflake"]);
        assert_eq!(bridges.helper(), USUAL_HELPER);
    }

    #[test]
    fn the_program_can_be_named() {
        let bridges = Bridges {
            lines: vec!["obfs4 192.0.2.55:38114 3131 cert=x".into()],
            helper: Some("/usr/bin/obfs4proxy".into()),
        };
        assert_eq!(bridges.helper(), "/usr/bin/obfs4proxy");
    }

    #[test]
    fn nothing_asked_for_is_nothing_configured() {
        assert!(Bridges::none().is_empty());
        assert!(Bridges::none().transports().is_empty());
    }
}
