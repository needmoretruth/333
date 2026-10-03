//! What a person can type: the command line, read by clap.
//!
//! Beside `main` rather than in it, so that the words and flags are one file and what
//! is done with them is another. `service install` reads its flags through [`Cli`] as
//! well, so a flag `serve` takes is one `service` carries without a copy of it.
//!
//! OLD NAMES STAY. `serve`, `bootstrap`, `id` and `languages` were the names before
//! `run`, `begin`, `name` and `language`, and are kept as aliases nobody is shown:
//! every service installed before the rename runs `333 serve`, and every note a person
//! wrote down says the old one.
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

use clap::{CommandFactory as _, FromArgMatches as _, Parser, Subcommand};

use crate::commands;
use crate::paths::NodePaths;
use crate::words;
use n333_net::PeerAddress;

pub(crate) mod address;
mod asked;
mod node;
mod wanted;

use asked::Asked;

// What each command and flag is for is not written here: clap would show it in
// English whoever asked. It is in the catalogs, `help-<command>-<flag>` in
// `words/<tag>/help.ftl`, and put in once the words are chosen (`crate::help`). The
// one example each command shows is `help-<command>-example`.

// The command line, as clap reads it. With no command, `333` says how this node is.
#[derive(Debug, Parser)]
#[command(name = "333", version, about = "help-about")]
pub(crate) struct Cli {
    #[arg(
        long,
        global = true,
        value_name = "DIR",
        help = "help-data-dir",
        help_heading = "help-frame-advanced"
    )]
    pub(crate) data_dir: Option<PathBuf>,

    #[arg(
        long,
        global = true,
        default_value_t = 300,
        value_name = "SECONDS",
        help = "help-timeout",
        long_help = "help-timeout-long",
        help_heading = "help-frame-advanced"
    )]
    pub(crate) timeout: u64,

    #[arg(
        long,
        global = true,
        help = "help-dangerously-trust-directory-permissions",
        long_help = "help-dangerously-trust-directory-permissions-long",
        help_heading = "help-frame-advanced"
    )]
    pub(crate) dangerously_trust_directory_permissions: bool,

    #[arg(
        long,
        global = true,
        help = "help-keep-everything",
        long_help = "help-keep-everything-long",
        help_heading = "help-frame-advanced"
    )]
    pub(crate) keep_everything: bool,

    #[arg(
        long = "bridge",
        global = true,
        value_name = "LINE",
        help = "help-bridges",
        long_help = "help-bridges-long",
        help_heading = "help-frame-advanced"
    )]
    pub(crate) bridges: Vec<String>,

    #[arg(
        long,
        global = true,
        value_name = "PROGRAM",
        help = "help-bridge-helper",
        long_help = "help-bridge-helper-long",
        help_heading = "help-frame-advanced"
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

    #[arg(long, global = true, value_name = "BASE", value_parser = words::count::Base::named, help = "help-count-in", long_help = "help-count-in-long", help_heading = "help-frame-advanced")]
    pub(crate) count_in: Option<words::count::Base>,

    #[command(subcommand)]
    pub(crate) command: Option<Command>,
}

// The commands, as clap reads them. Which of them `333 --help` lists first is in
// `crate::help::listing`.
#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    #[command(about = "help-start", after_help = "help-start-example")]
    Start {
        // The flags for `run`, as `service install` takes them.
        #[arg(
            trailing_var_arg = true,
            allow_hyphen_values = true,
            value_name = "RUN FLAGS",
            help = "help-start-flags"
        )]
        flags: Vec<String>,
    },
    #[command(about = "help-stop", after_help = "help-stop-example")]
    Stop,
    #[command(about = "help-restart", after_help = "help-restart-example")]
    Restart {
        // The flags for `run`, as `start` takes them.
        #[arg(
            trailing_var_arg = true,
            allow_hyphen_values = true,
            value_name = "RUN FLAGS",
            help = "help-start-flags"
        )]
        flags: Vec<String>,
    },
    #[command(about = "help-status", after_help = "help-status-example")]
    Status {
        #[arg(long, help = "help-status-all")]
        all: bool,
        #[arg(long, conflicts_with = "all", help = "help-status-sources")]
        sources: bool,
        #[arg(long, conflicts_with_all = ["sources", "all"], help = "help-status-json")]
        json: bool,
    },
    #[command(about = "help-logs", after_help = "help-logs-example")]
    Logs {
        #[arg(short, long, help = "help-logs-follow")]
        follow: bool,
    },
    #[command(
        name = "name",
        alias = "id",
        about = "help-id",
        after_help = "help-id-example"
    )]
    Id,
    #[command(about = "help-invite", after_help = "help-invite-example")]
    Invite,
    #[command(
        name = "begin",
        alias = "bootstrap",
        about = "help-bootstrap",
        long_about = "help-bootstrap-long",
        after_help = "help-bootstrap-example"
    )]
    Bootstrap {
        #[arg(long, default_value = n333_net::meeting::THE_PLACE, value_name = "HOST", help = "help-bootstrap-meet")]
        meet: String,

        #[arg(long, help = "help-bootstrap-anyway")]
        anyway: bool,
    },

    #[command(
        name = "run",
        alias = "serve",
        about = "help-serve",
        long_about = "help-serve-long",
        after_help = "help-serve-example"
    )]
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
    #[command(about = "help-say", after_help = "help-say-example")]
    Say {
        #[arg(value_name = "INDEX", help = "help-say-index")]
        index: String,
    },
    #[command(about = "help-join", after_help = "help-join-example")]
    Join {
        #[arg(value_parser = address::typed, help = "help-join-address")]
        address: PeerAddress,
    },
    #[command(
        name = "language",
        alias = "languages",
        about = "help-languages",
        after_help = "help-languages-example"
    )]
    Languages {
        #[arg(value_name = "TAG", help = "help-languages-tag")]
        tag: Option<String>,
    },
    #[command(about = "help-ping", after_help = "help-ping-example")]
    Ping {
        #[arg(value_parser = address::typed, help = "help-ping-address")]
        address: PeerAddress,
    },
    #[command(
        about = "help-pack",
        long_about = "help-pack-long",
        after_help = "help-pack-example"
    )]
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
    #[command(
        about = "help-unpack",
        long_about = "help-unpack-long",
        after_help = "help-unpack-example"
    )]
    Unpack {
        #[arg(value_name = "FILE", help = "help-unpack-file")]
        file: PathBuf,
    },
    #[command(
        about = "help-moved",
        long_about = "help-moved-long",
        after_help = "help-moved-example"
    )]
    Moved,
    #[command(
        about = "help-tell",
        long_about = "help-tell-long",
        after_help = "help-tell-example"
    )]
    Tell {
        #[arg(
            required = true,
            trailing_var_arg = true,
            allow_hyphen_values = true,
            help = "help-tell-order"
        )]
        order: Vec<String>,
    },
    #[command(
        about = "help-service",
        long_about = "help-service-long",
        after_help = "help-service-example"
    )]
    Service {
        #[command(subcommand)]
        order: commands::service::Order,
    },
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

/// Listen on every interface, on the port peers expect.
pub(crate) fn default_bind() -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], n333_net::DEFAULT_PORT))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::words::count::Base;
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
        // running here, and", and its second line lines up under the first.
        for (key, before) in [
            (
                "typed-kept-meet",
                "it looks for nodes only where it always does.\n\
                 \x20        Leave --meet out, or run this when it has stopped.",
            ),
            (
                "typed-kept-pack",
                "a node is packed only while it is not running. Nothing was\n\
                 \x20        written. `333 stop`, then pack it.",
            ),
            (
                "typed-kept-moved",
                "where a node lives is recorded only while it is not running.\n\
                 \x20        `333 stop`, then run this again.",
            ),
            (
                "typed-kept-languages",
                "it reads the catalogs, not the running node.",
            ),
            (
                "typed-kept-service",
                "it asks the service manager, not the running node.",
            ),
        ] {
            let now = reason("en", key);
            assert_eq!(now, before.replace("\n\x20        ", "\n"), "{key}");
            let said = line("busy", &format!("x is running here, and\n{now}"));
            assert_eq!(
                said,
                format!("busy     x is running here, and\n\x20        {before}")
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
    fn no_command_is_read_and_every_old_name_reads_as_the_command_it_was() {
        let read = |line: &str| {
            let args = line.split(' ').map(OsString::from).collect();
            words::speaking("en", Base::Ten, || parse_from(args)).map(|cli| cli.command)
        };
        assert!(matches!(read("333"), Ok(None)));
        assert!(matches!(
            read("333 serve --plain"),
            Ok(Some(Command::Serve { plain: true, .. }))
        ));
        assert!(matches!(read("333 run"), Ok(Some(Command::Serve { .. }))));
        assert!(matches!(
            read("333 bootstrap"),
            Ok(Some(Command::Bootstrap { .. }))
        ));
        assert!(matches!(
            read("333 begin"),
            Ok(Some(Command::Bootstrap { .. }))
        ));
        assert!(matches!(read("333 id"), Ok(Some(Command::Id))));
        assert!(matches!(read("333 name"), Ok(Some(Command::Id))));
        assert!(matches!(
            read("333 languages"),
            Ok(Some(Command::Languages { tag: None }))
        ));
        assert!(matches!(
            read("333 language ko"),
            Ok(Some(Command::Languages { tag: Some(_) }))
        ));
        assert!(matches!(
            read("333 logs -f"),
            Ok(Some(Command::Logs { follow: true }))
        ));
    }

    #[test]
    fn start_and_restart_take_the_flags_run_takes_as_they_were_typed() {
        let read = |line: &str| {
            let args = line.split(' ').map(OsString::from).collect();
            words::speaking("en", Base::Ten, || parse_from(args)).map(|cli| cli.command)
        };
        let typed = ["--tor", "--bind", "0.0.0.0:4444", "--no-meet"].map(str::to_owned);
        let Ok(Some(Command::Start { flags })) =
            read("333 start --tor --bind 0.0.0.0:4444 --no-meet")
        else {
            panic!("start with flags");
        };
        assert_eq!(flags, typed);
        let Ok(Some(Command::Restart { flags })) =
            read("333 --data-dir /n restart --tor --bind 0.0.0.0:4444 --no-meet")
        else {
            panic!("restart with flags");
        };
        assert_eq!(flags, typed);
        assert!(matches!(
            read("333 start"),
            Ok(Some(Command::Start { flags })) if flags.is_empty()
        ));
    }
}
