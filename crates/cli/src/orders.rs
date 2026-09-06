//! What a person can ask for from inside the screen, and how it is typed.
//!
//! The screen used to do two things: quit, and say one of the 333. Everything else a
//! node can be told to do was a separate command, run in a separate terminal, against
//! the same directory this node has open. That is not a second way of doing it. It is
//! a second program, and this one is holding the files.
//!
//! So the words are the same words. `ping 333:somewhere:3333` inside the screen does
//! what `333 ping 333:somewhere:3333` does outside it, and it does it in this process,
//! which is the one that already has the connection, the identity and the roll open.
//!
//! WHAT IS NOT HERE. Nothing that only makes sense before the node started: where its
//! directory is, how long it waits, whether it keeps everything. Those are settled by
//! the time there is a screen to type into, and the terminal cannot change them after
//! the fact either.

use std::fmt;

/// Something the person at the keyboard has asked this node to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Order {
    /// Reach a node and exchange one heartbeat with it.
    Ping(String),
    /// Ask whoever is at this address to hand over the file.
    Join(String),
    /// Begin a line of your own, if nobody else has begun one.
    Bootstrap {
        /// Go ahead even though somebody is already saying where they are.
        anyway: bool,
    },
    /// Say one of the 333 in this epoch.
    Say(String),
    /// Raise an onion address, so this node can be reached without a router being told
    /// anything.
    TorOn,
    /// Stop answering on the onion address.
    TorOff,
    /// Add a bridge line, for the next time Tor is started.
    Bridge(String),
    /// Name the program that speaks an obfuscated bridge.
    Helper(String),
    /// Say what this node is holding, in the log rather than on the dials.
    Status,
    /// Leave the screen. The node stops with it.
    Leave,
}

/// Why a typed line was not an order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NotAnOrder {
    /// Nothing was typed.
    Empty,
    /// The first word is not one of the words.
    Unknown(String),
    /// The word is right and what follows it is missing.
    Wants(&'static str),
}

impl fmt::Display for NotAnOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "nothing typed"),
            Self::Unknown(word) => write!(
                f,
                "there is no `{word}` here. \
                 ping, join, bootstrap, say, tor, bridge, helper, status, quit"
            ),
            Self::Wants(what) => write!(f, "that wants {what} after it"),
        }
    }
}

impl Order {
    /// Read a typed line as an order.
    ///
    /// # Errors
    /// Fails when the line is empty, the first word is not one of the words, or a word
    /// that needs something after it was given nothing.
    pub(crate) fn read(typed: &str) -> Result<Self, NotAnOrder> {
        let line = typed.trim();
        let (word, rest) = match line.split_once(char::is_whitespace) {
            Some((word, rest)) => (word, rest.trim()),
            None => (line, ""),
        };
        let missing = |what: &'static str| NotAnOrder::Wants(what);
        match word.to_ascii_lowercase().as_str() {
            "" => Err(NotAnOrder::Empty),
            "ping" if rest.is_empty() => Err(missing("an address")),
            "ping" => Ok(Self::Ping(rest.to_owned())),
            "join" if rest.is_empty() => Err(missing("an invitation")),
            "join" => Ok(Self::Join(rest.to_owned())),
            "bootstrap" => Ok(Self::Bootstrap {
                anyway: rest.eq_ignore_ascii_case("anyway"),
            }),
            "say" if rest.is_empty() => Err(missing("which of the 333")),
            "say" => Ok(Self::Say(rest.to_owned())),
            "tor" => match rest.to_ascii_lowercase().as_str() {
                "on" | "" => Ok(Self::TorOn),
                "off" => Ok(Self::TorOff),
                _ => Err(missing("on or off")),
            },
            "bridge" if rest.is_empty() => Err(missing("a bridge line")),
            "bridge" => Ok(Self::Bridge(rest.to_owned())),
            "helper" if rest.is_empty() => Err(missing("a program name or path")),
            "helper" => Ok(Self::Helper(rest.to_owned())),
            "status" => Ok(Self::Status),
            "quit" | "exit" => Ok(Self::Leave),
            other => Err(NotAnOrder::Unknown(other.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_words_are_the_terminals_words() {
        assert_eq!(
            Order::read("ping 333:somewhere:3333"),
            Ok(Order::Ping("333:somewhere:3333".into()))
        );
        assert_eq!(
            Order::read("join 333:somewhere:3333"),
            Ok(Order::Join("333:somewhere:3333".into()))
        );
        assert_eq!(Order::read("say 42"), Ok(Order::Say("42".into())));
        assert_eq!(Order::read("status"), Ok(Order::Status));
    }

    #[test]
    fn bootstrap_takes_the_one_word_that_insists() {
        assert_eq!(
            Order::read("bootstrap"),
            Ok(Order::Bootstrap { anyway: false })
        );
        assert_eq!(
            Order::read("bootstrap anyway"),
            Ok(Order::Bootstrap { anyway: true })
        );
    }

    #[test]
    fn tor_goes_both_ways_and_bare_tor_means_on() {
        assert_eq!(Order::read("tor"), Ok(Order::TorOn));
        assert_eq!(Order::read("tor on"), Ok(Order::TorOn));
        assert_eq!(Order::read("TOR OFF"), Ok(Order::TorOff));
        assert_eq!(
            Order::read("tor sideways"),
            Err(NotAnOrder::Wants("on or off"))
        );
    }

    #[test]
    fn a_bridge_line_is_kept_exactly_as_it_was_handed_over() {
        // Spacing and case are the bridge line's own, and it is passed on unread.
        let line = "Bridge obfs4 192.0.2.55:38114 316E64 cert=YXJl iat-mode=0";
        assert_eq!(
            Order::read(&format!("bridge  {line}")),
            Ok(Order::Bridge(line.into()))
        );
    }

    #[test]
    fn a_word_that_needs_something_says_what() {
        assert_eq!(Order::read("ping"), Err(NotAnOrder::Wants("an address")));
        assert_eq!(Order::read("join"), Err(NotAnOrder::Wants("an invitation")));
        assert_eq!(
            Order::read("say"),
            Err(NotAnOrder::Wants("which of the 333"))
        );
        assert_eq!(Order::read("  "), Err(NotAnOrder::Empty));
    }

    #[test]
    fn an_unknown_word_lists_the_known_ones() {
        let Err(NotAnOrder::Unknown(word)) = Order::read("dance now") else {
            panic!("that is not a word here");
        };
        assert_eq!(word, "dance");
        assert!(NotAnOrder::Unknown(word).to_string().contains("bootstrap"));
    }
}
