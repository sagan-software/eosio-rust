//! TODO module docs.

use crate::{PrimaryTableIndex, TableCursor, TableIndex, TableRow};
use eosio_core::{AccountName, ReadError, ScopeName, WriteError};

/// TODO docs
pub struct SingletonIndex<T: TableRow>(PrimaryTableIndex<T>);

impl<T: TableRow> SingletonIndex<T> {
    /// TODO docs
    #[inline]
    pub fn new<C, S>(code: C, scope: S) -> Self
    where
        C: Into<AccountName>,
        S: Into<ScopeName>,
    {
        Self(PrimaryTableIndex::new(code, scope, T::TABLE_NAME))
    }

    /// Checks if the singleton entry exists
    #[inline]
    pub fn exists(&self) -> bool {
        self.0.find(T::TABLE_NAME).is_some()
    }

    /// Gets the value stored inside the singleton. Returns `None` if no value is found,
    /// or `ReadError` if there was an issue reading the data.
    #[inline]
    pub fn get(&self) -> Option<Result<T, ReadError>> {
        self.0.find(T::TABLE_NAME).map(|c| c.get())
    }

    /// TODO docs
    #[inline]
    pub fn get_or_default(&self) -> Result<T, ReadError>
    where
        T: Default,
    {
        self.0
            .find(T::TABLE_NAME)
            .map_or_else(|| Ok(T::default()), |c| c.get())
    }

    /// Sets the singleton value
    #[inline]
    pub fn set(&self, value: &T, payer: AccountName) -> Result<(), WriteError> {
        match self.0.find(T::TABLE_NAME) {
            Some(cursor) => {
                cursor.modify(Some(payer), value)?;
                Ok(())
            }
            None => self.0.emplace(payer, value),
        }
    }

    /// Removes the singleton value if it exists. Returns `ReadError` if there was
    /// an issue reading the data, and None if there was no entry found
    #[inline]
    pub fn remove(&self) -> Result<Option<T>, ReadError> {
        match self.0.find(T::TABLE_NAME) {
            Some(cursor) => cursor.erase().map(Some),
            None => Ok(None),
        }
    }
}
