mod primary_table_index;
mod secondary_key;
mod secondary_keys;
mod secondary_table_index;
mod secondary_table_name;

pub use self::{
    primary_table_index::PrimaryTableIndex, secondary_key::SecondaryKey,
    secondary_keys::SecondaryKeys, secondary_table_index::SecondaryTableIndex,
    secondary_table_name::SecondaryTableName,
};
pub use eosio_macros::Table;

use crate::{
    account::AccountName,
    bytes::{NumBytes, Read, Write},
    name_type,
    symbol::{Symbol, SymbolCode},
};

name_type!(TableName);
name_type!(ScopeName);

impl From<AccountName> for ScopeName {
    #[must_use]
    fn from(value: AccountName) -> Self {
        Self::new(value.as_u64())
    }
}

impl From<Symbol> for ScopeName {
    #[must_use]
    fn from(value: Symbol) -> Self {
        Self::new(value.as_u64())
    }
}

impl From<SymbolCode> for ScopeName {
    #[must_use]
    fn from(value: SymbolCode) -> Self {
        Self::new(value.as_u64())
    }
}

/// TODO docs
pub trait Table: Sized {
    /// TODO docs
    const NAME: TableName;
    /// TODO docs
    type Row: Read + Write + NumBytes;
    /// TODO docs
    fn primary_key(row: &Self::Row) -> u64;
    /// TODO docs
    fn secondary_keys(_row: &Self::Row) -> SecondaryKeys {
        SecondaryKeys::default()
    }
    /// TODO docs
    #[inline]
    fn table<C, S>(code: C, scope: S) -> PrimaryTableIndex<Self>
    where
        C: Into<AccountName>,
        S: Into<ScopeName>,
    {
        PrimaryTableIndex::new(code, scope)
    }
}
