//! How a number a person reads is written, and how a number a person types is read.
//!
//! In ten unless somebody asks for twelve. Twelve divides by two, three, four and six;
//! ten divides by two and five. The Vision holds that a client which could count in
//! twelve for whoever wants it is closer to the thing than one that cannot, and this is
//! the one place that decides how every count is written, so asking for it changes all
//! of them at once.
//!
//! WHAT IS NEVER WRITTEN IN TWELVE. Anything that names something rather than counts
//! it: a node's name (it is hex, and it is the name), an address, a port, a version, a
//! byte count in an error, a line number in a file, a year. Those are passed to the
//! words as text ([`super::Arg::exact`]) and come out exactly as they went in. What
//! travels between nodes never changes either: a signal is the same number on the wire
//! whichever way the person who said it counts. Only the showing and the typing differ.
//!
//! The two digits past nine are U+218A and U+218B, the ones Unicode added for this.
//! A terminal whose locale is not UTF-8 gets `X` and `E` instead, which is what those
//! two digits were written as before Unicode had them.

/// The base every count a person reads is written in, and every count they type is
/// read in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Base {
    /// What people read. The default.
    #[default]
    Ten,
    /// Twelve, with ↊ for ten and ↋ for eleven.
    Twelve,
    /// Twelve, with `X` for ten and `E` for eleven, for a terminal that cannot show
    /// the other two.
    TwelveAscii,
}

impl Base {
    /// Read a base by the name a person gives it, on the command line or in
    /// `THE333_COUNT_IN`.
    ///
    /// # Errors
    /// Fails, naming the three there are, for anything else.
    pub(crate) fn named(name: &str) -> Result<Self, String> {
        match name.trim().to_ascii_lowercase().as_str() {
            "ten" | "10" => Ok(Self::Ten),
            "twelve" | "12" => Ok(Self::Twelve),
            "twelve-ascii" => Ok(Self::TwelveAscii),
            _ => Err(format!("{name} is not ten, twelve or twelve-ascii")),
        }
    }

    /// The name [`Base::named`] reads back as this base, for a command line written
    /// out to be run elsewhere: a service's, or an order handed to a vigil.
    #[must_use]
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Ten => "ten",
            Self::Twelve => "twelve",
            Self::TwelveAscii => "twelve-ascii",
        }
    }

    /// Twelve with the digits this terminal can show.
    #[must_use]
    pub(crate) fn shown(self, utf8: bool) -> Self {
        match self {
            Self::Twelve if !utf8 => Self::TwelveAscii,
            other => other,
        }
    }

    /// How many a digit counts before the next place is used.
    const fn radix(self) -> u64 {
        match self {
            Self::Ten => 10,
            Self::Twelve | Self::TwelveAscii => 12,
        }
    }

    /// The digits written for ten and eleven, for the line that says counting is in
    /// twelve. Nothing in ten.
    #[must_use]
    pub(crate) const fn past_nine(self) -> Option<(char, char)> {
        match self {
            Self::Ten => None,
            Self::Twelve => Some(('\u{218A}', '\u{218B}')),
            Self::TwelveAscii => Some(('X', 'E')),
        }
    }

    /// One digit, which the caller has already made smaller than the radix.
    fn digit(self, value: u64) -> char {
        match (value, self.past_nine()) {
            (10, Some((ten, _))) => ten,
            (11, Some((_, eleven))) => eleven,
            _ => u32::try_from(value)
                .ok()
                .and_then(|value| char::from_digit(value, 10))
                .unwrap_or('?'),
        }
    }

    /// What one typed character is worth, if it is a digit in this base.
    ///
    /// Generous about how ten and eleven are typed, because a person asked for twelve
    /// is not going to find ↊ on a keyboard: `X` and `A` are ten, `E` and `B` are
    /// eleven, in either case.
    fn value_of(self, typed: char) -> Option<u64> {
        let value = match typed {
            '0'..='9' => u64::from(typed.to_digit(10)?),
            '\u{218A}' | 'X' | 'x' | 'A' | 'a' => 10,
            '\u{218B}' | 'E' | 'e' | 'B' | 'b' => 11,
            _ => return None,
        };
        (value < self.radix()).then_some(value)
    }
}

/// Write a count in the base this client counts in.
///
/// `grouped` puts the digits in threes, which is how a person reads a large number;
/// `at_least` pads with zeros, for the minutes of an hour.
#[must_use]
pub(crate) fn write(number: u64, grouped: bool, at_least: usize) -> String {
    written_in(super::current().base(), number, grouped, at_least)
}

/// [`write`], in a base given rather than chosen.
#[must_use]
pub(crate) fn written_in(base: Base, number: u64, grouped: bool, at_least: usize) -> String {
    let mut digits = Vec::new();
    let mut rest = number;
    loop {
        digits.push(base.digit(rest % base.radix()));
        rest /= base.radix();
        if rest == 0 {
            break;
        }
    }
    while digits.len() < at_least {
        digits.push('0');
    }
    let places = digits.len();
    let mut written = String::with_capacity(places * 2);
    for (place, digit) in digits.into_iter().rev().enumerate() {
        if grouped && place != 0 && (places - place).is_multiple_of(3) {
            written.push(',');
        }
        written.push(digit);
    }
    written
}

/// Read a number a person typed, in the base they are shown numbers in.
///
/// The commas and spaces a count is written with are allowed back in, so a number
/// copied off the screen reads as what it said.
#[must_use]
pub(crate) fn read(typed: &str) -> Option<u64> {
    read_in(super::current().base(), typed)
}

/// [`read`], in a base given rather than chosen.
#[must_use]
pub(crate) fn read_in(base: Base, typed: &str) -> Option<u64> {
    let mut number: Option<u64> = None;
    for typed in typed
        .trim()
        .chars()
        .filter(|c| !matches!(c, ',' | '_' | ' '))
    {
        let digit = base.value_of(typed)?;
        number = number
            .unwrap_or(0)
            .checked_mul(base.radix())?
            .checked_add(digit);
        number?;
    }
    number
}

/// Read one of the 333 as a person typed it. Nothing past what a signal can be.
#[must_use]
pub(crate) fn index(typed: &str) -> Option<u16> {
    read(typed).and_then(|number| u16::try_from(number).ok())
}

/// How Fluent writes a number: this client's way, in this client's base.
///
/// Every count reaches a catalog as a Fluent number, so that each language can choose
/// its own plural for it, and comes out through here.
// The signature is Fluent's: it hands over its memoizer whether or not it is wanted.
// The cast back is exact: only whole numbers reach here, and they were a u64 before
// Fluent made them an f64.
pub(super) fn fluent(
    value: &fluent_bundle::FluentValue<'_>,
    _: &intl_memoizer::concurrent::IntlLangMemoizer,
) -> Option<String> {
    let fluent_bundle::FluentValue::Number(number) = value else {
        return None;
    };
    if number.value.fract() != 0.0 || !number.value.is_finite() {
        return None;
    }
    let at_least = number.options.minimum_integer_digits.unwrap_or(0);
    let magnitude = write(
        number.value.abs() as u64,
        number.options.use_grouping,
        at_least,
    );
    Some(if number.value < 0.0 {
        format!("-{magnitude}")
    } else {
        magnitude
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twelve_is_written_with_the_two_digits_unicode_gave_it() {
        // By hand: 333 = 2·144 + 3·12 + 9. 19,683 = 11·1728 + 4·144 + 8·12 + 3.
        assert_eq!(written_in(Base::Twelve, 333, false, 0), "239");
        assert_eq!(written_in(Base::Twelve, 19_683, true, 0), "\u{218B},483");
        assert_eq!(written_in(Base::TwelveAscii, 19_683, true, 0), "E,483");
        assert_eq!(written_in(Base::Twelve, 10, false, 0), "\u{218A}");
        assert_eq!(written_in(Base::Twelve, 12, false, 0), "10");
        assert_eq!(written_in(Base::Twelve, 0, false, 0), "0");
    }

    #[test]
    fn ten_is_written_the_way_it_always_was() {
        assert_eq!(written_in(Base::Ten, 19_683, true, 0), "19,683");
        assert_eq!(written_in(Base::Ten, 19_683, false, 0), "19683");
        assert_eq!(written_in(Base::Ten, 5, false, 2), "05");
        assert_eq!(written_in(Base::Ten, 1_234_567, true, 0), "1,234,567");
    }

    #[test]
    fn a_typed_number_is_read_in_the_base_it_would_be_shown_in() {
        assert_eq!(read_in(Base::Twelve, "239"), Some(333));
        assert_eq!(read_in(Base::Twelve, "\u{218B},483"), Some(19_683));
        assert_eq!(read_in(Base::Twelve, "e483"), Some(19_683));
        assert_eq!(read_in(Base::Twelve, "X"), Some(10));
        assert_eq!(read_in(Base::Ten, "42"), Some(42));
        assert_eq!(read_in(Base::Ten, "X"), None, "X is not a digit in ten");
        assert_eq!(read_in(Base::Twelve, ""), None, "nothing typed is not zero");
        assert_eq!(read_in(Base::Ten, "99999999999999999999999"), None);
    }

    #[test]
    fn every_base_reads_back_from_its_own_name() {
        for base in [Base::Ten, Base::Twelve, Base::TwelveAscii] {
            assert_eq!(Base::named(base.name()), Ok(base));
        }
    }

    #[test]
    fn a_base_is_named_the_way_a_person_would_name_it() {
        assert_eq!(Base::named("Twelve"), Ok(Base::Twelve));
        assert_eq!(Base::named("twelve-ascii"), Ok(Base::TwelveAscii));
        assert!(Base::named("eight").is_err());
        assert_eq!(Base::Twelve.shown(false), Base::TwelveAscii);
        assert_eq!(Base::Ten.shown(false), Base::Ten);
    }
}
