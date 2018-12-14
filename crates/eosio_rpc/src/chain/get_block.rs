use crate::Builder;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GetBlockBuilder {
    block_num_or_id: String,
}

impl Builder for GetBlockBuilder {
    const PATH: &'static str = "/v1/chain/get_block";
    type Output = GetBlock;
}

pub fn get_block<B>(block_num_or_id: B) -> GetBlockBuilder
where
    B: ToString,
{
    GetBlockBuilder {
        block_num_or_id: block_num_or_id.to_string(),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlock {
    pub timestamp: String,
    pub producer: String,
    pub confirmed: u16,
    pub previous: String,
    pub transaction_mroot: String,
    pub action_mroot: String,
    pub schedule_version: u16,
    pub new_producers: Option<NewProducers>,
    pub header_extensions: Vec<Extension>,
    pub producer_signature: String,
    pub transactions: Vec<Transaction>,
    pub block_extensions: Vec<Extension>,
    pub id: String,
    pub block_num: u64,
    pub ref_block_prefix: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewProducers {
    pub version: u32,
    pub producers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Extension {
    #[serde(rename = "type")]
    pub type_: u16,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub status: String,
    pub cpu_usage_us: u64,
    pub net_usage_words: u64,
    pub trx: Trx,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trx {
    pub id: String,
    pub signatures: Vec<String>,
    pub compression: String,
    pub packed_context_free_data: String,
    pub packed_trx: String,
    pub transaction: TransactionInner,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionInner {
    pub expiration: String,
    pub ref_block_num: u64,
    pub ref_block_prefix: u64,
    pub max_net_usage_words: u64,
    pub max_cpu_usage_ms: u64,
    pub delay_sec: u64,
    pub context_free_actions: Vec<Action>,
    pub actions: Vec<Action>,
    pub transaction_extensions: Vec<Extension>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    pub account: String,
    pub name: String,
    pub authorization: Vec<::eosio::Authorization>,
    pub data: ::serde_json::Value,
    pub hex_data: String,
}
