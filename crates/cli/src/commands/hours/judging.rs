//! Reaching a verdict about one of this node's own epochs, once, and writing it down.
//!
//! It judges the epoch that is three epochs old, because that is how long a statement
//! about it may take to arrive, and it never revisits one. What it concludes is about
//! ITSELF and nobody else: the statements it published about others are the evidence
//! other people judge them by, and this is only the record it is entitled to keep.

use n333_core::attestation::{self, Because, Evidence, JUDGEMENT_DELAY_EPOCHS};
use n333_core::challenge::{self, Exchange};
use n333_core::chain::evidence_digest;
use n333_core::enrollment;
use n333_core::Epoch;

use crate::node::Node;

/// Judge the epoch that has now had long enough for statements to arrive.
pub(super) async fn judge_what_is_ready(node: &Node, now: Epoch) {
    let Some(ready) = now.0.checked_sub(JUDGEMENT_DELAY_EPOCHS).map(Epoch) else {
        return;
    };
    // A node nobody has admitted has nothing to judge: no verifier is ever drawn for
    // it, so every epoch would be an empty entry saying nothing.
    let Some(joined) = node.joined_in().await else {
        return;
    };
    // Epochs before this node counted are not absences and not excluded epochs. They
    // are epochs it was not here for, and its record has nothing to say about them.
    if !enrollment::covers(joined, ready) {
        return;
    }
    match node.last_judged().await {
        Ok(Some(last)) if last.0 >= ready.0 => return,
        Ok(_) => {}
        Err(e) => {
            println!("failed   reading this node's own record: {e:#}");
            return;
        }
    }
    if let Err(e) = judge_one(node, ready).await {
        println!("failed   judging epoch {}: {e:#}", ready.0);
    }
}

/// Read one epoch's statements, reach a verdict, and write it down.
async fn judge_one(node: &Node, epoch: Epoch) -> anyhow::Result<()> {
    let frames = node.statements(epoch).await?;
    let me = node.identity().public_key();
    let roll = node.roll().await;

    let published: Vec<_> = frames
        .iter()
        .filter_map(|frame| attestation::open(frame).ok())
        .collect();
    let receipt = receipt_in(&frames, &me);
    let evidence = Evidence {
        attestations: published.iter().collect(),
        receipt: receipt.as_ref(),
    };
    let verdict = attestation::read(epoch, &me, &roll, &evidence);
    let head = node
        .record(epoch, verdict.attendance, evidence_digest(&frames))
        .await?;
    println!("judged   epoch {}: {}", epoch.0, said(verdict.because));
    println!("record   {}", super::epochs(head.length));
    Ok(())
}

/// This node's own proof that it answered somebody, if it kept one.
///
/// Two signatures by two different keys: a challenge somebody put, and the answer this
/// node gave to it. It does not say *when* the answer was given, which is why it
/// withdraws an accusation rather than earning a presence — see [`attestation::judge`].
fn receipt_in(frames: &[Vec<u8>], me: &[u8; 32]) -> Option<Exchange> {
    let mine: Vec<_> = frames
        .iter()
        .filter_map(|frame| challenge::open_answer(frame).ok())
        .filter(|answer| answer.answer.prover == *me)
        .collect();
    frames
        .iter()
        .filter_map(|frame| challenge::open_challenge(frame).ok())
        .find_map(|question| {
            mine.iter()
                .find_map(|answer| Exchange::assemble(question.clone(), answer.clone()).ok())
        })
}

/// What happened, said as the thing that happened.
///
/// Three of the five ways an epoch leaves the count are different events with different
/// meanings, and one sentence for all three would be false in two of them. The
/// arithmetic does not distinguish them; a person reading this must.
const fn said(because: Because) -> &'static str {
    match because {
        Because::Answered => "present. it will not be judged again.",
        Because::Denounced => {
            "absent. everyone drawn to ask you has sworn nothing came back. it will\n\
             \x20        not be judged again."
        }
        Because::NoneDrawn => {
            "outside the count. nobody was drawn to ask you, and an epoch that asked\n\
             \x20        nothing of you takes nothing from you."
        }
        Because::ReceiptWithdrew => {
            "outside the count. you kept the question and the answer you gave to it,\n\
             \x20        which withdraws the accusation without earning a presence."
        }
        Because::NotAllSpoke => {
            "outside the count. some of those drawn said nothing at all, and silence\n\
             \x20        is not agreement."
        }
    }
}

