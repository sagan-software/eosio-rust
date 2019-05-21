use crate::{AccountName, PermissionName};
use eosio_bytes::*;

/// A permission
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Default,
    Read,
    Write,
    NumBytes,
    Hash,
    PartialOrd,
    Ord,
)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[eosio_bytes_root_path = "::eosio_bytes"]
pub struct Authorization {
    /// Docs
    pub actor: AccountName,
    /// Docs
    pub permission: PermissionName,
}

/// RAM in bytes
pub struct RamBytes(i64);

impl From<i64> for RamBytes {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

/// Net Weight
pub struct NetWeight(i64);

impl From<i64> for NetWeight {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

/// CPU Weight
pub struct CpuWeight(i64);

impl From<i64> for CpuWeight {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
