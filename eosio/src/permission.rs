use account::AccountName;
use eosio_macros::*;

eosio_name!(PermissionName);

#[derive(Read, Write, Clone, Debug)]
pub struct PermissionLevel {
    pub actor: AccountName,
    pub permission: PermissionName,
}
