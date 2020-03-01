use core::fmt;

/// All possible characters that can be used in EOSIO symbol codes.
pub const SYMBOL_CODE_CHARS: [u8; 26] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// The maximum allowed length of EOSIO symbol codes.
pub const SYMBOL_CODE_MAX_LEN: usize = 7;

/// An error which can be returned when parsing an EOSIO symbol.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseSymbolCodeError {
    /// The symbol is too long. Symbols must be 7 characters or less.
    TooLong,
    /// The symbol contains an invalid character. Symbols can only contain
    /// uppercase letters A-Z.
    BadChar(u8),
}

impl fmt::Display for ParseSymbolCodeError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::TooLong => {
                write!(f, "symbol is too long, must be 7 chars or less")
            }
            Self::BadChar(c) => write!(
                f,
                "symbol contains invalid character '{}'",
                char::from(c)
            ),
        }
    }
}

/// Attempts to create an EOSIO symbol from an `Iterator`.
///
/// # Errors
///
/// Will return `Err` if the symbol contains invalid characters, is too long, or
/// is empty.
///
/// # Examples
///
/// ```
/// use eosio_numstr::{symbol_code_from_bytes, ParseSymbolCodeError};
/// assert_eq!(symbol_code_from_bytes("EOS".bytes()), Ok(5459781));
/// assert_eq!(symbol_code_from_bytes("TGFT".bytes()), Ok(1413891924));
/// assert_eq!(symbol_code_from_bytes("SYS".bytes()), Ok(5462355));
/// assert_eq!(
///     symbol_code_from_bytes("TSt".bytes()),
///     Err(ParseSymbolCodeError::BadChar(b't'))
/// );
/// assert_eq!(
///     symbol_code_from_bytes("TESTING".bytes()),
///     Ok(20070800200779092)
/// );
/// assert_eq!(
///     symbol_code_from_bytes("TESTINGG".bytes()),
///     Err(ParseSymbolCodeError::TooLong)
/// );
/// ```
#[inline]
pub fn symbol_code_from_bytes<I>(iter: I) -> Result<u64, ParseSymbolCodeError>
where
    I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
{
    let mut value = 0_u64;
    for (i, c) in iter.enumerate().rev() {
        if i == SYMBOL_CODE_MAX_LEN {
            return Err(ParseSymbolCodeError::TooLong);
        } else if c < b'A' || c > b'Z' {
            return Err(ParseSymbolCodeError::BadChar(c));
        } else {
            value <<= 8;
            value |= u64::from(c);
        }
    }
    Ok(value)
}

/// Converts an EOSIO symbol into an array of UTF-8 characters.
///
/// # Examples
///
/// ```
/// use eosio_numstr::symbol_code_to_bytes;
/// assert_eq!(symbol_code_to_bytes(5459781), *b"EOS    ");
/// assert_eq!(symbol_code_to_bytes(20070800200779092), *b"TESTING");
/// assert_eq!(symbol_code_to_bytes(1413891924), *b"TGFT   ");
/// assert_eq!(symbol_code_to_bytes(5462355), *b"SYS    ");
/// assert_eq!(symbol_code_to_bytes(0), *b"       ");
/// ```
#[inline]
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn symbol_code_to_bytes(value: u64) -> [u8; SYMBOL_CODE_MAX_LEN] {
    let mut chars = [b' '; SYMBOL_CODE_MAX_LEN];
    let mut v = value;
    for c in &mut chars {
        if v == 0 {
            break;
        }
        *c = (v & 0xFF) as u8;
        v >>= 8;
    }
    chars
}

#[cfg(test)]
mod tests {
    use super::{symbol_code_from_bytes, symbol_code_to_bytes};
    use core::str;
    use proptest::prelude::*;

    #[test]
    fn from_bytes_to_bytes() {
        proptest!(|(input in "[A-Z]{0,7}")| {
            let value = symbol_code_from_bytes(input.bytes()).unwrap();
            let bytes = symbol_code_to_bytes(value);
            let string = str::from_utf8(&bytes).unwrap();
            prop_assert_eq!(string, format!("{:<7}", input));
        });
    }

    #[test]
    fn from_bytes_too_long() {
        proptest!(|(input in "[A-Z]{8}")| {
            if symbol_code_from_bytes(input.bytes()).is_ok() {
                panic!("Should've gotten TooLong error with input '{}'", input);
            };
        });
    }

    #[test]
    fn to_bytes_doesnt_crash() {
        proptest!(|(input in 0_u64..)| {
            let _ = symbol_code_to_bytes(input);
        });
    }
}
