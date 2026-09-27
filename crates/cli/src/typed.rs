//! What a person can type: the command line, read by clap.
//!
//! Beside `main` rather than in it, so that the words and flags are one file and what
//! is done with them is another. `service install` reads its flags through [`Cli`] as
//! well, so a flag `serve` takes is one `service` carries without a copy of it.

use std::net::SocketAddr;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::commands;
use crate::commands::elsewhere::Wanted;
use crate::version;
use n333_net::PeerAddress;

/// One node of the 333 network.
#[derive(Debug, Parser)]
#[command(
    name = "333",
    version,
    long_version = version::long(),
    about = "One node of 333. It keeps the hours, answers when asked, and passes the file on."
)]
pub(crate) struct Cli {
    /// Directory holding everything this node owns: its name, and Tor's state if it
    /// uses Tor.
    #[arg(long, global = true, value_name = "DIR")]
    pub(crate) data_dir: Option<PathBuf>,

    /// Seconds to wait for any single step that talks to the network.
    ///
    /// A ceiling rather than a delay. A direct connection is done in milliseconds
    /// and fails on its own; this is sized for a Tor bootstrap, which is the one
    /// step here that can legitimately take minutes.
    #[arg(long, global = true, default_value_t = 300, value_name = "SECONDS")]
    pub(crate) timeout: u64,

    /// Accept a directory that others on this machine can enter.
    ///
    /// Both this client and arti refuse to start on a loosely permissioned
    /// directory, which is the right default: that directory holds the only copy of
    /// this node's name. The flag exists for scratch directories and containers with
    /// odd ownership, and it does what its name says.
    #[arg(long, global = true)]
    pub(crate) dangerously_trust_directory_permissions: bool,

    /// Keep every statement for ever, instead of the window standing is read over.
    ///
    /// It confers nothing. Every statement carries its own signature and verifies the
    /// same wherever it was kept, so there is no archive of record and nobody becomes
    /// an archivist by doing this. It is for people who would rather the bytes still
    /// existed somewhere, which nothing here requires of anyone.
    #[arg(long, global = true)]
    pub(crate) keep_everything: bool,

    /// A bridge line, for a network that blocks the ordinary way into Tor.
    ///
    /// Give it once for each bridge you were handed, exactly as it was handed to you.
    /// Nothing here fetches bridges for you: they are scarce and they are given out by
    /// people, slowly and on purpose, because a list that could simply be collected
    /// would simply be blocked. Without any of these Tor is reached the ordinary way,
    /// which is what almost everybody wants.
    #[arg(long = "bridge", global = true, value_name = "LINE")]
    pub(crate) bridges: Vec<String>,

    /// The program that speaks an obfuscated bridge, by name or by path.
    ///
    /// Only needed when a bridge line asks for one, and only when it is not called
    /// `lyrebird` or is not on the path. It is not bundled: it is a separate program
    /// chasing a moving target, and a copy frozen inside this would be the wrong copy
    /// within a year while looking like the right one.
    #[arg(long, global = true, value_name = "PROGRAM")]
    pub(crate) bridge_helper: Option<String>,

    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Show this node's name, asking for one on first run.
    Id,
    /// Begin a line of your own, when there is nobody to be given the file by.
    ///
    /// The ordinary way in is an invitation from somebody who already has the file, and
    /// this is not that. It looks at the meeting point first, and if anybody is there it
    /// tells you to go and join them instead. If nobody is, it fetches the file, checks
    /// it against the hash this client carries, and writes it down. Your node is then
    /// the start of its own line and nobody signed for it, which anybody reading your
    /// record can see.
    Bootstrap {
        /// Where to look for people before beginning on your own.
        #[arg(long, default_value = n333_net::meeting::THE_PLACE, value_name = "HOST")]
        meet: String,

        /// Begin even though somebody is already there.
        #[arg(long)]
        anyway: bool,
    },

    /// Keep the vigil: answer whoever asks, until interrupted.
    ///
    /// This is what a node does almost all of the time. It answers heartbeats and
    /// challenges, trades what it knows with whoever it can reach, and at every epoch
    /// boundary asks the ones it was drawn to ask. On a terminal it opens the screen.
    Serve {
        /// Address and port to listen on.
        #[arg(long, default_value_t = default_bind(), value_name = "ADDR:PORT")]
        bind: SocketAddr,

        /// Also raise an onion address, so others can reach this node without
        /// learning where it is. Waking Tor takes seconds to minutes.
        #[arg(long)]
        tor: bool,

        /// Do not open a socket at all. Only useful with --tor, and the only way to
        /// keep the vigil with your address nowhere on the wire.
        #[arg(long)]
        no_direct: bool,

        /// The address to tell other nodes to reach this one at.
        ///
        /// Needed when the socket cannot say: listening on every interface, or behind
        /// something that forwards a port. Without it a node on a wildcard bind can
        /// answer whoever finds it and can never be found.
        #[arg(long, value_name = "HOST:PORT")]
        announce: Option<PeerAddress>,

        /// Do not say on the local network that this node is here.
        ///
        /// What goes out otherwise is that something on this machine speaks 333 and
        /// on which port — not this node's name — which is what a port scan of the
        /// same network would find anyway. It is how two nodes in one house find each
        /// other with nobody typing an invitation. A node listening only through Tor
        /// never does this at all.
        #[arg(long)]
        no_mdns: bool,

        /// Do not ask the router to send the port to this machine.
        ///
        /// Asking is what makes a socket on a home connection answer anybody: the
        /// router in front of it drops what nobody inside asked for until a program on
        /// the inside asks it not to, over UPnP-IGD, PCP or NAT-PMP, one of which most
        /// of them already speak. It is a real change to somebody's network, so it is
        /// said out loud when it is made and this refuses it outright. `--no-upnp` is
        /// the older name for the same thing, from when UPnP was all that was asked.
        #[arg(long, visible_alias = "no-upnp")]
        no_router: bool,

        /// Where to look for nodes nobody introduced this one to.
        ///
        /// One fixed address holding signed statements about where nodes say they
        /// are. Everything read there is verified here, and nothing there is
        /// believed. It is the only way two machines on two networks meet without
        /// somebody handing over an invitation.
        #[arg(long, default_value = n333_net::meeting::THE_PLACE, value_name = "HOST")]
        meet: String,

        /// Do not use a meeting point at all.
        ///
        /// This node is then reachable by whoever was handed an invitation and by
        /// nodes on this network, and by nobody else.
        #[arg(long)]
        no_meet: bool,

        /// Say the lines instead of drawing the screen.
        ///
        /// The screen is what this client does on a terminal. Anywhere else — a pipe,
        /// a service manager's log, a file — it says the lines instead, and this flag
        /// asks for that on a terminal too.
        #[arg(long)]
        plain: bool,
    },
    /// Speak one of the 333, once in this epoch. What travels is the number.
    Say {
        /// Which of them, from 0 to 332. The words are not written yet.
        #[arg(value_name = "INDEX")]
        index: u16,
    },
    /// Show what this node has seen: how many of us are answering, where this node
    /// stands over the window, and how much of the silence is left if it has begun.
    Status {
        /// List every address this node holds: whose it is, where it was first heard
        /// of and when, and where it was heard of last. The addresses are printed;
        /// this is your own node's disk and nobody else's.
        #[arg(long)]
        sources: bool,
        /// Say what this node observed as JSON, for a program to read. No address,
        /// onion address or port of any kind is in it.
        #[arg(long, conflicts_with = "sources")]
        json: bool,
    },
    /// Ask a node that has the file to hand it over. Write the file yourself and you
    /// hold a file: you are one of us from the moment somebody gives it to
    /// you and you both sign for it.
    Join {
        /// An invitation (`333:host:port`) from somebody who already has it.
        #[arg(value_parser = n333_net::invite::address_or_invite)]
        address: PeerAddress,
    },
    /// Knock on another node, and exchange one heartbeat with it.
    Ping {
        /// An invitation (`333:host:port`), or an address typed by hand as `host`,
        /// `host:port`, `[::1]:port` or `something.onion`. An onion address is
        /// reached through Tor; everything else directly.
        #[arg(value_parser = n333_net::invite::address_or_invite)]
        address: PeerAddress,
    },
    /// Write this node into one file, to carry it to another machine.
    ///
    /// Everything it is goes: its name, its record, what others signed about it, the
    /// file if it holds it, and the key to its onion address. Afterwards this
    /// directory refuses to act as it, because one name in two places is a node
    /// contradicting itself. The file is not encrypted: whoever holds it is this node,
    /// so carry it, unpack it, and delete it.
    Pack {
        /// The file to write. It must not exist yet.
        #[arg(value_name = "FILE", required_unless_present = "undo")]
        file: Option<PathBuf>,

        /// Take back a packing here, for a move that was abandoned.
        ///
        /// Only if the file was never unpacked anywhere: if it was, this makes two.
        #[arg(long, conflicts_with = "file")]
        undo: bool,
    },
    /// Put a packed node into this machine's node directory.
    ///
    /// Refused where a node already lives, and it says what that node holds. Nothing
    /// is written until the file has been read through and its key and record check
    /// out.
    Unpack {
        /// The file `333 pack` wrote.
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    /// Say that this node's directory was moved or renamed, not copied.
    ///
    /// A node that finds itself somewhere other than where it was says so on every
    /// run until this is typed, because a copy with the original still running is one
    /// name in two places, and from inside the directory the two look the same.
    Moved,
    /// Tell the vigil running in this directory something, in its screen's words.
    ///
    /// `tor on`, `tor off`, `bridge <line>`, `helper <program>`, and every other word
    /// the screen takes after `:`. The vigil carries it out and what it says about it
    /// is printed here. `say`, `join`, `ping`, `bootstrap` and `status` are handed to a
    /// running vigil the same way without this.
    Tell {
        /// The order, as it would be typed into the screen.
        #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
        order: Vec<String>,
    },
    /// Keep the vigil through logouts and reboots, with this system's service manager.
    ///
    /// Nothing is installed until you ask for it here, everything that is done is said
    /// as it is done, and `333 service uninstall` undoes it.
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
            Self::Bootstrap { meet, .. } if meet != n333_net::meeting::THE_PLACE => Wanted::Kept(
                "it looks for people only where it always does.\n\
                 \x20        Leave --meet out, or run this when it has stopped.",
            ),
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
            Self::Pack { .. } => Wanted::Kept(
                "a node is packed only while nothing is keeping it. Nothing was\n\
                 \x20        written. Stop the vigil, then pack it.",
            ),
            Self::Moved => Wanted::Kept(
                "where a node lives is said while nothing is keeping it. Stop the\n\
                 \x20        vigil, then run this again.",
            ),
            // Never asked: `unpack` takes the directory itself, and refuses on its own.
            Self::Unpack { .. } => Wanted::Kept(commands::unpack::KEPT),
            // Never asked: `service` is dispatched before the directory is taken, because
            // it asks the service manager and reads the awake stamp and nothing else.
            Self::Service { .. } => Wanted::Kept("it asks the service manager, not the vigil."),
        }
    }
}

/// Listen on every interface, on the port peers expect.
pub(crate) fn default_bind() -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], n333_net::DEFAULT_PORT))
}
