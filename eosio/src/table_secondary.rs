use account::AccountName;
use bytes::{ReadError, WriteError};
use eosio_sys::ctypes::*;
use lib::PhantomData;
use table::*;
use table_primary::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash, PartialOrd, Ord)]
pub struct SecondaryTableName(PrimaryTableName, usize);

impl SecondaryTableName {
    pub fn new(primary: PrimaryTableName, index: usize) -> Self {
        SecondaryTableName(primary, index)
    }
}

impl From<SecondaryTableName> for u64 {
    fn from(t: SecondaryTableName) -> u64 {
        let index = t.1 as u64;
        let table: u64 = t.0.into();
        (table & 0xFFFF_FFFF_FFFF_FFF0u64) | (index & 0x0000_0000_0000_000Fu64)
    }
}

pub trait SecondaryTableKey {
    fn end(&self, code: AccountName, scope: TableScope, table: SecondaryTableName) -> i32;

    fn next(&self, iterator: i32) -> (i32, u64);

    fn remove(&self, iterator: i32);

    fn previous(&self, iterator: i32) -> (i32, u64);

    fn store(
        &self,
        scope: TableScope,
        table: SecondaryTableName,
        payer: AccountName,
        id: u64,
    ) -> i32;

    fn update(&self, iterator: i32, payer: AccountName);

    fn lower_bound(
        &self,
        code: AccountName,
        scope: TableScope,
        table: SecondaryTableName,
    ) -> (i32, u64);

    fn upper_bound(
        &self,
        code: AccountName,
        scope: TableScope,
        table: SecondaryTableName,
    ) -> (i32, u64);

    fn find_primary(
        &self,
        code: AccountName,
        scope: TableScope,
        table: SecondaryTableName,
        primary: u64,
    ) -> i32;

    fn find_secondary(
        &self,
        code: AccountName,
        scope: TableScope,
        table: SecondaryTableName,
    ) -> (i32, u64);

    fn upsert(
        &self,
        code: AccountName,
        scope: TableScope,
        table: SecondaryTableName,
        payer: AccountName,
        id: u64,
    ) {
        let end = self.end(code, scope, table);
        let itr = self.find_primary(code, scope, table, id);
        if itr == end {
            self.store(scope, table, payer, id);
        } else {
            self.update(itr, payer);
        }
    }
}

macro_rules! secondary_keys_converted {
    ($($to:ty, $from:ty)*) => ($(
        impl SecondaryTableKey for $from {
            fn end(&self, code: AccountName, scope: TableScope, table: SecondaryTableName) -> i32 {
                <$to as From<$from>>::from(*self).end(code, scope, table)
            }
            fn next(&self, iterator: i32) -> (i32, u64) {
                <$to as From<$from>>::from(*self).next(iterator)
            }
            fn previous(&self, iterator: i32) -> (i32, u64) {
                <$to as From<$from>>::from(*self).previous(iterator)
            }
            fn remove(&self, iterator: i32) {
                <$to as From<$from>>::from(*self).remove(iterator)
            }
            fn store(
                &self,
                scope: TableScope,
                table: SecondaryTableName,
                payer: AccountName,
                id: u64,
            ) -> i32 {
                <$to as From<$from>>::from(*self).store(scope, table, payer, id)
            }
            fn update(&self, iterator: i32, payer: AccountName) {
                <$to as From<$from>>::from(*self).update(iterator, payer)
            }
            fn lower_bound(
                &self,
                code: AccountName,
                scope: TableScope,
                table: SecondaryTableName,
            ) -> (i32, u64) {
                <$to as From<$from>>::from(*self).lower_bound(code, scope, table)
            }
            fn upper_bound(
                &self,
                code: AccountName,
                scope: TableScope,
                table: SecondaryTableName,
            ) -> (i32, u64) {
                <$to as From<$from>>::from(*self).upper_bound(code, scope, table)
            }
            fn find_primary(
                &self,
                code: AccountName,
                scope: TableScope,
                table: SecondaryTableName,
                primary: u64,
            ) -> i32 {
                 <$to as From<$from>>::from(*self).find_primary(code, scope, table, primary)
            }
            fn find_secondary(
                &self,
                code: AccountName,
                scope: TableScope,
                table: SecondaryTableName,
            ) -> (i32, u64) {
                <$to as From<$from>>::from(*self).find_secondary(code, scope, table)
            }
        }
    )*)
}

macro_rules! secondary_keys_impl {
    ($($t:ty, $i:ident)*) => ($(
        impl SecondaryTableKey for $t {
            fn end(&self, code: AccountName, scope: TableScope, table: SecondaryTableName) -> i32 {
                use ::eosio_sys::*;
                unsafe { concat_idents!(db_, $i, _end)(code.into(), scope.into(), table.into()) }
            }
            fn next(&self, iterator: i32) -> (i32, u64) {
                use ::eosio_sys::*;
                let mut pk = 0u64;
                let ptr: *mut u64 = &mut pk;
                let itr = unsafe { concat_idents!(db_, $i, _next)(iterator, ptr) };
                (itr, pk)
            }
            fn previous(&self, iterator: i32) -> (i32, u64) {
                use ::eosio_sys::*;
                let mut pk = 0u64;
                let ptr: *mut u64 = &mut pk;
                let itr = unsafe { concat_idents!(db_, $i, _previous)(iterator, ptr) };
                (itr, pk)
            }
            fn remove(&self, iterator: i32) {
                use ::eosio_sys::*;
                unsafe { concat_idents!(db_, $i, _remove)(iterator) }
            }
            fn store(
                &self,
                scope: TableScope,
                table: SecondaryTableName,
                payer: AccountName,
                id: u64,
            ) -> i32 {
                use ::eosio_sys::*;
                let secondary: *const Self = self;
                unsafe {
                    concat_idents!(db_, $i, _store)(scope.into(), table.into(), payer.into(), id, secondary)
                }
            }
            fn update(&self, iterator: i32, payer: AccountName) {
                use ::eosio_sys::*;
                let secondary: *const Self = self;
                unsafe {
                    concat_idents!(db_, $i, _update)(iterator, payer.into(), secondary)
                }
            }
            fn lower_bound(
                &self,
                code: AccountName,
                scope: TableScope,
                table: SecondaryTableName,
            ) -> (i32, u64) {
                use ::eosio_sys::*;
                let mut pk = 0u64;
                let mut sk = self.clone();
                let itr = unsafe {
                    concat_idents!(db_, $i, _lowerbound)(
                        code.into(),
                        scope.into(),
                        table.into(),
                        &mut sk as *mut $t,
                        &mut pk as *mut u64,
                    )
                };
                (itr, pk)
            }
            fn upper_bound(
                &self,
                code: AccountName,
                scope: TableScope,
                table: SecondaryTableName,
            ) -> (i32, u64) {
                use ::eosio_sys::*;
                let mut pk = 0u64;
                let mut sk = self.clone();
                let itr = unsafe {
                    concat_idents!(db_, $i, _upperbound)(
                        code.into(),
                        scope.into(),
                        table.into(),
                        &mut sk as *mut $t,
                        &mut pk as *mut u64,
                    )
                };
                (itr, pk)
            }
            fn find_primary(
                &self,
                code: AccountName,
                scope: TableScope,
                table: SecondaryTableName,
                primary: u64,
            ) -> i32 {
                use ::eosio_sys::*;
                let mut sk = self.clone();
                unsafe {
                    concat_idents!(db_, $i, _find_primary)(
                        code.into(),
                        scope.into(),
                        table.into(),
                        &mut sk as *mut $t,
                        primary,
                    )
                }
            }
            fn find_secondary(
                &self,
                code: AccountName,
                scope: TableScope,
                table: SecondaryTableName,
            ) -> (i32, u64) {
                use ::eosio_sys::*;
                let mut pk = 0u64;
                let secondary: *const Self = self;
                let itr = unsafe {
                    concat_idents!(db_, $i, _find_secondary)(
                        code.into(),
                        scope.into(),
                        table.into(),
                        secondary,
                        &mut pk as *mut u64,
                    )
                };
                (itr, pk)
            }
        }
    )*)
}

secondary_keys_impl!(
    u64, idx64
    f64, idx_double
    // TODO: u128, idx128
    // TODO: u256, idx256
    // TODO: f128, idx_long_double
);

secondary_keys_converted!(
    u64, u8
    u64, u16
    u64, u32
    f64, f32
);

#[derive(Debug, Copy, Clone)]
pub struct SecondaryTableCursor<'a, K, T>
where
    K: SecondaryTableKey,
    T: TableRow,
{
    value: i32,
    pk: u64,
    index: &'a SecondaryTableIndex<K, T>,
}

impl<'a, K, T> TableCursor<T> for SecondaryTableCursor<'a, K, T>
where
    K: SecondaryTableKey,
    T: TableRow,
{
    fn get(&self) -> Result<T, ReadError> {
        let pk_itr = unsafe {
            ::eosio_sys::db_find_i64(
                self.index.code.into(),
                self.index.scope.into(),
                self.index.table.0.into(),
                self.pk,
            )
        };
        let mut bytes = [0u8; 1000]; // TODO: don't hardcode this
        let ptr: *mut c_void = &mut bytes[..] as *mut _ as *mut c_void;
        unsafe {
            ::eosio_sys::db_get_i64(pk_itr, ptr, 1000);
        }
        T::read(&bytes, 0).map(|(t, _)| t)
    }

    fn remove(&self) -> Result<T, ReadError> {
        let table = self.index.to_primary_index();
        match table.find(self.pk) {
            Some(cursor) => cursor.remove(),
            None => Err(ReadError::NotEnoughBytes), // TODO: better error
        }
    }

    fn update(&self, payer: Option<AccountName>, item: &T) -> Result<usize, WriteError> {
        let table = self.index.to_primary_index();
        match table.find(self.pk) {
            Some(cursor) => cursor.update(payer, item),
            None => Err(WriteError::NotEnoughSpace), // TODO: better error
        }
    }
}

impl<'a, K, T> IntoIterator for SecondaryTableCursor<'a, K, T>
where
    K: SecondaryTableKey,
    T: TableRow,
{
    type Item = Self;
    type IntoIter = SecondaryTableIterator<'a, K, T>;
    fn into_iter(self) -> Self::IntoIter {
        let sk_end = self
            .index
            .key
            .end(self.index.code, self.index.scope, self.index.table);
        let pk_end = unsafe {
            ::eosio_sys::db_end_i64(
                self.index.code.into(),
                self.index.scope.into(),
                self.index.table.0.into(),
            )
        };
        SecondaryTableIterator {
            value: self.value,
            pk: self.pk,
            pk_end,
            sk_end,
            index: self.index,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SecondaryTableIterator<'a, K, T>
where
    K: SecondaryTableKey,
    T: TableRow,
{
    value: i32,
    pk: u64,
    pk_end: i32,
    sk_end: i32,
    index: &'a SecondaryTableIndex<K, T>,
}

impl<'a, K, T> Iterator for SecondaryTableIterator<'a, K, T>
where
    K: SecondaryTableKey,
    T: TableRow,
{
    type Item = SecondaryTableCursor<'a, K, T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.value == self.sk_end {
            return None;
        }

        let cursor = SecondaryTableCursor {
            value: self.value,
            pk: self.pk,
            index: self.index,
        };
        let (itr, pk) = self.index.key.next(self.value);
        self.value = itr;
        self.pk = pk;

        Some(cursor)
    }
}

impl<'a, K, T> DoubleEndedIterator for SecondaryTableIterator<'a, K, T>
where
    K: SecondaryTableKey,
    T: TableRow,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.value == -1 {
            return None;
        }

        let cursor = SecondaryTableCursor {
            value: self.value,
            pk: self.pk,
            index: self.index,
        };
        let (itr, pk) = self.index.key.previous(self.value);
        self.value = itr;
        self.pk = pk;

        Some(cursor)
    }
}

impl<'a, K, T> TableIterator for SecondaryTableIterator<'a, K, T>
where
    K: SecondaryTableKey,
    T: TableRow,
{}

#[derive(Copy, Clone, Debug)]
pub struct SecondaryTableIndex<K, T>
where
    K: SecondaryTableKey,
    T: TableRow,
{
    code: AccountName,
    scope: TableScope,
    table: SecondaryTableName,
    key: K,
    _data: PhantomData<T>,
}

impl<K, T> SecondaryTableIndex<K, T>
where
    K: SecondaryTableKey,
    T: TableRow,
{
    pub fn new<C, S, N>(
        code: C,
        scope: S,
        name: N,
        key: K,
        index: usize,
    ) -> SecondaryTableIndex<K, T>
    where
        C: Into<AccountName>,
        S: Into<TableScope>,
        N: Into<PrimaryTableName>,
    {
        SecondaryTableIndex {
            code: code.into(),
            scope: scope.into(),
            table: SecondaryTableName(name.into(), index),
            key,
            _data: PhantomData,
        }
    }

    fn to_primary_index(&self) -> PrimaryTableIndex<T> {
        PrimaryTableIndex::new(self.code, self.scope, self.table.0)
    }
}

impl<'a, K, T> TableIndex<'a, K, T> for SecondaryTableIndex<K, T>
where
    K: SecondaryTableKey + Clone + 'a,
    T: TableRow + 'a,
{
    type Cursor = SecondaryTableCursor<'a, K, T>;

    fn lower_bound<N>(&'a self, key: N) -> Option<Self::Cursor>
    where
        N: Into<K>,
    {
        let key = key.into();
        let (value, pk) = key.lower_bound(self.code, self.scope, self.table);
        let end = key.end(self.code, self.scope, self.table);
        if value != end {
            Some(SecondaryTableCursor {
                value,
                pk,
                index: self,
            })
        } else {
            None
        }
    }

    fn upper_bound<N>(&'a self, key: N) -> Option<Self::Cursor>
    where
        N: Into<K>,
    {
        let key = key.into();
        let (value, pk) = key.upper_bound(self.code, self.scope, self.table);
        let end = key.end(self.code, self.scope, self.table);
        if value != end {
            Some(SecondaryTableCursor {
                value,
                pk,
                index: self,
            })
        } else {
            None
        }
    }

    fn insert(&self, payer: AccountName, item: &T) -> Result<(), WriteError> {
        let table = self.to_primary_index();
        table.insert(payer, item)
    }
}
