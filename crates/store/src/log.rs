//! An append-only file of signed frames.
//!
//! The same four-byte big-endian length prefix the wire uses, so a record on disk and
//! a record in flight are the same bytes and nothing has to be re-encoded to be
//! stored, sent, or checked.
//!
//! WHY THERE IS NO CHECKSUM AND NO DATABASE. Every record here already carries an
//! Ed25519 signature over its own bytes, so damage is detected by the check that has
//! to happen anyway. A database would add a format that has to keep being readable
//! for as long as the network lives, a dependency that has to keep compiling for that
//! long, and a repair path for a corruption a signature already catches. What is here
//! instead is a file of frames and one rule for reading it.
//!
//! A TORN TAIL IS EXPECTED, NOT AN ERROR. Power fails mid-write. Opening walks back
//! from the end until what remains is a whole record, truncates there, and says how
//! many bytes it dropped — a partial record is a record that was never finished, and
//! keeping it would leave the file unreadable for ever.

use std::fs::{File, OpenOptions};
use std::io::{Read as _, Seek as _, SeekFrom, Write as _};
use std::path::{Path, PathBuf};

/// Length of the size prefix. The same one the wire uses.
pub const LENGTH_PREFIX_LEN: usize = 4;

/// The largest record this will read back.
///
/// Deliberately larger than the wire's frame limit: a record kept on disk may one day
/// hold more than a frame does, and a reader that refused would make the file
/// unreadable rather than the writer wrong. It exists so that a corrupted length
/// cannot ask for an unbounded allocation.
pub const MAX_RECORD_LEN: usize = 1 << 20;

/// Things that can go wrong reading or writing a log.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The file could not be read, written or created.
    #[error("{path}: {source}")]
    Io {
        /// Which file.
        path: PathBuf,
        /// What happened.
        source: std::io::Error,
    },
    /// A length prefix asks for more than this will read.
    ///
    /// Not a torn tail — a torn tail is short, and this is a length that could only
    /// come from damage in the middle of the file.
    #[error("{path}: a record announces {got} bytes, over the {MAX_RECORD_LEN}-byte limit")]
    RecordTooLong {
        /// Which file.
        path: PathBuf,
        /// The announced size.
        got: usize,
    },
    /// A record was handed in that is larger than this will store.
    #[error("a record of {got} bytes is over the {MAX_RECORD_LEN}-byte limit")]
    TooLongToWrite {
        /// The size handed in.
        got: usize,
    },
}

/// An append-only file of records.
#[derive(Debug)]
pub struct Log {
    /// The open file, positioned at the end.
    file: File,
    /// Kept for error messages: a failure that does not say which file is a failure
    /// somebody has to guess at.
    path: PathBuf,
    /// How many records the file holds.
    length: u64,
}

/// What opening a log found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Opened {
    /// How many whole records the file holds.
    pub records: u64,
    /// How many bytes were dropped from the end because they were not a whole record.
    ///
    /// Anything but zero means the process died mid-write. Worth showing once rather
    /// than hiding: it is the only sign a node gets that it lost something.
    pub truncated: u64,
}

impl Log {
    /// Open a log, creating it if it is not there, and repair a torn tail.
    ///
    /// # Errors
    /// Fails if the file cannot be opened or read, or if a length prefix in the
    /// middle of the file is impossible.
    pub fn open(path: &Path) -> Result<(Self, Opened), Error> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| Error::Io {
                path: parent.to_path_buf(),
                source,
            })?;
        }
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(path)
            .map_err(|source| Error::Io {
                path: path.to_path_buf(),
                source,
            })?;

        let (records, whole) = Self::scan(&mut file, path)?;
        let size = Self::size(&file, path)?;
        let truncated = size - whole;
        if truncated != 0 {
            file.set_len(whole).map_err(|source| Error::Io {
                path: path.to_path_buf(),
                source,
            })?;
        }
        Ok((
            Self {
                file,
                path: path.to_path_buf(),
                length: records,
            },
            Opened { records, truncated },
        ))
    }

    /// Walk the file, counting whole records and finding where the last one ends.
    fn scan(file: &mut File, path: &Path) -> Result<(u64, u64), Error> {
        let size = Self::size(file, path)?;
        file.seek(SeekFrom::Start(0)).map_err(|source| Error::Io {
            path: path.to_path_buf(),
            source,
        })?;

        let (mut records, mut whole) = (0_u64, 0_u64);
        let mut prefix = [0_u8; LENGTH_PREFIX_LEN];
        loop {
            if size - whole < LENGTH_PREFIX_LEN as u64 {
                return Ok((records, whole));
            }
            file.read_exact(&mut prefix).map_err(|source| Error::Io {
                path: path.to_path_buf(),
                source,
            })?;
            let announced = u32::from_be_bytes(prefix) as u64;
            if announced > MAX_RECORD_LEN as u64 {
                return Err(Error::RecordTooLong {
                    path: path.to_path_buf(),
                    got: announced as usize,
                });
            }
            let ends_at = whole + LENGTH_PREFIX_LEN as u64 + announced;
            if ends_at > size {
                // The prefix arrived but the body did not. Everything from here on is
                // the torn tail.
                return Ok((records, whole));
            }
            file.seek(SeekFrom::Start(ends_at))
                .map_err(|source| Error::Io {
                    path: path.to_path_buf(),
                    source,
                })?;
            records += 1;
            whole = ends_at;
        }
    }

    /// How many bytes the file holds.
    fn size(file: &File, path: &Path) -> Result<u64, Error> {
        Ok(file
            .metadata()
            .map_err(|source| Error::Io {
                path: path.to_path_buf(),
                source,
            })?
            .len())
    }

    /// How many records the log holds.
    #[must_use]
    pub const fn len(&self) -> u64 {
        self.length
    }

    /// Is the log empty?
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Append one record and make it durable before returning.
    ///
    /// The sync is not optional and is not batched. This log holds the entries that
    /// are a node's whole history; one of them is written every 333 minutes, so the
    /// cost is one flush per five and a half hours, and losing one to a power cut
    /// would mean answering later with a chain head nobody else ever saw.
    ///
    /// # Errors
    /// Fails if the record is too long, or the file cannot be written or synced.
    pub fn append(&mut self, record: &[u8]) -> Result<(), Error> {
        if record.len() > MAX_RECORD_LEN {
            return Err(Error::TooLongToWrite { got: record.len() });
        }
        let length =
            u32::try_from(record.len()).map_err(|_| Error::TooLongToWrite { got: record.len() })?;
        let mut framed = Vec::with_capacity(LENGTH_PREFIX_LEN + record.len());
        framed.extend_from_slice(&length.to_be_bytes());
        framed.extend_from_slice(record);
        // One write call, so a crash can tear the record but cannot interleave it
        // with the next one.
        self.file.write_all(&framed).map_err(|source| Error::Io {
            path: self.path.clone(),
            source,
        })?;
        self.file.sync_data().map_err(|source| Error::Io {
            path: self.path.clone(),
            source,
        })?;
        self.length += 1;
        Ok(())
    }

    /// Read every record, in the order they were written.
    ///
    /// # Errors
    /// Fails if the file cannot be read, or holds a length that is impossible.
    pub fn read_all(&mut self) -> Result<Vec<Vec<u8>>, Error> {
        self.file
            .seek(SeekFrom::Start(0))
            .map_err(|source| Error::Io {
                path: self.path.clone(),
                source,
            })?;
        let mut bytes = Vec::new();
        self.file
            .read_to_end(&mut bytes)
            .map_err(|source| Error::Io {
                path: self.path.clone(),
                source,
            })?;
        self.file
            .seek(SeekFrom::End(0))
            .map_err(|source| Error::Io {
                path: self.path.clone(),
                source,
            })?;

        let mut records = Vec::with_capacity(self.length as usize);
        let mut rest = bytes.as_slice();
        loop {
            let Some((prefix, body)) = rest.split_at_checked(LENGTH_PREFIX_LEN) else {
                // Fewer bytes than a prefix: the torn tail, already truncated on open
                // and possible again only if something else is writing to the file.
                return Ok(records);
            };
            let announced =
                u32::from_be_bytes(prefix.try_into().map_err(|_| Error::RecordTooLong {
                    path: self.path.clone(),
                    got: 0,
                })?) as usize;
            if announced > MAX_RECORD_LEN {
                return Err(Error::RecordTooLong {
                    path: self.path.clone(),
                    got: announced,
                });
            }
            let Some((record, after)) = body.split_at_checked(announced) else {
                return Ok(records);
            };
            records.push(record.to_vec());
            rest = after;
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::expect_used,
        clippy::unwrap_used,
        clippy::panic,
        clippy::indexing_slicing
    )]

    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("n333-log-test-{name}"));
        let _ = std::fs::remove_dir_all(&dir);
        dir.join("records.log")
    }

    #[test]
    fn what_goes_in_comes_back_in_order() {
        let path = scratch("roundtrip");
        let (mut log, opened) = Log::open(&path).expect("opens");
        assert_eq!(
            opened,
            Opened {
                records: 0,
                truncated: 0
            }
        );
        assert!(log.is_empty());

        for record in [b"one".as_slice(), b"two", b"", b"four"] {
            log.append(record).expect("appends");
        }
        assert_eq!(log.len(), 4);
        assert_eq!(
            log.read_all().expect("reads"),
            vec![
                b"one".to_vec(),
                b"two".to_vec(),
                Vec::new(),
                b"four".to_vec()
            ]
        );
        let _ = std::fs::remove_dir_all(path.parent().expect("has a parent"));
    }

    #[test]
    fn a_log_survives_being_closed_and_opened_again() {
        let path = scratch("reopen");
        {
            let (mut log, _) = Log::open(&path).expect("opens");
            log.append(b"kept").expect("appends");
        }
        let (mut log, opened) = Log::open(&path).expect("opens");
        assert_eq!(
            opened,
            Opened {
                records: 1,
                truncated: 0
            }
        );
        assert_eq!(log.read_all().expect("reads"), vec![b"kept".to_vec()]);
        log.append(b"and more").expect("appends");
        assert_eq!(log.len(), 2);
        let _ = std::fs::remove_dir_all(path.parent().expect("has a parent"));
    }

    #[test]
    fn a_record_torn_by_a_power_cut_is_dropped_and_reported() {
        // The case the scan exists for: the length prefix reached the disk and the
        // body did not. Anything else would leave the file unreadable for ever.
        let path = scratch("torn-body");
        {
            let (mut log, _) = Log::open(&path).expect("opens");
            log.append(b"whole").expect("appends");
        }
        {
            let mut file = OpenOptions::new().append(true).open(&path).expect("opens");
            file.write_all(&7_u32.to_be_bytes()).expect("writes");
            file.write_all(b"abc").expect("writes only part of it");
        }
        let (mut log, opened) = Log::open(&path).expect("opens");
        assert_eq!(
            opened,
            Opened {
                records: 1,
                truncated: 7
            }
        );
        assert_eq!(log.read_all().expect("reads"), vec![b"whole".to_vec()]);

        // ...and the file is usable again straight away.
        log.append(b"after").expect("appends");
        assert_eq!(log.len(), 2);
        let _ = std::fs::remove_dir_all(path.parent().expect("has a parent"));
    }

    #[test]
    fn a_prefix_torn_in_half_is_dropped_too() {
        let path = scratch("torn-prefix");
        {
            let (mut log, _) = Log::open(&path).expect("opens");
            log.append(b"whole").expect("appends");
        }
        {
            let mut file = OpenOptions::new().append(true).open(&path).expect("opens");
            file.write_all(&[0, 0]).expect("writes half a prefix");
        }
        let (mut log, opened) = Log::open(&path).expect("opens");
        assert_eq!(
            opened,
            Opened {
                records: 1,
                truncated: 2
            }
        );
        assert_eq!(log.read_all().expect("reads"), vec![b"whole".to_vec()]);
        let _ = std::fs::remove_dir_all(path.parent().expect("has a parent"));
    }

    #[test]
    fn a_damaged_length_in_the_middle_is_an_error_rather_than_a_huge_allocation() {
        let path = scratch("damaged");
        {
            let (mut log, _) = Log::open(&path).expect("opens");
            log.append(b"whole").expect("appends");
        }
        {
            let mut file = OpenOptions::new().write(true).open(&path).expect("opens");
            file.seek(SeekFrom::Start(0)).expect("seeks");
            file.write_all(&u32::MAX.to_be_bytes()).expect("writes");
        }
        assert!(matches!(Log::open(&path), Err(Error::RecordTooLong { .. })));
        let _ = std::fs::remove_dir_all(path.parent().expect("has a parent"));
    }

    #[test]
    fn a_record_too_large_to_store_is_refused_before_anything_is_written() {
        let path = scratch("too-long");
        let (mut log, _) = Log::open(&path).expect("opens");
        let huge = vec![0_u8; MAX_RECORD_LEN + 1];
        assert!(matches!(
            log.append(&huge),
            Err(Error::TooLongToWrite { .. })
        ));
        assert!(log.is_empty(), "nothing should have been written");
        let _ = std::fs::remove_dir_all(path.parent().expect("has a parent"));
    }

    #[test]
    fn an_error_says_which_file_it_was() {
        // A failure that does not name the file is a failure somebody has to guess at,
        // and a node keeps several of these.
        let path = scratch("named");
        {
            let (mut log, _) = Log::open(&path).expect("opens");
            log.append(b"x").expect("appends");
        }
        {
            let mut file = OpenOptions::new().write(true).open(&path).expect("opens");
            file.seek(SeekFrom::Start(0)).expect("seeks");
            file.write_all(&u32::MAX.to_be_bytes()).expect("writes");
        }
        let failure = Log::open(&path).expect_err("fails").to_string();
        assert!(failure.contains("named"), "{failure}");
        let _ = std::fs::remove_dir_all(path.parent().expect("has a parent"));
    }
}
