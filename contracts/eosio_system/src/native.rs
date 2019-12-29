use eosio::*;
use std::marker::PhantomData;

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/eosio.system.cpp#L312-L358>
#[eosio::action]
pub fn newaccount(
    creator: AccountName,
    newact: AccountName,
    owner: PhantomData<PermissionLevel>,
    active: PhantomData<PermissionLevel>,
) {
}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/eosio.system.cpp#L360-L373>
#[eosio::action]
pub fn setabi(acnt: AccountName, abi: Vec<char>) {}
