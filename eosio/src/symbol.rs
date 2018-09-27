use account::AccountName;
use eosio_macros::*;
use print::Printable;

#[derive(Debug, PartialEq, Clone, Copy, Default, Readable, Writeable)]
#[eosio_internal]
pub struct SymbolName(u64);

impl From<u64> for SymbolName {
    fn from(n: u64) -> Self {
        SymbolName(n)
    }
}

impl Into<u64> for SymbolName {
    fn into(self) -> u64 {
        self.0
    }
}

impl Printable for SymbolName {
    fn print(&self) {
        let mut sym = self.0;
        let ff: u64 = 0xff;
        for _i in 0..7 {
            let c = sym & ff;
            if c == 0 {
                return;
            }
            (c as u8 as char).print();
            sym >>= 8;
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default, Readable, Writeable)]
#[eosio_internal]
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
}

impl From<u64> for Symbol {
    fn from(n: u64) -> Self {
        Symbol(n)
    }
}

impl Printable for Symbol {
    fn print(&self) {
        self.precision().print();
        ','.print();
        self.name().print();
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default, Readable, Writeable)]
#[eosio_internal]
pub struct ExtendedSymbol {
    pub symbol: Symbol,
    pub contract: AccountName,
}
