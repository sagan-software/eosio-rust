use crate::account::AccountName;
use crate::print::Print;
use eosio_macros::*;
pub use eosio_sys::ParseSymbolError;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Default, Read, Write, NumBytes, Hash, PartialOrd, Ord,
)]
pub struct SymbolCode(u64);

impl From<u64> for SymbolCode {
    #[inline]
    fn from(n: u64) -> Self {
        SymbolCode(n)
    }
}

impl From<SymbolCode> for u64 {
    #[inline]
    fn from(s: SymbolCode) -> Self {
        s.0
    }
}

impl From<SymbolCode> for [char; 7] {
    #[inline]
    fn from(s: SymbolCode) -> Self {
        chars_from_symbol_value(s.0)
    }
}

impl fmt::Display for SymbolCode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let chars: [char; 7] = (*self).into();
        let s: String = chars.iter().collect();
        write!(f, "{}", s.trim())
    }
}

impl SymbolCode {
    #[inline]
    pub fn is_valid(self) -> bool {
        let chars = chars_from_symbol_value(self.0);
        for &c in &chars {
            if c == ' ' {
                continue;
            }
            if !('A' <= c && c <= 'Z') {
                return false;
            }
        }
        true
    }

    #[inline]
    pub const fn raw(self) -> u64 {
        self.0
    }
}

impl TryFrom<&str> for SymbolCode {
    type Error = ParseSymbolError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let symbol: Symbol = eosio_sys::string_to_symbol(0, value)?.into();
        Ok(symbol.code())
    }
}

impl TryFrom<String> for SymbolCode {
    type Error = ParseSymbolError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl FromStr for SymbolCode {
    type Err = ParseSymbolError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

fn chars_from_symbol_value(value: u64) -> [char; 7] {
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

#[cfg(feature = "contract")]
impl Print for SymbolCode {
    #[inline]
    fn print(&self) {
        let chars: [char; 7] = (*self).into();
        for &c in &chars {
            if c == ' ' {
                return;
            }
            c.print();
        }
    }
}

#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Default, Read, Write, NumBytes, Hash, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Symbol(u64);

impl Symbol {
    #[inline]
    pub const fn precision(self) -> u64 {
        self.0 & 255
    }
    #[inline]
    pub const fn code(self) -> SymbolCode {
        SymbolCode(self.0 >> 8)
    }
    #[inline]
    pub fn name_length(self) -> usize {
        ::eosio_sys::symbol_name_length(self.0)
    }
    #[inline]
    pub const fn raw(self) -> u64 {
        self.0
    }
    #[inline]
    pub fn is_valid(self) -> bool {
        self.code().is_valid()
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.precision(), self.code())
    }
}

impl TryFrom<&str> for Symbol {
    type Error = ParseSymbolError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        let mut chars = value.chars();

        let precision: u32 = match chars.next() {
            Some(c) => {
                if '0' <= c && c <= '9' {
                    match c.to_digit(10) {
                        Some(p) => p,
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

        let mut result: u64 = 0;
        let mut index = 0;
        for c in chars {
            if index == 0 && c == ' ' {
                // Allow spaces between precision and symbol
                continue;
            }
            if c < 'A' || c > 'Z' {
                return Err(ParseSymbolError::BadChar(c));
            } else {
                result |= (c as u64) << (8 * (index + 1));
            }
            index += 1;
        }

        result |= u64::from(precision);

        Ok(Symbol(result))
    }
}

impl TryFrom<String> for Symbol {
    type Error = ParseSymbolError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl FromStr for Symbol {
    type Err = ParseSymbolError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl From<u64> for Symbol {
    #[inline]
    fn from(n: u64) -> Self {
        Symbol(n)
    }
}

#[cfg(feature = "contract")]
impl Print for Symbol {
    #[inline]
    fn print(&self) {
        self.precision().print();
        ','.print();
        self.code().print();
    }
}

impl PartialEq<u64> for Symbol {
    #[inline]
    fn eq(&self, other: &u64) -> bool {
        self.raw() == *other
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default, Read, Write, NumBytes)]
pub struct ExtendedSymbol {
    pub symbol: Symbol,
    pub contract: AccountName,
}

#[cfg(feature = "contract")]
impl Print for ExtendedSymbol {
    #[inline]
    fn print(&self) {
        self.symbol.print();
        '@'.print();
        self.contract.print();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        test_ok("   4, EOS    ", s!(4, EOS));
        test_ok("4,  EOS", s!(4, EOS));
        test_err("10,EOS", ParseSymbolError::BadChar('0'));
        test_err("A", ParseSymbolError::BadChar('A'));
        test_err("a", ParseSymbolError::BadChar('a'));
    }

    #[test]
    fn code_from_str() {
        use std::str::FromStr;
        assert_eq!(SymbolCode::from_str("TST"), Ok(5_526_356.into()));
        assert_eq!(
            SymbolCode::from_str("tst"),
            Err(ParseSymbolError::BadChar('t'))
        );
    }

}