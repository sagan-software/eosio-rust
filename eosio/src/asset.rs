use account::AccountName;
use eosio_macros::*;
use symbol::Symbol;

#[derive(Debug, PartialEq, Clone, Copy, Default, Readable, Writeable)]
#[eosio_internal]
pub struct Asset {
    pub amount: i64,
    pub symbol: Symbol,
}

#[derive(Debug, PartialEq, Clone, Copy, Default, Readable, Writeable)]
#[eosio_internal]
pub struct ExtendedAsset {
    pub quantity: Asset,
    pub contract: AccountName,
}
