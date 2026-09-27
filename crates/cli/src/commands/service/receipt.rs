//! What `333 service install` did, written down so that uninstalling can undo exactly it.
//!
//! One small text file in this user's configuration directory, not in the node's own
//! directory: a service manager keeps one vigil per user under one name, so what was
//! installed belongs to the user, and the file has to be findable by `uninstall` and
//! `status` whichever `--data-dir` they were run with.
//!
//! It names the node directory the service keeps, every file install wrote, every
//! scheduled task it created, where the service's log is, and whether install was the
//! thing that turned systemd's linger on — so that uninstall turns it off again only
//! if it was.

use std::path::{Path, PathBuf};

/// The file's name inside the configuration directory.
const FILE: &str = "service";

/// Everything one install changed on this machine.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Receipt {
    /// The node directory the service keeps the vigil for.
    pub(crate) node: PathBuf,
    /// Every file install wrote, in the order it wrote them.
    pub(crate) wrote: Vec<PathBuf>,
    /// Every scheduled task install created, by name.
    pub(crate) tasks: Vec<String>,
    /// Where the service's lines go, when the service manager does not keep them itself.
    pub(crate) log: Option<PathBuf>,
    /// Whether install turned systemd's linger on for this user.
    pub(crate) linger_turned_on: bool,
}

/// Where the receipt lives for this user, if the system has a configuration directory.
#[must_use]
pub(crate) fn path() -> Option<PathBuf> {
    directories::ProjectDirs::from_path(PathBuf::from("333"))
        .map(|dirs| dirs.config_dir().join(FILE))
}

/// Read this user's receipt, if a service is installed.
#[must_use]
pub(crate) fn read() -> Option<Receipt> {
    path().and_then(|path| read_from(&path))
}

/// Read a receipt from one file.
#[must_use]
pub(crate) fn read_from(path: &Path) -> Option<Receipt> {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|text| Receipt::from_text(&text))
}

/// Whether a service is installed that keeps the vigil for this node directory.
#[must_use]
pub(crate) fn keeps(root: &Path) -> bool {
    read().is_some_and(|receipt| same_directory(&receipt.node, root))
}

/// Two ways of naming a directory, compared as the directory they name.
fn same_directory(one: &Path, other: &Path) -> bool {
    let absolute = |path: &Path| std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());
    absolute(one) == absolute(other)
}

impl Receipt {
    /// The receipt as the lines it is kept in: one fact a line, a word and a value.
    #[must_use]
    pub(crate) fn to_text(&self) -> String {
        let mut text = String::from(
            "# Written by `333 service install`, and read by `333 service uninstall` to undo it.\n",
        );
        text.push_str(&format!("node {}\n", self.node.display()));
        for wrote in &self.wrote {
            text.push_str(&format!("wrote {}\n", wrote.display()));
        }
        for task in &self.tasks {
            text.push_str(&format!("task {task}\n"));
        }
        if let Some(log) = &self.log {
            text.push_str(&format!("log {}\n", log.display()));
        }
        if self.linger_turned_on {
            text.push_str("linger turned on\n");
        }
        text
    }

    /// Read the lines back. A receipt without a node in it is not a receipt.
    #[must_use]
    pub(crate) fn from_text(text: &str) -> Option<Self> {
        let mut receipt = Self::default();
        let mut named_a_node = false;
        for line in text.lines().filter(|line| !line.starts_with('#')) {
            let Some((word, value)) = line.split_once(' ') else {
                continue;
            };
            match word {
                "node" => {
                    receipt.node = PathBuf::from(value);
                    named_a_node = true;
                }
                "wrote" => receipt.wrote.push(PathBuf::from(value)),
                "task" => receipt.tasks.push(value.to_owned()),
                "log" => receipt.log = Some(PathBuf::from(value)),
                "linger" => receipt.linger_turned_on = value == "turned on",
                _ => {}
            }
        }
        named_a_node.then_some(receipt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_receipt_reads_back_as_everything_it_recorded() {
        let receipt = Receipt {
            node: PathBuf::from("/home/someone/.local/share/333"),
            wrote: vec![
                PathBuf::from("/home/someone/.config/systemd/user/333.service"),
                PathBuf::from("/home/someone/.config/systemd/user/333 check.timer"),
            ],
            tasks: vec!["333 vigil".to_owned(), "333 check".to_owned()],
            log: Some(PathBuf::from("/home/someone/Library/Logs/333.log")),
            linger_turned_on: true,
        };
        assert_eq!(Receipt::from_text(&receipt.to_text()), Some(receipt));
    }

    #[test]
    fn linger_is_turned_off_again_only_when_the_receipt_says_install_turned_it_on() {
        let receipt = Receipt::from_text("node /x\n").unwrap();
        assert!(!receipt.linger_turned_on);
    }

    #[test]
    fn a_file_naming_no_node_is_not_a_receipt() {
        assert_eq!(Receipt::from_text("wrote /x\n"), None);
    }

    #[test]
    fn a_directory_is_the_same_directory_however_it_was_typed() {
        assert!(same_directory(Path::new("/a/b"), Path::new("/a/b/")));
        assert!(!same_directory(Path::new("/a/b"), Path::new("/a/c")));
    }
}
