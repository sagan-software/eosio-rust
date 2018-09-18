#![no_std]
#![feature(alloc)]

extern crate alloc;

use alloc::prelude::*;
use core::char;
use core::str;

#[derive(Debug, PartialEq, Clone)]
pub struct Name(u64);

impl Name {
    pub fn new(n: u64) -> Name {
        Name(n)
    }
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

fn char_to_symbol(c: char) -> Option<char> {
    if c >= 'a' && c <= 'z' {
        char::from_u32((c as u32 - 'a' as u32) + 6)
    } else if c >= '1' && c <= '5' {
        char::from_u32((c as u32 - '1' as u32) + 1)
    } else {
        None
    }
}

#[derive(Debug, PartialEq)]
pub enum ToNameError {
    IsEmpty,
    TooLong,
    BadChar(char),
}

pub fn string_to_name(s: &str) -> Result<u64, ToNameError> {
    if s.is_empty() {
        return Err(ToNameError::IsEmpty);
    }

    if s.len() > 12 {
        return Err(ToNameError::TooLong);
    }

    let mut value = 0;

    for (i, c) in s.chars().enumerate() {
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
                return Err(ToNameError::BadChar(c));
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
        let mut c = NAME_CHARS[charmap_index as usize];
        chars[12 - i] = c;
        t >>= if i == 0 { 4 } else { 5 };
    }
    str::from_utf8(&chars)
        .unwrap()
        .trim_matches('.')
        .to_string()
}

pub type AccountName = Name;
pub type PermissionName = Name;
pub type TableName = Name;
pub type Time = u32;
pub type ScopeName = Name;
pub type ActionName = Name;
pub type WeightType = u16;

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
        string_to_name_empty, "", Err(ToNameError::IsEmpty)
        string_to_name_single_char, "a", Ok(3_458_764_513_820_540_928)
        string_to_name_bad_number, "123456789012", Err(ToNameError::BadChar('6'))
        string_to_name_only_numbers, "123451234512", Ok(614_251_535_012_020_768)
        string_to_name_too_long, "1234512345123", Err(ToNameError::TooLong)
        string_to_name_uppercase, "TEST", Err(ToNameError::BadChar('T'))
        string_to_name_only_letters, "test", Ok(14_605_613_396_213_628_928)
        string_to_name_special_char, "test!", Err(ToNameError::BadChar('!'))
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
    );
}
