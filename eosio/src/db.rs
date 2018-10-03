use account::AccountName;
use bytes::{Read, ReadError};
use bytes::{Write, WriteError};
use eosio_macros::*;
use eosio_sys::ctypes::*;
use lib::PhantomData;
use print::Printable;
use scope::ScopeName;

eosio_name!(TableName);

#[derive(Clone, Copy, Debug)]
pub enum SecondaryKey {
    U64(u64),
    // TODO: U128 (idx128)
    // TODO: U256 (idx256)
    F64(f64),
    // TODO: F128 (idx_long_double)
}

pub trait TableRow: Read + Write {
    fn primary_key(&self) -> u64;

    fn secondary_keys(&self) -> [Option<SecondaryKey>; 16] {
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

#[derive(Copy, Clone)]
pub struct TableIter<T>
where
    T: TableRow,
{
    value: i32,
    code: AccountName,
    scope: ScopeName,
    table: TableName,
    _data: PhantomData<T>,
}

impl<T> PartialEq for TableIter<T>
where
    T: TableRow,
{
    fn eq(&self, other: &TableIter<T>) -> bool {
        self.value == other.value
    }
}

impl<T> Printable for TableIter<T>
where
    T: TableRow,
{
    fn print(&self) {
        c!("TableIter(").print();
        self.value.print();
        c!(")").print();
    }
}

impl<T> TableIter<T>
where
    T: TableRow,
{
    pub fn get(&self) -> Result<T, ReadError> {
        let mut bytes = [0u8; 10000]; // TODO: don't hardcode this?
        let ptr: *mut c_void = &mut bytes[..] as *mut _ as *mut c_void;
        unsafe {
            ::eosio_sys::db_get_i64(self.value, ptr, 10000);
        }
        T::read(&bytes, 0).map(|(t, _)| t)
    }

    pub fn erase(&self) {
        unsafe {
            ::eosio_sys::db_remove_i64(self.value);
        }
    }
}

pub trait TableIndex {
    fn lowerbound(&self);
    fn upperbound(&self);
}

pub struct SecondaryIndex<K> {
    code: AccountName,
    scope: ScopeName,
    table: TableName,
    index: usize,
    _key_type: PhantomData<K>,
}

impl<K> SecondaryIndex<K> {
    pub fn new<C, S, N>(code: C, scope: S, name: N, index: usize) -> Self
    where
        C: Into<AccountName>,
        S: Into<ScopeName>,
        N: Into<TableName>,
    {
        SecondaryIndex {
            code: code.into(),
            scope: scope.into(),
            table: name.into(),
            index,
            _key_type: PhantomData,
        }
    }

    fn table_name(&self) -> u64 {
        let base: u64 = self.table.into();
        let index = self.index as u64;
        (base & 0xFFFFFFFFFFFFFFF0u64) | (index & 0x000000000000000Fu64)
    }
}

impl TableIndex for SecondaryIndex<u64> {
    fn lowerbound(&self) {
        "LOWERBOUND u64\0".print();
    }

    fn upperbound(&self) {
        "UPPERBOUND u64\0".print();
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

    pub fn end(&self) -> TableIter<T> {
        let itr = unsafe {
            ::eosio_sys::db_end_i64(self.code.into(), self.scope.into(), self.name.into())
        };
        TableIter {
            value: itr,
            code: self.code,
            scope: self.scope,
            table: self.name,
            _data: self._row_type,
        }
    }

    pub fn is_end(&self, itr: &TableIter<T>) -> bool {
        itr.value == self.end().value
    }

    pub fn exists<Id>(&self, id: Id) -> bool
    where
        Id: Into<u64>,
    {
        let itr = self.find(id);
        !self.is_end(&itr)
    }

    pub fn find<Id>(&self, id: Id) -> TableIter<T>
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
        TableIter {
            value: itr,
            code: self.code,
            scope: self.scope,
            table: self.name,
            _data: self._row_type,
        }
    }

    // pub fn get(&self, itr: TableIter<T>) -> Result<T, ReadError> {
    //     let mut bytes = [0u8; 10000]; // TODO: don't hardcode this?
    //     let ptr: *mut c_void = &mut bytes[..] as *mut _ as *mut c_void;
    //     unsafe {
    //         ::eosio_sys::db_get_i64(itr.0, ptr, 10000);
    //     }
    //     T::read(&bytes, 0).map(|(t, _)| t)
    // }

    pub fn emplace<P>(&self, payer: P, item: T) -> Result<TableIter<T>, WriteError>
    where
        P: Into<AccountName>,
    {
        let mut bytes = [0u8; 10000]; // TODO: don't hardcode this?
        let pos = item.write(&mut bytes, 0)?;
        let ptr: *const c_void = &bytes[..] as *const _ as *const c_void;
        let itr = unsafe {
            ::eosio_sys::db_store_i64(
                self.scope.into(),
                self.name.into(),
                payer.into().into(),
                item.primary_key(),
                ptr,
                pos as u32,
            )
        };

        // store secondary indexes
        // for (i, k) in item.secondary_keys().iter().enumerate() {
        //     match k {
        //         SecondaryKey::U64()
        //     }
        // }

        Ok(TableIter {
            value: itr,
            code: self.code,
            scope: self.scope,
            table: self.name,
            _data: self._row_type,
        })
    }

    pub fn modify<P>(&self, itr: &TableIter<T>, payer: P, item: T) -> Result<usize, WriteError>
    where
        P: Into<AccountName>,
    {
        let mut bytes = [0u8; 10000]; // TODO: don't hardcode this?
        let pos = item.write(&mut bytes, 0)?;
        let ptr: *const c_void = &bytes[..] as *const _ as *const c_void;
        let payer: AccountName = payer.into();
        unsafe { ::eosio_sys::db_update_i64(itr.value, payer.into(), ptr, pos as u32) }

        // update secondary indexes

        Ok(pos)
    }
}
