use eosio_bytes::*;
use eosio_derives::*;
use names::AccountName;

#[derive(Debug, PartialEq, Clone, Copy, Default, Readable, Writeable)]
pub struct Symbol(u64);

impl Symbol {
    pub fn precision(&self) -> u64 {
        self.0 & 0xff
    }
    pub fn name(&self) -> u64 {
        self.0 >> 8
    }
    pub fn name_length(&self) -> usize {
        symbol_name_length(self.0)
    }
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl From<u64> for Symbol {
    fn from(n: u64) -> Self {
        Symbol(n)
    }
}

#[derive(Debug, PartialEq)]
pub enum ToSymbolError {
    IsEmpty,
    TooLong,
    BadChar(char),
}

pub fn string_to_symbol(precision: u8, s: &str) -> Result<u64, ToSymbolError> {
    let mut result: u64 = 0;
    for (i, c) in s.chars().enumerate() {
        if c < 'A' || c > 'Z' {
            return Err(ToSymbolError::BadChar(c));
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
    while sym & 0xff > 0 && len <= 7 {
        len += 1;
        sym >>= 8;
    }
    len
}

#[derive(Debug, PartialEq, Clone, Copy, Default, Readable, Writeable)]
pub struct ExtendedSymbol {
    pub symbol: Symbol,
    pub contract: AccountName,
}
