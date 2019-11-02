use eosio::{AccountName, ActionName, PermissionLevel};
use serde::de::{self, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;

crate::builder!("/v1/chain/get_block", GetBlock, Block);

#[derive(Serialize, Clone)]
pub struct GetBlock {
    pub block_num_or_id: String,
}

pub fn get_block<B: ToString>(block_num_or_id: B) -> GetBlock {
    GetBlock {
        block_num_or_id: block_num_or_id.to_string(),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub timestamp: String,
    pub producer: AccountName,
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

impl Block {
    pub fn num_actions(&self) -> usize {
        self.transactions.iter().map(Transaction::num_actions).sum()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewProducers {
    pub version: u32,
    pub producers: Vec<AccountName>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Extension {
    #[serde(rename = "type")]
    pub type_: u16,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub status: String,
    pub cpu_usage_us: u64,
    pub net_usage_words: u64,
    pub trx: Trx,
}

impl Transaction {
    pub fn num_actions(&self) -> usize {
        match &self.trx {
            Trx::Standard(trx) => {
                trx.transaction.actions.len() + trx.transaction.context_free_actions.len()
            }
            Trx::Deferred(_) => 0,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub enum Trx {
    Standard(StandardTrx),
    Deferred(String), // TODO use TransactionId, convert String to u128
}

struct TrxVisitor;

impl<'de> Visitor<'de> for TrxVisitor {
    type Value = Trx;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a transaction struct or deferred transaction ID")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
        Ok(Trx::Deferred(v.to_string()))
    }

    fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        Deserialize::deserialize(de::value::MapAccessDeserializer::new(map)).map(Trx::Standard)
    }
}

impl<'de> Deserialize<'de> for Trx {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(TrxVisitor)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StandardTrx {
    pub id: String,
    pub signatures: Vec<String>,
    pub compression: String,
    pub packed_context_free_data: String,
    pub packed_trx: String,
    pub transaction: TransactionInner,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    pub account: AccountName,
    pub name: ActionName,
    pub authorization: Vec<PermissionLevel>,
    pub data: serde_json::Value,
    pub hex_data: Option<String>,
}
