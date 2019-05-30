use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseNameError {
    TooLong,
    BadChar(char),
}

impl Error for ParseNameError {}

/// All possible characters that can be used in EOSIO names.
pub const NAME_CHARS: [u8; 32] = *b".12345abcdefghijklmnopqrstuvwxyz";

/// The maximum character length of an EOSIO name.
pub const MAX_NAME_LEN: usize = 12;

impl fmt::Display for ParseNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // ParseNameError::IsEmpty => write!(f, "empty string is not a valid EOSIO name"),
            ParseNameError::TooLong => write!(
                f,
                "name is too long, must be {} chars or less",
                MAX_NAME_LEN
            ),
            ParseNameError::BadChar(c) => {
                write!(f, "name contains invalid character '{}'", c)
            }
        }
    }
}

/// Attempts to create an EOSIO name from a `&str`.
///
/// # Examples
///
/// ```
/// use eosio_numstr::{name_from_str, ParseNameError};
/// assert_eq!(name_from_str("eosio.token"), Ok(6138663591592764928));
/// assert_eq!(name_from_str("eosio.bpay"), Ok(6138663581940940800));
/// assert_eq!(name_from_str("A"), Err(ParseNameError::BadChar('A')));
/// assert_eq!(name_from_str("1234512345123"), Err(ParseNameError::TooLong));
/// ```
pub fn name_from_str(s: &str) -> Result<u64, ParseNameError> {
    name_from_iter(s.chars())
}

/// Attempts to create an EOSIO name from an `Iterator`.
///
/// # Examples
///
/// ```
/// use eosio_numstr::{name_from_iter, ParseNameError};
/// assert_eq!(name_from_iter("eosio.token".chars()), Ok(6138663591592764928));
/// assert_eq!(name_from_iter("eosio.bpay".chars()), Ok(6138663581940940800));
/// assert_eq!(name_from_iter("A".chars()), Err(ParseNameError::BadChar('A')));
/// assert_eq!(name_from_iter("1234512345123".chars()), Err(ParseNameError::TooLong));
/// ```
pub fn name_from_iter<I>(iter: I) -> Result<u64, ParseNameError>
where
    I: Iterator<Item = char>,
{
    let mut value = 0;
    for (i, c) in iter.enumerate() {
        if i == MAX_NAME_LEN {
            return Err(ParseNameError::TooLong);
        } else if c == '.' {
            continue;
        }
        match char_to_symbol(c) {
            Some(symbol) => {
                let mut n = symbol as u64;
                if i < MAX_NAME_LEN {
                    n &= 31u64;
                    n <<= 64 - 5 * (i + 1);
                } else {
                    n &= 15u64;
                }
                value |= n;
            }
            None => {
                return Err(ParseNameError::BadChar(c));
            }
        }
    }

    Ok(value)
}

fn char_to_symbol(c: char) -> Option<char> {
    if c >= 'a' && c <= 'z' {
        ::std::char::from_u32((c as u32 - 'a' as u32) + 6)
    } else if c >= '1' && c <= '5' {
        ::std::char::from_u32((c as u32 - '1' as u32) + 1)
    } else {
        None
    }
}

/// Converts an EOSIO name value into a String.
///
/// # Examples
///
/// ```
/// use eosio_numstr::name_to_string;
/// assert_eq!(name_to_string(6138663591592764928), "eosio.token");
/// assert_eq!(name_to_string(6138663581940940800), "eosio.bpay");
/// ```
pub fn name_to_string(name: u64) -> String {
    let mut chars = [b'.'; 13]; // TODO: make this 12 instead of 13
    let mut t = name;
    for (i, c) in chars.iter_mut().rev().enumerate() {
        let charmap_index = t & if i == 0 { 15 } else { 31 };
        *c = NAME_CHARS[charmap_index as usize];
        t >>= if i == 0 { 4 } else { 5 };
    }
    String::from_utf8_lossy(&chars)
        .trim_matches('.')
        .to_string()
}

macro_rules! test_name_from_str {
    ($($n:ident, $i:expr, $o:expr)*) => ($(
        #[cfg(test)]
        #[test]
        fn $n() {
            assert_eq!(name_from_str($i), $o);
        }
    )*)
}

test_name_from_str!(
    from_str_empty, "", Ok(0)
    from_str_single_char, "a", Ok(3_458_764_513_820_540_928)
    from_str_bad_number, "123456789012", Err(ParseNameError::BadChar('6'))
    from_str_only_numbers, "123451234512", Ok(614_251_535_012_020_768)
    from_str_too_long, "1234512345123", Err(ParseNameError::TooLong)
    from_str_uppercase, "TEST", Err(ParseNameError::BadChar('T'))
    from_str_only_letters, "test", Ok(14_605_613_396_213_628_928)
    from_str_special_char, "test!", Err(ParseNameError::BadChar('!'))
    from_str_with_periods, "a.b.c", Ok(3_462_709_561_541_001_216)
    from_str_with_only_periods, "...", Ok(0) // TODO is this valid?
);

macro_rules! test_name_to_string {
    ($($n:ident, $i:expr, $o:expr)*) => ($(
        #[cfg(test)]
        #[test]
        fn $n() {
            assert_eq!(name_to_string($i), $o);
        }
    )*)
}

test_name_to_string!(
    to_string_single_char, 3_458_764_513_820_540_928, "a"
    to_string_only_numbers, 614_251_535_012_020_768, "123451234512"
    to_string_only_letters, 14_605_613_396_213_628_928, "test"
    to_string_zero, 0, ""
    to_string_with_periods, 3_462_709_561_541_001_216, "a.b.c"
);
