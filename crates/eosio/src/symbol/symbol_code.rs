use crate::bytes::{NumBytes, Read, Write};
use eosio_numstr::{
    symbol_code, symbol_from_str, symbol_to_string, symbol_to_utf8,
    ParseSymbolError,
};
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

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
#[__eosio_path = "crate::bytes"]
pub struct SymbolCode(u64);

impl From<u64> for SymbolCode {
    #[inline]
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl From<SymbolCode> for u64 {
    #[inline]
    fn from(s: SymbolCode) -> Self {
        s.0
    }
}

impl From<SymbolCode> for [u8; 7] {
    #[inline]
    fn from(s: SymbolCode) -> Self {
        symbol_to_utf8(s.0)
    }
}

impl fmt::Display for SymbolCode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", symbol_to_string(self.0 << 8))
    }
}

impl SymbolCode {
    /// TODO docs
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// TODO docs
    #[inline]
    pub fn is_valid(&self) -> bool {
        let chars = symbol_to_utf8(self.0);
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
    pub const fn as_u64(&self) -> u64 {
        self.0
    }
}

impl TryFrom<&str> for SymbolCode {
    type Error = ParseSymbolError;
    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let symbol = symbol_from_str(0, value)?;
        Ok(symbol_code(symbol).into())
    }
}

impl TryFrom<String> for SymbolCode {
    type Error = ParseSymbolError;
    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl FromStr for SymbolCode {
    type Err = ParseSymbolError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

#[cfg(test)]
mod symbol_code_tests {
    use super::*;
    use eosio_macros::s;
    use eosio_numstr::symbol_code;

    macro_rules! test_to_string {
        ($($name:ident, $value:expr, $expected:expr)*) => ($(
            #[test]
            fn $name() {
                assert_eq!(
                    SymbolCode::from(symbol_code($value)).to_string(),
                    $expected
                );
            }
        )*)
    }

    test_to_string! {
        to_string, s!(4, EOS), "EOS"
        to_string_zero_precision, s!(0, TGFT), "TGFT"
        to_string_nine_precision, s!(9, SYS), "SYS"
    }

    macro_rules! test_from_str_ok {
        ($($name:ident, $input:expr, $expected:expr)*) => ($(
            #[test]
            fn $name() {
                let ok = Ok(crate::Symbol::from($expected).code());
                assert_eq!(SymbolCode::from_str($input), ok);
                assert_eq!(SymbolCode::try_from($input), ok);
            }
        )*)
    }

    test_from_str_ok! {
        from_str_ok1, "TST", s!(0, TST)
        from_str_ok2, "EOS", s!(4, EOS)
        from_str_ok3, "TGFT", s!(0, TGFT)
    }

    macro_rules! test_from_str_err {
        ($($name:ident, $input:expr, $expected:expr)*) => ($(
            #[test]
            fn $name() {
            let err = Err($expected);
            assert_eq!(SymbolCode::from_str($input), err);
            assert_eq!(SymbolCode::try_from($input), err);
            }
        )*)
    }

    test_from_str_err! {
        from_str_bad_char,
        "tst",
        ParseSymbolError::BadChar('t')
    }
}
