use crate::{PrimaryTableIndex, SecondaryTableKey};
use eosio_core::{
    AccountName, NumBytes, Read, ReadError, ScopeName, Write, WriteError,
};

/// TODO docs
pub trait TableRow: Read + Write + NumBytes {
    /// TODO docs
    const TABLE_NAME: u64;

    /// TODO docs
    fn primary_key(&self) -> u64;

    /// TODO docs
    #[inline]
    fn secondary_keys(&self) -> [Option<&SecondaryTableKey>; 16] {
        [None; 16]
    }

    /// TODO docs
    #[inline]
    fn table<C, S>(code: C, scope: S) -> PrimaryTableIndex<Self>
    where
        C: Into<AccountName>,
        S: Into<ScopeName>,
    {
        PrimaryTableIndex::new(code, scope, Self::TABLE_NAME)
    }
}

/// Table Cursor
pub trait TableCursor<T>: IntoIterator
where
    T: TableRow,
{
    /// TODO docs
    fn get(&self) -> Result<T, ReadError>;
    /// TODO docs
    fn erase(&self) -> Result<T, ReadError>;
    /// TODO docs
    fn modify(
        &self,
        payer: Option<AccountName>,
        item: &T,
    ) -> Result<usize, WriteError>;
}

/// Table index
pub trait TableIndex<'a, K, T>
where
    T: TableRow + 'a,
{
    /// TODO docs
    type Cursor: TableCursor<T> + 'a;
    /// TODO docs
    fn lower_bound<N>(&'a self, key: N) -> Option<Self::Cursor>
    where
        N: Into<K>;
    /// TODO docs
    fn upper_bound<N>(&'a self, key: N) -> Option<Self::Cursor>
    where
        N: Into<K>;
    /// TODO docs
    fn emplace(
        &'a self,
        payer: AccountName,
        item: &'a T,
    ) -> Result<(), WriteError>;
}

/// Table iterator
pub trait TableIterator: DoubleEndedIterator {}
