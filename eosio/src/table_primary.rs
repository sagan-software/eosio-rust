use account::AccountName;
use bytes::{ReadError, WriteError};
use eosio_macros::*;
use eosio_sys::ctypes::*;
use lib::PhantomData;
use print::Print;
use table::*;

eosio_name!(PrimaryTableName);

#[derive(Copy, Clone, Debug)]
pub struct PrimaryTableCursor<T>
where
    T: TableRow,
{
    value: i32,
    code: AccountName,
    scope: TableScope,
    table: PrimaryTableName,
    _data: PhantomData<T>,
}

impl<T> PartialEq for PrimaryTableCursor<T>
where
    T: TableRow,
{
    fn eq(&self, other: &PrimaryTableCursor<T>) -> bool {
        self.value == other.value
            && self.code == other.code
            && self.scope == other.scope
            && self.table == other.table
    }
}

impl<T> Print for PrimaryTableCursor<T>
where
    T: TableRow,
{
    fn print(&self) {
        c!("PrimaryTableCursor(").print();
        self.value.print();
        c!(")").print();
    }
}

impl<T> TableCursor<T> for PrimaryTableCursor<T>
where
    T: TableRow,
{
    fn get(&self) -> Result<T, ReadError> {
        let mut bytes = [0u8; 1000]; // TODO: don't hardcode this
        let ptr: *mut c_void = &mut bytes[..] as *mut _ as *mut c_void;
        unsafe {
            ::eosio_sys::db_get_i64(self.value, ptr, 1000);
        }
        T::read(&bytes, 0).map(|(t, _)| t)
    }

    fn remove(&self) -> Result<T, ReadError> {
        let item = self.get()?;
        let pk = item.primary_key();
        unsafe {
            ::eosio_sys::db_remove_i64(self.value);
        }

        for (i, k) in item.secondary_keys().iter().enumerate() {
            if let Some(k) = k {
                let table = ::table_secondary::SecondaryTableName::new(self.table, i);
                let end = k.end(self.code, self.scope, table);
                let itr = k.find_primary(self.code, self.scope, table, pk);
                if itr != end {
                    k.remove(itr);
                }
            }
        }
        Ok(item)
    }

    fn update<P>(&self, payer: P, item: &T) -> Result<usize, WriteError>
    where
        P: Into<AccountName>,
    {
        let table = PrimaryTableIndex::new(self.code, self.scope, self.table);
        table.update(&self, payer, item)
    }
}

impl<'a, T> IntoIterator for PrimaryTableCursor<T>
where
    T: TableRow,
{
    type Item = Self;
    type IntoIter = PrimaryTableIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        let end = unsafe {
            ::eosio_sys::db_end_i64(self.code.into(), self.scope.into(), self.table.into())
        };
        PrimaryTableIterator {
            value: self.value,
            end,
            code: self.code,
            scope: self.scope,
            table: self.table,
            _data: PhantomData,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PrimaryTableIterator<T>
where
    T: TableRow,
{
    value: i32,
    end: i32,
    code: AccountName,
    scope: TableScope,
    table: PrimaryTableName,
    _data: PhantomData<T>,
}

impl<T> Iterator for PrimaryTableIterator<T>
where
    T: TableRow,
{
    type Item = PrimaryTableCursor<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.value == self.end {
            return None;
        }

        let cursor = PrimaryTableCursor {
            value: self.value,
            code: self.code,
            scope: self.scope,
            table: self.table,
            _data: PhantomData,
        };

        let mut pk = 0u64;
        let ptr: *mut u64 = &mut pk;
        self.value = unsafe { ::eosio_sys::db_next_i64(self.value, ptr) };

        Some(cursor)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PrimaryTableIndex<T>
where
    T: TableRow,
{
    code: AccountName,
    scope: TableScope,
    name: PrimaryTableName,
    _data: PhantomData<T>,
}

impl<'a, K, T> TableIndex<'a, K, T> for PrimaryTableIndex<T>
where
    K: Clone + Into<u64>,
    T: TableRow + 'a,
{
    type Cursor = PrimaryTableCursor<T>;

    fn lower_bound(&'a self, key: &'a K) -> Option<Self::Cursor> {
        let itr = unsafe {
            ::eosio_sys::db_lowerbound_i64(
                self.code.into(),
                self.scope.into(),
                self.name.into(),
                key.clone().into(),
            )
        };
        let end = unsafe {
            ::eosio_sys::db_end_i64(self.code.into(), self.scope.into(), self.name.into())
        };
        if itr != end {
            Some(PrimaryTableCursor {
                value: itr,
                code: self.code,
                scope: self.scope,
                table: self.name,
                _data: self._data,
            })
        } else {
            None
        }
    }

    fn upper_bound(&'a self, key: &'a K) -> Option<Self::Cursor> {
        let itr = unsafe {
            ::eosio_sys::db_upperbound_i64(
                self.code.into(),
                self.scope.into(),
                self.name.into(),
                key.clone().into(),
            )
        };
        let end = unsafe {
            ::eosio_sys::db_end_i64(self.code.into(), self.scope.into(), self.name.into())
        };
        if itr != end {
            Some(PrimaryTableCursor {
                value: itr,
                code: self.code,
                scope: self.scope,
                table: self.name,
                _data: self._data,
            })
        } else {
            None
        }
    }
}

impl<T> PrimaryTableIndex<T>
where
    T: TableRow,
{
    pub fn new<C, S, N>(code: C, scope: S, name: N) -> Self
    where
        C: Into<AccountName>,
        S: Into<TableScope>,
        N: Into<PrimaryTableName>,
    {
        PrimaryTableIndex {
            code: code.into(),
            scope: scope.into(),
            name: name.into(),
            _data: PhantomData,
        }
    }

    pub fn exists<Id>(&self, id: Id) -> bool
    where
        Id: Into<u64>,
    {
        self.find(id).is_some()
    }

    pub fn find<Id>(&self, id: Id) -> Option<PrimaryTableCursor<T>>
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
            Some(PrimaryTableCursor {
                value: itr,
                code: self.code,
                scope: self.scope,
                table: self.name,
                _data: self._data,
            })
        }
    }

    pub fn insert<P>(&self, payer: P, item: &T) -> Result<PrimaryTableCursor<T>, WriteError>
    where
        P: Into<AccountName>,
    {
        let id = item.primary_key();
        let payer = payer.into();

        let size = ::lib::size_of_val(item);
        let mut bytes = vec![0u8; size];
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
                let table = ::table_secondary::SecondaryTableName::new(self.name, i);
                k.store(self.scope, table, payer, id);
            }
        }

        Ok(PrimaryTableCursor {
            value: itr,
            code: self.code,
            scope: self.scope,
            table: self.name,
            _data: self._data,
        })
    }

    fn update<P>(
        &self,
        itr: &PrimaryTableCursor<T>,
        payer: P,
        item: &T,
    ) -> Result<usize, WriteError>
    where
        P: Into<AccountName>,
    {
        let size = ::lib::size_of_val(item);
        let mut bytes = vec![0u8; size];
        let pos = item.write(&mut bytes, 0)?;
        let ptr: *const c_void = &bytes[..] as *const _ as *const c_void;
        let payer: AccountName = payer.into();
        unsafe { ::eosio_sys::db_update_i64(itr.value, payer.into(), ptr, pos as u32) }

        let pk = item.primary_key();

        for (i, k) in item.secondary_keys().iter().enumerate() {
            if let Some(k) = k {
                let table = ::table_secondary::SecondaryTableName::new(self.name, i);
                k.upsert(self.code, self.scope, table, payer, pk);
            }
        }

        Ok(pos)
    }
}
