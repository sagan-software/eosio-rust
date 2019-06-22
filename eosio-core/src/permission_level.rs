//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/contracts/eosio/action.hpp#L180-L217>
use crate::{AccountName, NumBytes, PermissionName, Read, Write};
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
#[eosio_core_root_path = "crate"]
pub struct PermissionLevel {
    /// TODO docs
    pub actor: AccountName,
    /// TODO docs
    pub permission: PermissionName,
}
