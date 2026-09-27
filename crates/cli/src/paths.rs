//! Where this node keeps its files.
//!
//! One directory holds everything: the identity key and the two directories arti
//! wants. Keeping them together means a node can be moved, backed up or thrown away
//! as a unit, and means two nodes on one machine are two directories rather than a
//! configuration puzzle.
//!
//! Moving one is `333 pack` at one end and `333 unpack` at the other, rather than a
//! copy: a copy leaves the name in two places. What goes and what stays is in
//! [`crate::archive`]; how a copy is noticed is in [`crate::dwelling`].
//!
//! The default location is the conventional one for each system, worked out by the
//! `directories` crate rather than assumed here:
//!
//! | system | default |
//! |---|---|
//! | Linux and the BSDs | `$XDG_DATA_HOME/333`, or `~/.local/share/333` |
//! | macOS | `~/Library/Application Support/333` |
//! | Windows | `%LOCALAPPDATA%\333\data` |
//!
//! On Windows this is the local application data directory and not the roaming one,
//! because a node's identity and its Tor state belong to one machine. Copied to a
//! second machine by a roaming profile, both would answer to the same name, and the
//! record would show one node in two places at once.

use std::path::{Path, PathBuf};

/// The layout of a node's directory.
#[derive(Debug, Clone)]
pub(crate) struct NodePaths {
    root: PathBuf,
}

impl NodePaths {
    /// Use this directory as the node's home.
    #[must_use]
    pub(crate) fn at(root: PathBuf) -> Self {
        Self { root }
    }

    /// The conventional home for this system.
    ///
    /// Falls back to `./333-data` when the system has no home directory to offer,
    /// which is better than refusing to start and better than writing somewhere
    /// surprising.
    #[must_use]
    pub(crate) fn default_home() -> Self {
        directories::ProjectDirs::from_path(PathBuf::from("333")).map_or_else(
            || Self::at(PathBuf::from("333-data")),
            Self::from_project_dirs,
        )
    }

    /// Read the node's home out of a resolved set of project directories.
    fn from_project_dirs(dirs: directories::ProjectDirs) -> Self {
        Self::at(dirs.data_local_dir().to_path_buf())
    }

    /// The directory holding everything this node owns.
    #[must_use]
    pub(crate) fn root(&self) -> &Path {
        &self.root
    }

    /// The directories arti needs, if this build has arti in it.
    ///
    /// They sit under the node's own home rather than arti's default, which is shared
    /// by every arti program on the machine: two nodes started with the defaults would
    /// fight over the same lock files, quietly.
    #[cfg(feature = "tor")]
    #[must_use]
    pub(crate) fn tor(&self) -> n333_net::tor::Paths {
        n333_net::tor::Paths {
            state_dir: self.root.join("tor").join("state"),
            cache_dir: self.root.join("tor").join("cache"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_path_lives_under_the_root() {
        let paths = NodePaths::at(PathBuf::from("/tmp/example"));
        assert!(paths.root().starts_with("/tmp/example"));
    }

    #[cfg(feature = "tor")]
    #[test]
    fn tor_keeps_its_directories_under_the_node_and_apart_from_each_other() {
        // Two nodes sharing either one fail quietly rather than loudly, so the two
        // must never collapse into the same path by accident.
        let paths = NodePaths::at(PathBuf::from("/tmp/example"));
        assert!(paths.tor().state_dir.starts_with("/tmp/example"));
        assert!(paths.tor().cache_dir.starts_with("/tmp/example"));
        assert_ne!(paths.tor().state_dir, paths.tor().cache_dir);
    }

    #[cfg(feature = "tor")]
    #[test]
    fn the_keystore_a_pack_carries_is_the_one_arti_keeps_the_onion_key_in() {
        // arti puts its keystore at `<state_dir>/keystore`. If the state directory
        // moved and this did not, a packed node would arrive without its address.
        let root = PathBuf::from("/tmp/example");
        let paths = NodePaths::at(root.clone());
        let carried = crate::archive::KEYSTORE
            .split('/')
            .fold(root, |path, part| path.join(part));
        assert_eq!(paths.tor().state_dir.join("keystore"), carried);
    }

    #[test]
    fn the_default_home_is_named_after_the_protocol() {
        // Not asserting the whole path: it is different on every system, and a test
        // that pinned it would only be asserting what `directories` already decides.
        let home = NodePaths::default_home();
        assert!(
            home.root().components().any(|c| c.as_os_str() == "333"),
            "{} does not name the protocol",
            home.root().display()
        );
    }
}
