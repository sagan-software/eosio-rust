use crate::bytes::{NumBytes, Read, Write};
use core::{
    fmt,
    str::{self, FromStr},
};
use eosio_numstr::{
    symbol_code_from_bytes, symbol_code_to_bytes, ParseSymbolCodeError,
};

/// Stores the symbol code as a `u64` value
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
)]
#[eosio(crate_path = "crate::bytes")]
pub struct SymbolCode(u64);

impl From<u64> for SymbolCode {
    #[inline]
    #[must_use]
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl From<SymbolCode> for u64 {
    #[inline]
    #[must_use]
    fn from(s: SymbolCode) -> Self {
        s.0
    }
}

impl From<SymbolCode> for [u8; 7] {
    #[inline]
    #[must_use]
    fn from(s: SymbolCode) -> Self {
        symbol_code_to_bytes(s.0)
    }
}

impl fmt::Display for SymbolCode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bytes = symbol_code_to_bytes(self.0);
        let value = str::from_utf8(&bytes)
            .map(str::trim)
            .map_err(|_| fmt::Error)?;
        write!(f, "{}", value)
    }
}

impl SymbolCode {
    /// TODO docs
    #[inline]
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// TODO docs
    #[inline]
    #[must_use]
    pub fn is_valid(&self) -> bool {
        let chars = symbol_code_to_bytes(self.0);
        for &c in &chars {
            if c == b' ' {
                continue;
            }
            if !(b'A' <= c && c <= b'Z') {
                return false;
            }
        }
        true
    }

    /// TODO docs
    #[inline]
    #[must_use]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }
}

impl FromStr for SymbolCode {
    type Err = ParseSymbolCodeError;

    #[inline]
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        symbol_code_from_bytes(value.bytes()).map(Into::into)
    }
}

#[cfg(test)]
mod symbol_code_tests {
    use super::{FromStr, ParseSymbolCodeError, SymbolCode};
    use alloc::string::ToString;
    use proptest::prelude::*;

    #[test]
    fn from_to_string() {
        proptest!(|(input in "[A-Z]{1,7}")| {
            let symbol = SymbolCode::from_str(&input).unwrap();
            let result = symbol.to_string();
            prop_assert_eq!(result, input);
        })
    }

    #[allow(clippy::unnecessary_operation)]
    #[test]
    fn from_str_err() {
        proptest!(|(input in "[A-Z]{1,3}[a-z]{1,3}")| {
            match SymbolCode::from_str(&input) {
                Err(ParseSymbolCodeError::BadChar(..)) => (),
                Err(err) => prop_assert!(false, "`SymbolCode::from_str` failed with the wrong error (expected `BadChar`): {}", err),
                Ok(..) => prop_assert!(false, "from_str should've failed"),
            }
        });
    }
}
