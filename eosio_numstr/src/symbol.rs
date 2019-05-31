use std::error::Error;
use std::fmt;

/// All possible characters that can be used in EOSIO symbol names.
pub const SYMBOL_UTF8_CHARS: [u8; 26] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// The maximum allowed length of EOSIO symbol names.
pub const SYMBOL_LEN_MAX: usize = 7;

/// An error which can be returned when parsing an EOSIO name.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseSymbolError {
    IsEmpty,
    TooLong,
    BadChar(char),
}

impl Error for ParseSymbolError {}

impl fmt::Display for ParseSymbolError {
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
        }
    }
}

/// Attemps to create an EOSIO symbol from a `&str`.
///
/// # Examples
///
/// ```
/// use eosio_numstr::{symbol_from_str, ParseSymbolError};
/// assert_eq!(symbol_from_str(4, "EOS"), Ok(1397703940));
/// assert_eq!(symbol_from_str(0, "TGFT"), Ok(361956332544));
/// assert_eq!(symbol_from_str(2, "SYS"), Ok(1398362882));
/// assert_eq!(symbol_from_str(4, "TSt"), Err(ParseSymbolError::BadChar('t')));
/// assert_eq!(symbol_from_str(0, "TESTING"), Ok(5138124851399447552));
/// assert_eq!(symbol_from_str(0, "TESTINGG"), Err(ParseSymbolError::TooLong));
/// ```
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
/// use eosio_numstr::{symbol_from_chars, ParseSymbolError};
/// assert_eq!(symbol_from_chars(4, "EOS".chars()), Ok(1397703940));
/// assert_eq!(symbol_from_chars(0, "TGFT".chars()), Ok(361956332544));
/// assert_eq!(symbol_from_chars(2, "SYS".chars()), Ok(1398362882));
/// assert_eq!(symbol_from_chars(4, "TSt".chars()), Err(ParseSymbolError::BadChar('t')));
/// assert_eq!(symbol_from_chars(0, "TESTING".chars()), Ok(5138124851399447552));
/// assert_eq!(symbol_from_chars(0, "TESTINGG".chars()), Err(ParseSymbolError::TooLong));
/// ```
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
/// use eosio_numstr::symbol_to_string;
/// assert_eq!(symbol_to_string(1397703940), "EOS");
/// assert_eq!(symbol_to_string(5138124851399447552), "TESTING");
/// assert_eq!(symbol_to_string(361956332544), "TGFT");
/// assert_eq!(symbol_to_string(1398362882), "SYS");
/// assert_eq!(symbol_to_string(0), "");
/// ```
pub fn symbol_to_string(name: u64) -> String {
    String::from_utf8_lossy(&symbol_to_utf8(name)).trim().into()
}

/// Converts an EOSIO symbol into an array of UTF-8 characters.
///
/// # Examples
///
/// ```
/// use eosio_numstr::symbol_to_utf8;
/// assert_eq!(symbol_to_utf8(1397703940), *b"EOS    ");
/// assert_eq!(symbol_to_utf8(5138124851399447552), *b"TESTING");
/// assert_eq!(symbol_to_utf8(361956332544), *b"TGFT   ");
/// assert_eq!(symbol_to_utf8(1398362882), *b"SYS    ");
/// assert_eq!(symbol_to_utf8(0), *b"       ");
/// ```
pub fn symbol_to_utf8(value: u64) -> [u8; SYMBOL_LEN_MAX] {
    let mask: u64 = 0xff;
    let mut chars = [b' '; SYMBOL_LEN_MAX];
    let mut v = value;
    for c in &mut chars {
        v >>= 8;
        if v == 0 {
            break;
        }
        *c = (v & mask) as u8;
    }
    chars
}

/// Gets an EOSIO symbol's precision.
///
/// # Examples
///
/// ```
/// use eosio_numstr::symbol_precision;
/// assert_eq!(symbol_precision(1397703940), 4); // 4,EOS
/// assert_eq!(symbol_precision(1398362882), 2); // 2,SYS
/// assert_eq!(symbol_precision(5138124851399447552), 0); // 0,TESTING
/// ```
pub fn symbol_precision(value: u64) -> u8 {
    (value & 255) as u8 // TODO overflow protection
}

/// Gets an EOSIO symbol's code.
///
/// # Examples
///
/// ```
/// use eosio_numstr::symbol_code;
/// assert_eq!(symbol_code(1397703940), 5459781); // 4,EOS
/// assert_eq!(symbol_code(1398362882), 5462355); // 2,SYS
/// assert_eq!(symbol_code(5138124851399447552), 20070800200779092); // 0,TESTING
/// ```
pub fn symbol_code(value: u64) -> u64 {
    value >> 8
}

/// Gets the length of an EOSIO symbol's code
///
/// # Examples
///
/// ```
/// use eosio_numstr::symbol_code_length;
/// assert_eq!(symbol_code_length(1397703940), 3); // 4,EOS
/// assert_eq!(symbol_code_length(1398362882), 3); // 2,SYS
/// assert_eq!(symbol_code_length(5138124851399447552), 7); // 0,TESTING
/// ```
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
