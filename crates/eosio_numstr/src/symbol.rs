use crate::{symbol_code_from_bytes, ParseSymbolCodeError};
use core::{fmt, num::ParseIntError};

/// An error which can be returned when parsing an EOSIO symbol.
#[derive(Debug, PartialEq, Clone)]
pub enum ParseSymbolError {
    /// The symbol precision couldn't be parsed.
    Precision(ParseIntError),
    /// The symbol code is too long.
    CodeTooLong,
    /// The symbol is not in the correct format
    BadFormat,
    /// The symbol contains an invalid character.
    BadChar(u8),
}

impl fmt::Display for ParseSymbolError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Precision(e) => {
                write!(f, "symbol precision couldn't be parsed: {}", e)
            }
            Self::CodeTooLong => {
                write!(f, "symbol code is too long, must be 7 chars or less")
            }
            Self::BadFormat => write!(
                f,
                "symbol is not in the correct format, should be similar to: \
                 4,EOS"
            ),
            Self::BadChar(c) => write!(
                f,
                "symbol contains invalid character '{}'",
                char::from(*c)
            ),
        }
    }
}

impl From<ParseSymbolCodeError> for ParseSymbolError {
    fn from(err: ParseSymbolCodeError) -> Self {
        match err {
            ParseSymbolCodeError::TooLong => Self::CodeTooLong,
            ParseSymbolCodeError::BadChar(c) => Self::BadChar(c),
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
/// use eosio_numstr::{symbol_from_bytes, ParseSymbolError};
/// assert_eq!(symbol_from_bytes(4, "EOS".bytes()), Ok(1397703940));
/// assert_eq!(symbol_from_bytes(0, "TGFT".bytes()), Ok(361956332544));
/// assert_eq!(symbol_from_bytes(2, "SYS".bytes()), Ok(1398362882));
/// assert_eq!(
///     symbol_from_bytes(4, "TSt".bytes()),
///     Err(ParseSymbolError::BadChar(b't'))
/// );
/// assert_eq!(
///     symbol_from_bytes(0, "TESTING".bytes()),
///     Ok(5138124851399447552)
/// );
/// assert_eq!(
///     symbol_from_bytes(0, "TESTINGG".bytes()),
///     Err(ParseSymbolError::CodeTooLong)
/// );
/// ```
#[inline]
pub fn symbol_from_bytes<I>(
    precision: u8,
    iter: I,
) -> Result<u64, ParseSymbolError>
where
    I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
{
    let code = symbol_code_from_bytes(iter)?;
    Ok(symbol_from_code(precision, code))
}

#[inline]
#[must_use]
pub const fn symbol_from_code(precision: u8, code: u64) -> u64 {
    (code << 8) | (precision as u64)
}

/// Gets an EOSIO symbol's precision.
///
/// # Examples
///
/// ```
/// use eosio_numstr::symbol_to_precision;
/// assert_eq!(symbol_to_precision(1397703940), 4); // 4,EOS
/// assert_eq!(symbol_to_precision(1398362882), 2); // 2,SYS
/// assert_eq!(symbol_to_precision(5138124851399447552), 0); // 0,TESTING
/// ```
#[inline]
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub const fn symbol_to_precision(value: u64) -> u8 {
    (value & 0xFF) as u8
}

/// Gets an EOSIO symbol's code.
///
/// # Examples
///
/// ```
/// use eosio_numstr::symbol_to_code;
/// assert_eq!(symbol_to_code(1162826500), 4542291); // 4,EOS
/// assert_eq!(symbol_to_code(1398362882), 5462355); // 2,SYS
/// assert_eq!(symbol_to_code(6072351294051206912), 23720122242387527); // 0,TESTING
/// ```
#[inline]
#[must_use]
pub const fn symbol_to_code(value: u64) -> u64 {
    value >> 8
}

#[cfg(test)]
mod tests {
    use crate::{
        symbol_code_from_bytes, symbol_from_bytes, symbol_to_code,
        symbol_to_precision,
    };
    use proptest::prelude::*;

    #[test]
    fn from_bytes_to_code_and_precision() {
        proptest!(|(precision in 0_u8.., code in "[A-Z]{1,7}")| {
            let expected_code = symbol_code_from_bytes(code.bytes()).unwrap();
            let symbol = symbol_from_bytes(precision, code.bytes()).unwrap();
            let result_precision = symbol_to_precision(symbol);
            prop_assert_eq!(result_precision, precision);
            let result_code = symbol_to_code(symbol);
            prop_assert_eq!(result_code, expected_code);
        });
    }
}
