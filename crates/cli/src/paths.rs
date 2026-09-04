//! Where this node keeps its files.
//!
//! One directory holds everything: the identity key and the two directories arti
//! wants. Keeping them together means a node can be moved, backed up or thrown away
//! as a unit, and means two nodes on one machine are two directories rather than a
//! configuration puzzle.

use std::path::{Path, PathBuf};

use n333_net::tor;

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

    /// The default home: `$XDG_DATA_HOME/333`, or `~/.local/share/333`.
    ///
    /// Falls back to `./333-data` when neither variable is set, which is better than
    /// refusing to start and better than writing somewhere surprising.
    #[must_use]
    pub(crate) fn default_home() -> Self {
        let base = std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME").map(|home| Path::new(&home).join(".local").join("share"))
            })
            .unwrap_or_else(|| PathBuf::from("."));
        Self::at(base.join("333"))
    }

    /// The file holding this node's identity seed.
    #[must_use]
    pub(crate) fn identity_file(&self) -> PathBuf {
        self.root.join("identity.key")
    }

    /// The directories arti needs.
    #[must_use]
    pub(crate) fn tor(&self) -> tor::Paths {
        tor::Paths {
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
        assert!(paths.identity_file().starts_with("/tmp/example"));
        assert!(paths.tor().state_dir.starts_with("/tmp/example"));
        assert!(paths.tor().cache_dir.starts_with("/tmp/example"));
    }

    #[test]
    fn tor_state_and_cache_are_different_directories() {
        // Two nodes sharing either one fail quietly rather than loudly, so the two
        // must never collapse into the same path by accident.
        let paths = NodePaths::at(PathBuf::from("/tmp/example"));
        assert_ne!(paths.tor().state_dir, paths.tor().cache_dir);
    }
}
