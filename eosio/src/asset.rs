use account::AccountName;
use eosio_macros::*;
use symbol::Symbol;

#[derive(Debug, PartialEq, Clone, Copy, Default, Read, Write)]
pub struct Asset {
    pub amount: i64,
    pub symbol: Symbol,
}

#[derive(Debug, PartialEq, Clone, Copy, Default, Read, Write)]
pub struct ExtendedAsset {
    pub quantity: Asset,
    pub contract: AccountName,
}
