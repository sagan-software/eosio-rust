use super::{ScopeName, Table};
use crate::AccountName;
use core::marker::PhantomData;

/// TODO docs
#[derive(Copy, Clone, Debug)]
pub struct PrimaryTableIndex<T>
where
    T: Table,
{
    /// TODO docs
    pub code: AccountName,
    /// TODO docs
    pub scope: ScopeName,
    /// TODO docs
    _data: PhantomData<T>,
}

impl<T> PrimaryTableIndex<T>
where
    T: Table,
{
    /// TODO docs
    #[inline]
    pub fn new<C, S>(code: C, scope: S) -> Self
    where
        C: Into<AccountName>,
        S: Into<ScopeName>,
    {
        Self {
            code: code.into(),
            scope: scope.into(),
            _data: PhantomData,
        }
    }
}
