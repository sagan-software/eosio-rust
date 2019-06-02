//! Types and functions related to parsing/formating EOSIO names.

use std::convert::TryFrom;
use std::error::Error;
use std::fmt;

/// All possible characters that can be used in EOSIO names.
pub const NAME_UTF8_CHARS: [u8; 32] = *b".12345abcdefghijklmnopqrstuvwxyz";

/// The maximum character length of an EOSIO name.
pub const NAME_LEN_MAX: usize = 12;

/// An error which can be returned when parsing an EOSIO name.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseNameError {
    /// The name is over the maximum allowed length.
    TooLong,
    /// The name contains an unallowed character.
    BadChar(char),
}

impl Error for ParseNameError {}

impl fmt::Display for ParseNameError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseNameError::TooLong => write!(
                f,
                "name is too long, must be {} chars or less",
                NAME_LEN_MAX
            ),
            ParseNameError::BadChar(c) => write!(
                f,
                "name contains invalid character '{}'; must only contain the following characters: {}",
                c,
                String::from_utf8_lossy(&NAME_UTF8_CHARS)
            ),
        }
    }
}

/// Attempts to create an EOSIO name from a `&str`.
///
/// # Examples
///
/// ```
/// use eosio_numstr::{name_from_str, ParseNameError};
/// assert_eq!(name_from_str(""), Ok(0));
/// assert_eq!(name_from_str("a"), Ok(3458764513820540928));
/// assert_eq!(name_from_str("123456789012"), Err(ParseNameError::BadChar('6')));
/// assert_eq!(name_from_str("123451234512"), Ok(614251535012020768));
/// assert_eq!(name_from_str("1234512345123"), Err(ParseNameError::TooLong));
/// assert_eq!(name_from_str("eosio.token"), Ok(6138663591592764928));
/// assert_eq!(name_from_str("eosio.bpay"), Ok(6138663581940940800));
/// assert_eq!(name_from_str("A"), Err(ParseNameError::BadChar('A')));
/// assert_eq!(name_from_str("TEST"), Err(ParseNameError::BadChar('T')));
/// ```
#[inline]
pub fn name_from_str(value: &str) -> Result<u64, ParseNameError> {
    name_from_chars(value.chars())
}

/// Attempts to create an EOSIO name from an `Iterator`.
///
/// # Examples
///
/// ```
/// use eosio_numstr::{name_from_chars, ParseNameError};
/// assert_eq!(name_from_chars("".chars()), Ok(0));
/// assert_eq!(name_from_chars("a".chars()), Ok(3458764513820540928));
/// assert_eq!(name_from_chars("123456789012".chars()), Err(ParseNameError::BadChar('6')));
/// assert_eq!(name_from_chars("123451234512".chars()), Ok(614251535012020768));
/// assert_eq!(name_from_chars("1234512345123".chars()), Err(ParseNameError::TooLong));
/// assert_eq!(name_from_chars("eosio.token".chars()), Ok(6138663591592764928));
/// assert_eq!(name_from_chars("eosio.bpay".chars()), Ok(6138663581940940800));
/// assert_eq!(name_from_chars("A".chars()), Err(ParseNameError::BadChar('A')));
/// assert_eq!(name_from_chars("TEST".chars()), Err(ParseNameError::BadChar('T')));
/// ```
#[inline]
pub fn name_from_chars<I>(chars: I) -> Result<u64, ParseNameError>
where
    I: Iterator<Item = char>,
{
    let mut value = 0;
    for (i, c) in chars.enumerate() {
        if i == NAME_LEN_MAX {
            return Err(ParseNameError::TooLong);
        } else if c == '.' {
            continue;
        } else if let Some(symbol) = char_to_symbol(c) {
            let mut n = symbol as u64;
            if i < NAME_LEN_MAX {
                n &= 31;
                n <<= 64 - 5 * (i + 1);
            } else {
                n &= 15;
            }
            value |= n;
        } else {
            return Err(ParseNameError::BadChar(c));
        }
    }

    Ok(value)
}

/// Converts a character to a symbol.
fn char_to_symbol(c: char) -> Option<char> {
    if c >= 'a' && c <= 'z' {
        ::std::char::from_u32((c as u32 - 'a' as u32) + 6)
    } else if c >= '1' && c <= '5' {
        ::std::char::from_u32((c as u32 - '1' as u32) + 1)
    } else {
        None
    }
}

/// Converts an EOSIO name value into a string.
///
/// # Examples
///
/// ```
/// use eosio_numstr::name_to_string;
/// assert_eq!(name_to_string(6138663591592764928), "eosio.token");
/// assert_eq!(name_to_string(6138663581940940800), "eosio.bpay");
/// assert_eq!(name_to_string(0), "");
/// assert_eq!(name_to_string(614251535012020768), "123451234512");
/// ```
#[inline]
pub fn name_to_string(name: u64) -> String {
    String::from_utf8_lossy(&name_to_utf8(name))
        .trim_matches('.')
        .into()
}

/// Converts an EOSIO name into an array of UTF-8 characters.
///
/// # Examples
///
/// ```
/// use eosio_numstr::name_to_utf8;
/// assert_eq!(name_to_utf8(6138663591592764928), *b"eosio.token..");
/// assert_eq!(name_to_utf8(6138663581940940800), *b"eosio.bpay...");
/// assert_eq!(name_to_utf8(0), *b".............");
/// assert_eq!(name_to_utf8(614251535012020768), *b"123451234512.");
/// ```
#[inline]
pub fn name_to_utf8(name: u64) -> [u8; 13] {
    let mut chars = [b'.'; 13]; // TODO: make this 12 instead of 13
    let mut t = name;
    for (i, c) in chars.iter_mut().rev().enumerate() {
        let index = t & if i == 0 { 15 } else { 31 };
        let index = usize::try_from(index).unwrap_or_default();
        if let Some(v) = NAME_UTF8_CHARS.get(index) {
            *c = *v;
        }
        t >>= if i == 0 { 4 } else { 5 };
    }
    chars
}
