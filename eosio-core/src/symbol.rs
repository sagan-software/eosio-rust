//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/symbol.hpp#L234-L337>
use crate::{NumBytes, Read, SymbolCode, Write};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// All possible characters that can be used in EOSIO symbol codes.
pub const SYMBOL_UTF8_CHARS: [u8; 26] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// The maximum allowed length of EOSIO symbol codes.
pub const SYMBOL_LEN_MAX: usize = 7;

/// An error which can be returned when parsing an EOSIO symbol.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseSymbolError {
    /// Empty strings are not valid symbols.
    IsEmpty,
    /// Symbols must be 7 characters or less.
    TooLong,
    /// Symbols can only contain uppercase letters A-Z.
    BadChar(char),
    /// TODO docs
    BadPrecision,
}

impl Error for ParseSymbolError {}

impl fmt::Display for ParseSymbolError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseSymbolError::IsEmpty => write!(f, "symbol is empty"),
            ParseSymbolError::TooLong => write!(
                f,
                "symbol is too long, must be {} chars or less",
                SYMBOL_LEN_MAX
            ),
            ParseSymbolError::BadChar(c) => {
                write!(f, "symbol contains invalid character '{}'", c)
            }
            ParseSymbolError::BadPrecision => {
                write!(f, "symbol precision is > 255")
            }
        }
    }
}

/// Attemps to create an EOSIO symbol from a `&str`.
///
/// # Examples
///
/// ```
/// use eosio_core::{symbol_from_str, ParseSymbolError};
/// assert_eq!(symbol_from_str(4, "EOS"), Ok(1397703940));
/// assert_eq!(symbol_from_str(0, "TGFT"), Ok(361956332544));
/// assert_eq!(symbol_from_str(2, "SYS"), Ok(1398362882));
/// assert_eq!(symbol_from_str(4, "TSt"), Err(ParseSymbolError::BadChar('t')));
/// assert_eq!(symbol_from_str(0, "TESTING"), Ok(5138124851399447552));
/// assert_eq!(symbol_from_str(0, "TESTINGG"), Err(ParseSymbolError::TooLong));
/// ```
#[inline]
pub fn symbol_from_str(
    precision: u8,
    value: &str,
) -> Result<u64, ParseSymbolError> {
    symbol_from_chars(precision, value.chars())
}

/// Attempts to create an EOSIO symbol from an `Iterator`.
///
/// # Examples
///
/// ```
/// use eosio_core::{symbol_from_chars, ParseSymbolError};
/// assert_eq!(symbol_from_chars(4, "EOS".chars()), Ok(1397703940));
/// assert_eq!(symbol_from_chars(0, "TGFT".chars()), Ok(361956332544));
/// assert_eq!(symbol_from_chars(2, "SYS".chars()), Ok(1398362882));
/// assert_eq!(symbol_from_chars(4, "TSt".chars()), Err(ParseSymbolError::BadChar('t')));
/// assert_eq!(symbol_from_chars(0, "TESTING".chars()), Ok(5138124851399447552));
/// assert_eq!(symbol_from_chars(0, "TESTINGG".chars()), Err(ParseSymbolError::TooLong));
/// ```
#[inline]
pub fn symbol_from_chars<I>(
    precision: u8,
    chars: I,
) -> Result<u64, ParseSymbolError>
where
    I: Iterator<Item = char>,
{
    // TODO check precision. what is max precision?
    let mut result: u64 = 0;
    for (i, c) in chars.enumerate() {
        if i == SYMBOL_LEN_MAX {
            return Err(ParseSymbolError::TooLong);
        } else if c < 'A' || c > 'Z' {
            return Err(ParseSymbolError::BadChar(c));
        } else {
            result |= (c as u64) << (8 * (i + 1));
        }
    }

    // TODO check if zero, IsEmpty error

    result |= u64::from(precision);
    Ok(result)
}

/// Converts an EOSIO symbol value into a string.
///
/// # Examples
///
/// ```
/// use eosio_core::symbol_to_string;
/// assert_eq!(symbol_to_string(1397703940), "EOS");
/// assert_eq!(symbol_to_string(5138124851399447552), "TESTING");
/// assert_eq!(symbol_to_string(361956332544), "TGFT");
/// assert_eq!(symbol_to_string(1398362882), "SYS");
/// assert_eq!(symbol_to_string(0), "");
/// ```
#[inline]
pub fn symbol_to_string(name: u64) -> String {
    String::from_utf8_lossy(&symbol_to_utf8(name)).trim().into()
}

/// Converts an EOSIO symbol into an array of UTF-8 characters.
///
/// # Examples
///
/// ```
/// use eosio_core::symbol_to_utf8;
/// assert_eq!(symbol_to_utf8(1397703940), *b"EOS    ");
/// assert_eq!(symbol_to_utf8(5138124851399447552), *b"TESTING");
/// assert_eq!(symbol_to_utf8(361956332544), *b"TGFT   ");
/// assert_eq!(symbol_to_utf8(1398362882), *b"SYS    ");
/// assert_eq!(symbol_to_utf8(0), *b"       ");
/// ```
#[inline]
pub fn symbol_to_utf8(value: u64) -> [u8; SYMBOL_LEN_MAX] {
    let mask: u64 = 0xff;
    let mut chars = [b' '; SYMBOL_LEN_MAX];
    let mut v = value;
    for c in &mut chars {
        v >>= 8;
        if v == 0 {
            break;
        }
        *c = u8::try_from(v & mask).unwrap_or_default();
    }
    chars
}

/// Gets an EOSIO symbol's precision.
///
/// # Examples
///
/// ```
/// use eosio_core::symbol_precision;
/// assert_eq!(symbol_precision(1397703940), 4); // 4,EOS
/// assert_eq!(symbol_precision(1398362882), 2); // 2,SYS
/// assert_eq!(symbol_precision(5138124851399447552), 0); // 0,TESTING
/// ```
#[inline]
pub fn symbol_precision(value: u64) -> u8 {
    u8::try_from(value & 255).unwrap_or_default()
}

/// Gets an EOSIO symbol's code.
///
/// # Examples
///
/// ```
/// use eosio_core::symbol_code;
/// assert_eq!(symbol_code(1397703940), 5459781); // 4,EOS
/// assert_eq!(symbol_code(1398362882), 5462355); // 2,SYS
/// assert_eq!(symbol_code(5138124851399447552), 20070800200779092); // 0,TESTING
/// ```
#[inline]
pub const fn symbol_code(value: u64) -> u64 {
    value >> 8
}

/// Gets the length of an EOSIO symbol's code
///
/// # Examples
///
/// ```
/// use eosio_core::symbol_code_length;
/// assert_eq!(symbol_code_length(1397703940), 3); // 4,EOS
/// assert_eq!(symbol_code_length(1398362882), 3); // 2,SYS
/// assert_eq!(symbol_code_length(5138124851399447552), 7); // 0,TESTING
/// ```
#[inline]
pub fn symbol_code_length(symbol: u64) -> usize {
    let mut sym = symbol;
    sym >>= 8; // skip precision
    let mut len = 0;
    while sym & 255 > 0 && len <= SYMBOL_LEN_MAX {
        len += 1;
        sym >>= 8;
    }
    len
}

/// Stores information about a symbol, the symbol can be 7 characters long.
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Default,
    Read,
    Write,
    NumBytes,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[eosio_core_root_path = "crate"]
pub struct Symbol(u64);

impl Symbol {
    /// Construct a new symbol given a value.
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Construct a new symbol given a `u8` precision and `SymbolCode`.
    #[inline]
    pub fn new_with_code(precision: u8, code: SymbolCode) -> Self {
        let mut value = code.as_u64();
        value |= u64::from(precision);
        Self(value)
    }

    /// This symbol's precision
    #[inline]
    pub fn precision(&self) -> u8 {
        symbol_precision(self.as_u64())
    }

    /// Returns representation of symbol name
    #[inline]
    pub fn code(&self) -> SymbolCode {
        symbol_code(self.as_u64()).into()
    }

    /// TODO docs
    #[inline]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    /// Is this symbol valid
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.code().is_valid()
    }
}

impl fmt::Display for Symbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.precision(), self.code())
    }
}

impl FromStr for Symbol {
    type Err = ParseSymbolError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.trim();
        let mut chars = value.chars();

        let precision: u8 = match chars.next() {
            Some(c) => {
                if '0' <= c && c <= '9' {
                    match c.to_digit(10) {
                        Some(p) => u8::try_from(p)
                            .map_err(|_| ParseSymbolError::BadPrecision)?,
                        None => return Err(ParseSymbolError::BadChar(c)),
                    }
                } else {
                    return Err(ParseSymbolError::BadChar(c));
                }
            }
            None => return Err(ParseSymbolError::IsEmpty),
        };

        match chars.next() {
            Some(',') => (),
            Some(c) => return Err(ParseSymbolError::BadChar(c)),
            None => return Err(ParseSymbolError::IsEmpty), // TODO better error message
        }

        let symbol = symbol_from_chars(precision, chars)?;
        Ok(symbol.into())
    }
}

impl TryFrom<&str> for Symbol {
    type Error = ParseSymbolError;
    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for Symbol {
    type Error = ParseSymbolError;
    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl From<u64> for Symbol {
    #[inline]
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl PartialEq<u64> for Symbol {
    #[inline]
    fn eq(&self, other: &u64) -> bool {
        self.as_u64() == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eosio_core_macros::{n, s};

    #[test]
    fn from_int() {
        let symbol = Symbol::from(361_956_332_546);
        assert_eq!(symbol.precision(), 2);

        let name = symbol.code();
        let num: u64 = name.into();
        assert_eq!(num, 1_413_891_924);
    }

    #[test]
    fn is_valid() {
        let symbol = Symbol::from(361_956_332_546);
        assert_eq!(symbol.is_valid(), true);
    }

    #[test]
    fn to_string() {
        fn test(value: u64, expected: &str) {
            assert_eq!(Symbol::from(value).to_string(), expected);
        }
        test(s!(2, TGFT), "2,TGFT");
        test(s!(0, TGFT), "0,TGFT");
        test(s!(4, EOS), "4,EOS");
    }

    #[test]
    fn code_to_string() {
        fn test(value: u64, expected: &str) {
            assert_eq!(Symbol::from(value).code().to_string(), expected);
        }
        test(s!(4, EOS), "EOS");
        test(s!(0, TGFT), "TGFT");
        test(s!(9, SYS), "SYS");
    }

    #[test]
    fn from_str() {
        use std::str::FromStr;

        fn test_ok(input: &str, expected: u64) {
            let ok = Ok(expected.into());
            assert_eq!(Symbol::try_from(input), ok);
            assert_eq!(Symbol::try_from(input.to_string()), ok);
            assert_eq!(Symbol::from_str(input), ok);
        }

        fn test_err(input: &str, err: ParseSymbolError) {
            let err = Err(err);
            assert_eq!(Symbol::try_from(input), err);
            assert_eq!(Symbol::try_from(input.to_string()), err);
            assert_eq!(Symbol::from_str(input), err);
        }

        test_ok("4,EOS", s!(4, EOS));
        test_ok("0,TST", s!(0, TST));
        test_ok("9,TGFT", s!(9, TGFT));
        test_ok("   4,EOS    ", s!(4, EOS));
        test_err("4,  EOS", ParseSymbolError::BadChar(' '));
        test_err("   4, EOS    ", ParseSymbolError::BadChar(' '));
        test_err("10,EOS", ParseSymbolError::BadChar('0'));
        test_err("A", ParseSymbolError::BadChar('A'));
        test_err("a", ParseSymbolError::BadChar('a'));
    }

    #[test]
    fn code_from_str() {
        use std::str::FromStr;

        fn test_ok(input: &str, expected: u64) {
            let ok = Ok(Symbol::from(expected).code());
            assert_eq!(SymbolCode::from_str(input), ok);
            assert_eq!(SymbolCode::try_from(input), ok);
        }

        fn test_err(input: &str, expected: ParseSymbolError) {
            let err = Err(expected);
            assert_eq!(SymbolCode::from_str(input), err);
            assert_eq!(SymbolCode::try_from(input), err);
        }

        test_ok("TST", s!(0, TST));
        test_ok("EOS", s!(4, EOS));
        test_ok("TGFT", s!(0, TGFT));
        test_err("tst", ParseSymbolError::BadChar('t'));
    }

}
