//! Writing a file so that a reader finds the old one or the new one and never half.
//!
//! The board and the observation are read by another process (or by this one after a
//! restart) at any moment. Written in place, a crash or a reader arriving mid-write sees
//! a torn file; written beside and renamed, it sees one whole file or the other. The
//! data is synced before the rename and the directory after it, so a power cut cannot
//! leave the name pointing at a file whose contents never reached the disk.

use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::io::{self, Write as _};
use std::path::Path;

/// Replace `path` with `bytes`, readable as `mode` says (on Unix).
///
/// # Errors
/// Fails if the temporary file cannot be written, synced or renamed into place. The
/// temporary file is removed on failure and `path` is left as it was.
pub(crate) fn write(path: &Path, bytes: &[u8], mode: u32) -> io::Result<()> {
    let dir = match path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent,
        _ => Path::new("."),
    };
    let Some(name) = path.file_name() else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "a file to write needs a name",
        ));
    };
    // Hidden and named after the process, so two writers never share one and a listing
    // of the directory does not show it as a page.
    let mut temporary = OsString::from(".");
    temporary.push(name);
    temporary.push(format!(".{}.tmp", std::process::id()));
    let temporary = dir.join(temporary);

    let written = (|| {
        let mut file = create(&temporary, mode)?;
        file.write_all(bytes)?;
        file.sync_all()?;
        std::fs::rename(&temporary, path)?;
        sync_directory(dir)
    })();
    if written.is_err() {
        // Best effort: the error that matters is the one being returned.
        let _ = std::fs::remove_file(&temporary);
    }
    written
}

/// Open the temporary file with exactly `mode`, whatever the umask or a leftover says.
#[cfg(unix)]
fn create(path: &Path, mode: u32) -> io::Result<File> {
    use std::os::unix::fs::{OpenOptionsExt as _, PermissionsExt as _};
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(mode)
        .open(path)?;
    file.set_permissions(std::fs::Permissions::from_mode(mode))?;
    Ok(file)
}

/// Open the temporary file. Modes are a Unix idea; elsewhere the default stands.
#[cfg(not(unix))]
fn create(path: &Path, _mode: u32) -> io::Result<File> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
}

/// Make the rename itself durable.
#[cfg(unix)]
fn sync_directory(dir: &Path) -> io::Result<()> {
    File::open(dir)?.sync_all()
}

/// Directories cannot be opened to sync on every system; the rename is still atomic.
#[cfg(not(unix))]
fn sync_directory(_dir: &Path) -> io::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn a_written_file_has_its_bytes_its_mode_and_no_leftover_beside_it() {
        use std::os::unix::fs::PermissionsExt as _;
        let dir = std::env::temp_dir().join(format!("n333-site-atomic-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("board.json");
        write(&path, b"old", 0o644).unwrap();
        write(&path, b"new", 0o600).unwrap();

        assert_eq!(std::fs::read(&path).unwrap(), b"new");
        let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
