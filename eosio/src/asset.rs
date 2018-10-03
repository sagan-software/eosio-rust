use account::AccountName;
use eosio_macros::*;
use symbol::Symbol;

#[derive(Debug, PartialEq, Clone, Copy, Default, Read, Write)]
pub struct Asset {
    pub amount: i64,
    pub symbol: Symbol,
}

impl Asset {
    pub fn is_valid(&self) -> bool {
        self.symbol.is_valid()
    }
}

// TODO: add/sub/div/mul ops for assets

#[derive(Debug, PartialEq, Clone, Copy, Default, Read, Write)]
pub struct ExtendedAsset {
    pub quantity: Asset,
    pub contract: AccountName,
}

// TODO: add/sub/div/mul ops for extended assets
