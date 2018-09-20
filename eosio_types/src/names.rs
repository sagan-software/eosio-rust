use eosio_bytes::*;
use eosio_derives::*;
use lib::*;

#[derive(Debug, PartialEq, Clone, Copy, Default, Readable, Writeable)]
pub struct Name(u64);

impl Name {
    pub fn new(n: u64) -> Name {
        Name(n)
    }
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl From<u64> for Name {
    fn from(n: u64) -> Self {
        Name::new(n)
    }
}

impl Into<u64> for Name {
    fn into(self) -> u64 {
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

#[cfg(any(feature = "std", feature = "alloc"))]
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
pub type ScopeName = Name;
pub type ActionName = Name;
