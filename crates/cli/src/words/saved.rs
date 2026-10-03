//! The language a person saved for a node with `333 language <TAG>`.
//!
//! In the node's own directory rather than beside the program, so that two nodes on one
//! machine can speak two languages, and so that the node a service manager starts reads
//! the same choice the person made at a terminal. A flag written into the service's
//! definition would do that too, and would go on speaking the old language after the
//! person changed it.

use std::path::Path;

/// The file, inside the node's directory: the tag on one line.
pub(crate) const FILE: &str = "language";

/// The tag saved for the node at `root`, if one was.
#[must_use]
pub(crate) fn read(root: &Path) -> Option<String> {
    let text = std::fs::read_to_string(root.join(FILE)).ok()?;
    let tag = text.trim();
    (!tag.is_empty()).then(|| tag.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_saved_tag_is_read_back_and_an_empty_file_is_none() {
        let root = std::env::temp_dir().join(format!("333-saved-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        assert_eq!(read(&root), None);
        std::fs::write(root.join(FILE), "ko\n").unwrap();
        assert_eq!(read(&root).as_deref(), Some("ko"));
        std::fs::write(root.join(FILE), "  \n").unwrap();
        assert_eq!(read(&root), None);
        std::fs::remove_dir_all(&root).unwrap();
    }
}
