use account::AccountName;
use bytes::{Read, ReadError};
use bytes::{Write, WriteError};
use eosio_macros::*;
use eosio_sys::ctypes::*;
use lib::PhantomData;
use print::Printable;
use scope::ScopeName;

eosio_name!(TableName);

#[derive(Clone, Copy)]
pub struct SecondaryTableName(TableName, usize);

impl From<SecondaryTableName> for u64 {
    fn from(t: SecondaryTableName) -> u64 {
        let index = t.1 as u64;
        let table: u64 = t.0.into();
        (table & 0xFFFFFFFFFFFFFFF0u64) | (index & 0x000000000000000Fu64)
    }
}

pub trait TableRow: Read + Write {
    fn primary_key(&self) -> u64;

    fn secondary_keys(&self) -> [Option<&SecondaryKey>; 16] {
        [None; 16]
    }

    fn table<C, S, N>(code: C, scope: S, name: N) -> Table<Self>
    where
        C: Into<AccountName>,
        S: Into<ScopeName>,
        N: Into<TableName>,
    {
        Table::new(code, scope, name)
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
    fn erase(&self);
    // fn modify(&self);
    // fn previous(&self);
}

#[derive(Copy, Clone)]
pub struct PrimaryIterator<T>
where
    T: TableRow,
{
    value: i32,
    code: AccountName,
    scope: ScopeName,
    table: TableName,
    _data: PhantomData<T>,
}

impl<T> PartialEq for PrimaryIterator<T>
where
    T: TableRow,
{
    fn eq(&self, other: &PrimaryIterator<T>) -> bool {
        self.value == other.value
            && self.code == other.code
            && self.scope == other.scope
            && self.table == other.table
    }
}

impl<T> Printable for PrimaryIterator<T>
where
    T: TableRow,
{
    fn print(&self) {
        c!("PrimaryIterator(").print();
        self.value.print();
        c!(")").print();
    }
}

impl<T> Iterator for PrimaryIterator<T>
where
    T: TableRow,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl<T> TableIterator<T> for PrimaryIterator<T>
where
    T: TableRow,
{
    fn get(&self) -> Result<T, ReadError> {
        let mut bytes = [0u8; 10000]; // TODO: don't hardcode this?
        let ptr: *mut c_void = &mut bytes[..] as *mut _ as *mut c_void;
        unsafe {
            ::eosio_sys::db_get_i64(self.value, ptr, 10000);
        }
        T::read(&bytes, 0).map(|(t, _)| t)
    }

    fn erase(&self) {
        unsafe {
            ::eosio_sys::db_remove_i64(self.value);
        }
    }
}

#[derive(Copy, Clone)]
pub struct SecondaryIterator<K, T>
where
    K: SecondaryKey,
    T: TableRow,
{
    value: i32,
    code: AccountName,
    scope: ScopeName,
    table: TableName,
    index: usize,
    _data: PhantomData<(K, T)>,
}

// impl<T> Iterator for SecondaryIterator<T>
// where
//     T: TableRow,
// {
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }

// impl<T> TableIterator<T> for SecondaryIterator<u32, T> where T: TableRow {
//     fn get(&self) -> Result<T, ReadError> {

//     }
// }

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

    pub fn end(&self) -> PrimaryIterator<T> {
        let itr = unsafe {
            ::eosio_sys::db_end_i64(self.code.into(), self.scope.into(), self.name.into())
        };
        PrimaryIterator {
            value: itr,
            code: self.code,
            scope: self.scope,
            table: self.name,
            _data: self._row_type,
        }
    }

    pub fn is_end(&self, itr: &PrimaryIterator<T>) -> bool {
        itr.value == self.end().value
    }

    pub fn exists<Id>(&self, id: Id) -> bool
    where
        Id: Into<u64>,
    {
        let itr = self.find(id);
        !self.is_end(&itr)
    }

    pub fn find<Id>(&self, id: Id) -> PrimaryIterator<T>
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
        PrimaryIterator {
            value: itr,
            code: self.code,
            scope: self.scope,
            table: self.name,
            _data: self._row_type,
        }
    }

    // pub fn get(&self, itr: PrimaryIterator<T>) -> Result<T, ReadError> {
    //     let mut bytes = [0u8; 10000]; // TODO: don't hardcode this?
    //     let ptr: *mut c_void = &mut bytes[..] as *mut _ as *mut c_void;
    //     unsafe {
    //         ::eosio_sys::db_get_i64(itr.0, ptr, 10000);
    //     }
    //     T::read(&bytes, 0).map(|(t, _)| t)
    // }

    pub fn emplace<P>(&self, payer: P, item: T) -> Result<PrimaryIterator<T>, WriteError>
    where
        P: Into<AccountName>,
    {
        let id = item.primary_key();
        let payer = payer.into();

        let mut bytes = [0u8; 10000]; // TODO: don't hardcode this?
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

        Ok(PrimaryIterator {
            value: itr,
            code: self.code,
            scope: self.scope,
            table: self.name,
            _data: self._row_type,
        })
    }

    pub fn modify<P>(
        &self,
        itr: &PrimaryIterator<T>,
        payer: P,
        item: T,
    ) -> Result<usize, WriteError>
    where
        P: Into<AccountName>,
    {
        let mut bytes = [0u8; 10000]; // TODO: don't hardcode this?
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
