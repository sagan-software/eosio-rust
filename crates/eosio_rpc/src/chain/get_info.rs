use eosio::AccountName;
use serde_derive::{Deserialize, Serialize};

const PATH: &str = "/v1/chain/get_info";

pub type ChainId = String;

pub type BlockId = String;

pub type BlockNum = u32;

pub type ServerVersion = String;

pub type BlockTimestamp = String;

#[derive(Deserialize, Serialize, Debug)]
pub struct GetInfo {
    pub server_version: ServerVersion,
    pub server_version_string: String,
    pub chain_id: ChainId,
    pub head_block_num: BlockNum,
    pub head_block_id: BlockId,
    pub head_block_time: BlockTimestamp,
    pub head_block_producer: AccountName,
    pub last_irreversible_block_num: BlockNum,
    pub last_irreversible_block_id: BlockId,
    pub virtual_block_cpu_limit: u32,
    pub virtual_block_net_limit: u32,
    pub block_cpu_limit: u32,
    pub block_net_limit: u32,
}

pub fn get_info(node: &str) -> impl ::futures::Future<Item = GetInfo, Error = crate::Error> {
    crate::http::get(node, PATH)
}
