//! Orders from other terminals on this machine, taken in where the vigil keeps its
//! files.
//!
//! The socket is in the node's directory, set so only its owner can open it, and the
//! vigil also asks the system who is on the other end and refuses anybody who is not
//! that owner — the directory's own permissions can be waived with a flag, and this
//! should not go with them. It is made when the vigil starts and removed when it ends.
//! One left behind by a vigil that was killed is replaced, which is safe to do because
//! this process holds the directory's lock: nobody else can be behind it.
//!
//! What comes in is one line, an order in the screen's own words; what goes out is what
//! the vigil said while carrying it out, and whether it was done. See [`crate::control`]
//! for the format.

#[cfg(unix)]
pub(super) use unix::listen;

/// Say that on this system orders cannot come from another terminal.
#[cfg(not(unix))]
pub(super) fn listen(
    _home: &std::path::Path,
    _asks: tokio::sync::mpsc::UnboundedSender<super::carrying::Ask>,
) -> Option<std::convert::Infallible> {
    aloud_in!("serve-told-not-here");
    None
}

#[cfg(unix)]
mod unix {
    use std::os::unix::fs::{FileTypeExt as _, MetadataExt as _, PermissionsExt as _};
    use std::path::{Path, PathBuf};
    use std::time::Duration;

    use tokio::io::{AsyncBufReadExt as _, AsyncReadExt as _, AsyncWriteExt as _, BufReader};
    use tokio::net::{UnixListener, UnixStream};
    use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
    use tokio::sync::oneshot;

    use crate::control;
    use crate::orders::Order;
    use crate::words::Arg;

    use super::super::carrying::Ask;

    /// How long a terminal that connected has to say what it wants.
    const PATIENCE: Duration = Duration::from_secs(10);

    /// The socket, for as long as the vigil runs. Dropping it takes it away.
    pub(in super::super) struct Told {
        /// Where it is.
        path: PathBuf,
    }

    impl Drop for Told {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.path);
        }
    }

    /// Start taking orders from other terminals, or say why this vigil cannot.
    ///
    /// Never fails the vigil. A node that cannot be told things from elsewhere still
    /// answers everybody who knocks, and that is the part it owes.
    pub(in super::super) fn listen(home: &Path, asks: UnboundedSender<Ask>) -> Option<Told> {
        let path = home.join(control::SOCKET_FILE);
        let owner = std::fs::metadata(home).map(|held| held.uid());
        if !clear_the_way(&path) {
            return None;
        }
        let listener = UnixListener::bind(&path)
            .map_err(|e| aloud_in!("serve-told-cannot", why = e.to_string()))
            .ok()?;
        // From here on dropping this removes the socket, including on the way out below.
        let told = Told { path };
        let private = std::fs::set_permissions(&told.path, std::fs::Permissions::from_mode(0o600));
        let (Ok(owner), Ok(())) = (owner, private) else {
            aloud_in!("serve-told-cannot", why = words!("serve-told-not-private"));
            return None;
        };
        if cfg!(feature = "screen") {
            aloud_in!("serve-told-taking");
        } else {
            aloud_in!("serve-told-taking-light");
        }
        tokio::spawn(take_orders(listener, owner, asks));
        Some(told)
    }

    /// Make room for the socket, replacing one left by a vigil that did not end cleanly.
    fn clear_the_way(path: &Path) -> bool {
        match std::fs::symlink_metadata(path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => true,
            // Whoever made it is gone: this process holds the lock it would be holding.
            Ok(there) if there.file_type().is_socket() => std::fs::remove_file(path)
                .map_err(|e| aloud_in!("serve-told-old-socket", why = e.to_string()))
                .is_ok(),
            Ok(_) => {
                aloud_in!("serve-told-not-a-socket", path = path.display().to_string());
                false
            }
            Err(e) => {
                aloud_in!("serve-told-cannot", why = e.to_string());
                false
            }
        }
    }

    /// Answer every terminal that connects, each on a task of its own.
    async fn take_orders(listener: UnixListener, owner: u32, asks: UnboundedSender<Ask>) {
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    tokio::spawn(answer(stream, owner, asks.clone()));
                }
                Err(e) => {
                    aloud_in!("serve-told-no-longer", why = e.to_string());
                    return;
                }
            }
        }
    }

    /// Read one order, hand it to the carrier, and send back what came of it.
    async fn answer(stream: UnixStream, owner: u32, asks: UnboundedSender<Ask>) {
        let from_the_owner = stream.peer_cred().is_ok_and(|them| them.uid() == owner);
        let (reading, mut writing) = stream.into_split();
        let mut line = String::new();
        let limit = u64::try_from(control::LONGEST_ORDER + 1).unwrap_or(u64::MAX);
        let mut reading = BufReader::new(reading.take(limit));
        match tokio::time::timeout(PATIENCE, reading.read_line(&mut line)).await {
            // Connected and said nothing: somebody looking to see whether a vigil is here.
            Ok(Ok(0)) | Err(_) | Ok(Err(_)) => return,
            Ok(Ok(_)) => {}
        }
        // In the vigil's words: nothing of what they asked has been read yet, including
        // how they read.
        let refusal = if !from_the_owner {
            Some(words!("serve-told-not-the-owner"))
        } else if !line.ends_with('\n') {
            Some(words!(
                "serve-told-too-long",
                bytes = Arg::exact(control::LONGEST_ORDER)
            ))
        } else {
            None
        };
        if let Some(refusal) = refusal {
            return end(&mut writing, &refusal, false).await;
        }
        let (text, words) = match control::order_in(&line) {
            Ok(read) => {
                let words = crate::words::spoken::asked_for(read.language, read.count_in);
                (read.order.trim(), words)
            }
            Err(version) => {
                let why = words!(
                    "serve-told-other-version",
                    ours = control::VERSION,
                    theirs = version.to_string()
                );
                return end(&mut writing, &why, false).await;
            }
        };
        // Read as whoever typed it counts, since what it names is theirs.
        let read = crate::words::spoken::spoken_now(words, || {
            Order::read(text).map_err(|why| words!("serve-told-unread", why = why.to_string()))
        });
        let order = match read {
            Ok(order) => order,
            Err(why) => return end(&mut writing, &why, false).await,
        };
        // In the vigil only, the way the screen echoes what was typed into it; and only
        // for an order that changes something. `333 status` run every few seconds from
        // another terminal would otherwise fill the vigil's log with its own asking.
        if !matches!(order, Order::Status(_)) {
            aloud_in!("serve-told-asked", order = text);
        }
        let (lines, mut heard) = unbounded_channel();
        let (done, mut finished) = oneshot::channel();
        let ask = Ask {
            order,
            words,
            lines,
            done,
        };
        if asks.send(ask).is_err() {
            let why = crate::words::spoken::spoken_now(words, || words!("serve-told-unheard"));
            return end(&mut writing, &why, false).await;
        }
        loop {
            tokio::select! {
                biased;
                Some(said) = heard.recv() => {
                    if writing.write_all(control::said(&said).as_bytes()).await.is_err() {
                        // They stopped waiting. The order goes on without them.
                        return;
                    }
                }
                ended = &mut finished => {
                    let mut rest = String::new();
                    while let Ok(said) = heard.try_recv() {
                        rest.push_str(&control::said(&said));
                    }
                    rest.push_str(&control::ended(ended.unwrap_or(false)));
                    let _ = writing.write_all(rest.as_bytes()).await;
                    return;
                }
            }
        }
    }

    /// Send one line and the end.
    async fn end(writing: &mut tokio::net::unix::OwnedWriteHalf, said: &str, done: bool) {
        let reply = format!("{}{}", control::said(said), control::ended(done));
        let _ = writing.write_all(reply.as_bytes()).await;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn the_socket_is_private_and_goes_when_the_vigil_does() {
            let home = std::env::temp_dir().join("n333-told-test-private");
            let _ = std::fs::remove_dir_all(&home);
            std::fs::create_dir_all(&home).expect("creates");
            let (asks, _carried) = unbounded_channel();
            let told = listen(&home, asks).expect("listens");
            let socket = home.join(control::SOCKET_FILE);
            let mode = std::fs::metadata(&socket)
                .expect("stats")
                .permissions()
                .mode();
            assert_eq!(mode & 0o777, 0o600);
            drop(told);
            assert!(
                !socket.exists(),
                "a vigil that ended leaves no socket behind"
            );
            let _ = std::fs::remove_dir_all(&home);
        }

        #[tokio::test]
        async fn a_socket_left_behind_is_replaced_and_anything_else_is_left_alone() {
            let home = std::env::temp_dir().join("n333-told-test-stale");
            let _ = std::fs::remove_dir_all(&home);
            std::fs::create_dir_all(&home).expect("creates");
            let socket = home.join(control::SOCKET_FILE);
            // What a killed vigil leaves: the socket file, and nobody listening on it.
            drop(std::os::unix::net::UnixListener::bind(&socket).expect("binds"));
            let (asks, _carried) = unbounded_channel();
            assert!(
                listen(&home, asks.clone()).is_some(),
                "the stale one is replaced"
            );

            std::fs::write(&socket, b"somebody's file").expect("writes");
            assert!(listen(&home, asks).is_none());
            assert_eq!(std::fs::read(&socket).expect("reads"), b"somebody's file");
            let _ = std::fs::remove_dir_all(&home);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::words::Arg;

    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let pairs = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            [
                (
                    words!("serve-told-not-here"),
                    "orders   on this system they reach this node through its screen and nowhere\n\
                     \x20        else. A second 333 started beside it is refused.",
                ),
                (
                    words!("serve-told-cannot", why = "Permission denied (os error 13)"),
                    "orders   cannot be taken from other terminals: Permission denied (os error 13)",
                ),
                (
                    words!("serve-told-cannot", why = words!("serve-told-not-private")),
                    "orders   cannot be taken from other terminals: the socket could not be made private",
                ),
                (
                    words!("serve-told-taking"),
                    "orders   from any terminal on this machine, in the screen's words:\n\
                     \x20        `333 say 7`, `333 join <invitation>`, `333 tell 'tor on'`, `333 stop`.\n\
                     \x20        This node carries them out and answers there.",
                ),
                (
                    words!("serve-told-old-socket", why = "busy"),
                    "orders   an old socket is in the way and stays there: busy",
                ),
                (
                    words!("serve-told-not-a-socket", path = "/n/333.sock"),
                    "orders   /n/333.sock is there and is not a socket, so it is left alone, and\n\
                     \x20        nothing can be handed to this node from another terminal until\n\
                     \x20        it is moved.",
                ),
                (
                    words!("serve-told-no-longer", why = "busy"),
                    "orders   are no longer taken from other terminals: busy",
                ),
                (
                    words!("serve-told-not-the-owner"),
                    "refused  only whoever owns this node's directory can tell it anything",
                ),
                (
                    words!("serve-told-too-long", bytes = Arg::exact(4096)),
                    "unread   that is longer than any order. The longest is 4096 bytes.",
                ),
                (
                    words!("serve-told-other-version", ours = "333/1", theirs = "333/2"),
                    "refused  this node speaks 333/1 and was asked in 333/2. The 333 that asked\n\
                     \x20        is a different version from the one running; run that one.",
                ),
                (
                    words!("serve-told-unread", why = "nothing to say"),
                    "unread   nothing to say",
                ),
                (
                    words!("serve-told-asked", order = "say 7"),
                    "asked    say 7, from another terminal",
                ),
                (
                    words!("serve-told-unheard"),
                    "unheard  nothing is carrying orders out any more",
                ),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }
}
