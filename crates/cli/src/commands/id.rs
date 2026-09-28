//! `333 id` — show this node's identity, creating one on first run.

use std::path::Path;

use crate::commands::Common;
use crate::identity_file::{self, Origin};

/// Print this node's name, how it came to have one, and when.
///
/// # Errors
/// Fails if the identity file cannot be read or written, or the node was packed for
/// moving.
pub(crate) fn run(common: &Common) -> anyhow::Result<()> {
    let home = common.paths.root();
    let (identity, origin) = identity_file::load_or_create(&common.mistrust(), home)?;
    // Out loud, like every other command, so that a reader who walks away after the
    // first lines (`333 id | head -2`) ends the output rather than the program.
    for line in said(&identity.node_id().to_string(), origin, home) {
        crate::aloud::line(&line);
    }
    crate::named::report(origin, home);
    crate::aloud::line(&crate::began::describe(crate::began::read(home).as_ref()));
    Ok(())
}

/// Say the name and where it lives, which is all there is to say while a vigil holds
/// the directory: the run that made the name is long past.
pub(crate) fn beside_the_vigil(name: &str, home: &Path) {
    for line in said(name, Origin::Loaded, home) {
        crate::aloud::line(&line);
    }
}

/// The lines `333 id` says before the naming, in the order it says them.
fn said(name: &str, origin: Origin, home: &Path) -> Vec<String> {
    let mut lines = vec![words!("id-name", name = name)];
    // The naming says where home is, once, beside the warning to keep it.
    if matches!(origin, Origin::Loaded) {
        lines.push(words!("id-home", home = home.display().to_string()));
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::words::count::Base;
    use crate::words::layout::{COLUMN, where_the_words_begin};

    const NAME: &str = "333ac0bdd148d7f9a783194ee6a9102c2e53b1227a8a41e7621f133fdba16cd4";

    fn a_later_run() -> Vec<String> {
        said(NAME, Origin::Loaded, Path::new("/tmp/333-node"))
    }

    #[test]
    fn in_english_a_later_run_says_what_it_said_before_the_words_were_moved() {
        let lines = crate::words::speaking("en", Base::Ten, a_later_run);
        assert_eq!(
            lines,
            [
                format!("name     {NAME}"),
                "home     /tmp/333-node".to_owned()
            ]
        );
    }

    #[test]
    fn the_run_that_makes_the_name_leaves_home_to_the_naming() {
        let lines = said(
            NAME,
            Origin::Created { not_called: 1 },
            Path::new("/tmp/333-node"),
        );
        assert_eq!(lines.len(), 1, "{lines:?}");
    }

    #[test]
    fn in_korean_every_line_of_id_begins_its_words_in_the_same_column() {
        let lines = crate::words::speaking("ko", Base::Ten, a_later_run);
        assert!(lines[0].starts_with("이름 "), "{lines:?}");
        for line in &lines {
            assert_eq!(where_the_words_begin(line), COLUMN, "{line:?}");
        }
    }
}
