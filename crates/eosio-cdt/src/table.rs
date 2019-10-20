use eosio::{AccountName, ReadError, ScopeName, Table, WriteError};

/// Table Cursor
pub trait TableCursor<T>: IntoIterator
where
    T: Table,
{
    /// TODO docs
    fn get(&self) -> Result<T::Row, ReadError>;
    /// TODO docs
    fn erase(&self) -> Result<T::Row, ReadError>;
    /// TODO docs
    fn modify(
        &self,
        payer: Option<AccountName>,
        item: &T::Row,
    ) -> Result<usize, WriteError>;
}

/// Table index
pub trait TableIndex<'a, K, T>
where
    T: Table + 'a,
{
    /// TODO docs
    type Cursor: TableCursor<T> + 'a;
    /// TODO docs
    fn code(&'a self) -> AccountName;
    /// TODO docs
    fn scope(&'a self) -> ScopeName;
    /// TODO docs
    fn lower_bound<N: Into<K>>(&'a self, key: N) -> Option<Self::Cursor>;
    /// TODO docs
    fn upper_bound<N: Into<K>>(&'a self, key: N) -> Option<Self::Cursor>;
    /// TODO docs
    fn emplace(
        &'a self,
        payer: AccountName,
        item: &'a T::Row,
    ) -> Result<(), WriteError>;
}

/// Table iterator
pub trait TableIterator: DoubleEndedIterator {}
