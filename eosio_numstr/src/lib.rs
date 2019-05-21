use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseNameError {
    // IsEmpty,
    TooLong,
    BadChar(char),
}

impl fmt::Display for ParseNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // ParseNameError::IsEmpty => write!(f, "empty string is not a valid EOSIO name"),
            ParseNameError::TooLong => {
                write!(f, "name is too long, must be 12 chars or less")
            }
            ParseNameError::BadChar(c) => {
                write!(f, "name contains invalid character '{}'", c)
            }
        }
    }
}

pub fn char_to_symbol(c: char) -> Option<char> {
    if c >= 'a' && c <= 'z' {
        ::std::char::from_u32((c as u32 - 'a' as u32) + 6)
    } else if c >= '1' && c <= '5' {
        ::std::char::from_u32((c as u32 - '1' as u32) + 1)
    } else {
        None
    }
}

pub fn string_to_name(s: &str) -> Result<u64, ParseNameError> {
    // if s.is_empty() {
    //     return Err(ParseNameError::IsEmpty);
    // }

    if s.len() > 12 {
        return Err(ParseNameError::TooLong);
    }

    let mut value = 0;

    for (i, c) in s.chars().enumerate() {
        if c == '.' {
            continue;
        }
        match char_to_symbol(c) {
            Some(symbol) => {
                let mut n = symbol as u64;
                if i < 12 {
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

pub const NAME_CHARS: [u8; 32] = *b".12345abcdefghijklmnopqrstuvwxyz";

pub fn name_to_string(name: u64) -> String {
    let mut chars = [b'.'; 13];
    let mut t = name;
    for i in 0..13 {
        let charmap_index = t & if i == 0 { 15 } else { 31 };
        let c = NAME_CHARS[charmap_index as usize];
        chars[12 - i] = c;
        t >>= if i == 0 { 4 } else { 5 };
    }
    ::std::str::from_utf8(&chars)
        .unwrap()
        .trim_matches('.')
        .to_string()
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseSymbolError {
    IsEmpty,
    TooLong,
    BadChar(char),
}

pub fn string_to_symbol(
    precision: u8,
    s: &str,
) -> Result<u64, ParseSymbolError> {
    let mut result: u64 = 0;
    for (i, c) in s.chars().enumerate() {
        if c < 'A' || c > 'Z' {
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
    while sym & 255 > 0 && len <= 7 {
        len += 1;
        sym >>= 8;
    }
    len
}

pub fn chars_from_symbol_value(value: u64) -> [char; 7] {
    let mut sym = value;
    let ff: u64 = 0xff;
    let mut chars = [' '; 7];
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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_string_to_name {
        ($($n:ident, $i:expr, $o:expr)*) => ($(
            #[test]
            fn $n() {
                assert_eq!(string_to_name($i), $o);
            }
        )*)
    }

    test_string_to_name!(
        string_to_name_empty, "", Ok(0)
        string_to_name_single_char, "a", Ok(3_458_764_513_820_540_928)
        string_to_name_bad_number, "123456789012", Err(ParseNameError::BadChar('6'))
        string_to_name_only_numbers, "123451234512", Ok(614_251_535_012_020_768)
        string_to_name_too_long, "1234512345123", Err(ParseNameError::TooLong)
        string_to_name_uppercase, "TEST", Err(ParseNameError::BadChar('T'))
        string_to_name_only_letters, "test", Ok(14_605_613_396_213_628_928)
        string_to_name_special_char, "test!", Err(ParseNameError::BadChar('!'))
        string_to_name_with_periods, "a.b.c", Ok(3_462_709_561_541_001_216)
        string_to_name_with_only_periods, "...", Ok(0) // TODO is this valid?
    );

    macro_rules! test_name_to_string {
        ($($n:ident, $i:expr, $o:expr)*) => ($(
            #[test]
            fn $n() {
                assert_eq!(name_to_string($i), $o);
            }
        )*)
    }

    test_name_to_string!(
        name_to_string_single_char, 3_458_764_513_820_540_928, "a"
        name_to_string_only_numbers, 614_251_535_012_020_768, "123451234512"
        name_to_string_only_letters, 14_605_613_396_213_628_928, "test"
        name_to_string_zero, 0, ""
        name_to_string_with_periods, 3_462_709_561_541_001_216, "a.b.c"
    );

    macro_rules! test_symbol_name_length {
        ($($n:ident, $i:expr, $o:expr)*) => ($(
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

}
