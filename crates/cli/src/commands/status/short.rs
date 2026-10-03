//! `333 status` when nothing more is asked of it: the name, where others can reach
//! this node and the invitation they would use, the epoch and when it ends, anything
//! that is wrong, and the count.
//!
//! What each of those means is `333 status --all`. Whether the node is running is said
//! before any of this by the terminal that asked ([`crate::commands::running`]): only it
//! can tell how the node is run, and the page a running node hands back is the same
//! page either way.

use anyhow::Context as _;
use n333_core::Epoch;
use n333_core::epoch::unix_now_seconds;
use n333_core::extinction::Verdict;
use n333_core::presence::Census;

use crate::commands::service::awake;
use crate::node::Node;

/// Say it.
///
/// # Errors
/// Fails if what is held cannot be read or written out.
pub(super) async fn write(
    out: &mut impl std::io::Write,
    node: &Node,
    now: Epoch,
) -> anyhow::Result<()> {
    let name = node.identity().node_id().to_string();
    writeln!(out, "{}", words!("status-name", name = name))?;
    let mine = node.lately_said().await;
    if mine.is_empty() {
        writeln!(out, "{}", words!("status-short-unreachable"))?;
    }
    for address in &mine {
        writeln!(out, "{}", words!("status-short-address", address = address))?;
    }
    for invitation in mine
        .iter()
        .filter_map(|one| crate::commands::invite::to(one))
    {
        let line = words!("status-short-invite", invitation = invitation);
        writeln!(out, "{line}")?;
    }
    let line = node.line_epoch(now).await;
    writeln!(out, "{}", ends(now, unix_now_seconds(), line))?;
    writeln!(out)?;
    super::known::copies(out, node).await?;
    let answering = u64::try_from(node.answering(now).await?.len()).unwrap_or(u64::MAX);
    let roll = u64::try_from(node.roll().await.len()).unwrap_or(u64::MAX);
    super::table(
        out,
        &Census::of(answering, 0, roll.saturating_sub(answering)),
    )?;
    // The one thing a short page cannot leave out: that nobody is left.
    let watch = node
        .watched(now)
        .await
        .with_context(|| words!("status-reading-the-watch"))?;
    if matches!(watch.verdict(), Verdict::Ended { .. }) {
        let remaining = watch.remaining_at(unix_now_seconds());
        let can_walk = super::CAN_WALK_THE_UNSEEN_ROAD;
        writeln!(
            out,
            "\n{}",
            super::silence(watch.verdict(), remaining, can_walk)
        )?;
    }
    Ok(())
}

/// The epoch, which epoch of this line it is if this node holds an admission, when it
/// ends, and how long that is from `unix`, in seconds since 1970.
fn ends(now: Epoch, unix: u64, line: Option<u64>) -> String {
    let end = Epoch(now.0 + 1).starts_at_unix_seconds();
    let (ends, left) = (awake::iso(end), awake::how_long(end.saturating_sub(unix)));
    let Some(nth) = line else {
        return words!(
            "status-short-epoch",
            epoch = now.0,
            ends = ends,
            left = left
        );
    };
    words!(
        "status-short-epoch-in-line",
        epoch = now.0,
        line = super::heading::the_line(nth),
        ends = ends,
        left = left
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_english_the_epoch_says_when_it_ends_and_how_long_that_is() {
        let now = Epoch(89_615);
        let unix = now.starts_at_unix_seconds() + 60;
        let said = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            [ends(now, unix, None), ends(now, unix, Some(3))]
        });
        let end = awake::iso(now.starts_at_unix_seconds() + n333_core::epoch::EPOCH_SECONDS);
        assert_eq!(
            said[0],
            format!("epoch    89615, ends {end}, in 332 minutes")
        );
        assert_eq!(
            said[1],
            format!("epoch    89615, this line's 3rd, ends {end}, in 332 minutes")
        );
    }
}
