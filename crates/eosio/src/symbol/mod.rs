//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/symbol.hpp#L234-L337>
mod extended_symbol;
mod symbol_code;

pub use self::{extended_symbol::ExtendedSymbol, symbol_code::SymbolCode};

use crate::bytes::{NumBytes, Read, Write};
use eosio_numstr::{symbol_code, symbol_from_chars, symbol_precision};
pub use eosio_numstr::{ParseSymbolError, SYMBOL_LEN_MAX, SYMBOL_UTF8_CHARS};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

/// Stores information about a symbol, the symbol can be 7 characters long.
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
    Serialize,
    Deserialize,
)]
#[__eosio_path = "crate::bytes"]
pub struct Symbol(u64);

impl Symbol {
    /// Construct a new symbol given a value.
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Construct a new symbol given a `u8` precision and `SymbolCode`.
    #[inline]
    pub fn new_with_code(precision: u8, code: SymbolCode) -> Self {
        let mut value = code.as_u64();
        value |= u64::from(precision);
        Self(value)
    }

    /// This symbol's precision
    #[inline]
    pub fn precision(&self) -> u8 {
        symbol_precision(self.as_u64())
    }

    /// Returns representation of symbol name
    #[inline]
    pub fn code(&self) -> SymbolCode {
        symbol_code(self.as_u64()).into()
    }

    /// TODO docs
    #[inline]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    /// Is this symbol valid
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.code().is_valid()
    }
}

impl fmt::Display for Symbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.precision(), self.code())
    }
}

impl FromStr for Symbol {
    type Err = ParseSymbolError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.trim();
        let mut chars = value.chars();

        let precision: u8 = match chars.next() {
            Some(c) => {
                if '0' <= c && c <= '9' {
                    match c.to_digit(10) {
                        Some(p) => u8::try_from(p)
                            .map_err(|_| ParseSymbolError::BadPrecision)?,
                        None => return Err(ParseSymbolError::BadChar(c)),
                    }
                } else {
                    return Err(ParseSymbolError::BadChar(c));
                }
            }
            None => return Err(ParseSymbolError::IsEmpty),
        };

        match chars.next() {
            Some(',') => (),
            Some(c) => return Err(ParseSymbolError::BadChar(c)),
            None => return Err(ParseSymbolError::IsEmpty), // TODO better error message
        }

        let symbol = symbol_from_chars(precision, chars)?;
        Ok(symbol.into())
    }
}

impl TryFrom<&str> for Symbol {
    type Error = ParseSymbolError;
    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for Symbol {
    type Error = ParseSymbolError;
    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl From<u64> for Symbol {
    #[inline]
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl PartialEq<u64> for Symbol {
    #[inline]
    fn eq(&self, other: &u64) -> bool {
        self.as_u64() == *other
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
                let ok = Ok(Symbol::from($expected).code());
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

#[cfg(test)]
mod symbol_tests {
    use super::*;
    use eosio_macros::s;

    #[test]
    fn from_int() {
        let symbol = Symbol::from(361_956_332_546);
        assert_eq!(symbol.precision(), 2);

        let name = symbol.code();
        let num: u64 = name.into();
        assert_eq!(num, 1_413_891_924);
    }

    #[test]
    fn is_valid() {
        let symbol = Symbol::from(361_956_332_546);
        assert_eq!(symbol.is_valid(), true);
    }

    #[test]
    fn to_string() {
        fn test(value: u64, expected: &str) {
            assert_eq!(Symbol::from(value).to_string(), expected);
        }
        test(s!(2, TGFT), "2,TGFT");
        test(s!(0, TGFT), "0,TGFT");
        test(s!(4, EOS), "4,EOS");
    }

    #[test]
    fn code_to_string() {
        fn test(value: u64, expected: &str) {
            assert_eq!(Symbol::from(value).code().to_string(), expected);
        }
        test(s!(4, EOS), "EOS");
        test(s!(0, TGFT), "TGFT");
        test(s!(9, SYS), "SYS");
    }

    #[test]
    fn from_str() {
        use std::str::FromStr;

        fn test_ok(input: &str, expected: u64) {
            let ok = Ok(expected.into());
            assert_eq!(Symbol::try_from(input), ok);
            assert_eq!(Symbol::try_from(input.to_string()), ok);
            assert_eq!(Symbol::from_str(input), ok);
        }

        fn test_err(input: &str, err: ParseSymbolError) {
            let err = Err(err);
            assert_eq!(Symbol::try_from(input), err);
            assert_eq!(Symbol::try_from(input.to_string()), err);
            assert_eq!(Symbol::from_str(input), err);
        }

        test_ok("4,EOS", s!(4, EOS));
        test_ok("0,TST", s!(0, TST));
        test_ok("9,TGFT", s!(9, TGFT));
        test_ok("   4,EOS    ", s!(4, EOS));
        test_err("4,  EOS", ParseSymbolError::BadChar(' '));
        test_err("   4, EOS    ", ParseSymbolError::BadChar(' '));
        test_err("10,EOS", ParseSymbolError::BadChar('0'));
        test_err("A", ParseSymbolError::BadChar('A'));
        test_err("a", ParseSymbolError::BadChar('a'));
    }

    #[test]
    fn code_from_str() {
        use std::str::FromStr;

        fn test_ok(input: &str, expected: u64) {
            let ok = Ok(Symbol::from(expected).code());
            assert_eq!(SymbolCode::from_str(input), ok);
            assert_eq!(SymbolCode::try_from(input), ok);
        }

        fn test_err(input: &str, expected: ParseSymbolError) {
            let err = Err(expected);
            assert_eq!(SymbolCode::from_str(input), err);
            assert_eq!(SymbolCode::try_from(input), err);
        }

        test_ok("TST", s!(0, TST));
        test_ok("EOS", s!(4, EOS));
        test_ok("TGFT", s!(0, TGFT));
        test_err("tst", ParseSymbolError::BadChar('t'));
    }
}
