use eosio_bytes::*;
use eosio_derives::*;
use names::*;

#[cfg(any(feature = "std", feature = "alloc"))]
use lib::Vec;

#[cfg(any(feature = "std", feature = "alloc"))]
#[derive(Writeable, Clone, Debug)]
pub struct Action<'a, Data>
where
    Data: Writeable,
{
    pub account: AccountName,
    pub name: ActionName,
    pub authorization: &'a [PermissionLevel],
    pub data: Data,
}

#[derive(Readable, Writeable, Clone, Debug)]
pub struct PermissionLevel {
    pub actor: AccountName,
    pub permission: PermissionName,
}
