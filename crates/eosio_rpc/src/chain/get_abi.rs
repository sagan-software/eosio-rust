use crate::{Builder, Client, Error};
use eosio::AccountName;
use eosio_abi::Abi;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GetAbiBuilder {
    account_name: AccountName,
}

impl Builder for GetAbiBuilder {
    const PATH: &'static str = "/v1/chain/get_abi";
    type Output = GetAbi;
}

pub fn get_abi(account_name: AccountName) -> GetAbiBuilder {
    GetAbiBuilder { account_name }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAbi {
    pub account_name: AccountName,
    pub abi: Abi,
}
