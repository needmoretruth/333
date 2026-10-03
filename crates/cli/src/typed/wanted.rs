//! What each command wants of a node, put the way a running vigil could be asked for it.

use std::sync::OnceLock;

use super::Command;
use crate::commands;
use crate::commands::elsewhere::Wanted;

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
            Self::Status { all, sources, json } => {
                let show = commands::status::Show::of(*all, *sources, *json);
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
            // Never asked: `language` reads the catalogs and writes one small file, and
            // is dispatched before the directory is taken.
            Self::Languages { .. } => {
                static WHY: OnceLock<String> = OnceLock::new();
                kept(&WHY, || words!("typed-kept-languages"))
            }
            // Never asked: these ask the service manager, read the awake stamp and the
            // lock, and are dispatched before the directory is taken.
            Self::Service { .. }
            | Self::Start { .. }
            | Self::Stop
            | Self::Restart { .. }
            | Self::Logs { .. } => {
                static WHY: OnceLock<String> = OnceLock::new();
                kept(&WHY, || words!("typed-kept-service"))
            }
            // Never asked: `invite` reads the file the vigil writes whole, and is
            // dispatched before the directory is taken.
            Self::Invite => {
                static WHY: OnceLock<String> = OnceLock::new();
                kept(&WHY, || words!("typed-kept-invite"))
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
