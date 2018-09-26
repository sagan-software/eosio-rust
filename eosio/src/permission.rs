use account::AccountName;
use eosio_macros::*;

eosio_name!(PermissionName);

#[derive(Readable, Writeable, Clone, Debug)]
#[eosio_internal]
pub struct PermissionLevel {
    pub actor: AccountName,
    pub permission: PermissionName,
}
