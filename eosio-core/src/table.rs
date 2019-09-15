use crate::{AccountName, NumBytes, Read, ScopeName, TableName, Write};
use std::marker::PhantomData;

/// TODO docs
pub trait Table: Sized {
    /// TODO docs
    const NAME: u64;
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

/// TODO docs
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash, PartialOrd, Ord)]
pub struct SecondaryTableName(TableName, usize);

impl SecondaryTableName {
    /// TODO docs
    #[inline]
    pub const fn new(primary: TableName, index: usize) -> Self {
        Self(primary, index)
    }

    /// TODO docs
    #[inline]
    pub const fn primary(&self) -> TableName {
        self.0
    }

    /// TODO docs
    #[inline]
    pub const fn index(&self) -> usize {
        self.1
    }
}

impl From<SecondaryTableName> for u64 {
    #[inline]
    fn from(t: SecondaryTableName) -> Self {
        let index = t.1 as Self;
        let table: Self = t.0.into();
        (table & 0xFFFF_FFFF_FFFF_FFF0_u64)
            | (index & 0x0000_0000_0000_000F_u64)
    }
}

/// TODO docs
#[derive(Clone, Copy, Debug)]
pub enum SecondaryKey {
    /// TODO docs
    U64(u64),
    /// TODO docs
    F64(f64),
}

impl From<u64> for SecondaryKey {
    fn from(v: u64) -> Self {
        Self::U64(v)
    }
}

impl From<u32> for SecondaryKey {
    fn from(v: u32) -> Self {
        Self::U64(v.into())
    }
}

impl From<f64> for SecondaryKey {
    fn from(v: f64) -> Self {
        Self::F64(v)
    }
}

/// TODO docs
#[derive(Default, Clone, Copy)]
pub struct SecondaryKeys([Option<SecondaryKey>; 16]);

impl From<[Option<SecondaryKey>; 16]> for SecondaryKeys {
    fn from(v: [Option<SecondaryKey>; 16]) -> Self {
        Self(v)
    }
}

impl SecondaryKeys {
    /// TODO docs
    pub fn iter(&self) -> impl Iterator<Item = &Option<SecondaryKey>> {
        self.0.iter()
    }

    /// TODO docs
    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut Option<SecondaryKey>> {
        self.0.iter_mut()
    }
}

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
    pub fn primary_index(&self) -> PrimaryTableIndex<T> {
        PrimaryTableIndex::new(self.code, self.scope)
    }
}
