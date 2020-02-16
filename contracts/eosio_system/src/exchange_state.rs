use crate::SELF;
use eosio::{
    n, s, Asset, NumBytes, PrimaryTableIndex, Read, Symbol, Table, TableName,
    Write,
};
use lazy_static::lazy_static;

#[derive(Read, Write, NumBytes, Default)]
pub struct ExchangeState {
    pub supply: Asset,
    pub base: Connector,
    pub quote: Connector,
}

#[derive(Read, Write, NumBytes)]
pub struct Connector {
    pub balance: Asset,
    pub weight: f64,
}

impl Default for Connector {
    fn default() -> Self {
        Self {
            balance: Asset::default(),
            weight: 0.5,
        }
    }
}

pub struct RamMarket;

impl Table for RamMarket {
    type Row = ExchangeState;

    const NAME: TableName = TableName::new(n!("rammarket"));

    fn primary_key(row: &Self::Row) -> u64 {
        row.supply.symbol.as_u64()
    }
}

pub const RAMCORE_SYMBOL: Symbol = Symbol::new(s!(4, "RAMCORE"));
pub const RAM_SYMBOL: Symbol = Symbol::new(s!(0, "RAM"));

lazy_static! {
    pub static ref RAMMARKET: PrimaryTableIndex<RamMarket> =
        RamMarket::table(*SELF, *SELF);
}
