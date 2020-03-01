use core::{char, convert::TryFrom, fmt};

/// All possible characters that can be used in EOSIO names.
pub const NAME_CHARS: [u8; 32] = *b".12345abcdefghijklmnopqrstuvwxyz";

/// The maximum character length of an EOSIO name.
pub const NAME_MAX_LEN: usize = 13;

/// An error which can be returned when parsing an EOSIO name.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParseNameError {
    /// The name contains a disallowed character.
    BadChar(u8),
    /// The name is over the maximum allowed length.
    TooLong,
}

impl fmt::Display for ParseNameError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::BadChar(c) => {
                write!(f, "name contains invalid character '{}'", char::from(c))
            }
            Self::TooLong => {
                write!(f, "name is too long, must be 13 chars or less")
            }
        }
    }
}

/// Attempts to create an EOSIO name from an `Iterator`.
///
/// # Errors
///
/// Will return `Err` if the name contains invalid characters or is too long.
///
/// # Examples
///
/// ```
/// use eosio_numstr::{name_from_bytes, ParseNameError};
/// assert_eq!(name_from_bytes("".bytes()), Ok(0));
/// assert_eq!(name_from_bytes("a".bytes()), Ok(3458764513820540928));
/// assert_eq!(
///     name_from_bytes("123456789012".bytes()),
///     Err(ParseNameError::BadChar(b'6'))
/// );
/// assert_eq!(
///     name_from_bytes("123451234512".bytes()),
///     Ok(614251535012020768)
/// );
/// assert_eq!(
///     name_from_bytes("123451234512j".bytes()),
///     Ok(614251535012020783)
/// );
/// assert_eq!(
///     name_from_bytes("123451234512k".bytes()),
///     Err(ParseNameError::BadChar(b'k'))
/// );
/// assert_eq!(
///     name_from_bytes("12345123451234".bytes()),
///     Err(ParseNameError::TooLong)
/// );
/// assert_eq!(
///     name_from_bytes("eosio.token".bytes()),
///     Ok(6138663591592764928)
/// );
/// assert_eq!(
///     name_from_bytes("eosio.token".bytes()),
///     Ok(6138663591592764928)
/// );
/// assert_eq!(
///     name_from_bytes("eosio.bpay".bytes()),
///     Ok(6138663581940940800)
/// );
/// assert_eq!(
///     name_from_bytes("A".bytes()),
///     Err(ParseNameError::BadChar(b'A'))
/// );
/// assert_eq!(
///     name_from_bytes("TEST".bytes()),
///     Err(ParseNameError::BadChar(b'T'))
/// );
/// ```
#[inline]
pub fn name_from_bytes<I>(mut iter: I) -> Result<u64, ParseNameError>
where
    I: Iterator<Item = u8>,
{
    let mut value = 0_u64;
    let mut len = 0_u64;

    // Loop through up to 12 characters
    while let Some(c) = iter.next() {
        let v = char_to_value(c).ok_or_else(|| ParseNameError::BadChar(c))?;
        value <<= 5;
        value |= u64::from(v);
        len += 1;

        if len == 12 {
            break;
        }
    }

    if len == 0 {
        return Ok(0);
    }

    value <<= 4 + 5 * (12 - len);

    // Check if we have a 13th character
    if let Some(c) = iter.next() {
        let v = char_to_value(c).ok_or_else(|| ParseNameError::BadChar(c))?;

        // The 13th character can only be 4 bits, it has to be between letters
        // 'a' to 'j'
        if v > 0x0F {
            return Err(ParseNameError::BadChar(c));
        }

        value |= u64::from(v);

        // Check if we have more than 13 characters
        if iter.next().is_some() {
            return Err(ParseNameError::TooLong);
        }
    }

    Ok(value)
}

/// Converts a character to a symbol.
fn char_to_value(c: u8) -> Option<u8> {
    if c == b'.' {
        Some(0)
    } else if c >= b'1' && c <= b'5' {
        Some(c - b'1' + 1)
    } else if c >= b'a' && c <= b'z' {
        Some(c - b'a' + 6)
    } else {
        None
    }
}

/// Converts an EOSIO name into an array of UTF-8 characters.
///
/// # Examples
///
/// ```
/// use eosio_numstr::name_to_bytes;
/// assert_eq!(name_to_bytes(6138663591592764928), *b"eosio.token..");
/// assert_eq!(name_to_bytes(6138663581940940800), *b"eosio.bpay...");
/// assert_eq!(name_to_bytes(0), *b".............");
/// assert_eq!(name_to_bytes(614251535012020768), *b"123451234512.");
/// assert_eq!(name_to_bytes(614251535012020783), *b"123451234512j");
/// ```
#[inline]
#[must_use]
pub fn name_to_bytes(value: u64) -> [u8; NAME_MAX_LEN] {
    let mut chars = [b'.'; NAME_MAX_LEN];
    if value == 0 {
        return chars;
    }

    let mask = 0xF800_0000_0000_0000;
    let mut v = value;
    for (i, c) in chars.iter_mut().enumerate() {
        let index = (v & mask) >> (if i == 12 { 60 } else { 59 });
        let index = usize::try_from(index).unwrap_or_default();
        if let Some(v) = NAME_CHARS.get(index) {
            *c = *v;
        }
        v <<= 5;
    }
    chars
}

#[cfg(test)]
mod tests {
    use super::{char, name_from_bytes, name_to_bytes, ParseNameError};
    use core::str;
    use proptest::prelude::*;

    #[test]
    fn from_bytes_to_bytes() {
        proptest!(|(input in "[[1-5][a-z]\\.]{0,12}[[1-5][a-j]\\.]{0,1}")| {
            let name = match name_from_bytes(input.bytes()) {
                Ok(name) => name,
                Err(error) => panic!("Failed with input '{}': {}", input, error),
            };
            let bytes = name_to_bytes(name);
            let string = str::from_utf8(&bytes).expect("Failed to convert bytes into str");
            prop_assert_eq!(
                string,
                format!("{:.<13}", input),
                "Names don't match"
            );
        });
    }

    #[test]
    fn from_bytes_too_long() {
        proptest!(|(input in "[[1-5][a-z]\\.]{12}[[1-5][a-j]\\.]{2}")| {
            let result = name_from_bytes(input.bytes());
            prop_assert_eq!(
                result,
                Err(ParseNameError::TooLong),
                "Should've gotten TooLong error"
            );
        });
    }

    #[test]
    fn from_bytes_bad_char() {
        proptest!(|(
            input in "[[1-5][a-z]\\.]{11}",
            bad_char in "[^[1-5][a-z]\\.]{1}"
        )| {
            let input = input + &bad_char;
            let result = name_from_bytes(input.bytes());
            let bad_char = bad_char.bytes().next().unwrap();
            prop_assert_eq!(
                result,
                Err(ParseNameError::BadChar(bad_char)),
                "Should've gotten BadChar error with char '{}'",
                char::from(bad_char)
            );
        });
    }

    #[test]
    fn from_bytes_bad_last_char() {
        proptest!(|(
            input in "[[1-5][a-z]\\.]{12}",
            bad_char in "[^[1-5][a-j]\\.]{1}"
        )| {
            let input = input + &bad_char;
            let result = name_from_bytes(input.bytes());
            let bad_char = bad_char.bytes().next().unwrap();
            prop_assert_eq!(
                result,
                Err(ParseNameError::BadChar(bad_char)),
                "Should've gotten BadChar error with char '{}'",
                char::from(bad_char)
            );
        });
    }

    #[test]
    fn to_bytes_doesnt_crash() {
        proptest!(|(input in 0_u64..)| {
            let _ = name_to_bytes(input);
        });
    }
}
