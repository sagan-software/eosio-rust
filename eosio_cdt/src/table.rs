use eosio_bytes::{NumBytes, Read, ReadError, Write, WriteError};
use eosio_core::{AccountName, ScopeName};

pub trait TableRow: Read + Write + NumBytes {
    const TABLE_NAME: u64;

    fn primary_key(&self) -> u64;

    #[inline]
    fn secondary_keys(
        &self,
    ) -> [Option<&crate::table_secondary::SecondaryTableKey>; 16] {
        [None; 16]
    }

    #[inline]
    fn table<C, S>(
        code: C,
        scope: S,
    ) -> crate::table_primary::PrimaryTableIndex<Self>
    where
        C: Into<AccountName>,
        S: Into<ScopeName>,
    {
        crate::table_primary::PrimaryTableIndex::new(
            code,
            scope,
            Self::TABLE_NAME,
        )
    }
}

/// Table Cursor
pub trait TableCursor<T>: IntoIterator
where
    T: TableRow,
{
    fn get(&self) -> Result<T, ReadError>;
    fn erase(&self) -> Result<T, ReadError>;
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
    type Cursor: TableCursor<T> + 'a;
    fn lower_bound<N>(&'a self, key: N) -> Option<Self::Cursor>
    where
        N: Into<K>;
    fn upper_bound<N>(&'a self, key: N) -> Option<Self::Cursor>
    where
        N: Into<K>;
    fn emplace(
        &'a self,
        payer: AccountName,
        item: &'a T,
    ) -> Result<(), WriteError>;
}

/// Table iterator
pub trait TableIterator: DoubleEndedIterator {}
