use account::AccountName;
use bytes::{Read, ReadError};
use bytes::{Write, WriteError};
use eosio_macros::*;
use eosio_sys::ctypes::*;
use lib::PhantomData;
use print::Print;
use scope::ScopeName;

eosio_name!(TableName);

#[derive(Clone, Copy)]
pub struct SecondaryTableName(TableName, usize);

impl From<SecondaryTableName> for u64 {
    fn from(t: SecondaryTableName) -> u64 {
        let index = t.1 as u64;
        let table: u64 = t.0.into();
        (table & 0xFFFF_FFFF_FFFF_FFF0u64) | (index & 0x0000_0000_0000_000Fu64)
    }
}

pub trait TableRow: Read + Write {
    const NAME: u64;

    fn primary_key(&self) -> u64;

    fn secondary_keys(&self) -> [Option<&SecondaryKey>; 16] {
        [None; 16]
    }

    fn table<C, S>(code: C, scope: S) -> Table<Self>
    where
        C: Into<AccountName>,
        S: Into<ScopeName>,
    {
        Table::new(code, scope, Self::NAME)
    }
}

pub trait SecondaryKey {
    fn end(&self, code: AccountName, scope: ScopeName, table: SecondaryTableName) -> i32;

    fn next(&self, iterator: i32) -> (i32, u64);

    fn remove(&self, iterator: i32);

    fn previous(&self, iterator: i32) -> (i32, u64);

    fn store(
        &self,
        scope: ScopeName,
        table: SecondaryTableName,
        payer: AccountName,
        id: u64,
    ) -> i32;

    fn update(&self, iterator: i32, payer: AccountName);

    fn lower_bound(
        &self,
        code: AccountName,
        scope: ScopeName,
        table: SecondaryTableName,
    ) -> (i32, u64);

    fn upper_bound(
        &self,
        code: AccountName,
        scope: ScopeName,
        table: SecondaryTableName,
    ) -> (i32, u64);

    fn find_primary(
        &self,
        code: AccountName,
        scope: ScopeName,
        table: SecondaryTableName,
        primary: u64,
    ) -> i32;

    fn find_secondary(
        &self,
        code: AccountName,
        scope: ScopeName,
        table: SecondaryTableName,
    ) -> (i32, u64);

    fn upsert(
        &self,
        code: AccountName,
        scope: ScopeName,
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
        impl SecondaryKey for $from {
            fn end(&self, code: AccountName, scope: ScopeName, table: SecondaryTableName) -> i32 {
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
                scope: ScopeName,
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
                scope: ScopeName,
                table: SecondaryTableName,
            ) -> (i32, u64) {
                <$to as From<$from>>::from(*self).lower_bound(code, scope, table)
            }
            fn upper_bound(
                &self,
                code: AccountName,
                scope: ScopeName,
                table: SecondaryTableName,
            ) -> (i32, u64) {
                <$to as From<$from>>::from(*self).upper_bound(code, scope, table)
            }
            fn find_primary(
                &self,
                code: AccountName,
                scope: ScopeName,
                table: SecondaryTableName,
                primary: u64,
            ) -> i32 {
                 <$to as From<$from>>::from(*self).find_primary(code, scope, table, primary)
            }
            fn find_secondary(
                &self,
                code: AccountName,
                scope: ScopeName,
                table: SecondaryTableName,
            ) -> (i32, u64) {
                <$to as From<$from>>::from(*self).find_secondary(code, scope, table)
            }
        }
    )*)
}

macro_rules! secondary_keys_impl {
    ($($t:ty, $i:ident)*) => ($(
        impl SecondaryKey for $t {
            fn end(&self, code: AccountName, scope: ScopeName, table: SecondaryTableName) -> i32 {
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
                scope: ScopeName,
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
                scope: ScopeName,
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
                scope: ScopeName,
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
                scope: ScopeName,
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
                scope: ScopeName,
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

pub trait TableIterator<T>: Iterator
where
    T: TableRow,
{
    fn get(&self) -> Result<T, ReadError>;
    fn erase(&self) -> Result<T, ReadError>;
    // fn update(&self);
    // fn previous(&self);
}

#[derive(Copy, Clone)]
pub struct PrimaryCursor<T>
where
    T: TableRow,
{
    value: i32,
    code: AccountName,
    scope: ScopeName,
    table: TableName,
    _data: PhantomData<T>,
}

impl<T> PartialEq for PrimaryCursor<T>
where
    T: TableRow,
{
    fn eq(&self, other: &PrimaryCursor<T>) -> bool {
        self.value == other.value
            && self.code == other.code
            && self.scope == other.scope
            && self.table == other.table
    }
}

impl<T> Print for PrimaryCursor<T>
where
    T: TableRow,
{
    fn print(&self) {
        c!("PrimaryCursor(").print();
        self.value.print();
        c!(")").print();
    }
}

impl<T> Iterator for PrimaryCursor<T>
where
    T: TableRow,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let end = unsafe {
            ::eosio_sys::db_end_i64(self.code.into(), self.scope.into(), self.table.into())
        };
        if self.value == end {
            None
        } else {
            let item = self.get();
            let mut pk = 0u64;
            let ptr: *mut u64 = &mut pk;
            self.value = unsafe { ::eosio_sys::db_next_i64(self.value, ptr) };
            match item {
                Ok(item) => Some(item),
                Err(_) => None,
            }
        }
    }
}

impl<T> TableIterator<T> for PrimaryCursor<T>
where
    T: TableRow,
{
    fn get(&self) -> Result<T, ReadError> {
        let mut bytes = [0u8; 1000]; // TODO: don't hardcode this?
        let ptr: *mut c_void = &mut bytes[..] as *mut _ as *mut c_void;
        unsafe {
            ::eosio_sys::db_get_i64(self.value, ptr, 1000);
        }
        T::read(&bytes, 0).map(|(t, _)| t)
    }

    fn erase(&self) -> Result<T, ReadError> {
        let item = self.get()?;
        let pk = item.primary_key();
        unsafe {
            ::eosio_sys::db_remove_i64(self.value);
        }

        for (i, k) in item.secondary_keys().iter().enumerate() {
            if let Some(k) = k {
                let table = SecondaryTableName(self.table, i);
                let end = k.end(self.code, self.scope, table);
                let itr = k.find_primary(self.code, self.scope, table, pk);
                if itr != end {
                    k.remove(itr);
                }
            }
        }
        Ok(item)
    }
}

impl<T> PrimaryCursor<T>
where
    T: TableRow,
{
    pub fn update<P>(&self, payer: P, item: &T) -> Result<usize, WriteError>
    where
        P: Into<AccountName>,
    {
        let table = Table::new(self.code, self.scope, self.table);
        table.update(&self, payer, item)
    }
}

#[derive(Copy, Clone)]
pub struct SecondaryCursor<'a, K, T>
where
    K: SecondaryKey,
    T: TableRow,
{
    value: i32,
    pk: u64,
    index: &'a SecondaryIndex<K, T>,
}

impl<'a, K, T> SecondaryCursor<'a, K, T>
where
    K: SecondaryKey,
    T: TableRow,
{
    pub fn get(&self) -> Result<T, ReadError> {
        let pk_itr = unsafe {
            ::eosio_sys::db_find_i64(
                self.index.code.into(),
                self.index.scope.into(),
                self.index.table.0.into(),
                self.pk,
            )
        };
        let mut bytes = [0u8; 1000]; // TODO: don't hardcode this?
        let ptr: *mut c_void = &mut bytes[..] as *mut _ as *mut c_void;
        unsafe {
            ::eosio_sys::db_get_i64(pk_itr, ptr, 1000);
        }
        T::read(&bytes, 0).map(|(t, _)| t)
    }

    pub fn update<P>(&self, payer: P, item: &T) -> Result<usize, WriteError>
    where
        P: Into<AccountName>,
    {
        let table = Table::new(self.index.code, self.index.scope, self.index.table.0);
        match table.find(self.pk) {
            Some(pk_itr) => table.update(&pk_itr, payer, item),
            None => Err(WriteError::NotEnoughSpace),
        }
    }

    pub fn iter(&'a self) -> SecondaryIter<'a, K, T> {
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
        SecondaryIter {
            value: self.value,
            pk: self.pk,
            pk_end,
            sk_end,
            index: self.index,
        }
    }
}

pub struct SecondaryIter<'a, K, T>
where
    K: SecondaryKey,
    T: TableRow,
{
    value: i32,
    pk: u64,
    pk_end: i32,
    sk_end: i32,
    index: &'a SecondaryIndex<K, T>,
}

impl<'a, K, T> Iterator for SecondaryIter<'a, K, T>
where
    K: SecondaryKey,
    T: TableRow,
{
    type Item = SecondaryCursor<'a, K, T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.value == self.sk_end {
            return None;
        }

        let pk_itr = unsafe {
            ::eosio_sys::db_find_i64(
                self.index.code.into(),
                self.index.scope.into(),
                self.index.table.0.into(),
                self.pk,
            )
        };

        if pk_itr == self.pk_end {
            return None;
        }

        let cursor = SecondaryCursor {
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

#[derive(Copy, Clone)]
pub struct SecondaryIndex<K, T>
where
    K: SecondaryKey,
    T: TableRow,
{
    code: AccountName,
    scope: ScopeName,
    table: SecondaryTableName,
    key: K,
    _data: PhantomData<T>,
}

impl<'a, K, T> SecondaryIndex<K, T>
where
    K: SecondaryKey,
    T: TableRow,
{
    pub fn new<C, S, N>(code: C, scope: S, name: N, key: K, index: usize) -> SecondaryIndex<K, T>
    where
        C: Into<AccountName>,
        S: Into<ScopeName>,
        N: Into<TableName>,
    {
        SecondaryIndex {
            code: code.into(),
            scope: scope.into(),
            table: SecondaryTableName(name.into(), index),
            key,
            _data: PhantomData,
        }
    }

    pub fn lower_bound(&self, key: &K) -> SecondaryCursor<K, T> {
        let (itr, pk) = key.lower_bound(self.code, self.scope, self.table);
        SecondaryCursor {
            value: itr,
            pk,
            index: self,
        }
    }
}

pub struct Table<T>
where
    T: TableRow,
{
    code: AccountName,
    scope: ScopeName,
    name: TableName,
    _row_type: PhantomData<T>,
}

impl<T> Table<T>
where
    T: TableRow,
{
    pub fn new<C, S, N>(code: C, scope: S, name: N) -> Table<T>
    where
        C: Into<AccountName>,
        S: Into<ScopeName>,
        N: Into<TableName>,
    {
        Table {
            code: code.into(),
            scope: scope.into(),
            name: name.into(),
            _row_type: PhantomData,
        }
    }

    pub fn end(&self) -> PrimaryCursor<T> {
        let itr = unsafe {
            ::eosio_sys::db_end_i64(self.code.into(), self.scope.into(), self.name.into())
        };
        PrimaryCursor {
            value: itr,
            code: self.code,
            scope: self.scope,
            table: self.name,
            _data: self._row_type,
        }
    }

    pub fn is_end(&self, itr: &PrimaryCursor<T>) -> bool {
        itr.value == self.end().value
    }

    pub fn exists<Id>(&self, id: Id) -> bool
    where
        Id: Into<u64>,
    {
        self.find(id).is_some()
    }

    pub fn find<Id>(&self, id: Id) -> Option<PrimaryCursor<T>>
    where
        Id: Into<u64>,
    {
        let itr = unsafe {
            ::eosio_sys::db_find_i64(
                self.code.into(),
                self.scope.into(),
                self.name.into(),
                id.into(),
            )
        };
        let end = unsafe {
            ::eosio_sys::db_end_i64(self.code.into(), self.scope.into(), self.name.into())
        };
        if itr == end {
            None
        } else {
            Some(PrimaryCursor {
                value: itr,
                code: self.code,
                scope: self.scope,
                table: self.name,
                _data: self._row_type,
            })
        }
    }

    // pub fn get(&self, itr: PrimaryCursor<T>) -> Result<T, ReadError> {
    //     let mut bytes = [0u8; 1000]; // TODO: don't hardcode this?
    //     let ptr: *mut c_void = &mut bytes[..] as *mut _ as *mut c_void;
    //     unsafe {
    //         ::eosio_sys::db_get_i64(itr.0, ptr, 1000);
    //     }
    //     T::read(&bytes, 0).map(|(t, _)| t)
    // }

    pub fn insert<P>(&self, payer: P, item: &T) -> Result<PrimaryCursor<T>, WriteError>
    where
        P: Into<AccountName>,
    {
        let id = item.primary_key();
        let payer = payer.into();

        let mut bytes = [0u8; 1000]; // TODO: don't hardcode this?
        let pos = item.write(&mut bytes, 0)?;
        let ptr: *const c_void = &bytes[..] as *const _ as *const c_void;
        let itr = unsafe {
            ::eosio_sys::db_store_i64(
                self.scope.into(),
                self.name.into(),
                payer.into(),
                id,
                ptr,
                pos as u32,
            )
        };

        // store secondary indexes
        for (i, k) in item.secondary_keys().iter().enumerate() {
            if let Some(k) = k {
                let table = SecondaryTableName(self.name, i);
                k.store(self.scope, table, payer, id);
            }
        }

        Ok(PrimaryCursor {
            value: itr,
            code: self.code,
            scope: self.scope,
            table: self.name,
            _data: self._row_type,
        })
    }

    fn update<P>(&self, itr: &PrimaryCursor<T>, payer: P, item: &T) -> Result<usize, WriteError>
    where
        P: Into<AccountName>,
    {
        let mut bytes = [0u8; 1000]; // TODO: don't hardcode this?
        let pos = item.write(&mut bytes, 0)?;
        let ptr: *const c_void = &bytes[..] as *const _ as *const c_void;
        let payer: AccountName = payer.into();
        unsafe { ::eosio_sys::db_update_i64(itr.value, payer.into(), ptr, pos as u32) }

        let pk = item.primary_key();

        for (i, k) in item.secondary_keys().iter().enumerate() {
            if let Some(k) = k {
                let table = SecondaryTableName(self.name, i);
                k.upsert(self.code, self.scope, table, payer, pk);
            }
        }

        Ok(pos)
    }
}
