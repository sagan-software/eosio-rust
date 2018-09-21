use eosio_bytes::*;
use eosio_derives::*;
use names::*;

#[cfg(any(feature = "std", feature = "alloc"))]
use lib::Vec;

#[cfg(any(feature = "std", feature = "alloc"))]
#[derive(Readable, Writeable, Clone, Debug)]
pub struct Action<Data>
where
    Data: Readable + Writeable,
{
    pub account: AccountName,
    pub name: ActionName,
    pub authorization: Vec<PermissionLevel>,
    pub data: Data,
}

#[derive(Readable, Writeable, Clone, Debug)]
pub struct PermissionLevel {
    pub actor: AccountName,
    pub permission: PermissionName,
}
