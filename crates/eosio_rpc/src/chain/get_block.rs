use serde_derive::{Deserialize, Serialize};

const PATH: &str = "/v1/chain/get_block";

#[derive(Serialize)]
struct Params {
    block_num_or_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlock {
    timestamp: String,
    producer: String,
    confirmed: u16,
    previous: String,
    transaction_mroot: String,
    action_mroot: String,
    schedule_version: u16,
    new_producers: Option<NewProducers>,
    header_extensions: Vec<Extension>,
    producer_signature: String,
    transactions: Vec<Transaction>,
    block_extensions: Vec<Extension>,
    id: String,
    block_num: u64,
    ref_block_prefix: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewProducers {
    version: u32,
    producers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Extension {
    #[serde(rename = "type")]
    type_: u16,
    data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    status: String,
    cpu_usage_us: u64,
    net_usage_words: u64,
    trx: Trx,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trx {
    id: String,
    signatures: Vec<String>,
    compression: String,
    packed_context_free_data: String,
    packed_trx: String,
    transaction: TransactionInner,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionInner {
    expiration: String,
    ref_block_num: u64,
    ref_block_prefix: u64,
    max_net_usage_words: u64,
    max_cpu_usage_ms: u64,
    delay_sec: u64,
    context_free_actions: Vec<Action>,
    actions: Vec<Action>,
    transaction_extensions: Vec<Extension>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    account: String,
    name: String,
    authorization: Vec<::eosio::Authorization>,
    data: ::serde_json::Value,
    hex_data: String,
}

pub fn get_block<B>(
    node: &str,
    block_num_or_id: B,
) -> impl ::futures::Future<Item = GetBlock, Error = crate::Error>
where
    B: ToString,
{
    crate::http::post(
        node,
        PATH,
        Params {
            block_num_or_id: block_num_or_id.to_string(),
        },
    )
}
