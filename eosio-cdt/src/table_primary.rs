//! TODO module docs.

use crate::{
    NativeSecondaryKey, Print, TableCursor, TableIndex, TableIterator,
};
use eosio_cdt_sys::*;
use eosio_core::{
    AccountName, NumBytes, PrimaryTableIndex, Read, ReadError, ScopeName,
    SecondaryKey, SecondaryTableName, Table, Write, WriteError,
};
use std::marker::PhantomData;

/// TODO docs
#[allow(clippy::missing_inline_in_public_items)]
#[derive(Copy, Clone, Debug)]
pub struct PrimaryTableCursor<T>
where
    T: Table,
{
    /// TODO docs
    value: i32,
    /// TODO docs
    code: AccountName,
    /// TODO docs
    scope: ScopeName,
    /// TODO docs
    data: PhantomData<T>,
}

impl<T> PartialEq for PrimaryTableCursor<T>
where
    T: Table,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
            && self.code == other.code
            && self.scope == other.scope
    }
}

impl<T> Print for PrimaryTableCursor<T>
where
    T: Table,
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
    T: Table,
{
    #[inline]
    fn get(&self) -> Result<T::Row, ReadError> {
        let nullptr: *mut c_void =
            ::std::ptr::null_mut() as *mut _ as *mut c_void;
        let size = unsafe { db_get_i64(self.value, nullptr, 0) };
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let mut bytes = vec![0_u8; size as usize];
        let ptr: *mut c_void = &mut bytes[..] as *mut _ as *mut c_void;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        unsafe {
            db_get_i64(self.value, ptr, size as u32);
        }
        let mut pos = 0;
        T::Row::read(&bytes, &mut pos)
    }

    #[inline]
    fn erase(&self) -> Result<T::Row, ReadError> {
        let item = self.get()?;
        let pk = T::primary_key(&item);
        unsafe {
            db_remove_i64(self.value);
        }

        for (i, k) in T::secondary_keys(&item).iter().enumerate() {
            if let Some(k) = k {
                let table = SecondaryTableName::new(T::NAME.into(), i);
                match k {
                    SecondaryKey::U64(v) => {
                        let end = u64::db_idx_end(self.code, self.scope, table);
                        let itr = v.clone().db_idx_find_primary(
                            self.code, self.scope, table, pk,
                        );
                        if itr != end {
                            u64::db_idx_remove(itr);
                        }
                    }
                    SecondaryKey::F64(v) => {
                        let end = f64::db_idx_end(self.code, self.scope, table);
                        let itr = v.clone().db_idx_find_primary(
                            self.code, self.scope, table, pk,
                        );
                        if itr != end {
                            f64::db_idx_remove(itr);
                        }
                    }
                }
            }
        }
        Ok(item)
    }

    #[inline]
    fn modify(
        &self,
        payer: Option<AccountName>,
        item: &T::Row,
    ) -> Result<usize, WriteError> {
        let size = item.num_bytes();
        let mut bytes = vec![0_u8; size];
        let mut pos = 0;
        item.write(&mut bytes, &mut pos)?;
        let bytes_ptr: *const c_void = &bytes[..] as *const _ as *const c_void;
        let payer = payer.unwrap_or_else(|| 0_u64.into());
        #[allow(clippy::cast_possible_truncation)]
        unsafe {
            db_update_i64(self.value, payer.into(), bytes_ptr, pos as u32)
        }

        let pk = T::primary_key(item);

        for (i, k) in T::secondary_keys(item).iter_mut().enumerate() {
            if let Some(k) = k {
                let table = SecondaryTableName::new(T::NAME.into(), i);
                match k {
                    SecondaryKey::U64(v) => {
                        v.db_idx_upsert(self.code, self.scope, table, payer, pk)
                    }
                    SecondaryKey::F64(v) => {
                        v.db_idx_upsert(self.code, self.scope, table, payer, pk)
                    }
                };
            }
        }

        Ok(pos)
    }
}

impl<'a, T> IntoIterator for PrimaryTableCursor<T>
where
    T: Table,
{
    type Item = Self;
    type IntoIter = PrimaryTableIterator<T>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let end =
            unsafe { db_end_i64(self.code.into(), self.scope.into(), T::NAME) };
        PrimaryTableIterator {
            value: self.value,
            end,
            code: self.code,
            scope: self.scope,
            data: PhantomData,
        }
    }
}

/// TODO docs
#[allow(clippy::missing_inline_in_public_items)]
#[derive(Copy, Clone, Debug)]
pub struct PrimaryTableIterator<T>
where
    T: Table,
{
    /// TODO docs
    value: i32,
    /// TODO docs
    end: i32,
    /// TODO docs
    code: AccountName,
    /// TODO docs
    scope: ScopeName,
    /// TODO docs
    data: PhantomData<T>,
}

impl<T> Iterator for PrimaryTableIterator<T>
where
    T: Table,
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
            data: PhantomData,
        };

        let mut pk = 0_u64;
        let ptr: *mut u64 = &mut pk;
        self.value = unsafe { db_next_i64(self.value, ptr) };

        Some(cursor)
    }
}

impl<T> DoubleEndedIterator for PrimaryTableIterator<T>
where
    T: Table,
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
            data: PhantomData,
        };

        let mut pk = 0_u64;
        let ptr: *mut u64 = &mut pk;
        self.value = unsafe { db_previous_i64(self.value, ptr) };

        Some(cursor)
    }
}

impl<T> TableIterator for PrimaryTableIterator<T> where T: Table {}

impl<'a, T> TableIndex<'a, u64, T> for PrimaryTableIndex<T>
where
    T: Table + 'a,
{
    type Cursor = PrimaryTableCursor<T>;

    #[inline]
    fn code(&self) -> AccountName {
        self.code
    }

    #[inline]
    fn scope(&self) -> ScopeName {
        self.scope
    }

    #[inline]
    fn lower_bound<N: Into<u64>>(&'a self, key: N) -> Option<Self::Cursor> {
        let itr = unsafe {
            db_lowerbound_i64(
                self.code.into(),
                self.scope.into(),
                T::NAME,
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
                data: PhantomData,
            })
        }
    }

    #[inline]
    fn upper_bound<N: Into<u64>>(&'a self, key: N) -> Option<Self::Cursor> {
        let itr = unsafe {
            db_upperbound_i64(
                self.code.into(),
                self.scope.into(),
                T::NAME,
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
                data: PhantomData,
            })
        }
    }

    #[inline]
    fn emplace(
        &self,
        payer: AccountName,
        item: &T::Row,
    ) -> Result<(), WriteError> {
        let id = T::primary_key(item);
        let size = item.num_bytes();
        let mut bytes = vec![0_u8; size];
        let mut pos = 0;
        item.write(&mut bytes, &mut pos)?;
        let ptr: *const c_void = &bytes[..] as *const _ as *const c_void;
        #[allow(clippy::cast_possible_truncation)]
        unsafe {
            db_store_i64(
                self.scope.into(),
                T::NAME,
                payer.into(),
                id,
                ptr,
                pos as u32,
            )
        };

        // store secondary indexes
        for (i, k) in T::secondary_keys(item).iter().enumerate() {
            if let Some(k) = k {
                let table = SecondaryTableName::new(T::NAME.into(), i);
                match k {
                    SecondaryKey::U64(v) => {
                        v.db_idx_store(self.scope, table, payer, id)
                    }
                    SecondaryKey::F64(v) => {
                        v.db_idx_store(self.scope, table, payer, id)
                    }
                };
            }
        }

        Ok(())
    }
}

/// TODO docs
pub trait PrimaryTableIndexExt<'a, T>:
    TableIndex<'a, u64, T, Cursor = PrimaryTableCursor<T>>
where
    T: Table + 'a,
{
    /// TODO docs
    #[inline]
    fn begin(&'a self) -> Option<Self::Cursor> {
        self.lower_bound(u64::min_value())
    }

    /// TODO docs
    #[inline]
    fn iter(&'a self) -> PrimaryTableIterator<T> {
        self.begin().map_or_else(
            || PrimaryTableIterator {
                value: 0,
                end: 0,
                code: self.code(),
                scope: self.scope(),
                data: PhantomData,
            },
            std::iter::IntoIterator::into_iter,
        )
    }

    /// TODO docs
    #[inline]
    fn count(&'a self) -> usize {
        self.iter().count()
    }

    /// TODO docs
    #[inline]
    fn exists<Id>(&'a self, id: Id) -> bool
    where
        Id: Into<u64>,
    {
        self.find(id).is_some()
    }

    /// TODO docs
    #[inline]
    fn end(&'a self) -> i32 {
        unsafe { db_end_i64(self.code().into(), self.scope().into(), T::NAME) }
    }

    /// TODO docs
    #[inline]
    fn find<Id>(&'a self, id: Id) -> Option<PrimaryTableCursor<T>>
    where
        Id: Into<u64>,
    {
        let code = self.code();
        let scope = self.scope();
        let itr = unsafe {
            db_find_i64(code.into(), scope.into(), T::NAME, id.into())
        };
        let end = self.end();
        if itr == end {
            None
        } else {
            Some(PrimaryTableCursor {
                value: itr,
                code,
                scope,
                data: PhantomData,
            })
        }
    }

    /// TODO docs
    #[inline]
    fn available_primary_key(&'a self) -> Option<u64> {
        if self.begin().is_none() {
            return Some(0);
        }

        let end = self.end();
        let mut pk = 0_u64;
        let ptr: *mut u64 = &mut pk;
        unsafe { db_previous_i64(end, ptr) };
        pk.checked_add(1)
    }
}

impl<'a, T> PrimaryTableIndexExt<'a, T> for PrimaryTableIndex<T> where
    T: Table + 'a
{
}
