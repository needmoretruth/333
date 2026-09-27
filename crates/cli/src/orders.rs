//! What a person can ask a running vigil for, and how it is typed.
//!
//! The screen used to do two things: quit, and say one of the 333. Everything else a
//! node can be told to do was a separate command, run in a separate terminal, against
//! the same directory this node has open. That is not a second way of doing it. It is
//! a second program, and this one is holding the files.
//!
//! So the words are the same words, and they are carried out by the vigil wherever
//! they were typed. `ping 333:somewhere:3333` inside the screen does what
//! `333 ping 333:somewhere:3333` does outside it, and while a vigil is running both of
//! them are done in that one process, which is the one that already has the
//! connection, the identity and the roll open. A node with no screen at all — the
//! Light edition, or one kept by a service manager — is told the same words through
//! its directory, by `333 tell` and by the commands that share a name with them.
//!
//! WHAT IS NOT HERE. Nothing that only makes sense before the node started: where its
//! directory is, how long it waits, whether it keeps everything. Those are settled by
//! the time there is a vigil to tell anything, and it cannot change them after the
//! fact either.

use std::fmt;

use crate::commands::status::Show;

/// Something a person has asked this node to do, in the screen or from another
/// terminal.
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
    /// Say what this node is holding, in the log rather than on the dials, or to the
    /// terminal that asked, in the way it asked for.
    Status(Show),
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
    Wants(Wanted),
}

/// What an order word needs after it, named so that it is said in the reader's words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Wanted {
    /// `ping`: where to knock.
    Address,
    /// `join`: the invitation to follow.
    Invitation,
    /// `say`: the index.
    Signal,
    /// `tor`: which way.
    OnOrOff,
    /// `bridge`: the line as it was handed over.
    BridgeLine,
    /// `helper`: the program.
    Program,
    /// `status`: one of its flags, or nothing.
    StatusWord,
}

/// The first words of every order, in the order a person reaches for them.
///
/// Never translated: they are the same words as the command line, and a person who
/// learned one learned the other.
pub(crate) const WORDS: &str = "ping, join, bootstrap, say, tor, bridge, helper, status, quit";

impl fmt::Display for Wanted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&match self {
            Self::Address => words!("orders-wants-address"),
            Self::Invitation => words!("orders-wants-invitation"),
            Self::Signal => words!("orders-wants-signal"),
            Self::OnOrOff => words!("orders-wants-on-or-off"),
            Self::BridgeLine => words!("orders-wants-bridge-line"),
            Self::Program => words!("orders-wants-program"),
            Self::StatusWord => words!("orders-wants-status-word"),
        })
    }
}

impl fmt::Display for NotAnOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&match self {
            Self::Empty => words!("orders-nothing-typed"),
            Self::Unknown(word) => words!("orders-unknown", word = word, words = WORDS),
            Self::Wants(what) => words!("orders-wants", what = what.to_string()),
        })
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
        let missing = NotAnOrder::Wants;
        match word.to_ascii_lowercase().as_str() {
            "" => Err(NotAnOrder::Empty),
            "ping" if rest.is_empty() => Err(missing(Wanted::Address)),
            "ping" => Ok(Self::Ping(rest.to_owned())),
            "join" if rest.is_empty() => Err(missing(Wanted::Invitation)),
            "join" => Ok(Self::Join(rest.to_owned())),
            "bootstrap" => Ok(Self::Bootstrap {
                anyway: rest.eq_ignore_ascii_case("anyway"),
            }),
            "say" if rest.is_empty() => Err(missing(Wanted::Signal)),
            "say" => Ok(Self::Say(rest.to_owned())),
            "tor" => match rest.to_ascii_lowercase().as_str() {
                "on" | "" => Ok(Self::TorOn),
                "off" => Ok(Self::TorOff),
                _ => Err(missing(Wanted::OnOrOff)),
            },
            "bridge" if rest.is_empty() => Err(missing(Wanted::BridgeLine)),
            "bridge" => Ok(Self::Bridge(rest.to_owned())),
            "helper" if rest.is_empty() => Err(missing(Wanted::Program)),
            "helper" => Ok(Self::Helper(rest.to_owned())),
            // Every word `333 status` takes, and nothing else: a word read as nothing
            // would answer something other than what was asked.
            "status" => match rest.to_ascii_lowercase().as_str() {
                "" => Ok(Self::Status(Show::Everything)),
                "--sources" | "sources" => Ok(Self::Status(Show::Sources)),
                "--json" | "json" => Ok(Self::Status(Show::Json)),
                _ => Err(missing(Wanted::StatusWord)),
            },
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
        assert_eq!(Order::read("status"), Ok(Order::Status(Show::Everything)));
    }

    #[test]
    fn status_takes_the_flags_the_command_takes_and_nothing_else() {
        // What another terminal sends for `333 status --json`, and must be read as it.
        assert_eq!(Order::read("status --json"), Ok(Order::Status(Show::Json)));
        assert_eq!(
            Order::read("status --sources"),
            Ok(Order::Status(Show::Sources))
        );
        assert_eq!(
            Order::read("status --everything"),
            Err(NotAnOrder::Wants(Wanted::StatusWord))
        );
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
            Err(NotAnOrder::Wants(Wanted::OnOrOff))
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
        assert_eq!(Order::read("ping"), Err(NotAnOrder::Wants(Wanted::Address)));
        assert_eq!(
            Order::read("join"),
            Err(NotAnOrder::Wants(Wanted::Invitation))
        );
        assert_eq!(Order::read("say"), Err(NotAnOrder::Wants(Wanted::Signal)));
        assert_eq!(Order::read("  "), Err(NotAnOrder::Empty));
    }

    #[test]
    fn in_english_every_refusal_says_exactly_what_it_said_before() {
        let pairs = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            [
                (NotAnOrder::Empty.to_string(), "nothing typed"),
                (
                    NotAnOrder::Unknown("dance".into()).to_string(),
                    "there is no `dance` here. \
                     ping, join, bootstrap, say, tor, bridge, helper, status, quit",
                ),
                (
                    NotAnOrder::Wants(Wanted::Address).to_string(),
                    "that wants an address after it",
                ),
                (
                    NotAnOrder::Wants(Wanted::Invitation).to_string(),
                    "that wants an invitation after it",
                ),
                (
                    NotAnOrder::Wants(Wanted::Signal).to_string(),
                    "that wants which of the 333 after it",
                ),
                (
                    NotAnOrder::Wants(Wanted::OnOrOff).to_string(),
                    "that wants on or off after it",
                ),
                (
                    NotAnOrder::Wants(Wanted::BridgeLine).to_string(),
                    "that wants a bridge line after it",
                ),
                (
                    NotAnOrder::Wants(Wanted::Program).to_string(),
                    "that wants a program name or path after it",
                ),
                (
                    NotAnOrder::Wants(Wanted::StatusWord).to_string(),
                    "that wants nothing, --sources or --json after it",
                ),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }

    #[test]
    fn in_korean_the_order_words_stay_the_command_line_s() {
        let said = crate::words::speaking("ko", crate::words::count::Base::Ten, || {
            NotAnOrder::Unknown("춤".into()).to_string()
        });
        assert!(said.contains(WORDS) && !said.contains("there is"), "{said}");
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
