use crate::account::AccountName;
use crate::bytes::{ReadError, WriteError};
use crate::lib::PhantomData;
use crate::table::*;
use crate::table_primary::*;
use crate::time::Time;
use eosio_cdt_sys::ctypes::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash, PartialOrd, Ord)]
pub struct SecondaryTableName(TableName, usize);

impl SecondaryTableName {
    #[inline]
    pub fn new(primary: TableName, index: usize) -> Self {
        SecondaryTableName(primary, index)
    }
}

impl From<SecondaryTableName> for u64 {
    #[inline]
    fn from(t: SecondaryTableName) -> Self {
        let index = t.1 as Self;
        let table: Self = t.0.into();
        (table & 0xFFFF_FFFF_FFFF_FFF0_u64) | (index & 0x0000_0000_0000_000F_u64)
    }
}

pub trait SecondaryTableKey {
    fn end(&self, code: AccountName, scope: ScopeName, table: SecondaryTableName) -> i32;

    fn next(&self, iterator: i32) -> (i32, u64);

    fn erase(&self, iterator: i32);

    fn previous(&self, iterator: i32) -> (i32, u64);

    fn store(
        &self,
        scope: ScopeName,
        table: SecondaryTableName,
        payer: AccountName,
        id: u64,
    ) -> i32;

    fn modify(&self, iterator: i32, payer: AccountName);

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

    #[inline]
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
            self.modify(itr, payer);
        }
    }
}

macro_rules! secondary_keys_converted {
    ($($to:ty, $from:ty)*) => ($(
        impl SecondaryTableKey for $from {
            #[inline]
            fn end(&self, code: AccountName, scope: ScopeName, table: SecondaryTableName) -> i32 {
                <$to as From<$from>>::from(*self).end(code, scope, table)
            }
            #[inline]
            fn next(&self, iterator: i32) -> (i32, u64) {
                <$to as From<$from>>::from(*self).next(iterator)
            }
            #[inline]
            fn previous(&self, iterator: i32) -> (i32, u64) {
                <$to as From<$from>>::from(*self).previous(iterator)
            }
            #[inline]
            fn erase(&self, iterator: i32) {
                <$to as From<$from>>::from(*self).erase(iterator)
            }
            #[inline]
            fn store(
                &self,
                scope: ScopeName,
                table: SecondaryTableName,
                payer: AccountName,
                id: u64,
            ) -> i32 {
                <$to as From<$from>>::from(*self).store(scope, table, payer, id)
            }
            #[inline]
            fn modify(&self, iterator: i32, payer: AccountName) {
                <$to as From<$from>>::from(*self).modify(iterator, payer)
            }
            #[inline]
            fn lower_bound(
                &self,
                code: AccountName,
                scope: ScopeName,
                table: SecondaryTableName,
            ) -> (i32, u64) {
                <$to as From<$from>>::from(*self).lower_bound(code, scope, table)
            }
            #[inline]
            fn upper_bound(
                &self,
                code: AccountName,
                scope: ScopeName,
                table: SecondaryTableName,
            ) -> (i32, u64) {
                <$to as From<$from>>::from(*self).upper_bound(code, scope, table)
            }
            #[inline]
            fn find_primary(
                &self,
                code: AccountName,
                scope: ScopeName,
                table: SecondaryTableName,
                primary: u64,
            ) -> i32 {
                 <$to as From<$from>>::from(*self).find_primary(code, scope, table, primary)
            }
            #[inline]
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
    ($($t:ty, $i:ident)*) => {
        mashup! {
            $(
                m["end" $i] = db_ $i _end;
                m["next" $i] = db_ $i _next;
                m["previous" $i] = db_ $i _previous;
                m["remove" $i] = db_ $i _remove;
                m["store" $i] = db_ $i _store;
                m["update" $i] = db_ $i _update;
                m["lowerbound" $i] = db_ $i _lowerbound;
                m["upperbound" $i] = db_ $i _upperbound;
                m["find_primary" $i] = db_ $i _find_primary;
                m["find_secondary" $i] = db_ $i _find_secondary;
            )*
        }

        $(
            impl SecondaryTableKey for $t {
                #[inline]
                fn end(&self, code: AccountName, scope: ScopeName, table: SecondaryTableName) -> i32 {
                    use ::eosio_cdt_sys::*;
                    unsafe { m!["end" $i](code.into(), scope.into(), table.into()) }
                }
                #[inline]
                fn next(&self, iterator: i32) -> (i32, u64) {
                    use ::eosio_cdt_sys::*;
                    let mut pk = 0_u64;
                    let ptr: *mut u64 = &mut pk;
                    let itr = unsafe { m!["next" $i](iterator, ptr) };
                    (itr, pk)
                }
                #[inline]
                fn previous(&self, iterator: i32) -> (i32, u64) {
                    use ::eosio_cdt_sys::*;
                    let mut pk = 0_u64;
                    let ptr: *mut u64 = &mut pk;
                    let itr = unsafe { m!["previous" $i](iterator, ptr) };
                    (itr, pk)
                }
                #[inline]
                fn erase(&self, iterator: i32) {
                    use ::eosio_cdt_sys::*;
                    unsafe { m!["remove" $i](iterator) }
                }
                #[inline]
                fn store(
                    &self,
                    scope: ScopeName,
                    table: SecondaryTableName,
                    payer: AccountName,
                    id: u64,
                ) -> i32 {
                    use ::eosio_cdt_sys::*;
                    let secondary: *const Self = self;
                    unsafe {
                        m!["store" $i](scope.into(), table.into(), payer.into(), id, secondary)
                    }
                }
                #[inline]
                fn modify(&self, iterator: i32, payer: AccountName) {
                    use ::eosio_cdt_sys::*;
                    let secondary: *const Self = self;
                    unsafe {
                        m!["update" $i](iterator, payer.into(), secondary)
                    }
                }
                #[inline]
                fn lower_bound(
                    &self,
                    code: AccountName,
                    scope: ScopeName,
                    table: SecondaryTableName,
                ) -> (i32, u64) {
                    use ::eosio_cdt_sys::*;
                    let mut pk = 0_u64;
                    let mut sk = self.clone();
                    let itr = unsafe {
                        m!["lowerbound" $i](
                            code.into(),
                            scope.into(),
                            table.into(),
                            &mut sk as *mut $t,
                            &mut pk as *mut u64,
                        )
                    };
                    (itr, pk)
                }
                #[inline]
                fn upper_bound(
                    &self,
                    code: AccountName,
                    scope: ScopeName,
                    table: SecondaryTableName,
                ) -> (i32, u64) {
                    use ::eosio_cdt_sys::*;
                    let mut pk = 0_u64;
                    let mut sk = self.clone();
                    let itr = unsafe {
                        m!["upperbound" $i](
                            code.into(),
                            scope.into(),
                            table.into(),
                            &mut sk as *mut $t,
                            &mut pk as *mut u64,
                        )
                    };
                    (itr, pk)
                }
                #[inline]
                fn find_primary(
                    &self,
                    code: AccountName,
                    scope: ScopeName,
                    table: SecondaryTableName,
                    primary: u64,
                ) -> i32 {
                    use ::eosio_cdt_sys::*;
                    let mut sk = self.clone();
                    unsafe {
                        m!["find_primary" $i](
                            code.into(),
                            scope.into(),
                            table.into(),
                            &mut sk as *mut $t,
                            primary,
                        )
                    }
                }
                #[inline]
                fn find_secondary(
                    &self,
                    code: AccountName,
                    scope: ScopeName,
                    table: SecondaryTableName,
                ) -> (i32, u64) {
                    use ::eosio_cdt_sys::*;
                    let mut pk = 0_u64;
                    let secondary: *const Self = self;
                    let itr = unsafe {
                        m!["find_secondary" $i](
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
        )*
    }
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
    u64, Time
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
    #[inline]
    fn get(&self) -> Result<T, ReadError> {
        let pk_itr = unsafe {
            ::eosio_cdt_sys::db_find_i64(
                self.index.code.into(),
                self.index.scope.into(),
                self.index.table.0.into(),
                self.pk,
            )
        };
        let nullptr: *mut c_void = ::std::ptr::null_mut() as *mut _ as *mut c_void;
        let size = unsafe { ::eosio_cdt_sys::db_get_i64(self.value, nullptr, 0) };
        let mut bytes = vec![0_u8; size as usize];
        let ptr: *mut c_void = &mut bytes[..] as *mut _ as *mut c_void;
        unsafe {
            ::eosio_cdt_sys::db_get_i64(pk_itr, ptr, size as u32);
        }
        let mut pos = 0;
        T::read(&bytes, &mut pos)
    }

    #[inline]
    fn erase(&self) -> Result<T, ReadError> {
        let table = self.index.to_primary_index();
        match table.find(self.pk) {
            Some(cursor) => cursor.erase(),
            None => Err(ReadError::NotEnoughBytes), // TODO: better error
        }
    }

    #[inline]
    fn modify(&self, payer: Option<AccountName>, item: &T) -> Result<usize, WriteError> {
        let table = self.index.to_primary_index();
        match table.find(self.pk) {
            Some(cursor) => cursor.modify(payer, item),
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
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let secondary_end = self
            .index
            .key
            .end(self.index.code, self.index.scope, self.index.table);
        let primary_end = unsafe {
            ::eosio_cdt_sys::db_end_i64(
                self.index.code.into(),
                self.index.scope.into(),
                self.index.table.0.into(),
            )
        };
        SecondaryTableIterator {
            value: self.value,
            pk: self.pk,
            pk_end: primary_end,
            sk_end: secondary_end,
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
    #[inline]
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
    #[inline]
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
{
}

#[derive(Copy, Clone, Debug)]
pub struct SecondaryTableIndex<K, T>
where
    K: SecondaryTableKey,
    T: TableRow,
{
    code: AccountName,
    scope: ScopeName,
    table: SecondaryTableName,
    key: K,
    _data: PhantomData<T>,
}

impl<K, T> SecondaryTableIndex<K, T>
where
    K: SecondaryTableKey,
    T: TableRow,
{
    #[inline]
    pub fn new<C, S, N>(code: C, scope: S, name: N, key: K, index: usize) -> Self
    where
        C: Into<AccountName>,
        S: Into<ScopeName>,
        N: Into<TableName>,
    {
        Self {
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

    #[inline]
    fn lower_bound<N>(&'a self, key: N) -> Option<Self::Cursor>
    where
        N: Into<K>,
    {
        let secondary_key = key.into();
        let (value, primary_key) = secondary_key.lower_bound(self.code, self.scope, self.table);
        let end = secondary_key.end(self.code, self.scope, self.table);
        if value == end {
            None
        } else {
            Some(SecondaryTableCursor {
                value,
                pk: primary_key,
                index: self,
            })
        }
    }

    #[inline]
    fn upper_bound<N>(&'a self, key: N) -> Option<Self::Cursor>
    where
        N: Into<K>,
    {
        let k = key.into();
        let (value, pk) = k.upper_bound(self.code, self.scope, self.table);
        let end = k.end(self.code, self.scope, self.table);
        if value == end {
            None
        } else {
            Some(SecondaryTableCursor {
                value,
                pk,
                index: self,
            })
        }
    }

    #[inline]
    fn emplace(&self, payer: AccountName, item: &T) -> Result<(), WriteError> {
        let table = self.to_primary_index();
        table.emplace(payer, item)
    }
}
