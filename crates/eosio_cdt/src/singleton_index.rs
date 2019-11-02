use crate::{PrimaryTableIndexExt, TableCursor, TableIndex};
use eosio::{
    table::Table, AccountName, PrimaryTableIndex, ReadError, ScopeName,
    WriteError,
};

/// TODO docs
pub struct SingletonIndex<T: Table>(PrimaryTableIndex<T>);

impl<T: Table> SingletonIndex<T> {
    /// TODO docs
    #[inline]
    pub fn new<C, S>(code: C, scope: S) -> Self
    where
        C: Into<AccountName>,
        S: Into<ScopeName>,
    {
        Self(PrimaryTableIndex::new(code, scope))
    }

    /// Checks if the singleton entry exists
    #[inline]
    pub fn exists(&self) -> bool {
        self.0.find(T::NAME).is_some()
    }

    /// Gets the value stored inside the singleton. Returns `None` if no value is found,
    /// or `ReadError` if there was an issue reading the data.
    #[inline]
    pub fn get(&self) -> Option<Result<T::Row, ReadError>> {
        self.0.find(T::NAME).map(|c| c.get())
    }

    /// TODO docs
    #[inline]
    pub fn get_or_default(&self) -> Result<T::Row, ReadError>
    where
        T::Row: Default,
    {
        self.0
            .find(T::NAME)
            .map_or_else(|| Ok(T::Row::default()), |c| c.get())
    }

    /// Sets the singleton value
    #[inline]
    pub fn set(
        &self,
        value: &T::Row,
        payer: AccountName,
    ) -> Result<(), WriteError> {
        match self.0.find(T::NAME) {
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
    pub fn remove(&self) -> Result<Option<T::Row>, ReadError> {
        match self.0.find(T::NAME) {
            Some(cursor) => cursor.erase().map(Some),
            None => Ok(None),
        }
    }
}
