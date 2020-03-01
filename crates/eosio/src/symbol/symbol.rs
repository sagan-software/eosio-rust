//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/symbol.hpp#L234-L337>

use super::SymbolCode;
use crate::bytes::{NumBytes, Read, Write};
use core::fmt;
use eosio_numstr::{symbol_from_code, symbol_to_code, symbol_to_precision};

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
)]
#[eosio(crate_path = "crate::bytes")]
pub struct Symbol(u64);

impl Symbol {
    /// Construct a new symbol given a value.
    #[inline]
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Construct a new symbol given a `u8` precision and `SymbolCode`.
    #[inline]
    #[must_use]
    pub const fn new_with_code(precision: u8, code: SymbolCode) -> Self {
        Self(symbol_from_code(precision, code.as_u64()))
    }

    /// This symbol's precision
    #[inline]
    #[must_use]
    pub const fn precision(&self) -> u8 {
        symbol_to_precision(self.as_u64())
    }

    /// Returns representation of symbol name
    ///
    /// # Examples
    ///
    /// ```
    /// use eosio::{s, Symbol};
    /// let symbol: Symbol = s!(4, "EOS").into();
    /// let code = symbol.code();
    /// assert_eq!(code.to_string(), "EOS");
    /// ```
    #[inline]
    #[must_use]
    pub const fn code(&self) -> SymbolCode {
        SymbolCode::new(symbol_to_code(self.as_u64()))
    }

    /// TODO docs
    #[inline]
    #[must_use]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    /// Is this symbol valid
    #[inline]
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.code().is_valid()
    }
}

impl fmt::Display for Symbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use alloc::string::ToString;
        write!(f, "{},{}", self.precision(), self.code().to_string())
    }
}

impl From<u64> for Symbol {
    #[inline]
    #[must_use]
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl From<Symbol> for u64 {
    #[inline]
    #[must_use]
    fn from(n: Symbol) -> Self {
        n.0
    }
}

impl PartialEq<u64> for Symbol {
    #[inline]
    #[must_use]
    fn eq(&self, other: &u64) -> bool {
        self.as_u64() == *other
    }
}

#[cfg(test)]
mod tests {
    use super::{Symbol, SymbolCode};
    use alloc::string::ToString;
    use core::str::FromStr;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn from_str_to_code(precision in 1_u8.., code in "[A-Z]{1,7}") {
            let expected = SymbolCode::from_str(&code).unwrap();
            let symbol = Symbol::new_with_code(precision, expected);
            let result = symbol.code();
            prop_assert_eq!(result, expected);
        }
    }

    proptest! {
        #[test]
        fn from_str_to_string(precision in 0_u8.., code in "[A-Z]{1,7}") {
            let expected = format!("{},{}", precision, code);
            let code = SymbolCode::from_str(&code).unwrap();
            let symbol = Symbol::new_with_code(precision, code);
            let result = symbol.to_string();
            prop_assert_eq!(result, expected);
        }
    }

    // #[test]
    // fn from_int() {
    //     let symbol = Symbol::from(361_956_332_546);
    //     assert_eq!(symbol.precision(), 2);

    //     let name = symbol.code();
    //     let num: u64 = name.into();
    //     assert_eq!(num, 1_413_891_924);
    // }

    // #[test]
    // fn is_valid() {
    //     let symbol = Symbol::from(361_956_332_546);
    //     assert_eq!(symbol.is_valid(), true);
    // }

    // #[test]
    // fn to_string() {
    //     fn test(value: u64, expected: &str) {
    //         assert_eq!(Symbol::from(value).to_string(), expected);
    //     }
    //     test(s!(2, "TGFT"), "2,TGFT");
    //     test(s!(0, "TGFT"), "0,TGFT");
    //     test(s!(4, "EOS"), "4,EOS");
    // }

    // #[test]
    // fn code_to_string() {
    //     fn test(value: u64, expected: &str) {
    //         assert_eq!(Symbol::from(value).code().to_string(), expected);
    //     }
    //     test(s!(4, "EOS"), "EOS");
    //     test(s!(0, "TGFT"), "TGFT");
    //     test(s!(9, "SYS"), "SYS");
    // }
}
