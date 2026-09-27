//! What this node knows of where the others are, and where it learned each of them.
//!
//! The addresses themselves are printed only when asked for with `--sources`. They are
//! this node's own notes on its owner's own disk, and nobody but whoever can already
//! read that disk ever sees them; the ordinary `status` still says only how many.

use crate::node::Node;
use crate::node::sources::Heard;

/// Every statement under this node's key that it did not make, first and loudest.
///
/// Nothing at all when there are none, which is the ordinary case.
pub(super) async fn copies(out: &mut impl std::io::Write, node: &Node) -> anyhow::Result<()> {
    let copies = node.copies().await;
    if copies.is_empty() {
        return Ok(());
    }
    writeln!(out, "ANOTHER COPY OF THIS NAME\n")?;
    let me = node.identity().node_id().to_string();
    for sighting in &copies {
        writeln!(
            out,
            "  A statement signed with this node's key, which this node never made, says\n  \
             it is at {}, in epoch {}.\n  It arrived {}, in epoch {}.",
            sighting.address,
            sighting.said_in,
            sighting.arrived(&me),
            sighting.heard.epoch
        )?;
    }
    writeln!(
        out,
        "\nEither this directory was copied and the copy was started, or somebody else\n\
         has the key. Two nodes on one name contradict each other in every epoch either\n\
         is asked about. Stop one of them; `333 pack` is how a node moves. Nothing here\n\
         stops either copy for you: an old statement can be replayed by anybody, and a\n\
         node that stopped on seeing one could be switched off by whoever holds a copy\n\
         of its key.\n"
    )?;
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
        writeln!(
            out,
            "KNOWN    nowhere to knock yet. An invitation given to `333 ping` or\n\
             \x20        `333 join` is kept, and the vigil knocks there from then on."
        )?;
        return Ok(());
    }
    let addresses = if held == 1 { "address" } else { "addresses" };
    writeln!(
        out,
        "KNOWN    {held} {addresses}, by where each was first heard of"
    )?;
    let from_us = if known.peers == 1 {
        "from 1 of us".to_owned()
    } else {
        format!("from {} of us", known.peers)
    };
    let rows = [
        ("by hand", known.by_hand),
        ("this network", known.this_network),
        ("a meeting point", known.meeting_point),
        (from_us.as_str(), known.from_peers),
    ];
    for (name, count) in rows {
        writeln!(out, "  {name:<18} {count:>5}")?;
    }
    if known.unrecorded != 0 {
        writeln!(out, "  {:<18} {:>5}", "not noted", known.unrecorded)?;
    }
    writeln!(
        out,
        "Where each was heard of says nothing about whether anybody answers there.\n\
         `333 status --sources` lists them."
    )?;
    Ok(())
}

/// Every address this node holds, whose it is, and where it was heard of.
pub(super) async fn sources(out: &mut impl std::io::Write, node: &Node) -> anyhow::Result<()> {
    let sources = node.sources().await;
    if sources.is_empty() {
        return Ok(());
    }
    writeln!(out, "SOURCES")?;
    for (name, address, learned) in &sources {
        writeln!(
            out,
            "\n  {}",
            name.as_deref().unwrap_or("nobody has answered here yet")
        )?;
        writeln!(out, "    at     {address}")?;
        match learned {
            Some(learned) => {
                writeln!(out, "    first  {}", when(&learned.first))?;
                writeln!(out, "    last   {}", when(&learned.last))?;
            }
            None => writeln!(
                out,
                "    held from before this node wrote down where addresses came from"
            )?,
        }
    }
    Ok(())
}

/// "from 333ab…cd12, in epoch 89601", and the same for every other way.
fn when(heard: &Heard) -> String {
    format!("{}, in epoch {}", heard.from, heard.epoch)
}
