//! What `333 --version` says: which build of the client this is, not only which release.
//!
//! Twelve files come off every release, and a report about one of them is only useful if
//! it says which. The version number alone does not: Standard and Light, and the six
//! systems, all carry the same one. So the long form adds what separates them — whether
//! the screen is compiled in, whether Tor is, and what the file was built to run on —
//! read from the build itself rather than from anything a person could mistype.
//!
//! No build script and nothing from the environment at build time. Everything here is
//! something the compiler already knows about the binary it is making.

/// The long form, e.g. `0.5.0 (Standard, tor, x86_64-linux)`, in the words this
/// process speaks, for `-V` and `--version` alike. clap puts the name in front of it.
pub(crate) fn long() -> String {
    describe(
        env!("CARGO_PKG_VERSION"),
        cfg!(feature = "screen"),
        cfg!(feature = "tor"),
        std::env::consts::ARCH,
        std::env::consts::OS,
    )
}

/// The long form from its parts.
///
/// Standard is the build with the screen and Light the one without, which is the only
/// thing that separates the two editions; Tor is said separately because a build from
/// source can leave it out of either. The editions are names, as the release files
/// carry them, and are not translated.
fn describe(version: &str, screen: bool, tor: bool, arch: &str, os: &str) -> String {
    let edition = if screen { "Standard" } else { "Light" };
    let tor = if tor {
        words!("version-tor")
    } else {
        words!("version-no-tor")
    };
    words!(
        "version-long",
        version = version,
        edition = edition,
        tor = tor,
        target = format!("{arch}-{os}")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::words::count::Base;

    fn in_english(screen: bool, tor: bool, arch: &str, os: &str) -> String {
        crate::words::speaking("en", Base::Ten, || describe("0.5.0", screen, tor, arch, os))
    }

    #[test]
    fn the_standard_build_says_all_three() {
        assert_eq!(
            in_english(true, true, "x86_64", "linux"),
            "0.5.0 (Standard, tor, x86_64-linux)"
        );
    }

    #[test]
    fn a_build_without_the_screen_is_light_and_one_without_tor_says_so() {
        assert_eq!(
            in_english(false, true, "arm", "linux"),
            "0.5.0 (Light, tor, arm-linux)"
        );
        assert_eq!(
            in_english(false, false, "aarch64", "macos"),
            "0.5.0 (Light, no tor, aarch64-macos)"
        );
    }

    #[test]
    fn in_korean_the_version_and_the_names_stay_as_they_are() {
        let long = crate::words::speaking("ko", Base::Ten, || {
            describe("0.5.0", false, false, "aarch64", "macos")
        });
        assert!(long.starts_with("0.5.0 (Light, "), "{long}");
        assert!(long.ends_with(", aarch64-macos)"), "{long}");
        assert!(!long.contains("no tor"), "{long}");
    }

    #[test]
    fn this_build_describes_itself_with_its_own_version_and_target() {
        let long = crate::words::speaking("en", Base::Ten, long);
        assert!(long.starts_with(env!("CARGO_PKG_VERSION")), "{long}");
        assert!(
            long.ends_with(&format!(
                "{}-{})",
                std::env::consts::ARCH,
                std::env::consts::OS
            )),
            "{long}"
        );
    }
}
