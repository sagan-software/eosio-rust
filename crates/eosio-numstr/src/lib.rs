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
    /// The name is empty
    Empty,
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
            Self::Empty => write!(
                f,
                "name is empty",
            ),
            Self::TooLong => write!(
                f,
                "name is too long, must be {} chars or less",
                NAME_LEN_MAX
            ),
            Self::BadChar(c) => write!(
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
/// assert_eq!(name_from_str(""), Err(ParseNameError::Empty));
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
/// assert_eq!(name_from_chars("".chars()), Err(ParseNameError::Empty));
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

    if value > 0 {
        Ok(value)
    } else {
        Err(ParseNameError::Empty)
    }
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
            Self::IsEmpty => write!(f, "symbol is empty"),
            Self::TooLong => write!(
                f,
                "symbol is too long, must be {} chars or less",
                SYMBOL_LEN_MAX
            ),
            Self::BadChar(c) => {
                write!(f, "symbol contains invalid character '{}'", c)
            }
            Self::BadPrecision => write!(f, "symbol precision is > 255"),
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
/// use eosio_numstr::{symbol_from_chars, ParseSymbolError};
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
/// use eosio_numstr::symbol_to_string;
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
/// use eosio_numstr::symbol_to_utf8;
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
/// use eosio_numstr::symbol_precision;
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
/// use eosio_numstr::symbol_code;
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
/// use eosio_numstr::symbol_code_length;
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
