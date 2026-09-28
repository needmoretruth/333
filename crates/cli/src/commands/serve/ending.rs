//! What asks a vigil to end.
//!
//! Ctrl-C at the terminal, and every other way a system says the same thing: a service
//! manager stopping it (`systemctl stop`, `kill`, `timeout` send SIGTERM), a terminal
//! closing under it (SIGHUP), and on Windows Ctrl-Break and the console window being
//! closed. Each ends the vigil the one way it ends: what was asked of the router is
//! given back, the socket other terminals hand orders through is taken away, and the
//! farewell is said. Left to the system's default, all but the first killed the
//! process before any of that happened.

/// Wait until something asks this vigil to end.
///
/// A way of asking that cannot be listened for on this machine is waited on for ever
/// rather than taken as having asked: that would end the vigil the moment it began.
pub(super) async fn asked() {
    tokio::select! {
        () = ctrl_c() => {}
        () = the_system() => {}
    }
}

/// Ctrl-C, where there is a terminal to press it at.
async fn ctrl_c() {
    if tokio::signal::ctrl_c().await.is_err() {
        std::future::pending::<()>().await;
    }
}

/// SIGTERM and SIGHUP.
#[cfg(unix)]
async fn the_system() {
    use tokio::signal::unix::{SignalKind, signal};
    let one = |kind: SignalKind| async move {
        match signal(kind) {
            Ok(mut arriving) => {
                arriving.recv().await;
            }
            Err(_) => std::future::pending::<()>().await,
        }
    };
    tokio::select! {
        () = one(SignalKind::terminate()) => {}
        () = one(SignalKind::hangup()) => {}
    }
}

/// Ctrl-Break, and the console window being closed.
#[cfg(windows)]
async fn the_system() {
    use tokio::signal::windows::{ctrl_break, ctrl_close};
    let broken = async {
        match ctrl_break() {
            Ok(mut arriving) => {
                arriving.recv().await;
            }
            Err(_) => std::future::pending::<()>().await,
        }
    };
    let closed = async {
        match ctrl_close() {
            Ok(mut arriving) => {
                arriving.recv().await;
            }
            Err(_) => std::future::pending::<()>().await,
        }
    };
    tokio::select! {
        () = broken => {}
        () = closed => {}
    }
}

/// Nothing but Ctrl-C, where the system has no other way to ask.
#[cfg(not(any(unix, windows)))]
async fn the_system() {
    std::future::pending::<()>().await;
}
