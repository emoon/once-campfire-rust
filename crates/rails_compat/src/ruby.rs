//! Ruby's integer parsing of request strings: `String#to_i`, and the Active Record cast built on
//! it that decides which row an id parameter finds.

use std::cmp::Ordering;

/// The characters `String#to_i` skips before the number (Ruby's `ISSPACE`).
const RUBY_SPACE: [char; 6] = [' ', '\t', '\n', '\u{b}', '\u{c}', '\r'];

/// `String#to_i`: leading whitespace, a sign, an optional `0d` prefix, then digits with single
/// underscores between them. Anything else ends the number, and no number is 0. Where Ruby would
/// return a Bignum this saturates.
pub fn to_i(value: &str) -> i64 {
    let (negative, digits) = split_sign(value);
    signed(negative, digits).unwrap_or(if negative { i64::MIN } else { i64::MAX })
}

/// How Active Record casts a string for an integer column condition, as in `find(params[:id])`
/// or `find_by(id: cookies[:last_room])` (`ActiveModel::Type::Integer#serialize`): `to_i`, except
/// that a string not starting like a number (`non_numeric_string?`, `/\A\s*[+-]?\d/`) matches
/// nothing, and neither does a value out of the column's range. That range is i64's: SQLite's
/// integer type has an 8-byte limit (`SQLite3Adapter::SQLite3Integer#_limit`; `Type::Integer`'s
/// own default is 4).
pub fn cast_integer(value: &str) -> Option<i64> {
    let (negative, digits) = split_sign(value);
    if !digits.starts_with(|c: char| c.is_ascii_digit()) {
        return None;
    }
    signed(negative, digits)
}

/// `a.to_i <=> b.to_i`, exact where `to_i` saturates.
pub fn cmp_to_i(a: &str, b: &str) -> Ordering {
    let (a_negative, a) = significant_digits(a);
    let (b_negative, b) = significant_digits(b);
    match (a_negative, b_negative) {
        (false, true) => Ordering::Greater,
        (true, false) => Ordering::Less,
        (false, false) => cmp_magnitudes(a, b),
        (true, true) => cmp_magnitudes(b, a),
    }
}

/// Skips `to_i`'s leading whitespace, then splits off the sign.
fn split_sign(value: &str) -> (bool, &str) {
    let value = value.trim_start_matches(RUBY_SPACE);
    match value.as_bytes().first() {
        Some(b'-') => (true, &value[1..]),
        Some(b'+') => (false, &value[1..]),
        _ => (false, value),
    }
}

/// The number after the sign, `None` if it doesn't fit in an `i64`.
fn signed(negative: bool, digits: &str) -> Option<i64> {
    let magnitude = magnitude(digits)?;
    if negative {
        0i64.checked_sub_unsigned(magnitude)
    } else {
        i64::try_from(magnitude).ok()
    }
}

fn magnitude(digits: &str) -> Option<u64> {
    decimal_digits(digits).try_fold(0u64, |number, digit| number.checked_mul(10)?.checked_add(u64::from(digit)))
}

/// The sign and the digits of `value.to_i` without leading zeros. Zero has no sign.
fn significant_digits(value: &str) -> (bool, impl Iterator<Item = u8> + Clone + '_) {
    let (negative, digits) = split_sign(value);
    let mut digits = decimal_digits(digits).skip_while(|&digit| digit == 0).peekable();
    (negative && digits.peek().is_some(), digits)
}

fn cmp_magnitudes(a: impl Iterator<Item = u8> + Clone, b: impl Iterator<Item = u8> + Clone) -> Ordering {
    a.clone().count().cmp(&b.clone().count()).then_with(|| a.cmp(b))
}

/// The digits of the number after the sign: an optional `0d` prefix, then digits with single
/// underscores between them.
fn decimal_digits(digits: &str) -> impl Iterator<Item = u8> + Clone + '_ {
    let digits = digits.strip_prefix("0d").or_else(|| digits.strip_prefix("0D")).unwrap_or(digits);
    let mut previous_digit = false;
    digits
        .bytes()
        .map_while(move |byte| match byte {
            b'0'..=b'9' => {
                previous_digit = true;
                Some(Some(byte - b'0'))
            }
            b'_' if previous_digit => {
                previous_digit = false;
                Some(None)
            }
            _ => None,
        })
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Expected values are Ruby 3.4's `String#to_i`, and whether `Room.find_by(id:)` with rooms 5,
    // 10 and 56 found one, on the reference image.

    #[test]
    fn to_i_like_ruby() {
        assert_eq!(to_i("1717243200000"), 1717243200000);
        assert_eq!(to_i(" +12abc"), 12);
        assert_eq!(to_i("\t\n\u{b}\u{c}\r 7"), 7);
        assert_eq!(to_i("\u{a0}5"), 0);
        assert_eq!(to_i("abc"), 0);
        assert_eq!(to_i(""), 0);
        assert_eq!(to_i("-5"), -5);
        assert_eq!(to_i("--5"), 0);
        assert_eq!(to_i("- 5"), 0);
        assert_eq!(to_i("5_6"), 56);
        assert_eq!(to_i("1_0_"), 10);
        assert_eq!(to_i("5__6"), 5);
        assert_eq!(to_i("_5"), 0);
        assert_eq!(to_i("0__5"), 0);
        assert_eq!(to_i("0_5"), 5);
        assert_eq!(to_i("-0d5"), -5);
        assert_eq!(to_i("0D5"), 5);
        assert_eq!(to_i("0d_5"), 0);
        assert_eq!(to_i("00d5"), 0);
        assert_eq!(to_i("0x5"), 0);
        assert_eq!(to_i("0b1"), 0);
    }

    #[test]
    fn to_i_saturates_past_i64() {
        assert_eq!(to_i("9223372036854775807"), i64::MAX);
        assert_eq!(to_i("99999999999999999999"), i64::MAX);
        assert_eq!(to_i("-9223372036854775808"), i64::MIN);
        assert_eq!(to_i("-99999999999999999999"), i64::MIN);
    }

    #[test]
    fn compares_to_i_exactly() {
        assert_eq!(cmp_to_i("9223372036854775808", "9223372036854775807"), Ordering::Greater);
        assert_eq!(cmp_to_i("99999999999999999998", "99999999999999999999"), Ordering::Less);
        assert_eq!(cmp_to_i("-99999999999999999999", "-99999999999999999998"), Ordering::Less);
        assert_eq!(cmp_to_i("-99999999999999999999", "99999999999999999998"), Ordering::Less);
        assert_eq!(cmp_to_i("0000000000000000000000002", "10"), Ordering::Less);
        assert_eq!(cmp_to_i("1_0_", "0d10"), Ordering::Equal);
        assert_eq!(cmp_to_i("-0", "abc"), Ordering::Equal);
        assert_eq!(cmp_to_i("-1", "0"), Ordering::Less);
    }

    #[test]
    fn casts_integers_like_active_record() {
        assert_eq!(cast_integer("5"), Some(5));
        assert_eq!(cast_integer("12abc"), Some(12));
        assert_eq!(cast_integer(" -3"), Some(-3));
        assert_eq!(cast_integer("\t\n\u{b}\u{c}\r5"), Some(5));
        assert_eq!(cast_integer("+5"), Some(5));
        assert_eq!(cast_integer("1_0"), Some(10));
        assert_eq!(cast_integer("0_5"), Some(5));
        assert_eq!(cast_integer("5__6"), Some(5));
        assert_eq!(cast_integer("0d5"), Some(5));
        assert_eq!(cast_integer("0"), Some(0));
        assert_eq!(cast_integer("abc"), None);
        assert_eq!(cast_integer(""), None);
        assert_eq!(cast_integer("_5"), None);
        assert_eq!(cast_integer("--5"), None);
        assert_eq!(cast_integer("\u{a0}5"), None);
        assert_eq!(cast_integer("\u{2003}5"), None);
    }

    #[test]
    fn cast_integer_is_none_out_of_range() {
        assert_eq!(cast_integer("9223372036854775807"), Some(i64::MAX));
        assert_eq!(cast_integer("-9223372036854775808"), Some(i64::MIN));
        assert_eq!(cast_integer("9223372036854775808"), None);
        assert_eq!(cast_integer("99999999999999999999"), None);
    }
}
