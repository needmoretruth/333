//! The exchange over a real socket, rather than over a pipe in memory.
//!
//! The unit tests run both halves over `tokio::io::duplex`, which is the right place
//! to test what the protocol says. It cannot catch what this file is for: a socket
//! delivers a frame in however many pieces it feels like, closes in ways a pipe does
//! not, and reaches the exchange through a compatibility layer between two different
//! sets of async traits. Every one of those is a place where a passing unit test and
//! a client that never completes a handshake can coexist.

#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing
)]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use n333_core::Identity;
use n333_net::peer::PeerAddress;
use n333_net::{direct, initiate, respond};
use tokio::io::AsyncWriteExt as _;

/// A node identity that is the same on every run, so a failure is reproducible.
fn identity(seed: u8) -> Identity {
    Identity::from_seed(&[seed; 32])
}

/// Bind a listener on a port the operating system picks, and say how to reach it.
async fn listening() -> (direct::Listener, PeerAddress) {
    let listener = direct::Listener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0))
        .await
        .expect("binds");
    let port = listener.address().expect("has an address").port();
    let address = format!("127.0.0.1:{port}").parse().expect("readable");
    (listener, address)
}

#[tokio::test]
async fn two_nodes_witness_each_other_over_a_socket() {
    let (listener, address) = listening().await;
    let (opener, answerer) = (identity(1), identity(2));
    let (opener_name, answerer_name) = (opener.node_id(), answerer.node_id());

    let answering = tokio::spawn(async move {
        let (mut stream, _from) = listener.accept().await.expect("accepts");
        respond(&mut stream, &answerer).await
    });

    let mut stream = direct::connect(&address).await.expect("connects");
    let opened = initiate(&mut stream, &opener).await.expect("exchanges");
    let answered = answering.await.expect("task").expect("exchanges");

    assert_eq!(opened.peer.node_id, answerer_name);
    assert_eq!(answered.peer.node_id, opener_name);
    // The asymmetry the protocol is built around survives the socket: only the side
    // that chose the nonce learns the other was awake after it was chosen.
    assert!(opened.proves_peer_was_live);
    assert!(!answered.proves_peer_was_live);
    assert_eq!(opened.epoch_skew, 0, "one machine, one clock");
}

#[tokio::test]
async fn a_frame_split_across_packets_still_arrives_whole() {
    // Written a byte at a time with a flush after each, so the reader is handed the
    // length prefix in pieces and then the body in pieces. A reader that assumed one
    // read returns a whole frame passes every in-memory test and fails here.
    let (listener, address) = listening().await;
    let answerer = identity(2);
    let answering = tokio::spawn(async move {
        let (mut stream, _from) = listener.accept().await.expect("accepts");
        respond(&mut stream, &answerer).await
    });

    let opener = identity(1);
    let heartbeat = n333_core::heartbeat::Heartbeat::now(&opener, None);
    let body = heartbeat.seal(&opener).expect("seals");
    let mut framed = u32::try_from(body.len())
        .expect("fits")
        .to_be_bytes()
        .to_vec();
    framed.extend_from_slice(&body);

    let mut socket = tokio::net::TcpStream::connect(("127.0.0.1", address.port()))
        .await
        .expect("connects");
    for byte in &framed {
        socket.write_all(&[*byte]).await.expect("writes");
        socket.flush().await.expect("flushes");
    }

    let answered = answering.await.expect("task").expect("exchanges");
    assert_eq!(answered.peer.node_id, opener.node_id());
    assert_eq!(answered.peer.heartbeat.nonce, heartbeat.nonce);
}

#[tokio::test]
async fn a_peer_that_announces_a_frame_larger_than_the_limit_is_refused() {
    // The refusal has to happen on the announced length, before anything is read.
    // A node that allocated first would be emptied by a four-byte message.
    let (listener, address) = listening().await;
    let answerer = identity(2);
    let answering = tokio::spawn(async move {
        let (mut stream, _from) = listener.accept().await.expect("accepts");
        respond(&mut stream, &answerer).await
    });

    let mut socket = tokio::net::TcpStream::connect(("127.0.0.1", address.port()))
        .await
        .expect("connects");
    socket
        .write_all(&u32::MAX.to_be_bytes())
        .await
        .expect("writes");
    socket.flush().await.expect("flushes");

    let refused = answering.await.expect("task").expect_err("refuses");
    assert!(
        refused.to_string().contains("limit"),
        "expected the frame limit to be named, got: {refused}"
    );
}

#[tokio::test]
async fn a_peer_that_connects_and_says_nothing_ends_the_exchange_rather_than_hanging() {
    // Closing without sending is the ordinary shape of a port scan. It must end the
    // exchange as an error and never wait: the caller's deadline is a backstop, not
    // the mechanism.
    let (listener, address) = listening().await;
    let answerer = identity(2);
    let answering = tokio::spawn(async move {
        let (mut stream, _from) = listener.accept().await.expect("accepts");
        respond(&mut stream, &answerer).await
    });

    let socket = tokio::net::TcpStream::connect(("127.0.0.1", address.port()))
        .await
        .expect("connects");
    drop(socket);

    let ended = tokio::time::timeout(std::time::Duration::from_secs(5), answering)
        .await
        .expect("does not hang")
        .expect("task");
    assert!(ended.is_err(), "a closed connection is not an exchange");
}

#[tokio::test]
async fn a_forged_signature_is_refused_over_the_wire_too() {
    let (listener, address) = listening().await;
    let answerer = identity(2);
    let answering = tokio::spawn(async move {
        let (mut stream, _from) = listener.accept().await.expect("accepts");
        respond(&mut stream, &answerer).await
    });

    let opener = identity(1);
    let mut body = n333_core::heartbeat::Heartbeat::now(&opener, None)
        .seal(&opener)
        .expect("seals");
    // Flip a bit in the signature, which is the first 64 bytes of the frame. The
    // body still decodes, so the refusal has to come from the signature check and
    // cannot be a decoder error standing in for one.
    body[0] ^= 0x01;

    let mut socket = tokio::net::TcpStream::connect(("127.0.0.1", address.port()))
        .await
        .expect("connects");
    socket
        .write_all(&u32::try_from(body.len()).expect("fits").to_be_bytes())
        .await
        .expect("writes");
    socket.write_all(&body).await.expect("writes");
    socket.flush().await.expect("flushes");

    let refused = answering.await.expect("task").expect_err("refuses");
    assert!(
        refused.to_string().contains("signature"),
        "expected the signature to be named, got: {refused}"
    );
}

#[tokio::test]
async fn a_corrupted_body_is_refused_before_the_signature_is_considered() {
    // The other half of the previous test. Damage inside the encoded body stops it
    // decoding, and the node says so rather than reporting a signature failure for
    // bytes it never managed to read.
    let (listener, address) = listening().await;
    let answerer = identity(2);
    let answering = tokio::spawn(async move {
        let (mut stream, _from) = listener.accept().await.expect("accepts");
        respond(&mut stream, &answerer).await
    });

    let opener = identity(1);
    let mut body = n333_core::heartbeat::Heartbeat::now(&opener, None)
        .seal(&opener)
        .expect("seals");
    let last = body.len() - 1;
    body[last] ^= 0x01;

    let mut socket = tokio::net::TcpStream::connect(("127.0.0.1", address.port()))
        .await
        .expect("connects");
    socket
        .write_all(&u32::try_from(body.len()).expect("fits").to_be_bytes())
        .await
        .expect("writes");
    socket.write_all(&body).await.expect("writes");
    socket.flush().await.expect("flushes");

    let refused = answering.await.expect("task").expect_err("refuses");
    assert!(
        refused.to_string().contains("decode"),
        "expected a decoding failure, got: {refused}"
    );
}
