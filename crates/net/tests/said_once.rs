//! Every error a person can be shown is said once.
//!
//! The client prints a failure as the whole chain of it — each error's own sentence, then
//! its source's, joined by `: ` — because the outermost sentence alone is only the category
//! it was sorted into. That only works if no error both writes its source into its own
//! sentence and hands the same source on: then the cause is printed twice, on the lines an
//! operator reads most often. These pin every wrapper that reaches a person.

#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use std::error::Error;

use n333_core::wire;
use n333_net::{asked, frame, gossip, handover, liveness, session};

/// What the client prints for an error: its sentence, then each source's.
fn chain(error: &dyn Error) -> String {
    let mut said = error.to_string();
    let mut next = error.source();
    while let Some(cause) = next {
        said.push_str(": ");
        said.push_str(&cause.to_string());
        next = cause.source();
    }
    said
}

fn broken() -> frame::Error {
    frame::Error::from(std::io::Error::new(
        std::io::ErrorKind::UnexpectedEof,
        "early eof",
    ))
}

#[test]
fn a_broken_stream_is_said_once_at_every_layer() {
    for said in [
        chain(&broken()),
        chain(&session::Error::from(broken())),
        chain(&asked::Error::from(broken())),
        chain(&gossip::Error::from(broken())),
        chain(&handover::Error::from(broken())),
        chain(&liveness::Error::from(broken())),
    ] {
        assert_eq!(said, "early eof");
    }
}

#[test]
fn a_bad_message_is_said_once_at_every_layer() {
    let bad = || wire::Error::BadSignature;
    for said in [
        chain(&session::Error::from(bad())),
        chain(&gossip::Error::from(bad())),
        chain(&handover::Error::from(bad())),
        chain(&liveness::Error::from(bad())),
        chain(&n333_core::attestation::Invalid::from(bad())),
    ] {
        assert_eq!(said, "signature does not verify");
    }
}

#[test]
fn a_named_layer_keeps_its_name_and_says_its_cause_once() {
    let entry = n333_core::chain::Broken::Entry {
        index: 3,
        source: wire::Error::BadSignature,
    };
    assert_eq!(chain(&entry), "entry 3: signature does not verify");

    let statement = liveness::Error::from(n333_core::attestation::Invalid::from(
        wire::Error::BadSignature,
    ));
    assert_eq!(chain(&statement), "statement: signature does not verify");
}
