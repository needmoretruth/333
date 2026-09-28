//! What the network library says when the meeting point could not be reached or would
//! not take something, in the words this process speaks.
//!
//! The library below this client writes its reasons in English, once, for every
//! program that might use it. So a person who reads Korean and whose node cannot reach
//! the meeting point would get that reason in English in the middle of a Korean line.
//! The reasons are said again here, from the catalogs, by which reason it is; English
//! says exactly what the library says, and a test holds the two together.
//!
//! Only the library's own sentences are said here. What it carries inside them — the
//! system's words for a refused connection, the meeting point's own sentence — is
//! passed on as it came, because nobody here wrote it.

use n333_net::meeting::{Error as Meeting, LONGEST_STATEMENT};

use crate::words::Arg;

/// One cause from anywhere in a chain, in words when it is one of the library's.
pub(crate) fn cause(cause: &(dyn std::error::Error + 'static)) -> String {
    cause
        .downcast_ref::<Meeting>()
        .map_or_else(|| cause.to_string(), meeting)
}

/// Why a visit to the meeting point came to nothing.
pub(crate) fn meeting(e: &Meeting) -> String {
    match e {
        Meeting::Unreachable(why) => words!("failed-net-unreachable", why = why),
        Meeting::BrokeOff(why) => words!("failed-net-broke-off", why = why),
        Meeting::NotYet { said, .. } => words!("failed-net-not-yet", said = said),
        Meeting::FullForToday { said, .. } => words!("failed-net-full-for-today", said = said),
        Meeting::Refused {
            status,
            said: Some(said),
        } => words!(
            "failed-net-refused-because",
            status = Arg::exact(status),
            said = said
        ),
        Meeting::Refused { status, said: None } => {
            words!("failed-net-refused", status = Arg::exact(status))
        }
        Meeting::TooLong { got } => words!(
            "failed-net-statement-too-long",
            got = Arg::exact(got),
            longest = Arg::exact(LONGEST_STATEMENT)
        ),
        Meeting::NotAnAddress => words!("failed-net-not-an-address"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::words::count::Base;

    /// One of every reason, as the library would hand it over.
    fn every_reason() -> Vec<Meeting> {
        vec![
            Meeting::Unreachable("Connection refused".into()),
            Meeting::BrokeOff("reset".into()),
            Meeting::NotYet {
                again_in: None,
                said: "wait a minute".into(),
            },
            Meeting::FullForToday {
                again_in: None,
                said: "full".into(),
            },
            Meeting::Refused {
                status: 500,
                said: Some("broken".into()),
            },
            Meeting::Refused {
                status: 404,
                said: None,
            },
            Meeting::TooLong { got: 600 },
            Meeting::NotAnAddress,
        ]
    }

    #[test]
    fn in_english_every_reason_is_what_the_library_says() {
        for reason in every_reason() {
            let said = crate::words::speaking("en", Base::Ten, || cause(&reason));
            assert_eq!(said, reason.to_string());
        }
    }

    #[test]
    fn in_korean_no_reason_is_the_library_s_english_and_what_it_carries_stays() {
        for reason in every_reason() {
            let said = crate::words::speaking("ko", Base::Twelve, || cause(&reason));
            assert!(!said.contains("meeting point"), "{said}");
            for kept in ["Connection refused", "broken", "500", "600", "512"] {
                if reason.to_string().contains(kept) {
                    assert!(said.contains(kept), "{kept} is lost from {said}");
                }
            }
        }
    }

    #[test]
    fn anything_else_is_said_as_it_came() {
        let io = std::io::Error::other("Permission denied");
        assert_eq!(cause(&io), "Permission denied");
    }
}
