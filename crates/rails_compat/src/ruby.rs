//! Ruby's integer parsing of request strings: `String#to_i`, and the Active Record cast built on
//! it that decides which row an id parameter finds.

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
    let digits = digits.strip_prefix("0d").or_else(|| digits.strip_prefix("0D")).unwrap_or(digits);
    let mut number: u64 = 0;
    let mut previous_digit = false;
    for byte in digits.bytes() {
        match byte {
            b'0'..=b'9' => {
                number = number.checked_mul(10)?.checked_add(u64::from(byte - b'0'))?;
                previous_digit = true;
            }
            b'_' if previous_digit => previous_digit = false,
            _ => break,
        }
    }
    Some(number)
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
