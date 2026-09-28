//! What is said when a command could not do what it was asked.
//!
//! The same voice as everything else this client says: a keyword, then what happened.
//! What happened is the whole chain of it — what was being attempted, what that ran
//! into, and what that ran into — because the outermost sentence alone is the category
//! a failure was sorted into, and the cause is the part a person can act on.
//!
//! WHY THIS IS NOT RUST'S OWN. Returning an error from `main` prints its debug form after
//! `Error: `, which is a program talking to itself in front of a guest: a different
//! voice, a stack of `Caused by:` and nothing about what to do.

use std::fmt;

/// The one thing a person can do next, attached to a failure by the code that knows it.
///
/// Carried as a context rather than written into a sentence, so that it is said once, on a
/// line of its own and after the cause, instead of in the middle of the chain.
#[derive(Debug)]
pub(crate) struct NextStep(pub(crate) String);

impl fmt::Display for NextStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Attach the one next step to a failure.
pub(crate) fn next_step(error: impl Into<anyhow::Error>, step: String) -> anyhow::Error {
    error.into().context(NextStep(step))
}

/// Whether a failure is this node's own record refusing to be written or read.
///
/// Said as `failed` wherever it surfaces: a sentence that calls it `refused` blames
/// whoever this node was dealing with for its own disk.
pub(crate) fn is_our_own(error: &anyhow::Error) -> bool {
    error
        .chain()
        .any(|cause| cause.is::<n333_store::log::Error>())
}

/// Why saying one of the 333 did not happen, as the line that says so.
///
/// Every refusal `say` gives is a sentence of its own, so it follows the keyword as it
/// is; only a failure of this node's own record is not a refusal. Said by the vigil,
/// for an order typed into its screen or handed to it from another terminal.
pub(crate) fn not_said(error: &anyhow::Error) -> String {
    let text = format!("{error:#}");
    let mut lines = text.splitn(2, '\n');
    let first = lines.next().unwrap_or_default();
    let mut said = if is_our_own(error) {
        words!("failed-failed", why = first)
    } else {
        words!("failed-refused", why = first)
    };
    // What a refusal says after its first line follows as the refusal wrote it.
    if let Some(rest) = lines.next() {
        said.push('\n');
        said.push_str(rest);
    }
    said
}

/// A failed command, as the lines this client says.
///
/// `failed   ` and every sentence in the chain joined by `: `, then the next step when
/// something knew one. Every line after the first is indented to the column the
/// sentences start at, including the ones a sentence brought with it.
pub(crate) fn said(error: &anyhow::Error) -> String {
    let step = error.downcast_ref::<NextStep>().map(|step| step.0.as_str());
    let chain: Vec<String> = error
        .chain()
        .map(ToString::to_string)
        .filter(|sentence| Some(sentence.as_str()) != step)
        .collect();
    let mut text = words!("failed-failed", why = chain.join(": "));
    if let Some(step) = step {
        text.push('\n');
        text.push_str(step);
    }
    text.lines()
        .enumerate()
        .map(|(at, line)| match (at, line.trim_end()) {
            (0, line) | (_, line @ "") => line.to_owned(),
            (_, line) => format!("         {}", line.trim_start()),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context as _;

    #[test]
    fn in_english_a_refusal_the_vigil_says_is_exactly_what_it_said_before() {
        let refusal = anyhow::anyhow!(
            "you already said #3 in epoch 9. One each, and saying it again would\n\
             not replace it."
        );
        let own = anyhow::Error::new(n333_store::log::Error::TooLongToWrite { got: 9 })
            .context("writing");
        let said = crate::words::speaking("en", crate::words::count::Base::Ten, || {
            [not_said(&refusal), not_said(&own)]
        });
        assert_eq!(
            said,
            [format!("refused  {refusal:#}"), format!("failed   {own:#}"),]
        );
    }

    #[test]
    fn the_whole_chain_is_said_and_not_only_the_outermost() {
        let io = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Permission denied");
        let failed = Err::<(), _>(io)
            .context("/tmp/n/chain")
            .context("opening this node's record")
            .unwrap_err();
        assert_eq!(
            said(&failed),
            "failed   opening this node's record: /tmp/n/chain: Permission denied"
        );
    }

    #[test]
    fn the_next_step_comes_after_the_cause_on_a_line_of_its_own() {
        let io = std::io::Error::new(std::io::ErrorKind::AddrInUse, "Address already in use");
        let failed = next_step(
            anyhow::Error::new(io).context("listening on 0.0.0.0:3333"),
            "Stop whatever holds it, or pass --bind with another port.".to_owned(),
        );
        assert_eq!(
            said(&failed),
            "failed   listening on 0.0.0.0:3333: Address already in use\n\
             \x20        Stop whatever holds it, or pass --bind with another port."
        );
    }

    #[test]
    fn a_sentence_of_several_lines_keeps_the_column() {
        let failed = anyhow::anyhow!(
            "you already said #3 in epoch 9. One each, and saying it again would\n\
             not replace it.\n\
             \n\
             That is all."
        );
        assert_eq!(
            said(&failed),
            "failed   you already said #3 in epoch 9. One each, and saying it again would\n\
             \x20        not replace it.\n\
             \n\
             \x20        That is all."
        );
    }
}
