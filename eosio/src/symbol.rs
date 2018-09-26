use eosio_macros::*;
use names::AccountName;
use print::Printable;
use readable::Readable;
use writeable::Writeable;

#[derive(Debug, PartialEq, Clone, Copy, Default, Readable, Writeable)]
#[eosio_internal]
pub struct Symbol(u64);

impl Symbol {
    pub fn precision(self) -> u64 {
        self.0 & 255
    }
    pub fn name(self) -> u64 {
        self.0 >> 8
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

        // not working
        let mut sym = self.name();
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
pub struct ExtendedSymbol {
    pub symbol: Symbol,
    pub contract: AccountName,
}
