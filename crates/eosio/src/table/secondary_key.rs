use crate::{Checksum160, Checksum256};

/// TODO docs
#[derive(Clone, Copy, Debug)]
pub enum SecondaryKey {
    /// TODO docs
    U64(u64),
    /// TODO docs
    F64(f64),
    /// TODO docs
    U128(u128),
    /// TODO docs
    H256([u128; 2]),
}

impl From<u64> for SecondaryKey {
    #[must_use]
    fn from(v: u64) -> Self {
        Self::U64(v)
    }
}

impl From<f64> for SecondaryKey {
    #[must_use]
    fn from(v: f64) -> Self {
        Self::F64(v)
    }
}

impl From<u128> for SecondaryKey {
    #[must_use]
    fn from(v: u128) -> Self {
        Self::U128(v)
    }
}

impl From<[u128; 2]> for SecondaryKey {
    #[must_use]
    fn from(b: [u128; 2]) -> Self {
        Self::H256([b[0], b[1]])
    }
}

impl From<Checksum256> for SecondaryKey {
    #[must_use]
    fn from(v: Checksum256) -> Self {
        v.words().into()
    }
}

impl From<Checksum160> for SecondaryKey {
    #[must_use]
    fn from(v: Checksum160) -> Self {
        v.words().into()
    }
}

macro_rules! impl_into_type {
    ($($t:ty, $x:ty)*) => ($(
        impl From<$x> for SecondaryKey {
            #[must_use]
            fn from(v: $x) -> Self {
                let v: $t = v.into();
                v.into()
            }
        }
    )*)
}

impl_into_type! {
    u64, u8
    u64, u16
    u64, u32
}

macro_rules! impl_as_u64_type {
    ($($t:ty)*) => ($(
        impl From<$t> for SecondaryKey {
            #[must_use]
            fn from(v: $t) -> Self {
                Self::U64(v.as_u64())
            }
        }
    )*)
}

impl_as_u64_type! {
    crate::account::AccountName
    crate::action::ActionName
    crate::action::PermissionName
    crate::name::Name
    crate::symbol::Symbol
    crate::symbol::SymbolCode
    crate::table::ScopeName
    crate::table::TableName
}
