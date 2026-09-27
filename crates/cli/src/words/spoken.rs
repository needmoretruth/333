//! The words one order is said in, when whoever handed it over reads another language
//! or counts in another base than the vigil does.
//!
//! A vigil chooses its words once, when it starts. An order from another terminal is
//! answered to that terminal, whose person may have asked for Korean, or for twelve,
//! where the vigil did not. The task that carries the order out speaks their words
//! instead, and reads what they typed in their base: `333 --count-in twelve say 10` is
//! signal twelve whichever way the vigil counts. What that task says reaches the
//! vigil's own log too, in the same words, because it is one line said once.

use std::future::Future;
use std::sync::{Mutex, PoisonError};

use super::count::Base;
use super::{Words, catalog, choose};

tokio::task_local! {
    /// The words of whoever asked for what this task is doing.
    static SPOKEN: &'static Words;
}

/// Every language and base an order has been asked in, each read once.
///
/// Kept for the life of the process, and bounded: a tag is matched against the
/// catalogs there are before anything is read, so there is at most one entry for each
/// language there are words for in each of the three bases.
static OPENED: Mutex<Vec<&'static Words>> = Mutex::new(Vec::new());

/// The words of whoever asked for this task's work, if somebody did.
pub(super) fn here() -> Option<&'static Words> {
    SPOKEN.try_with(|words| *words).ok()
}

/// The words for somebody who asked in `language`, counting in `count_in`.
///
/// Either one missing, or naming nothing there are words or a base for, is the
/// process's own: an order from a client that does not say how it reads is answered
/// the way it always was.
pub(crate) fn asked_for(language: Option<&str>, count_in: Option<&str>) -> &'static Words {
    let own = super::process();
    let beside = own.beside.as_deref();
    let tag = language
        .and_then(|asked| choose::matching(asked, &catalog::tags(beside)))
        .unwrap_or_else(|| own.tag.clone());
    let base = count_in
        .and_then(|name| Base::named(name).ok())
        .unwrap_or(own.base);
    if tag == own.tag && base == own.base {
        return own;
    }
    // A lock another task panicked while holding still holds what was read.
    let mut opened = OPENED.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(words) = opened
        .iter()
        .find(|words| words.tag == tag && words.base == base)
    {
        return words;
    }
    // What is wrong with a catalog was said when the vigil started, in its own words.
    let words: &'static Words = Box::leak(Box::new(Words::open(&tag, base, beside).0));
    opened.push(words);
    words
}

/// Carry out `work` in `words`, and everything it says with them.
pub(crate) async fn spoken_in<F: Future>(words: &'static Words, work: F) -> F::Output {
    SPOKEN.scope(words, work).await
}

/// [`spoken_in`], for work that does not wait on anything.
pub(crate) fn spoken_now<T>(words: &'static Words, work: impl FnOnce() -> T) -> T {
    SPOKEN.sync_scope(words, work)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn an_order_is_said_in_the_words_and_read_in_the_base_it_was_asked_in() {
        let theirs = asked_for(Some("ko"), Some("twelve"));
        let (said, read) = spoken_in(theirs, async {
            let said = crate::words!("id-name", name = "x");
            (said, crate::words::count::index("10"))
        })
        .await;
        assert!(said.starts_with("이름 "), "{said:?}");
        assert_eq!(read, Some(12));
        // And outside it, the process's own, which in a test is English and ten.
        assert_eq!(crate::words!("id-name", name = "x"), "name     x");
        assert_eq!(crate::words::count::index("10"), Some(10));
    }

    #[test]
    fn an_order_that_does_not_say_how_it_reads_is_answered_in_the_vigils_words() {
        let own = super::super::process();
        assert!(std::ptr::eq(asked_for(None, None), own));
        assert!(std::ptr::eq(
            asked_for(Some("xx-nothing"), Some("nine")),
            own
        ));
    }

    #[test]
    fn the_same_asking_twice_is_read_once() {
        let first = asked_for(Some("ko"), Some("twelve-ascii"));
        assert!(std::ptr::eq(
            first,
            asked_for(Some("KO"), Some("twelve-ascii"))
        ));
        assert_eq!(first.base(), Base::TwelveAscii);
        assert_eq!(spoken_now(first, || crate::words::current().tag()), "ko");
    }
}
