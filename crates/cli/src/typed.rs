//! What a person can type: the command line, read by clap.
//!
//! Beside `main` rather than in it, so that the words and flags are one file and what
//! is done with them is another. `service install` reads its flags through [`Cli`] as
//! well, so a flag `serve` takes is one `service` carries without a copy of it.
//!
//! READ TWICE WHEN IT IS REFUSED. The words a person reads are chosen from what the
//! command line asks for, so they cannot be chosen before it is read. A line that
//! reads is read once and the words chosen from it. A line clap refuses, `--help`
//! and `--version` among them, has no reading to choose from: the language, the base
//! and the directory are picked out of it by hand, the words chosen from those, and
//! the line read again with them, so that the refusal or the help is in those words.

use std::ffi::OsString;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::OnceLock;

use clap::{CommandFactory as _, FromArgMatches as _, Parser, Subcommand};

use crate::commands;
use crate::commands::elsewhere::Wanted;
use crate::paths::NodePaths;
use crate::words;
use crate::words::count::Base;
use n333_net::PeerAddress;

pub(crate) mod address;

// What each command and flag is for is not written here: clap would show it in
// English whoever asked. It is in the catalogs, `help-<command>-<flag>` in
// `words/<tag>/help.ftl`, and put in once the words are chosen (`crate::help`).

// The command line, as clap reads it.
#[derive(Debug, Parser)]
#[command(name = "333", version, about = "help-about")]
pub(crate) struct Cli {
    #[arg(long, global = true, value_name = "DIR", help = "help-data-dir")]
    pub(crate) data_dir: Option<PathBuf>,

    #[arg(
        long,
        global = true,
        default_value_t = 300,
        value_name = "SECONDS",
        help = "help-timeout",
        long_help = "help-timeout-long"
    )]
    pub(crate) timeout: u64,

    #[arg(
        long,
        global = true,
        help = "help-dangerously-trust-directory-permissions",
        long_help = "help-dangerously-trust-directory-permissions-long"
    )]
    pub(crate) dangerously_trust_directory_permissions: bool,

    #[arg(
        long,
        global = true,
        help = "help-keep-everything",
        long_help = "help-keep-everything-long"
    )]
    pub(crate) keep_everything: bool,

    #[arg(
        long = "bridge",
        global = true,
        value_name = "LINE",
        help = "help-bridges",
        long_help = "help-bridges-long"
    )]
    pub(crate) bridges: Vec<String>,

    #[arg(
        long,
        global = true,
        value_name = "PROGRAM",
        help = "help-bridge-helper",
        long_help = "help-bridge-helper-long"
    )]
    pub(crate) bridge_helper: Option<String>,

    #[arg(
        long,
        global = true,
        value_name = "TAG",
        help = "help-language",
        long_help = "help-language-long"
    )]
    pub(crate) language: Option<String>,

    #[arg(long, global = true, value_name = "BASE", value_parser = words::count::Base::named, help = "help-count-in", long_help = "help-count-in-long")]
    pub(crate) count_in: Option<words::count::Base>,

    #[command(subcommand)]
    pub(crate) command: Command,
}

// The commands, as clap reads them.
#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    #[command(about = "help-id")]
    Id,
    #[command(about = "help-bootstrap", long_about = "help-bootstrap-long")]
    Bootstrap {
        #[arg(long, default_value = n333_net::meeting::THE_PLACE, value_name = "HOST", help = "help-bootstrap-meet")]
        meet: String,

        #[arg(long, help = "help-bootstrap-anyway")]
        anyway: bool,
    },

    #[command(about = "help-serve", long_about = "help-serve-long")]
    Serve {
        #[arg(long, default_value_t = default_bind(), value_name = "ADDR:PORT", value_parser = address::bind, help = "help-serve-bind")]
        bind: SocketAddr,

        #[arg(long, help = "help-serve-tor")]
        tor: bool,

        #[arg(long, help = "help-serve-no-direct")]
        no_direct: bool,

        #[arg(
            long,
            value_name = "HOST:PORT",
            value_parser = address::announced,
            help = "help-serve-announce",
            long_help = "help-serve-announce-long"
        )]
        announce: Option<PeerAddress>,

        #[arg(
            long,
            help = "help-serve-no-mdns",
            long_help = "help-serve-no-mdns-long"
        )]
        no_mdns: bool,

        #[arg(
            long,
            visible_alias = "no-upnp",
            help = "help-serve-no-router",
            long_help = "help-serve-no-router-long"
        )]
        no_router: bool,

        #[arg(long, default_value = n333_net::meeting::THE_PLACE, value_name = "HOST", help = "help-serve-meet", long_help = "help-serve-meet-long")]
        meet: String,

        #[arg(
            long,
            help = "help-serve-no-meet",
            long_help = "help-serve-no-meet-long"
        )]
        no_meet: bool,

        #[arg(long, help = "help-serve-plain", long_help = "help-serve-plain-long")]
        plain: bool,
    },
    #[command(about = "help-say")]
    Say {
        #[arg(value_name = "INDEX", help = "help-say-index")]
        index: String,
    },
    #[command(about = "help-status")]
    Status {
        #[arg(long, help = "help-status-sources")]
        sources: bool,
        #[arg(long, conflicts_with = "sources", help = "help-status-json")]
        json: bool,
    },
    #[command(about = "help-join")]
    Join {
        #[arg(value_parser = address::typed, help = "help-join-address")]
        address: PeerAddress,
    },
    #[command(about = "help-languages")]
    Languages,
    #[command(about = "help-ping")]
    Ping {
        #[arg(value_parser = address::typed, help = "help-ping-address")]
        address: PeerAddress,
    },
    #[command(about = "help-pack", long_about = "help-pack-long")]
    Pack {
        #[arg(
            value_name = "FILE",
            required_unless_present = "undo",
            help = "help-pack-file"
        )]
        file: Option<PathBuf>,

        #[arg(
            long,
            conflicts_with = "file",
            help = "help-pack-undo",
            long_help = "help-pack-undo-long"
        )]
        undo: bool,
    },
    #[command(about = "help-unpack", long_about = "help-unpack-long")]
    Unpack {
        #[arg(value_name = "FILE", help = "help-unpack-file")]
        file: PathBuf,
    },
    #[command(about = "help-moved", long_about = "help-moved-long")]
    Moved,
    #[command(about = "help-tell", long_about = "help-tell-long")]
    Tell {
        #[arg(
            required = true,
            trailing_var_arg = true,
            allow_hyphen_values = true,
            help = "help-tell-order"
        )]
        order: Vec<String>,
    },
    #[command(about = "help-service", long_about = "help-service-long")]
    Service {
        #[command(subcommand)]
        order: commands::service::Order,
    },
}

impl Command {
    /// What this command wants, put the way a running vigil could be asked for it.
    pub(crate) fn wanted(&self) -> Wanted {
        match self {
            Self::Id => Wanted::Name,
            Self::Serve { .. } => Wanted::Vigil,
            Self::Bootstrap { meet, .. } if meet != n333_net::meeting::THE_PLACE => {
                static WHY: OnceLock<String> = OnceLock::new();
                kept(&WHY, || words!("typed-kept-meet"))
            }
            Self::Bootstrap { anyway: true, .. } => Wanted::Order("bootstrap anyway".to_owned()),
            Self::Bootstrap { anyway: false, .. } => Wanted::Order("bootstrap".to_owned()),
            Self::Say { index } => Wanted::Order(format!("say {index}")),
            Self::Status { sources, json } => {
                let show = commands::status::Show::of(*sources, *json);
                Wanted::Page(format!("status {}", show.word()))
            }
            Self::Join { address } => Wanted::Order(format!("join {address}")),
            Self::Ping { address } => Wanted::Order(format!("ping {address}")),
            Self::Tell { order } => Wanted::Order(order.join(" ")),
            Self::Pack { .. } => {
                static WHY: OnceLock<String> = OnceLock::new();
                kept(&WHY, || words!("typed-kept-pack"))
            }
            Self::Moved => {
                static WHY: OnceLock<String> = OnceLock::new();
                kept(&WHY, || words!("typed-kept-moved"))
            }
            // Never asked: `unpack` takes the directory itself, and refuses on its own.
            Self::Unpack { .. } => Wanted::Kept(commands::unpack::KEPT),
            // Never asked: `languages` reads the catalogs, not the node, and is dispatched
            // before the directory is taken.
            Self::Languages => {
                static WHY: OnceLock<String> = OnceLock::new();
                kept(&WHY, || words!("typed-kept-languages"))
            }
            // Never asked: `service` is dispatched before the directory is taken, because
            // it asks the service manager and reads the awake stamp and nothing else.
            Self::Service { .. } => {
                static WHY: OnceLock<String> = OnceLock::new();
                kept(&WHY, || words!("typed-kept-service"))
            }
        }
    }
}

/// Why a command cannot be handed to a running vigil.
///
/// Said once, in the words this process speaks, and kept for the life of the process,
/// which is as long as the reason is ever read.
fn kept(cell: &'static OnceLock<String>, why: impl FnOnce() -> String) -> Wanted {
    Wanted::Kept(cell.get_or_init(why))
}

/// Read this process's command line, or say why it cannot be read — or the help or the
/// version it asked for — in the words it asked for, and exit as clap would.
pub(crate) fn read() -> Cli {
    let args: Vec<OsString> = std::env::args_os().collect();
    let first = match Cli::try_parse_from(&args) {
        Ok(cli) => return cli,
        Err(first) => first,
    };
    let asked = Asked::from(&args);
    let home = asked
        .data_dir
        .map_or_else(NodePaths::default_home, NodePaths::at);
    words::install(asked.language.as_deref(), asked.count_in, home.root());
    match parse_from(args) {
        Err(refused) => refused.exit(),
        // Read the second time with only the words changed, it cannot read; if it
        // somehow does, the first refusal is the one there is.
        Ok(_) => first.exit(),
    }
}

/// Read a command line with every word a person reads in the words this process speaks.
///
/// # Errors
/// What clap refuses, and `--help` and `--version`, which clap answers the same way.
pub(crate) fn parse_from(args: Vec<OsString>) -> Result<Cli, Refusal> {
    let mut command = crate::help::spoken(Cli::command());
    let mut matches = command.try_get_matches_from_mut(args).map_err(Refusal)?;
    Cli::from_arg_matches_mut(&mut matches).map_err(|e| Refusal(e.format(&mut command)))
}

/// A command line clap would not read, or the help or the version it was asked for.
#[derive(Debug)]
pub(crate) struct Refusal(clap::Error);

impl Refusal {
    /// Say it in the words this process speaks, and exit with clap's code.
    pub(crate) fn exit(self) -> ! {
        if words::current()
            .tag()
            .eq_ignore_ascii_case(words::catalog::ENGLISH)
        {
            self.0.exit()
        }
        self.0.apply::<crate::help::refused::Spoken>().exit()
    }

    /// What [`Refusal::exit`] would print, without the colours.
    #[cfg(test)]
    pub(crate) fn rendered(self) -> String {
        if words::current()
            .tag()
            .eq_ignore_ascii_case(words::catalog::ENGLISH)
        {
            return self.0.render().to_string();
        }
        self.0
            .apply::<crate::help::refused::Spoken>()
            .render()
            .to_string()
    }
}

/// What a command line asks to be read in and where its node is, picked out by hand
/// from one clap refused.
#[derive(Debug, Default, PartialEq, Eq)]
struct Asked {
    /// `--language`.
    language: Option<String>,
    /// `--count-in`, if it names a base.
    count_in: Option<Base>,
    /// `--data-dir`, where a folder of catalogs beside the node may be.
    data_dir: Option<PathBuf>,
}

impl Asked {
    /// Every `--language`, `--count-in` and `--data-dir` before a `--`, the last of
    /// each winning, as clap would have it; given as `--flag value` or `--flag=value`.
    fn from(args: &[OsString]) -> Self {
        let mut asked = Self::default();
        let mut words = args.iter().skip(1).map(|arg| arg.to_string_lossy());
        while let Some(word) = words.next() {
            if word == "--" {
                break;
            }
            let (flag, given) = match word.split_once('=') {
                Some((flag, value)) => (flag.to_owned(), Some(value.to_owned())),
                None => (word.into_owned(), None),
            };
            if !["--language", "--count-in", "--data-dir"].contains(&flag.as_str()) {
                continue;
            }
            let Some(value) = given.or_else(|| words.next().map(|value| value.into_owned())) else {
                break;
            };
            match flag.as_str() {
                "--language" => asked.language = Some(value),
                "--count-in" => asked.count_in = Base::named(&value).ok(),
                _ => asked.data_dir = Some(PathBuf::from(value)),
            }
        }
        asked
    }
}

/// Listen on every interface, on the port peers expect.
pub(crate) fn default_bind() -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], n333_net::DEFAULT_PORT))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::words::layout::{COLUMN, line, where_the_words_begin};

    fn reason(tag: &str, key: &str) -> String {
        words::speaking(tag, Base::Ten, || match key {
            "typed-kept-meet" => words!("typed-kept-meet"),
            "typed-kept-pack" => words!("typed-kept-pack"),
            "typed-kept-moved" => words!("typed-kept-moved"),
            "typed-kept-languages" => words!("typed-kept-languages"),
            _ => words!("typed-kept-service"),
        })
    }

    #[test]
    fn in_english_every_reason_a_vigil_is_not_handed_a_command_is_what_it_was() {
        // The column is the line's to give: each is said after a keyword and "is
        // keeping the vigil here, and", and its second line lines up under the first.
        for (key, before) in [
            (
                "typed-kept-meet",
                "it looks for people only where it always does.\n\
                 \x20        Leave --meet out, or run this when it has stopped.",
            ),
            (
                "typed-kept-pack",
                "a node is packed only while nothing is keeping it. Nothing was\n\
                 \x20        written. Stop the vigil, then pack it.",
            ),
            (
                "typed-kept-moved",
                "where a node lives is said while nothing is keeping it. Stop the\n\
                 \x20        vigil, then run this again.",
            ),
            (
                "typed-kept-languages",
                "it reads the catalogs, not the vigil.",
            ),
            (
                "typed-kept-service",
                "it asks the service manager, not the vigil.",
            ),
        ] {
            let now = reason("en", key);
            assert_eq!(now, before.replace("\n\x20        ", "\n"), "{key}");
            let said = line("busy", &format!("x is keeping the vigil here, and\n{now}"));
            assert_eq!(
                said,
                format!("busy     x is keeping the vigil here, and\n\x20        {before}")
            );
        }
    }

    #[test]
    fn in_korean_a_reason_keeps_the_column_it_is_said_in() {
        let why = reason("ko", "typed-kept-pack");
        let said = line(
            "사용중",
            &format!("누군가 여기서 철야를 지키고 있으며,\n{why}"),
        );
        let mut lines = said.lines();
        assert_eq!(where_the_words_begin(lines.next().unwrap()), COLUMN);
        for one in lines {
            assert_eq!(
                one.chars().take_while(|c| *c == ' ').count(),
                COLUMN,
                "{said}"
            );
        }
    }

    #[test]
    fn a_refused_line_is_read_for_its_language_base_and_directory_by_hand() {
        let args = |line: &str| -> Vec<OsString> { line.split(' ').map(OsString::from).collect() };
        let asked = Asked::from(&args(
            "333 --bogus --language=ko serve --count-in twelve --data-dir /tmp/n --language es",
        ));
        assert_eq!(
            asked,
            Asked {
                language: Some("es".to_owned()),
                count_in: Some(Base::Twelve),
                data_dir: Some(PathBuf::from("/tmp/n")),
            }
        );
        let after_the_end = Asked::from(&args("333 tell -- --language ko"));
        assert_eq!(after_the_end, Asked::default());
        let unfinished = Asked::from(&args("333 --count-in nine --language"));
        assert_eq!(unfinished, Asked::default());
    }
}
