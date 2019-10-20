//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/contracts/eosio/action.hpp#L249-L274>
use crate::account::AccountName;
use crate::bytes::{NumBytes, Read, Write};
use crate::name_type;
use serde::{Deserialize, Serialize};

name_type!(ActionName);
name_type!(PermissionName);

/// This is the packed representation of an action along with meta-data about
/// the authorization levels.
#[derive(
    Clone, Debug, Serialize, Deserialize, Read, Write, NumBytes, Default,
)]
#[__eosio_path = "crate::bytes"]
pub struct Action<T> {
    /// Name of the account the action is intended for
    pub account: AccountName,
    /// Name of the action
    pub name: ActionName,
    /// List of permissions that authorize this action
    pub authorization: Vec<PermissionLevel>,
    /// Payload data
    pub data: T,
}

/// TODO docs
pub trait ToAction: Write + NumBytes {
    /// TODO docs
    const NAME: ActionName;

    /// TODO docs
    #[inline]
    fn to_action(
        &self,
        account: AccountName,
        authorization: Vec<PermissionLevel>,
    ) -> Action<Vec<u8>> {
        let mut data = vec![0_u8; self.num_bytes()];
        self.write(&mut data, &mut 0).expect("write");

        Action {
            account,
            name: Self::NAME,
            authorization,
            data,
        }
    }
}

/// TODO docs.
pub trait ActionFn: ToAction + Read + Write + NumBytes + Clone {
    /// TODO docs.
    fn call(self);
}

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
#[__eosio_path = "crate::bytes"]
pub struct PermissionLevel {
    /// TODO docs
    pub actor: AccountName,
    /// TODO docs
    pub permission: PermissionName,
}
