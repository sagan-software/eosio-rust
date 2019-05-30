#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseSymbolError {
    IsEmpty,
    TooLong,
    BadChar(char),
    BadPrecision,
}

pub const MAX_SYMBOL_LEN: usize = 7;

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
/// assert_eq!(symbol_from_str(0, "TESTING2"), Err(ParseSymbolError::TooLong));
/// ```
pub fn symbol_from_str(
    precision: u8,
    s: &str,
) -> Result<u64, ParseSymbolError> {
    symbol_from_iter(precision, s.chars())
}

/// Attemps to create an EOSIO symbol from an `Iterator`.
///
/// # Examples
///
/// ```
/// use eosio_numstr::{symbol_from_iter, ParseSymbolError};
/// assert_eq!(symbol_from_iter(4, "EOS".chars()), Ok(1397703940));
/// assert_eq!(symbol_from_iter(0, "TGFT".chars()), Ok(361956332544));
/// assert_eq!(symbol_from_iter(2, "SYS".chars()), Ok(1398362882));
/// assert_eq!(symbol_from_iter(4, "TSt".chars()), Err(ParseSymbolError::BadChar('t')));
/// assert_eq!(symbol_from_iter(0, "TESTING".chars()), Ok(5138124851399447552));
/// assert_eq!(symbol_from_iter(0, "TESTING2".chars()), Err(ParseSymbolError::TooLong));
/// ```
pub fn symbol_from_iter<I>(
    precision: u8,
    iter: I,
) -> Result<u64, ParseSymbolError>
where
    I: Iterator<Item = char>,
{
    // TODO check precision. what is max precision?
    let mut result: u64 = 0;
    for (i, c) in iter.enumerate() {
        if i == MAX_SYMBOL_LEN {
            return Err(ParseSymbolError::TooLong);
        } else if c < 'A' || c > 'Z' {
            return Err(ParseSymbolError::BadChar(c));
        } else {
            result |= (c as u64) << (8 * (i + 1));
        }
    }

    result |= u64::from(precision);
    Ok(result)
}

pub fn symbol_name_length(symbol: u64) -> usize {
    let mut sym = symbol;
    sym >>= 8; // skip precision
    let mut len = 0;
    while sym & 255 > 0 && len <= MAX_SYMBOL_LEN {
        len += 1;
        sym >>= 8;
    }
    len
}

pub fn symbol_to_string(value: u64) -> String {
    symbol_to_chars(value).iter().collect()
}

pub fn symbol_to_chars(value: u64) -> [char; MAX_SYMBOL_LEN] {
    let mut sym = value;
    let ff: u64 = 0xff;
    let mut chars = [' '; MAX_SYMBOL_LEN];
    for c in &mut chars {
        let b = sym & ff;
        if b == 0 {
            break;
        }
        *c = b as u8 as char;
        sym >>= 8;
    }
    chars
}

macro_rules! test_symbol_name_length {
    ($($n:ident, $i:expr, $o:expr)*) => ($(
        #[cfg(test)]
        #[test]
        fn $n() {
            assert_eq!(symbol_name_length($i), $o);
        }
    )*)
}

test_symbol_name_length!(
    symbol_name_length_zero, 0, 0
    symbol_name_length_three, 1_397_703_940, 3
    symbol_name_length_four, 361_956_332_544, 4
);
