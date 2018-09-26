use eosio_macros::*;
use lib::*;
use print::Printable;
use readable::Readable;
use writeable::Writeable;

#[derive(Debug, PartialEq, Clone, Copy, Default, Readable, Writeable)]
#[eosio_internal]
pub struct Name(u64);

impl Name {
    pub fn new(n: u64) -> Name {
        Name(n)
    }
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl From<u64> for Name {
    fn from(n: u64) -> Self {
        Name::new(n)
    }
}

impl Into<u64> for Name {
    fn into(self) -> u64 {
        self.0
    }
}

impl Printable for Name {
    fn print(&self) {
        unsafe { ::eosio_sys::printn(self.as_u64()) }
    }
}

pub type AccountName = Name;
pub type PermissionName = Name;
pub type TableName = Name;
pub type ScopeName = Name;
pub type ActionName = Name;
