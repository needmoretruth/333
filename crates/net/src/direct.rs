//! Reaching a peer by opening a socket to it. The ordinary case.
//!
//! There is no library here beyond Tokio's own sockets. A node contacts one peer at a
//! time, exchanges a few hundred bytes and closes; the things a transport library
//! would add — hole punching, relays, dial-by-key, multiplexing — solve problems this
//! protocol does not have, and each of them is a dependency that has to keep working
//! for as long as the network does.
//!
//! Nothing in this module hides anything. A peer reached this way learns the address
//! it was reached from, and so does anyone watching the wire. That is the trade the
//! address itself declares: [`crate::peer`] explains why the choice is not a flag.

use std::net::SocketAddr;

use tokio::net::{TcpListener, TcpStream};
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt as _};

use crate::peer::PeerAddress;

/// A byte stream to a peer reached directly.
///
/// Tokio's socket wrapped so that it reads and writes through the same traits arti's
/// streams do, which is what lets one exchange be written once for both.
pub type Stream = Compat<TcpStream>;

/// Things that can go wrong reaching a peer directly.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The socket refused, timed out, or the name did not resolve.
    ///
    /// The reason is in the message and deliberately not behind `source()`. This
    /// wrapper adds nothing to what the operating system already said, and an error
    /// that both says a sentence and hands the same sentence to whoever walks the
    /// chain gets printed twice — which is what an operator sees on the one line they
    /// read most often.
    #[error("{cause}")]
    Io {
        /// What the operating system said.
        cause: std::io::Error,
    },
    /// An onion address was handed to the direct transport.
    ///
    /// Not an oversight to be worked around: onion addresses have no meaning outside
    /// Tor, and resolving one as a hostname would leak the lookup to a resolver.
    #[error("{0} can only be reached through Tor")]
    NeedsTor(String),
}

impl From<std::io::Error> for Error {
    fn from(cause: std::io::Error) -> Self {
        Self::Io { cause }
    }
}

/// Open a stream to a peer.
///
/// The socket is opened with Nagle's algorithm off. An exchange is one small write,
/// one read and one more small write, which is the exact shape Nagle delays: the
/// second write would sit in the kernel waiting for an acknowledgement that the
/// answer is itself waiting for.
///
/// # Errors
/// Fails if the address needs Tor, if the name does not resolve, or if the peer does
/// not accept the connection.
pub async fn connect(address: &PeerAddress) -> Result<Stream, Error> {
    if address.needs_tor() {
        return Err(Error::NeedsTor(address.to_string()));
    }
    let socket = TcpStream::connect((address.host(), address.port())).await?;
    socket.set_nodelay(true)?;
    Ok(socket.compat())
}

/// A socket this node answers on.
pub struct Listener {
    /// The bound socket.
    inner: TcpListener,
}

impl Listener {
    /// Bind and start listening.
    ///
    /// # Errors
    /// Fails if the address cannot be bound, usually because something else holds
    /// the port or the address does not belong to this machine.
    pub async fn bind(bind_address: SocketAddr) -> Result<Self, Error> {
        Ok(Self {
            inner: TcpListener::bind(bind_address).await?,
        })
    }

    /// The address actually bound, which is what to tell peers.
    ///
    /// Worth asking for even when the address was chosen by the caller: binding port
    /// 0 asks the operating system to pick one, and this is the only way to learn
    /// which.
    ///
    /// # Errors
    /// Fails only if the socket has been closed underneath this object.
    pub fn address(&self) -> Result<SocketAddr, Error> {
        Ok(self.inner.local_addr()?)
    }

    /// Wait for the next peer.
    ///
    /// # Errors
    /// Fails if the socket itself fails. A connection that dies between arriving and
    /// being accepted is reported here too, and a caller should keep listening rather
    /// than stop.
    pub async fn accept(&self) -> Result<(Stream, SocketAddr), Error> {
        let (socket, from) = self.inner.accept().await?;
        socket.set_nodelay(true)?;
        Ok((socket.compat(), from))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    fn loopback(port: u16) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port)
    }

    #[tokio::test]
    async fn a_node_can_be_reached_at_the_port_it_bound() {
        let listener = Listener::bind(loopback(0)).await.expect("binds");
        let bound = listener.address().expect("has an address");
        assert_ne!(bound.port(), 0, "port 0 must be resolved to a real port");

        let dialled = format!("127.0.0.1:{}", bound.port());
        let address: PeerAddress = dialled.parse().expect("a readable address");
        let (accepted, connected) =
            tokio::join!(listener.accept(), connect(&address));
        let (_stream, from) = accepted.expect("accepts");
        connected.expect("connects");
        assert_eq!(from.ip(), bound.ip());
    }

    #[tokio::test]
    async fn an_onion_address_is_refused_here_rather_than_resolved() {
        // Handing it to the resolver would tell a DNS server which onion address
        // this node was about to visit.
        let address: PeerAddress = "abcdefghij.onion".parse().expect("a readable address");
        let refused = connect(&address).await.expect_err("refuses");
        assert!(matches!(refused, Error::NeedsTor(_)), "{refused}");
    }

    #[tokio::test]
    async fn a_port_nobody_is_listening_on_fails_rather_than_hangs() {
        let listener = Listener::bind(loopback(0)).await.expect("binds");
        let port = listener.address().expect("has an address").port();
        drop(listener);
        let address: PeerAddress = format!("127.0.0.1:{port}")
            .parse()
            .expect("a readable address");
        assert!(connect(&address).await.is_err());
    }
}
