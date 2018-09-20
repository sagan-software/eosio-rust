use eosio_bytes::*;
use eosio_derives::*;
use names::AccountName;
use symbol::Symbol;

#[derive(Debug, PartialEq, Clone, Copy, Default, Readable, Writeable)]
pub struct Asset {
    pub amount: i64,
    pub symbol: Symbol,
}

#[derive(Debug, PartialEq, Clone, Copy, Default, Readable, Writeable)]
pub struct ExtendedAsset {
    pub quantity: Asset,
    pub contract: AccountName,
}
