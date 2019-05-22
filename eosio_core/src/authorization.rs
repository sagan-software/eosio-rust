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
