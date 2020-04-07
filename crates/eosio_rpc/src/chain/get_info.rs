use crate::builder;
use eosio::AccountName;
use serde::{Deserialize, Serialize};

builder!("/v1/chain/get_info", GetInfo, Info);

#[derive(Serialize, Clone)]
pub struct GetInfo {}

pub const fn get_info() -> GetInfo {
    GetInfo {}
}

pub type ChainId = String;

pub type BlockId = String;

pub type BlockNum = u32;

pub type ServerVersion = String;

pub type BlockTimestamp = String;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Info {
    pub server_version: ServerVersion,
    pub server_version_string: String,
    pub chain_id: String,
    pub head_block_num: u32,
    pub head_block_id: String,
    pub head_block_time: String,
    pub head_block_producer: String,
    pub last_irreversible_block_num: String,
    pub last_irreversible_block_id: String,
    pub virtual_block_cpu_limit: u32,
    pub virtual_block_net_limit: u32,
    pub block_cpu_limit: u32,
    pub block_net_limit: u32,
}
