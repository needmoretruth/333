//! What a command does when another 333 already has this node's directory.
//!
//! It does not wait for it and it does not open anything it would write. If the other
//! one is keeping the vigil and what was asked is something a vigil can be told, it is
//! handed over and carried out there, and this prints what the vigil said about it.
//! Anything else is refused, naming who has the directory and what to do instead.

use std::process::ExitCode;

use crate::claim::Holder;
use crate::commands::Common;

/// What a command wants, put the way a running vigil could be asked for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Wanted {
    /// Something a vigil carries out, in the screen's own words.
    Order(String),
    /// Something a vigil answers with a page that is the whole of what this prints,
    /// so nothing is added after it: `status --json` is read by programs.
    Page(String),
    /// This node's name, which can be read beside a vigil without writing anything.
    Name,
    /// A vigil of its own. There is one per directory.
    Vigil,
    /// Something that cannot be handed over, and why not.
    Kept(&'static str),
}

/// Do what can be done beside the holder, and say so.
///
/// # Errors
/// Fails if the vigil stops answering part way, or what it said cannot be printed.
pub(crate) async fn run(
    common: &Common,
    holder: Holder,
    wanted: Wanted,
) -> anyhow::Result<ExitCode> {
    let home = common.paths.root();
    let who = who(holder);
    match wanted {
        Wanted::Order(order) => match crate::control::hand_over(home, &order).await? {
            Some(true) => {
                println!(
                    "done     carried out by the vigil kept in this directory{}.",
                    pid(holder)
                );
                Ok(ExitCode::SUCCESS)
            }
            Some(false) => {
                println!(
                    "failed   the vigil kept in this directory{} did not do that.",
                    pid(holder)
                );
                Ok(ExitCode::FAILURE)
            }
            None => Ok(busy(&who)),
        },
        Wanted::Page(order) => match crate::control::hand_over(home, &order).await? {
            Some(true) => Ok(ExitCode::SUCCESS),
            Some(false) => {
                eprintln!(
                    "failed   the vigil kept in this directory{} did not do that.",
                    pid(holder)
                );
                Ok(ExitCode::FAILURE)
            }
            None => Ok(busy(&who)),
        },
        Wanted::Name => {
            let Some(identity) = crate::identity_file::load(&common.mistrust(), home)? else {
                println!(
                    "busy     {who} is still finding\n\
                     \x20        this node's name. Run this again when it has one."
                );
                return Ok(ExitCode::FAILURE);
            };
            println!("name     {}", identity.node_id());
            println!("home     {}", home.display());
            Ok(ExitCode::SUCCESS)
        }
        Wanted::Vigil if crate::control::answering(home).await => {
            println!(
                "busy     {who} is already keeping the vigil here, and\n\
                 \x20        one directory is one node. It can be told things from here:\n\
                 \x20        `333 say 7`, `333 join <invitation>`, `333 tell 'tor on'`. A\n\
                 \x20        second node needs a directory of its own, given with --data-dir."
            );
            Ok(ExitCode::FAILURE)
        }
        Wanted::Kept(why) if crate::control::answering(home).await => {
            println!("busy     {who} is keeping the vigil here, and\n\x20        {why}");
            Ok(ExitCode::FAILURE)
        }
        Wanted::Vigil | Wanted::Kept(_) => Ok(busy(&who)),
    }
}

/// What `333 tell` says when nobody holds the directory to be told.
#[must_use]
pub(crate) fn nobody_to_tell() -> ExitCode {
    println!(
        "unheard  nobody is keeping the vigil in this directory, so there is nobody to\n\
         \x20        tell. `333 serve` keeps it, and then this works."
    );
    ExitCode::FAILURE
}

/// Refuse, when whoever holds the directory cannot be handed anything.
fn busy(who: &str) -> ExitCode {
    if cfg!(unix) {
        println!(
            "busy     {who} has this node's\n\
             \x20        directory and is not a vigil that can be handed this. Nothing here\n\
             \x20        was read or written. Run this again when it has finished."
        );
    } else {
        println!(
            "busy     {who} has this node's\n\
             \x20        directory. On this system a running 333 cannot be handed anything\n\
             \x20        from another terminal yet, so nothing here was read or written. Type\n\
             \x20        it into that one's screen after `:`, or stop it and run this again."
        );
    }
    ExitCode::FAILURE
}

/// The other 333, by its process number when it wrote one down.
fn who(holder: Holder) -> String {
    format!("another 333{}", pid(holder))
}

/// ` (process 1234)`, or nothing when the number could not be read.
fn pid(holder: Holder) -> String {
    holder
        .pid
        .map_or_else(String::new, |pid| format!(" (process {pid})"))
}
