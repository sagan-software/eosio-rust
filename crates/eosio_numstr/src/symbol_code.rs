use core::{convert::TryFrom, fmt};

/// All possible characters that can be used in EOSIO symbol codes.
pub const SYMBOL_CODE_CHARS: [u8; 26] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// The maximum allowed length of EOSIO symbol codes.
pub const SYMBOL_CODE_MAX_LEN: usize = 7;

/// An error which can be returned when parsing an EOSIO symbol.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseSymbolCodeError {
    /// The symbol is too long. Symbols must be 7 characters or less.
    TooLong,
    /// The symbol contains an invalid character. Symbols can only contain uppercase letters A-Z.
    BadChar(u8),
}

impl fmt::Display for ParseSymbolCodeError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::TooLong => {
                write!(f, "symbol is too long, must be 7 chars or less")
            }
            Self::BadChar(c) => {
                write!(f, "symbol contains invalid character '{}'", c)
            }
        }
    }
}

/// Attempts to create an EOSIO symbol from an `Iterator`.
///
/// # Errors
///
/// Will return `Err` if the symbol contains invalid characters, is too long, or is empty.
///
/// # Examples
///
/// ```
/// use eosio_numstr::{symbol_code_from_bytes, ParseSymbolCodeError};
/// assert_eq!(symbol_code_from_bytes("EOS".bytes()), Ok(1397703936));
/// assert_eq!(symbol_code_from_bytes("TGFT".bytes()), Ok(361956332544));
/// assert_eq!(symbol_code_from_bytes("SYS".bytes()), Ok(1398362880));
/// assert_eq!(symbol_code_from_bytes("TSt".bytes()), Err(ParseSymbolCodeError::BadChar(b't')));
/// assert_eq!(symbol_code_from_bytes("TESTING".bytes()), Ok(5138124851399447552));
/// assert_eq!(symbol_code_from_bytes("TESTINGG".bytes()), Err(ParseSymbolCodeError::TooLong));
/// ```
#[inline]
pub fn symbol_code_from_bytes<I>(iter: I) -> Result<u64, ParseSymbolCodeError>
where
    I: Iterator<Item = u8>,
{
    let mut value = 0_u64;
    for (i, c) in iter.enumerate() {
        if i == 7 {
            return Err(ParseSymbolCodeError::TooLong);
        } else if c < b'A' || c > b'Z' {
            return Err(ParseSymbolCodeError::BadChar(c));
        } else {
            value |= u64::from(c) << (8 * (i + 1));
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
/// assert_eq!(symbol_code_to_bytes(1397703940), *b"EOS    ");
/// assert_eq!(symbol_code_to_bytes(5138124851399447552), *b"TESTING");
/// assert_eq!(symbol_code_to_bytes(361956332544), *b"TGFT   ");
/// assert_eq!(symbol_code_to_bytes(1398362882), *b"SYS    ");
/// assert_eq!(symbol_code_to_bytes(0), *b"       ");
/// ```
#[inline]
#[must_use]
pub fn symbol_code_to_bytes(value: u64) -> [u8; 7] {
    let mut chars = [b' '; 7];
    let mut v = value;
    for c in &mut chars {
        v >>= 8;
        if v == 0 {
            break;
        }
        *c = u8::try_from(v & 0xff).unwrap_or_default();
    }
    chars
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn from_to_bytes(input in "[A-Z]{1,7}") {
            let value = match symbol_code_from_bytes(input.bytes()) {
                Ok(value) => value,
                Err(error) => panic!("Failed with input '{}': {}", input, error),
            };
            let bytes = symbol_code_to_bytes(value);
            let string = str::from_utf8(&bytes).expect("Failed to convert bytes into str");
            prop_assert_eq!(
                string,
                format!("{:7}", input)
            );
        }

        #[test]
        fn from_bytes_too_long(input in "[A-Z]{8}") {
            if symbol_code_from_bytes(input.bytes()).is_ok() {
                panic!("Should've gotten TooLong error with input '{}'", input);
            };
        }

        #[test]
        fn to_bytes_doesnt_crash(input in 0_u64..) {
            let _ = symbol_code_to_bytes(input);
        }
    }
}
