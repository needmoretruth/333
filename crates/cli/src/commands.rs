//! What the client can be asked to do.
//!
//! One file per command. Each one owns its own output text, because the words a
//! person reads are part of the command, not a detail of it.

pub(crate) mod hours;
pub(crate) mod id;
pub(crate) mod join;
pub(crate) mod ping;
pub(crate) mod say;
pub(crate) mod serve;
pub(crate) mod status;

use std::time::Duration;

use n333_net::Exchange;

/// Options every command shares.
#[derive(Debug, Clone)]
pub(crate) struct Common {
    /// Where this node keeps its files.
    pub(crate) paths: crate::paths::NodePaths,
    /// How long to wait for a step that talks to the Tor network.
    pub(crate) timeout: Duration,
    /// How much of the past this node holds on to.
    pub(crate) keeping: crate::node::Keeping,
    /// Whether to accept state directories other users can read.
    ///
    /// One bool, not two policies: arti and this client have to agree about whether a
    /// directory is private, or the client would write a seed into a directory arti
    /// then refuses to start in.
    pub(crate) trust_directory_permissions: bool,
}

impl Common {
    /// How strictly to judge the permissions on this node's directory.
    ///
    /// The default consults `$FS_MISTRUST_DISABLE_PERMISSIONS_CHECKS`, which is what
    /// arti does with the same setting, so one variable governs both.
    #[must_use]
    pub(crate) fn mistrust(&self) -> fs_mistrust::Mistrust {
        if self.trust_directory_permissions {
            fs_mistrust::Mistrust::new_dangerously_trust_everyone()
        } else {
            fs_mistrust::Mistrust::new()
        }
    }
}

/// Start a Tor client, giving up after the shared timeout.
///
/// Only reached when an address asks for Tor, or when `serve --tor` was used. A node
/// that is not hiding never calls this and never pays for it.
///
/// Arti retries bootstrap 128 times by default, so without a deadline a broken
/// network hangs instead of failing.
///
/// # Errors
/// Fails if the timeout elapses or arti cannot start.
#[cfg(feature = "tor")]
pub(crate) async fn bootstrap(common: &Common) -> anyhow::Result<n333_net::tor::Client> {
    use anyhow::Context as _;
    println!("waking   Tor. the unseen road takes a while to open.");
    tokio::time::timeout(
        common.timeout,
        n333_net::tor::bootstrap(&common.paths.tor(), common.trust_directory_permissions),
    )
    .await
    .with_context(|| format!("no Tor connection after {} s", common.timeout.as_secs()))?
    .context("starting the Tor client")
}

/// What opening a node found, said once at the start.
///
/// Only the lines that are true of this node right now. A fresh node has no record
/// and no members, and saying "0 members" every start would train the operator to
/// ignore the line that matters when it is not zero.
pub(crate) fn report_opening(opened: &crate::node::Opened) {
    if let crate::identity_file::Origin::Created { not_called } = opened.origin {
        println!("{}", crate::commands::naming(not_called));
    }
    if opened.chain_truncated != 0 {
        println!(
            "torn     {} bytes of an unfinished entry were dropped from the record",
            opened.chain_truncated
        );
    }
    if opened.chain_length != 0 {
        let epochs = if opened.chain_length == 1 {
            "1 epoch".to_owned()
        } else {
            format!("{} epochs", opened.chain_length)
        };
        println!("record   {epochs} already answered for, none of them open to revision");
    }
    if opened.witnessed != 0 {
        println!(
            "witness  {} statements other keys signed about this node. They are kept\n\
             \x20        after the epochs they belong to are gone, because nothing else of\n\
             \x20        them survives the window.",
            opened.witnessed
        );
    }
    if opened.members != 0 {
        let us = if opened.members == 1 {
            "1 of us, which is this node".to_owned()
        } else {
            format!("{} of us", opened.members)
        };
        println!("roll     {us}");
    }
    if opened.addresses != 0 {
        println!("known    where {} of us said to look", opened.addresses);
    }
    if opened.has_the_file {
        println!("holding  the file, and able to pass it on");
    }
    if opened.keeping == crate::node::Keeping::Everything {
        println!(
            "keeping  everything, for ever. It buys this node nothing: every statement\n\
             \x20        carries its own signature and verifies the same wherever it was\n\
             \x20        kept. There is no archive of record and there is no archivist."
        );
    }
    if opened.read.unreadable != 0 {
        println!(
            "ignored  {} admissions that could not be read",
            opened.read.unreadable
        );
    }
}

/// What a trade of statements changed, when it changed anything.
///
/// Silent when it changed nothing, which is the ordinary case once a node has settled:
/// a line every time would be a line every 333 minutes per neighbour saying nothing
/// happened.
pub(crate) fn report_heard(heard: &crate::node::Heard) {
    if heard.addresses != 0 {
        println!("learned  where {} more of us are", heard.addresses);
    }
    if heard.members != 0 {
        if heard.were != 0 && heard.members >= heard.were {
            // One meeting brought more of us than this node had ever held. Two halves
            // of a network that had not spoken look exactly like this from one side.
            println!(
                "rejoined {} more of us by name, from a node that knew {}. There were\n\
                 \x20        two of us and now the counting is one count.",
                heard.members, heard.were
            );
        } else {
            println!("learned  {} more of us by name", heard.members);
        }
    }
    if heard.said != 0 {
        println!("heard    {} of us speak", heard.said);
    }
    if heard.witnessed != 0 {
        println!("carried  {} statements about epochs still open", heard.witnessed);
    }
}

/// The two sentences a handover actually puts a signature under, read back.
///
/// They are not a summary of the record. They are the record: those two lines, in two
/// hands, are the whole of what an admission is, and printing anything else in their
/// place would be printing a paraphrase of the only thing either node signed.
///
/// The closing line is deliberately identical at both ends. It is the one formula both
/// sides of the act speak, which is what a pair is.
#[must_use]
pub(crate) fn what_was_signed(transfer: &n333_core::Transfer, ours_was_the_giving: bool) -> String {
    let epoch = transfer.epoch().0;
    let (first, second) = if ours_was_the_giving {
        ("you", "they")
    } else {
        ("they", "you")
    };
    format!(
        "signed   {first} said: I handed the file to you in epoch {epoch}.\n\
         \x20        {second} said: I received the file from you in epoch {epoch}.\n\
         \x20        it is written in two hands, and neither hand can take it back."
    )
}

/// The line that says a name was found, said as the naming it is.
///
/// The number is how many keys were made and passed over. The one that was called is
/// not among them, which is the whole difference between reading a loop and reading
/// what happened.
#[must_use]
pub(crate) fn naming(not_called: u64) -> String {
    match not_called {
        0 => "called   the first key made was called.".to_owned(),
        1 => "called   1 key was made and not called. this one was.".to_owned(),
        many => format!("called   {many} keys were made and not called. this one was."),
    }
}

/// Say when a node had more to pass on than would fit in one run.
///
/// A cap that nothing reports is a cap that reads as "everything was sent" right up
/// until somebody wonders why the roll stopped growing.
pub(crate) fn report_left_behind(tidings: &crate::node::Tidings) {
    if tidings.left_behind != 0 {
        println!(
            "brimming {} statements would not fit in one run and wait for the next",
            tidings.left_behind
        );
    }
}

/// One line describing what a completed exchange showed.
///
/// The parenthesis is the part that matters and the part most easily overstated: one
/// of these two exchanges proves the peer was awake and the other does not.
#[must_use]
pub(crate) fn describe(exchange: &Exchange) -> String {
    let liveness = if exchange.proves_peer_was_live {
        "answered the challenge we chose"
    } else {
        "spoke first, which proves only that it spoke"
    };
    format!(
        "witness  {}  epoch {}  skew {:+}  ({liveness})",
        exchange.peer.node_id, exchange.peer.heartbeat.epoch, exchange.epoch_skew
    )
}
