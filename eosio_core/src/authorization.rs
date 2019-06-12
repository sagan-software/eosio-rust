//! TODO docs
use crate::{AccountName, PermissionName};
use eosio_bytes::*;
use serde::{Deserialize, Serialize};

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
    Serialize,
    Deserialize,
)]
#[eosio_bytes_root_path = "::eosio_bytes"]
pub struct Authorization {
    /// TODO docs
    pub actor: AccountName,
    /// TODO docs
    pub permission: PermissionName,
}
