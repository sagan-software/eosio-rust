use account::AccountName;
use eosio_macros::*;
use print::Print;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Read, Write, Hash, PartialOrd, Ord)]
pub struct SymbolName(u64);

impl From<u64> for SymbolName {
    fn from(n: u64) -> Self {
        SymbolName(n)
    }
}

impl From<SymbolName> for u64 {
    fn from(s: SymbolName) -> Self {
        s.0
    }
}

impl From<SymbolName> for [char; 7] {
    fn from(s: SymbolName) -> Self {
        chars_from_symbol_value(s.0)
    }
}

impl SymbolName {
    pub fn is_valid(self) -> bool {
        let chars = chars_from_symbol_value(self.0);
        for &c in chars.iter() {
            if !('A' <= c && c <= 'Z') {
                return false;
            }
        }
        true
    }
}

fn chars_from_symbol_value(value: u64) -> [char; 7] {
    let mut sym = value;
    let ff: u64 = 0xff;
    let mut chars = [' '; 7];
    for c in chars.iter_mut() {
        let b = sym & ff;
        if b == 0 {
            break;
        }
        *c = b as u8 as char;
        sym >>= 8;
    }
    chars
}

impl Print for SymbolName {
    fn print(&self) {
        let chars: [char; 7] = (*self).into();
        for &c in chars.iter() {
            if c == ' ' {
                return;
            }
            c.print();
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Read, Write, Hash, PartialOrd, Ord)]
pub struct Symbol(u64);

impl Symbol {
    pub fn precision(self) -> u64 {
        self.0 & 255
    }
    pub fn name(self) -> SymbolName {
        SymbolName(self.0 >> 8)
    }
    pub fn name_length(self) -> usize {
        ::eosio_sys::symbol_name_length(self.0)
    }
    pub fn value(self) -> u64 {
        self.0
    }
    pub fn is_valid(self) -> bool {
        self.name().is_valid()
    }
}

impl From<u64> for Symbol {
    fn from(n: u64) -> Self {
        Symbol(n)
    }
}

impl Print for Symbol {
    fn print(&self) {
        self.precision().print();
        ','.print();
        self.name().print();
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default, Read, Write)]
pub struct ExtendedSymbol {
    pub symbol: Symbol,
    pub contract: AccountName,
}

impl Print for ExtendedSymbol {
    fn print(&self) {
        self.symbol.print();
        '@'.print();
        self.contract.print();
    }
}
