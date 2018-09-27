use account::AccountName;
use bytes::{Read, ReadError};
use bytes::{Write, WriteError};
use eosio_macros::*;
use eosio_sys::ctypes::*;
use lib::PhantomData;
use print::Printable;
use scope::ScopeName;

eosio_name!(TableName);

pub trait TableRow: Read + Write {
    fn primary_key(&self) -> u64;

    fn table<C, S, N>(code: C, scope: S, name: N) -> Table<Self>
    where
        C: Into<AccountName>,
        S: Into<ScopeName>,
        N: Into<TableName>,
    {
        Table::new(code, scope, name)
    }
}

#[derive(PartialEq, Copy, Clone)]
pub struct TableIter(i32);

impl From<i32> for TableIter {
    fn from(itr: i32) -> Self {
        TableIter(itr)
    }
}

impl Printable for TableIter {
    fn print(&self) {
        c!("TableIter(").print();
        self.as_i32().print();
        c!(")").print();
    }
}

impl TableIter {
    pub fn as_i32(&self) -> i32 {
        self.0
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

    pub fn end(&self) -> TableIter {
        let itr = unsafe {
            ::eosio_sys::db_end_i64(self.code.into(), self.scope.into(), self.name.into())
        };
        itr.into()
    }

    pub fn is_end(&self, itr: TableIter) -> bool {
        itr == self.end()
    }

    pub fn exists<Id>(&self, id: Id) -> bool
    where
        Id: Into<u64>,
    {
        let itr = self.find(id);
        !self.is_end(itr)
    }

    pub fn find<Id>(&self, id: Id) -> TableIter
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
        itr.into()
    }

    pub fn get(&self, itr: TableIter) -> Result<T, ReadError> {
        let mut bytes = [0u8; 10000]; // TODO: don't hardcode this?
        let ptr: *mut c_void = &mut bytes[..] as *mut _ as *mut c_void;
        unsafe {
            ::eosio_sys::db_get_i64(itr.0, ptr, 10000);
        }
        T::read(&bytes, 0).map(|(t, _)| t)
    }

    pub fn emplace<P>(&self, payer: P, item: T) -> Result<TableIter, WriteError>
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
        Ok(TableIter(itr))
    }

    pub fn modify<P>(&self, itr: TableIter, payer: P, item: T) -> Result<usize, WriteError>
    where
        P: Into<AccountName>,
    {
        let mut bytes = [0u8; 10000]; // TODO: don't hardcode this?
        let pos = item.write(&mut bytes, 0)?;
        let ptr: *const c_void = &bytes[..] as *const _ as *const c_void;
        let payer: AccountName = payer.into();
        unsafe { ::eosio_sys::db_update_i64(itr.0, payer.into(), ptr, pos as u32) }
        Ok(pos)
    }

    pub fn erase(&self, itr: TableIter) {
        unsafe {
            ::eosio_sys::db_remove_i64(itr.0);
        }
    }
}
