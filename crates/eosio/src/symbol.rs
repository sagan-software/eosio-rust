use crate::account::AccountName;
use crate::print::Print;
use eosio_macros::*;

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

impl ToString for SymbolCode {
    #[inline]
    fn to_string(&self) -> String {
        let chars: [char; 7] = (*self).into();
        let s: String = chars.iter().collect();
        s.trim().to_string()
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

impl ToString for Symbol {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(self.precision().to_string().as_str());
        s.push_str(",");
        s.push_str(self.code().to_string().as_str());
        s
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
