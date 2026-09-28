//! Whether a command needs a node to be there already, and what it says when none is.

use super::Command;

impl Command {
    /// Does this command read a node that has to be there already?
    ///
    /// `id` shows the name and makes one the first time; `serve`, `ping`, `join` and
    /// `bootstrap` speak for a node, and a node is a name; `unpack` puts one in place;
    /// `tell`, `languages` and `service` do not open one. The rest read a node or act
    /// on one, and run where there is none they would make a name with nothing behind
    /// it and report on that — which is what a mistyped `--data-dir` looked like.
    pub(crate) const fn reads_a_node(&self) -> bool {
        match self {
            Self::Say { .. } | Self::Status { .. } | Self::Pack { .. } | Self::Moved => true,
            Self::Id
            | Self::Serve { .. }
            | Self::Ping { .. }
            | Self::Join { .. }
            | Self::Bootstrap { .. }
            | Self::Unpack { .. }
            | Self::Tell { .. }
            | Self::Languages
            | Self::Service { .. } => false,
        }
    }

    /// Refuse, making nothing, when this command reads a node and `home` holds none.
    ///
    /// # Errors
    /// There is no node in `home`, and `333 id` is what makes one.
    pub(crate) fn needs_a_node_in(&self, home: &std::path::Path) -> anyhow::Result<()> {
        if !self.reads_a_node() || crate::identity_file::holds_a_name(home) {
            return Ok(());
        }
        let none = anyhow::anyhow!(words!("typed-node-none", home = home.display().to_string()));
        Err(crate::failed::next_step(
            none,
            words!("typed-node-none-next"),
        ))
    }
}
