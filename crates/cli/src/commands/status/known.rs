//! What this node knows of where the others are, and where it learned each of them.
//!
//! The addresses themselves are printed only when asked for with `--sources`. They are
//! this node's own notes on its owner's own disk, and nobody but whoever can already
//! read that disk ever sees them; the ordinary `status` still says only how many.

use crate::node::Node;
use crate::node::sources::Heard;

use super::padded;

/// Every statement under this node's key that it did not make, first and loudest.
///
/// Nothing at all when there are none, which is the ordinary case.
pub(super) async fn copies(out: &mut impl std::io::Write, node: &Node) -> anyhow::Result<()> {
    let copies = node.copies().await;
    if copies.is_empty() {
        return Ok(());
    }
    writeln!(out, "{}\n", words!("status-known-another-copy"))?;
    let me = node.identity().node_id().to_string();
    for sighting in &copies {
        let seen = words!(
            "status-known-sighting",
            address = sighting.address.clone(),
            said_in = sighting.said_in,
            arrived = sighting.arrived(&me),
            epoch = sighting.heard.epoch
        );
        writeln!(out, "{}", indented(&seen, "  "))?;
    }
    writeln!(out, "\n{}\n", words!("status-known-either"))?;
    Ok(())
}

/// How many addresses this node holds, by how each was first heard of.
pub(super) async fn counts(out: &mut impl std::io::Write, node: &Node) -> anyhow::Result<()> {
    let known = node.known().await;
    let held = known.by_hand
        + known.this_network
        + known.meeting_point
        + known.from_peers
        + known.unrecorded;
    if held == 0 {
        writeln!(out, "{}", words!("status-known-nowhere"))?;
        return Ok(());
    }
    writeln!(out, "{}", words!("status-known-held", held = held))?;
    let rows = [
        (words!("status-known-by-hand"), known.by_hand),
        (words!("status-known-this-network"), known.this_network),
        (words!("status-known-meeting-point"), known.meeting_point),
        (
            words!("status-known-from-us", peers = known.peers),
            known.from_peers,
        ),
    ];
    let unrecorded =
        (known.unrecorded != 0).then(|| (words!("status-known-not-noted"), known.unrecorded));
    for (name, count) in rows.into_iter().chain(unrecorded) {
        let count = u64::try_from(count).unwrap_or(u64::MAX);
        let count = crate::words::count::write(count, false, 0);
        writeln!(out, "  {} {count:>5}", padded(&name, 18))?;
    }
    writeln!(out, "{}", words!("status-known-where-heard"))?;
    Ok(())
}

/// Every address this node holds, whose it is, and where it was heard of.
pub(super) async fn sources(out: &mut impl std::io::Write, node: &Node) -> anyhow::Result<()> {
    let sources = node.sources().await;
    if sources.is_empty() {
        return Ok(());
    }
    writeln!(out, "{}", words!("status-known-sources"))?;
    let label = |label: String| format!("    {}", padded(&label, 7));
    for (name, address, learned) in &sources {
        let nobody = || words!("status-known-nobody-answered");
        writeln!(out, "\n  {}", name.clone().unwrap_or_else(nobody))?;
        writeln!(out, "{}{address}", label(words!("status-known-at")))?;
        match learned {
            Some(learned) => {
                let first = label(words!("status-known-first"));
                writeln!(out, "{first}{}", when(&learned.first))?;
                let last = label(words!("status-known-last"));
                writeln!(out, "{last}{}", when(&learned.last))?;
            }
            None => writeln!(out, "    {}", words!("status-known-from-before"))?,
        }
    }
    Ok(())
}

/// "from 333ab…cd12, in epoch 89601", and the same for every other way.
fn when(heard: &Heard) -> String {
    words!(
        "status-known-when",
        from = heard.from.to_string(),
        epoch = heard.epoch
    )
}

/// Every line of `text` begun with `by`, for a paragraph set in from the margin.
fn indented(text: &str, by: &str) -> String {
    text.split('\n')
        .map(|line| format!("{by}{line}"))
        .collect::<Vec<_>>()
        .join("\n")
}
