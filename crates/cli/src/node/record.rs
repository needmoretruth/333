//! This node's own record: the one chain it writes, and what it says.
//!
//! Append-only, verified on every start, and never rewritten. An epoch that has been
//! judged is judged: the entry names the epoch, the epoch is strictly greater than the
//! last one, and there is no operation here that replaces anything.

use anyhow::Context as _;
use n333_core::Epoch;
use n333_core::chain::{self, Head};
use n333_core::presence::Attendance;

use super::Node;

impl Node {
    /// Where this node's record ends right now.
    pub(crate) async fn head(&self) -> Head {
        self.state.lock().await.head
    }

    /// Write one epoch's verdict into this node's own record.
    ///
    /// # Errors
    /// Fails if the entry cannot be sealed or written.
    pub(crate) async fn record(
        &self,
        epoch: Epoch,
        attendance: n333_core::presence::Attendance,
        evidence: [u8; 32],
    ) -> anyhow::Result<Head> {
        let mut state = self.state.lock().await;
        let entry = chain::Entry::following(
            Some(&state.head),
            &self.identity,
            epoch,
            attendance,
            evidence,
        );
        let frame = entry
            .seal(&self.identity)
            .with_context(|| words!("node-record-sealing"))?;
        // The head moves only after the bytes are on the disk. A head advanced first
        // and written second is a head that answers can commit to and nothing holds.
        state
            .chain
            .append(&frame)
            .with_context(|| words!("node-record-writing"))?;
        state.head = Head {
            digest: n333_core::subject::digest_of(&frame),
            length: state.head.length + 1,
        };
        Ok(state.head)
    }

    /// The newest epoch this node's own record judges, if it has judged any.
    pub(crate) async fn last_judged(&self) -> anyhow::Result<Option<Epoch>> {
        let mut state = self.state.lock().await;
        let frames = state
            .chain
            .read_all()
            .with_context(|| words!("node-record-reading"))?;
        let Some(last) = frames.last() else {
            return Ok(None);
        };
        Ok(Some(
            chain::open(last)
                .with_context(|| words!("node-record-reading-last"))?
                .entry
                .epoch(),
        ))
    }

    /// This node's own record, epoch by epoch, the way anybody else would read it.
    ///
    /// # Errors
    /// Fails if the record cannot be read, or an entry in it does not open.
    pub(crate) async fn own_record(&self) -> anyhow::Result<Vec<(Epoch, Attendance)>> {
        let mut state = self.state.lock().await;
        let frames = state
            .chain
            .read_all()
            .with_context(|| words!("node-record-reading"))?;
        frames
            .iter()
            .map(|frame| {
                let entry = chain::open(frame)
                    .with_context(|| words!("node-record-reading-entry"))?
                    .entry;
                Ok((entry.epoch(), entry.attendance))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn in_english_every_moved_line_says_exactly_what_it_said_before() {
        let pairs = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            [
                (words!("node-record-sealing"), "sealing the entry"),
                (words!("node-record-writing"), "writing the entry"),
                (words!("node-record-reading"), "reading this node's record"),
                (words!("node-record-reading-last"), "reading the last entry"),
                (words!("node-record-reading-entry"), "reading an entry"),
            ]
        });
        for (now, before) in pairs {
            assert_eq!(now, before);
        }
    }
}
