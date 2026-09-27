//! `333 moved` — say that this node's directory was moved here, not copied.
//!
//! A node that opens somewhere other than where it was is told so on every run,
//! because a copy with the original still running is one name in two places and
//! nothing inside the directory can tell a copy from a move (see [`crate::dwelling`]).
//! This is how a person who knows it was a move says so, once: it writes down where
//! the node is now, and the warning stops.

use anyhow::bail;

use crate::commands::Common;
use crate::dwelling::{self, How};
use crate::identity_file;

/// Write down that the node lives where it is now.
///
/// # Errors
/// Fails if there is no node here, it was packed for moving, or the record of where
/// it lives cannot be written.
pub(crate) fn run(common: &Common) -> anyhow::Result<()> {
    let root = common.paths.root();
    if !identity_file::holds_a_name(root) {
        bail!(words!("moved-no-node", root = root.display().to_string()));
    }
    dwelling::refuse_if_packed(root)?;
    let home = identity_file::secure(&common.mistrust(), root)?;
    let now = dwelling::canonical(root)?;
    let shown = now.display().to_string();
    if dwelling::check_here(&home)?.is_none() {
        aloud_in!("moved-already-here", now = shown);
        return Ok(());
    }
    dwelling::record_here(&home, How::Moved, &now)?;
    aloud_in!("moved-here", now = shown);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::words::count::Base;
    use crate::words::layout::{COLUMN, where_the_words_begin};

    const NOW: &str = "/tmp/333-node";

    fn lines() -> Vec<String> {
        vec![
            words!("moved-already-here", now = NOW),
            words!("moved-here", now = NOW),
        ]
    }

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let (said, refused) = crate::words::speaking("en", Base::Ten, || {
            (lines(), words!("moved-no-node", root = NOW))
        });
        assert_eq!(
            said,
            [
                "home     this node already lives at /tmp/333-node, so nothing was changed",
                "home     this node lives at /tmp/333-node, as you say.\n\
                 \x20        If a copy of this directory is still anywhere else, it is this name\n\
                 \x20        too: delete it rather than run it.",
            ]
        );
        assert_eq!(refused, "there is no node in /tmp/333-node to have moved.");
    }

    #[test]
    fn in_korean_every_line_of_moved_begins_its_words_in_the_same_column() {
        for line in crate::words::speaking("ko", Base::Ten, lines) {
            assert!(!line.is_ascii(), "{line:?}");
            assert_eq!(where_the_words_begin(&line), COLUMN, "{line:?}");
        }
    }

    #[test]
    fn saying_it_moved_stops_the_warning_and_nothing_else_changes() {
        let was = std::env::temp_dir().join("n333-moved-test-was");
        let now = std::env::temp_dir().join("n333-moved-test-now");
        for dir in [&was, &now] {
            let _ = std::fs::remove_dir_all(dir);
        }
        let common = |root: &std::path::Path| Common {
            paths: crate::paths::NodePaths::at(root.to_path_buf()),
            timeout: std::time::Duration::from_secs(1),
            keeping: crate::node::Keeping::TheWindow,
            bridges: std::sync::Arc::default(),
            trust_directory_permissions: true,
        };
        let (first, _) =
            identity_file::load_or_create(&common(&was).mistrust(), &was).expect("names");
        std::fs::rename(&was, &now).expect("moves the folder");
        let home = identity_file::secure(&common(&now).mistrust(), &now).expect("is private");
        assert!(dwelling::check_here(&home).expect("checks").is_some());

        run(&common(&now)).expect("says so");
        assert_eq!(dwelling::check_here(&home).expect("checks"), None);
        let (second, _) =
            identity_file::load_or_create(&common(&now).mistrust(), &now).expect("loads");
        assert_eq!(first.node_id(), second.node_id());
        let _ = std::fs::remove_dir_all(&now);
    }
}
