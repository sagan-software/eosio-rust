use crate::{Payer, TableCursor, TableIndex, TableIterator};
use alloc::vec::Vec;
use core::{borrow::Borrow, ptr::null_mut};
use eosio::{
    AccountName, Checksum160, Checksum256, ReadError, ScopeName,
    SecondaryTableIndex, SecondaryTableName, Table, WriteError,
};
use eosio_cdt_sys::{
    c_void, db_end_i64, db_find_i64, db_get_i64, db_idx128_end,
    db_idx128_find_primary, db_idx128_find_secondary, db_idx128_lowerbound,
    db_idx128_next, db_idx128_previous, db_idx128_remove, db_idx128_store,
    db_idx128_update, db_idx128_upperbound, db_idx256_end,
    db_idx256_find_primary, db_idx256_find_secondary, db_idx256_lowerbound,
    db_idx256_next, db_idx256_previous, db_idx256_remove, db_idx256_store,
    db_idx256_update, db_idx256_upperbound, db_idx64_end,
    db_idx64_find_primary, db_idx64_find_secondary, db_idx64_lowerbound,
    db_idx64_next, db_idx64_previous, db_idx64_remove, db_idx64_store,
    db_idx64_update, db_idx64_upperbound, db_idx_double_end,
    db_idx_double_find_primary, db_idx_double_find_secondary,
    db_idx_double_lowerbound, db_idx_double_next, db_idx_double_previous,
    db_idx_double_remove, db_idx_double_store, db_idx_double_update,
    db_idx_double_upperbound,
};

pub enum Either<A, B> {
    A(A),
    B(B),
}

pub type EndFn = unsafe extern "C" fn(code: u64, scope: u64, table: u64) -> i32;
pub type NextFn = unsafe extern "C" fn(itr: i32, primary: *mut u64) -> i32;
pub type PreviousFn = unsafe extern "C" fn(itr: i32, primary: *mut u64) -> i32;
pub type RemoveFn = unsafe extern "C" fn(itr: i32);

pub type StoreValueFn<T> = unsafe extern "C" fn(
    scope: u64,
    table: u64,
    payer: u64,
    id: u64,
    secondary: *const T,
) -> i32;

pub type StoreArrayFn<T> = unsafe extern "C" fn(
    scope: u64,
    table: u64,
    payer: u64,
    id: u64,
    data: *const T,
    data_len: u32,
) -> i32;

pub type StoreFn<T> = Either<StoreValueFn<T>, StoreArrayFn<T>>;

pub type UpdateValueFn<T> =
    unsafe extern "C" fn(itr: i32, payer: u64, secondary: *const T);

pub type UpdateArrayFn<T> =
    unsafe extern "C" fn(itr: i32, payer: u64, data: *const T, data_len: u32);

pub type UpdateFn<T> = Either<UpdateValueFn<T>, UpdateArrayFn<T>>;

pub type LowerboundValueFn<T> = unsafe extern "C" fn(
    code: u64,
    scope: u64,
    table: u64,
    secondary: *mut T,
    primary: *mut u64,
) -> i32;

pub type LowerboundArrayFn<T> = unsafe extern "C" fn(
    code: u64,
    scope: u64,
    table: u64,
    data: *mut T,
    data_len: u32,
    primary: *mut u64,
) -> i32;

pub type LowerboundFn<T> = Either<LowerboundValueFn<T>, LowerboundArrayFn<T>>;

pub type UpperboundValueFn<T> = unsafe extern "C" fn(
    code: u64,
    scope: u64,
    table: u64,
    secondary: *mut T,
    primary: *mut u64,
) -> i32;

pub type UpperboundArrayFn<T> = unsafe extern "C" fn(
    code: u64,
    scope: u64,
    table: u64,
    data: *mut T,
    data_len: u32,
    primary: *mut u64,
) -> i32;

pub type UpperboundFn<T> = Either<UpperboundValueFn<T>, UpperboundArrayFn<T>>;

pub type FindPrimaryValueFn<T> = unsafe extern "C" fn(
    code: u64,
    scope: u64,
    table: u64,
    secondary: *mut T,
    primary: u64,
) -> i32;

pub type FindPrimaryArrayFn<T> = unsafe extern "C" fn(
    code: u64,
    scope: u64,
    table: u64,
    data: *mut T,
    data_len: u32,
    primary: u64,
) -> i32;

pub type FindPrimaryFn<T> =
    Either<FindPrimaryValueFn<T>, FindPrimaryArrayFn<T>>;

pub type FindSecondaryValueFn<T> = unsafe extern "C" fn(
    code: u64,
    scope: u64,
    table: u64,
    secondary: *const T,
    primary: *mut u64,
) -> i32;

pub type FindSecondaryArrayFn<T> = unsafe extern "C" fn(
    code: u64,
    scope: u64,
    table: u64,
    data: *const T,
    data_len: u32,
    primary: *mut u64,
) -> i32;

pub type FindSecondaryFn<T> =
    Either<FindSecondaryValueFn<T>, FindSecondaryArrayFn<T>>;

/// Trait for types that are natively supported by EOSIO to be used as secondary
/// keys
pub trait NativeSecondaryKey: Default {
    type NativeType;
    /// Unsafe native `end` function
    const END: EndFn;
    /// Unsafe native `next` function
    const NEXT: NextFn;
    /// Unsafe native `previous` function
    const PREVIOUS: PreviousFn;
    /// Unsafe native `remove` function
    const REMOVE: RemoveFn;
    /// Unsafe native `store` function
    const STORE: StoreFn<Self::NativeType>;
    /// Unsafe native `update` function
    const UPDATE: UpdateFn<Self::NativeType>;
    /// Unsafe native `lowerbound` function
    const LOWERBOUND: LowerboundFn<Self::NativeType>;
    /// Unsafe native `upperbound` function
    const UPPERBOUND: UpperboundFn<Self::NativeType>;
    /// Unsafe native `find_primary` function
    const FIND_PRIMARY: FindPrimaryFn<Self::NativeType>;
    /// Unsafe native `find_secondary` function
    const FIND_SECONDARY: FindSecondaryFn<Self::NativeType>;
    fn as_ptr(&self) -> *const Self::NativeType;
    fn as_mut_ptr(&mut self) -> *mut Self::NativeType;
    const DATA_LEN: u32 = 0;
    /// Safe wrapper around unsafe native function
    #[must_use]
    #[inline]
    fn db_idx_end(
        code: AccountName,
        scope: ScopeName,
        table: SecondaryTableName,
    ) -> i32 {
        unsafe { Self::END(code.as_u64(), scope.as_u64(), table.as_u64()) }
    }
    /// Safe wrapper around unsafe native function
    #[must_use]
    #[inline]
    fn db_idx_next(iterator: i32) -> (i32, u64) {
        let mut pk = 0_u64;
        let ptr: *mut u64 = &mut pk;
        let itr = unsafe { Self::NEXT(iterator, ptr) };
        (itr, pk)
    }
    /// Safe wrapper around unsafe native function
    #[must_use]
    #[inline]
    fn db_idx_previous(iterator: i32) -> (i32, u64) {
        let mut pk = 0_u64;
        let ptr: *mut u64 = &mut pk;
        let itr = unsafe { Self::PREVIOUS(iterator, ptr) };
        (itr, pk)
    }
    /// Safe wrapper around unsafe native function
    #[inline]
    fn db_idx_remove(iterator: i32) {
        unsafe { Self::REMOVE(iterator) }
    }
    /// Safe wrapper around unsafe native function
    #[inline]
    fn db_idx_store(
        &self,
        scope: ScopeName,
        table: SecondaryTableName,
        payer: AccountName,
        id: u64,
    ) -> i32 {
        match Self::STORE {
            Either::A(func) => unsafe {
                func(
                    scope.as_u64(),
                    table.as_u64(),
                    payer.as_u64(),
                    id,
                    self.as_ptr(),
                )
            },
            Either::B(func) => unsafe {
                func(
                    scope.as_u64(),
                    table.as_u64(),
                    payer.as_u64(),
                    id,
                    self.as_ptr(),
                    Self::DATA_LEN,
                )
            },
        }
    }
    /// Safe wrapper around unsafe native function
    #[inline]
    fn db_idx_update(&self, iterator: i32, payer: AccountName) {
        match Self::UPDATE {
            Either::A(func) => unsafe {
                func(iterator, payer.as_u64(), self.as_ptr())
            },
            Either::B(func) => unsafe {
                func(iterator, payer.as_u64(), self.as_ptr(), Self::DATA_LEN)
            },
        }
    }
    /// Safe wrapper around unsafe native function
    #[inline]
    fn db_idx_lowerbound(
        &mut self,
        code: AccountName,
        scope: ScopeName,
        table: SecondaryTableName,
    ) -> (i32, u64) {
        let mut pk = 0_u64;
        let itr = match Self::LOWERBOUND {
            Either::A(func) => unsafe {
                func(
                    code.as_u64(),
                    scope.as_u64(),
                    table.as_u64(),
                    self.as_mut_ptr(),
                    &mut pk as *mut u64,
                )
            },
            Either::B(func) => unsafe {
                func(
                    code.as_u64(),
                    scope.as_u64(),
                    table.as_u64(),
                    self.as_mut_ptr(),
                    Self::DATA_LEN,
                    &mut pk as *mut u64,
                )
            },
        };
        (itr, pk)
    }
    /// Safe wrapper around unsafe native function
    #[inline]
    fn db_idx_upperbound(
        &mut self,
        code: AccountName,
        scope: ScopeName,
        table: SecondaryTableName,
    ) -> (i32, u64) {
        let mut pk = 0_u64;
        let itr = match Self::UPPERBOUND {
            Either::A(func) => unsafe {
                func(
                    code.as_u64(),
                    scope.as_u64(),
                    table.as_u64(),
                    self.as_mut_ptr(),
                    &mut pk as *mut u64,
                )
            },
            Either::B(func) => unsafe {
                func(
                    code.as_u64(),
                    scope.as_u64(),
                    table.as_u64(),
                    self.as_mut_ptr(),
                    Self::DATA_LEN,
                    &mut pk as *mut u64,
                )
            },
        };
        (itr, pk)
    }
    /// Safe wrapper around unsafe native function
    #[inline]
    fn db_idx_find_primary(
        &mut self,
        code: AccountName,
        scope: ScopeName,
        table: SecondaryTableName,
        primary: u64,
    ) -> i32 {
        match Self::FIND_PRIMARY {
            Either::A(func) => unsafe {
                func(
                    code.as_u64(),
                    scope.as_u64(),
                    table.as_u64(),
                    self.as_mut_ptr(),
                    primary,
                )
            },
            Either::B(func) => unsafe {
                func(
                    code.as_u64(),
                    scope.as_u64(),
                    table.as_u64(),
                    self.as_mut_ptr(),
                    Self::DATA_LEN,
                    primary,
                )
            },
        }
    }
    /// Safe wrapper around unsafe native function
    #[inline]
    fn db_idx_find_secondary(
        &self,
        code: AccountName,
        scope: ScopeName,
        table: SecondaryTableName,
    ) -> (i32, u64) {
        let mut pk = 0_u64;
        let itr = match Self::FIND_SECONDARY {
            Either::A(func) => unsafe {
                func(
                    code.as_u64(),
                    scope.as_u64(),
                    table.as_u64(),
                    self.as_ptr(),
                    &mut pk as *mut u64,
                )
            },
            Either::B(func) => unsafe {
                func(
                    code.as_u64(),
                    scope.as_u64(),
                    table.as_u64(),
                    self.as_ptr(),
                    Self::DATA_LEN,
                    &mut pk as *mut u64,
                )
            },
        };
        (itr, pk)
    }
    /// Safe wrapper around unsafe native function
    #[inline]
    fn db_idx_upsert(
        &mut self,
        code: AccountName,
        scope: ScopeName,
        table: SecondaryTableName,
        payer: AccountName,
        id: u64,
    ) {
        let end = Self::db_idx_end(code, scope, table);
        let itr = self.db_idx_find_primary(code, scope, table, id);
        if itr == end {
            self.db_idx_store(scope, table, payer, id);
        } else {
            self.db_idx_update(itr, payer);
        }
    }
}

impl NativeSecondaryKey for u64 {
    type NativeType = Self;

    const END: EndFn = db_idx64_end;
    const FIND_PRIMARY: FindPrimaryFn<Self::NativeType> =
        Either::A(db_idx64_find_primary);
    const FIND_SECONDARY: FindSecondaryFn<Self::NativeType> =
        Either::A(db_idx64_find_secondary);
    const LOWERBOUND: LowerboundFn<Self::NativeType> =
        Either::A(db_idx64_lowerbound);
    const NEXT: NextFn = db_idx64_next;
    const PREVIOUS: PreviousFn = db_idx64_previous;
    const REMOVE: RemoveFn = db_idx64_remove;
    const STORE: StoreFn<Self::NativeType> = Either::A(db_idx64_store);
    const UPDATE: UpdateFn<Self::NativeType> = Either::A(db_idx64_update);
    const UPPERBOUND: UpperboundFn<Self::NativeType> =
        Either::A(db_idx64_upperbound);

    #[inline]
    fn as_ptr(&self) -> *const Self::NativeType {
        self as *const Self::NativeType
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Self::NativeType {
        self as *mut Self::NativeType
    }
}

impl NativeSecondaryKey for f64 {
    type NativeType = Self;

    const END: EndFn = db_idx_double_end;
    const FIND_PRIMARY: FindPrimaryFn<Self::NativeType> =
        Either::A(db_idx_double_find_primary);
    const FIND_SECONDARY: FindSecondaryFn<Self::NativeType> =
        Either::A(db_idx_double_find_secondary);
    const LOWERBOUND: LowerboundFn<Self::NativeType> =
        Either::A(db_idx_double_lowerbound);
    const NEXT: NextFn = db_idx_double_next;
    const PREVIOUS: PreviousFn = db_idx_double_previous;
    const REMOVE: RemoveFn = db_idx_double_remove;
    const STORE: StoreFn<Self::NativeType> = Either::A(db_idx_double_store);
    const UPDATE: UpdateFn<Self::NativeType> = Either::A(db_idx_double_update);
    const UPPERBOUND: UpperboundFn<Self::NativeType> =
        Either::A(db_idx_double_upperbound);

    #[inline]
    fn as_ptr(&self) -> *const Self::NativeType {
        self as *const Self::NativeType
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Self::NativeType {
        self as *mut Self::NativeType
    }
}

impl NativeSecondaryKey for u128 {
    type NativeType = Self;

    const END: EndFn = db_idx128_end;
    const FIND_PRIMARY: FindPrimaryFn<Self::NativeType> =
        Either::A(db_idx128_find_primary);
    const FIND_SECONDARY: FindSecondaryFn<Self::NativeType> =
        Either::A(db_idx128_find_secondary);
    const LOWERBOUND: LowerboundFn<Self::NativeType> =
        Either::A(db_idx128_lowerbound);
    const NEXT: NextFn = db_idx128_next;
    const PREVIOUS: PreviousFn = db_idx128_previous;
    const REMOVE: RemoveFn = db_idx128_remove;
    const STORE: StoreFn<Self::NativeType> = Either::A(db_idx128_store);
    const UPDATE: UpdateFn<Self::NativeType> = Either::A(db_idx128_update);
    const UPPERBOUND: UpperboundFn<Self::NativeType> =
        Either::A(db_idx128_upperbound);

    #[inline]
    fn as_ptr(&self) -> *const Self::NativeType {
        self as *const Self::NativeType
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Self::NativeType {
        self as *mut Self::NativeType
    }
}

impl NativeSecondaryKey for [u128; 2] {
    type NativeType = u128;

    const DATA_LEN: u32 = 2;
    const END: EndFn = db_idx256_end;
    const FIND_PRIMARY: FindPrimaryFn<Self::NativeType> =
        Either::B(db_idx256_find_primary);
    const FIND_SECONDARY: FindSecondaryFn<Self::NativeType> =
        Either::B(db_idx256_find_secondary);
    const LOWERBOUND: LowerboundFn<Self::NativeType> =
        Either::B(db_idx256_lowerbound);
    const NEXT: NextFn = db_idx256_next;
    const PREVIOUS: PreviousFn = db_idx256_previous;
    const REMOVE: RemoveFn = db_idx256_remove;
    const STORE: StoreFn<Self::NativeType> = Either::B(db_idx256_store);
    const UPDATE: UpdateFn<Self::NativeType> = Either::B(db_idx256_update);
    const UPPERBOUND: UpperboundFn<Self::NativeType> =
        Either::B(db_idx256_upperbound);

    #[inline]
    fn as_ptr(&self) -> *const Self::NativeType {
        (&self[..]).as_ptr()
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Self::NativeType {
        (&mut self[..]).as_mut_ptr()
    }
}

/// Trait for types that can be turned into types that are native secondary keys
pub trait IntoNativeSecondaryKey {
    /// The native secondary key type
    type Native: NativeSecondaryKey;
    /// Turn self into the native secondary key type
    fn into_native_secondary_key(self) -> Self::Native;
}

impl IntoNativeSecondaryKey for u64 {
    type Native = Self;

    #[must_use]
    #[inline]
    fn into_native_secondary_key(self) -> Self::Native {
        self
    }
}

impl IntoNativeSecondaryKey for f64 {
    type Native = Self;

    #[must_use]
    #[inline]
    fn into_native_secondary_key(self) -> Self::Native {
        self
    }
}

impl IntoNativeSecondaryKey for u128 {
    type Native = Self;

    #[must_use]
    #[inline]
    fn into_native_secondary_key(self) -> Self::Native {
        self
    }
}

impl IntoNativeSecondaryKey for Checksum256 {
    type Native = [u128; 2];

    #[must_use]
    #[inline]
    fn into_native_secondary_key(self) -> Self::Native {
        self.words()
    }
}

impl IntoNativeSecondaryKey for Checksum160 {
    type Native = [u128; 2];

    #[must_use]
    #[inline]
    fn into_native_secondary_key(self) -> Self::Native {
        self.words()
    }
}

macro_rules! impl_into_type {
    ($($t:ty, $x:ty)*) => ($(
        impl IntoNativeSecondaryKey for $x {
            type Native = $t;
            #[must_use]
            #[inline]
            fn into_native_secondary_key(self) -> Self::Native {
                self.into()
            }
        }
    )*)
}

impl_into_type! {
    u64, u8
    u64, u16
    u64, u32
    u64, eosio::Name
    u64, eosio::AccountName
    u64, eosio::TableName
    u64, eosio::PermissionName
    u64, eosio::ScopeName
    u64, eosio::ActionName
    u64, eosio::Symbol
    u64, eosio::SymbolCode
}

/// Cursor for a `SecondaryTableIndex`
#[allow(clippy::missing_inline_in_public_items)]
#[derive(Debug, Copy, Clone)]
pub struct SecondaryTableCursor<'a, K, T>
where
    T: Table,
{
    value: i32,
    pk: u64,
    index: &'a SecondaryTableIndex<K, T>,
}

impl<'a, K, T> TableCursor<T> for SecondaryTableCursor<'a, K, T>
where
    K: IntoNativeSecondaryKey,
    T: Table,
{
    #[inline]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn bytes(&self) -> Vec<u8> {
        let pk_itr = unsafe {
            db_find_i64(
                self.index.code.as_u64(),
                self.index.scope.as_u64(),
                T::NAME.as_u64(),
                self.pk,
            )
        };
        let nullptr: *mut c_void = null_mut() as *mut _ as *mut c_void;
        let size = unsafe { db_get_i64(self.value, nullptr, 0) };
        let mut bytes = vec![0_u8; size as usize];
        let ptr: *mut c_void = &mut bytes[..] as *mut _ as *mut c_void;
        unsafe {
            db_get_i64(pk_itr, ptr, size as u32);
        }
        bytes
    }

    #[inline]
    fn erase(&self) -> Result<T::Row, ReadError> {
        let table = self.index.primary_index();
        match table.find(self.pk) {
            Some(cursor) => cursor.erase(),
            None => Err(ReadError::NotEnoughBytes), // TODO: better error
        }
    }

    #[inline]
    fn modify<I: Borrow<T::Row>>(
        &self,
        payer: Payer,
        item: I,
    ) -> Result<usize, WriteError> {
        let table = self.index.primary_index();
        match table.find(self.pk) {
            Some(cursor) => cursor.modify(payer, item),
            None => Err(WriteError::NotEnoughSpace), // TODO: better error
        }
    }
}

impl<'a, K, T> IntoIterator for SecondaryTableCursor<'a, K, T>
where
    K: IntoNativeSecondaryKey,
    T: Table,
{
    type IntoIter = SecondaryTableIterator<'a, K, T>;
    type Item = Self;

    #[must_use]
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let secondary_end = K::Native::db_idx_end(
            self.index.code,
            self.index.scope,
            self.index.table,
        );
        let primary_end = unsafe {
            db_end_i64(
                self.index.code.as_u64(),
                self.index.scope.as_u64(),
                T::NAME.as_u64(),
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

/// Iterate over a secondary table index
#[allow(clippy::missing_inline_in_public_items)]
#[derive(Copy, Clone, Debug)]
pub struct SecondaryTableIterator<'a, K, T>
where
    T: Table,
{
    value: i32,
    pk: u64,
    pk_end: i32,
    sk_end: i32,
    index: &'a SecondaryTableIndex<K, T>,
}

impl<'a, K, T> Iterator for SecondaryTableIterator<'a, K, T>
where
    K: IntoNativeSecondaryKey,
    T: Table,
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
        let (itr, pk) = K::Native::db_idx_next(self.value);
        self.value = itr;
        self.pk = pk;

        Some(cursor)
    }
}

impl<'a, K, T> DoubleEndedIterator for SecondaryTableIterator<'a, K, T>
where
    K: IntoNativeSecondaryKey,
    T: Table,
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
        let (itr, pk) = K::Native::db_idx_previous(self.value);
        self.value = itr;
        self.pk = pk;

        Some(cursor)
    }
}

impl<'a, K, T> TableIterator for SecondaryTableIterator<'a, K, T>
where
    K: IntoNativeSecondaryKey,
    T: Table,
{
}

impl<'a, K, T> TableIndex<'a, K, T> for SecondaryTableIndex<K, T>
where
    K: IntoNativeSecondaryKey + 'a,
    T: Table + 'a,
{
    type Cursor = SecondaryTableCursor<'a, K, T>;

    #[must_use]
    #[inline]
    fn code(&'a self) -> AccountName {
        self.code
    }

    #[must_use]
    #[inline]
    fn scope(&'a self) -> ScopeName {
        self.scope
    }

    #[inline]
    fn lower_bound<N: Into<K>>(&'a self, key: N) -> Option<Self::Cursor> {
        let (value, pk) = key
            .into()
            .into_native_secondary_key()
            .db_idx_lowerbound(self.code, self.scope, self.table);
        let end = K::Native::db_idx_end(self.code, self.scope, self.table);
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
    fn upper_bound<N: Into<K>>(&'a self, key: N) -> Option<Self::Cursor> {
        let (value, pk) = key
            .into()
            .into_native_secondary_key()
            .db_idx_upperbound(self.code, self.scope, self.table);
        let end = K::Native::db_idx_end(self.code, self.scope, self.table);
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
    fn find<N: Into<K>>(&'a self, key: N) -> Option<Self::Cursor> {
        let (value, pk) = key
            .into()
            .into_native_secondary_key()
            .db_idx_find_secondary(self.code, self.scope, self.table);
        let end = K::Native::db_idx_end(self.code, self.scope, self.table);
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
    fn emplace<I: Borrow<T::Row>>(
        &self,
        payer: AccountName,
        item: I,
    ) -> Result<(), WriteError> {
        let table = self.primary_index();
        table.emplace(payer, item)
    }
}
