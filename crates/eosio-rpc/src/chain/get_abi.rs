use eosio::AccountName;
use eosio_abi::Abi;
use serde::{Deserialize, Serialize};

crate::builder!("/v1/chain/get_abi", GetAbiParams, GetAbi);

#[derive(Serialize, Clone)]
pub struct GetAbiParams {
    account_name: AccountName,
}

pub const fn get_abi(account_name: AccountName) -> GetAbiParams {
    GetAbiParams { account_name }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAbi {
    pub account_name: AccountName,
    pub abi: Abi,
}
