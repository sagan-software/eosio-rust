use macros::*;
use names::AccountName;
use readable::*;
use symbol::Symbol;
use writeable::*;

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
