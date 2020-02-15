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
    H256([u8; 32]),
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

impl From<[u8; 32]> for SecondaryKey {
    #[must_use]
    fn from(v: [u8; 32]) -> Self {
        Self::H256(v)
    }
}

impl From<Checksum256> for SecondaryKey {
    #[must_use]
    fn from(v: Checksum256) -> Self {
        Self::H256(v.to_bytes())
    }
}

impl From<Checksum160> for SecondaryKey {
    #[must_use]
    fn from(v: Checksum160) -> Self {
        let b = v.to_bytes();
        Self::H256([
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10],
            b[11], b[12], b[13], b[14], b[15], b[16], b[17], b[18], b[19], 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ])
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
