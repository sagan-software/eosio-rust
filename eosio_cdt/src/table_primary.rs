use crate::{Print, TableCursor, TableIndex, TableIterator, TableRow};
use eosio_bytes::{ReadError, WriteError};
use eosio_cdt_sys::c_void;
use eosio_core::{AccountName, ScopeName, TableName};
use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
pub struct PrimaryTableCursor<T>
where
    T: TableRow,
{
    value: i32,
    code: AccountName,
    scope: ScopeName,
    table: TableName,
    data: PhantomData<T>,
}

impl<T> PartialEq for PrimaryTableCursor<T>
where
    T: TableRow,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
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
    #[inline]
    fn print(&self) {
        "PrimaryTableCursor(".print();
        self.value.print();
        ")".print();
    }
}

impl<T> TableCursor<T> for PrimaryTableCursor<T>
where
    T: TableRow,
{
    #[inline]
    fn get(&self) -> Result<T, ReadError> {
        let nullptr: *mut c_void =
            ::std::ptr::null_mut() as *mut _ as *mut c_void;
        let size =
            unsafe { ::eosio_cdt_sys::db_get_i64(self.value, nullptr, 0) };
        let mut bytes = vec![0_u8; size as usize];
        let ptr: *mut c_void = &mut bytes[..] as *mut _ as *mut c_void;
        unsafe {
            ::eosio_cdt_sys::db_get_i64(self.value, ptr, size as u32);
        }
        let mut pos = 0;
        T::read(&bytes, &mut pos)
    }

    #[inline]
    fn erase(&self) -> Result<T, ReadError> {
        let item = self.get()?;
        let pk = item.primary_key();
        unsafe {
            ::eosio_cdt_sys::db_remove_i64(self.value);
        }

        for (i, k) in item.secondary_keys().iter().enumerate() {
            if let Some(k) = k {
                let table = crate::table_secondary::SecondaryTableName::new(
                    self.table, i,
                );
                let end = k.end(self.code, self.scope, table);
                let itr = k.find_primary(self.code, self.scope, table, pk);
                if itr != end {
                    k.erase(itr);
                }
            }
        }
        Ok(item)
    }

    #[inline]
    fn modify(
        &self,
        payer: Option<AccountName>,
        item: &T,
    ) -> Result<usize, WriteError> {
        let table = PrimaryTableIndex::new(self.code, self.scope, self.table);
        table.modify(self, payer, item)
    }
}

impl<'a, T> IntoIterator for PrimaryTableCursor<T>
where
    T: TableRow,
{
    type Item = Self;
    type IntoIter = PrimaryTableIterator<T>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let end = unsafe {
            ::eosio_cdt_sys::db_end_i64(
                self.code.into(),
                self.scope.into(),
                self.table.into(),
            )
        };
        PrimaryTableIterator {
            value: self.value,
            end,
            code: self.code,
            scope: self.scope,
            table: self.table,
            data: PhantomData,
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
    scope: ScopeName,
    table: TableName,
    data: PhantomData<T>,
}

impl<T> Iterator for PrimaryTableIterator<T>
where
    T: TableRow,
{
    type Item = PrimaryTableCursor<T>;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.value == self.end {
            return None;
        }

        let cursor = PrimaryTableCursor {
            value: self.value,
            code: self.code,
            scope: self.scope,
            table: self.table,
            data: PhantomData,
        };

        let mut pk = 0_u64;
        let ptr: *mut u64 = &mut pk;
        self.value = unsafe { ::eosio_cdt_sys::db_next_i64(self.value, ptr) };

        Some(cursor)
    }
}

impl<T> DoubleEndedIterator for PrimaryTableIterator<T>
where
    T: TableRow,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.value == -1 {
            return None;
        }

        let cursor = PrimaryTableCursor {
            value: self.value,
            code: self.code,
            scope: self.scope,
            table: self.table,
            data: PhantomData,
        };

        let mut pk = 0_u64;
        let ptr: *mut u64 = &mut pk;
        self.value =
            unsafe { ::eosio_cdt_sys::db_previous_i64(self.value, ptr) };

        Some(cursor)
    }
}

impl<T> TableIterator for PrimaryTableIterator<T> where T: TableRow {}

#[derive(Copy, Clone, Debug)]
pub struct PrimaryTableIndex<T>
where
    T: TableRow,
{
    code: AccountName,
    scope: ScopeName,
    name: TableName,
    data: PhantomData<T>,
}

impl<'a, T> TableIndex<'a, u64, T> for PrimaryTableIndex<T>
where
    T: TableRow + 'a,
{
    type Cursor = PrimaryTableCursor<T>;

    #[inline]
    fn lower_bound<N>(&'a self, key: N) -> Option<Self::Cursor>
    where
        N: Into<u64>,
    {
        let itr = unsafe {
            ::eosio_cdt_sys::db_lowerbound_i64(
                self.code.into(),
                self.scope.into(),
                self.name.into(),
                key.into(),
            )
        };
        let end = self.end();
        if itr == end {
            None
        } else {
            Some(PrimaryTableCursor {
                value: itr,
                code: self.code,
                scope: self.scope,
                table: self.name,
                data: self.data,
            })
        }
    }

    #[inline]
    fn upper_bound<N>(&'a self, key: N) -> Option<Self::Cursor>
    where
        N: Into<u64>,
    {
        let itr = unsafe {
            ::eosio_cdt_sys::db_upperbound_i64(
                self.code.into(),
                self.scope.into(),
                self.name.into(),
                key.into(),
            )
        };
        let end = self.end();
        if itr == end {
            None
        } else {
            Some(PrimaryTableCursor {
                value: itr,
                code: self.code,
                scope: self.scope,
                table: self.name,
                data: self.data,
            })
        }
    }

    #[inline]
    fn emplace(&self, payer: AccountName, item: &T) -> Result<(), WriteError> {
        let id = item.primary_key();
        let size = item.num_bytes();
        let mut bytes = vec![0_u8; size];
        let mut pos = 0;
        item.write(&mut bytes, &mut pos)?;
        let ptr: *const c_void = &bytes[..] as *const _ as *const c_void;
        unsafe {
            ::eosio_cdt_sys::db_store_i64(
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
                let table = crate::table_secondary::SecondaryTableName::new(
                    self.name, i,
                );
                k.store(self.scope, table, payer, id);
            }
        }

        Ok(())
    }
}

impl<T> PrimaryTableIndex<T>
where
    T: TableRow,
{
    #[inline]
    pub fn new<C, S, N>(code: C, scope: S, name: N) -> Self
    where
        C: Into<AccountName>,
        S: Into<ScopeName>,
        N: Into<TableName>,
    {
        Self {
            code: code.into(),
            scope: scope.into(),
            name: name.into(),
            data: PhantomData,
        }
    }

    #[inline]
    pub fn begin(&self) -> Option<PrimaryTableCursor<T>> {
        self.lower_bound(u64::min_value())
    }

    #[inline]
    pub fn iter(&self) -> PrimaryTableIterator<T> {
        self.begin().map_or_else(
            || PrimaryTableIterator {
                value: 0,
                end: 0,
                code: self.code,
                scope: self.scope,
                table: self.name,
                data: PhantomData,
            },
            |c| c.into_iter(),
        )
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.iter().count()
    }

    #[inline]
    pub fn exists<Id>(&self, id: Id) -> bool
    where
        Id: Into<u64>,
    {
        self.find(id).is_some()
    }

    fn end(&self) -> i32 {
        unsafe {
            ::eosio_cdt_sys::db_end_i64(
                self.code.into(),
                self.scope.into(),
                self.name.into(),
            )
        }
    }

    #[inline]
    pub fn find<Id>(&self, id: Id) -> Option<PrimaryTableCursor<T>>
    where
        Id: Into<u64>,
    {
        let itr = unsafe {
            ::eosio_cdt_sys::db_find_i64(
                self.code.into(),
                self.scope.into(),
                self.name.into(),
                id.into(),
            )
        };
        let end = self.end();
        if itr == end {
            None
        } else {
            Some(PrimaryTableCursor {
                value: itr,
                code: self.code,
                scope: self.scope,
                table: self.name,
                data: self.data,
            })
        }
    }

    #[inline]
    pub fn available_primary_key(&self) -> Option<u64> {
        if self.begin().is_none() {
            return Some(0);
        }

        let end = self.end();
        let mut pk = 0_u64;
        let ptr: *mut u64 = &mut pk;
        unsafe { ::eosio_cdt_sys::db_previous_i64(end, ptr) };
        pk.checked_add(1)
    }

    fn modify(
        &self,
        itr: &PrimaryTableCursor<T>,
        payer: Option<AccountName>,
        item: &T,
    ) -> Result<usize, WriteError> {
        let size = item.num_bytes();
        let mut bytes = vec![0_u8; size];
        let mut pos = 0;
        item.write(&mut bytes, &mut pos)?;
        let bytes_ptr: *const c_void = &bytes[..] as *const _ as *const c_void;
        let payer = payer.unwrap_or_else(|| 0_u64.into());
        unsafe {
            ::eosio_cdt_sys::db_update_i64(
                itr.value,
                payer.into(),
                bytes_ptr,
                pos as u32,
            )
        }

        let pk = item.primary_key();

        for (i, k) in item.secondary_keys().iter().enumerate() {
            if let Some(k) = k {
                let table = crate::table_secondary::SecondaryTableName::new(
                    self.name, i,
                );
                k.upsert(self.code, self.scope, table, payer, pk);
            }
        }

        Ok(pos)
    }
}
