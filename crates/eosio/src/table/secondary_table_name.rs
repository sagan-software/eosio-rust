use super::TableName;

/// TODO docs
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash, PartialOrd, Ord)]
pub struct SecondaryTableName(TableName, usize);

impl SecondaryTableName {
    /// TODO docs
    #[inline]
    pub const fn new(primary: TableName, index: usize) -> Self {
        Self(primary, index)
    }

    /// TODO docs
    #[inline]
    pub const fn primary(&self) -> TableName {
        self.0
    }

    /// TODO docs
    #[inline]
    pub const fn index(&self) -> usize {
        self.1
    }

    /// TODO docs
    pub const fn as_u64(&self) -> u64 {
        let index = self.1 as u64;
        let table = self.0.as_u64();
        (table & 0xFFFF_FFFF_FFFF_FFF0_u64)
            | (index & 0x0000_0000_0000_000F_u64)
    }
}

impl From<SecondaryTableName> for u64 {
    #[inline]
    fn from(t: SecondaryTableName) -> Self {
        t.as_u64()
    }
}
