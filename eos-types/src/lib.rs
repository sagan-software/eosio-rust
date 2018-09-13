#![no_std]

pub struct Name(u64);

impl Name {
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

fn char_to_symbol(c: char) -> Option<char> {
    if c >= 'a' && c <= 'z' {
        ::core::char::from_u32((c as u32 - 'a' as u32) + 6)
    } else if c >= '1' && c <= '5' {
        ::core::char::from_u32((c as u32 - '1' as u32) + 1)
    } else {
        None
    }
}

#[derive(Debug)]
pub enum NameError {
    IsEmpty,
    TooLong,
    InvalidChar(char),
}

pub fn string_to_name(s: &str) -> Result<u64, NameError> {
    if s.is_empty() {
        return Err(NameError::IsEmpty);
    }

    if s.len() > 12 {
        return Err(NameError::TooLong);
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
                return Err(NameError::InvalidChar(c));
            }
        }
    }

    Ok(value)
}

pub type AccountName = u64;
pub type PermissionName = u64;
pub type TableName = u64;
pub type Time = u32;
pub type ScopeName = u64;
pub type ActionName = u64;
pub type WeightType = u16;
