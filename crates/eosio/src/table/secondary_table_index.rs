use super::{
    PrimaryTableIndex, ScopeName, SecondaryTableName, Table, TableName,
};
use crate::AccountName;
use std::marker::PhantomData;

/// TODO docs
#[derive(Copy, Clone, Debug)]
pub struct SecondaryTableIndex<K, T>
where
    T: Table,
{
    /// TODO docs
    pub code: AccountName,
    /// TODO docs
    pub scope: ScopeName,
    /// TODO docs
    pub table: SecondaryTableName,
    /// TODO docs
    _data: PhantomData<(K, T)>,
}

impl<K, T> SecondaryTableIndex<K, T>
where
    T: Table,
{
    /// TODO docs
    #[inline]
    pub fn new<C, S, N>(code: C, scope: S, name: N, index: usize) -> Self
    where
        C: Into<AccountName>,
        S: Into<ScopeName>,
        N: Into<TableName>,
    {
        Self {
            code: code.into(),
            scope: scope.into(),
            table: SecondaryTableName::new(name.into(), index),
            _data: PhantomData,
        }
    }

    /// TODO docs
    #[must_use]
    pub fn primary_index(&self) -> PrimaryTableIndex<T> {
        PrimaryTableIndex::new(self.code, self.scope)
    }
}
