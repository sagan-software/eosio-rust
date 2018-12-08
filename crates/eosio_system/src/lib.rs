use eosio::Time;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProducerInfo {
    pub owner: String,
    pub total_votes: String,
    pub producer_key: String,
    pub is_active: u8,
    pub url: String,
    pub unpaid_blocks: u64,
    pub last_claim_time: Time,
    pub location: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalState {
    pub max_block_net_usage: u64,
    pub target_block_net_usage_pct: u32,
    pub max_transaction_net_usage: u32,
    pub base_per_transaction_net_usage: u32,
    pub net_usage_leeway: u32,
    pub context_free_discount_net_usage_num: u32,
    pub context_free_discount_net_usage_den: u32,
    pub max_block_cpu_usage: u32,
    pub target_block_cpu_usage_pct: u32,
    pub max_transaction_cpu_usage: u32,
    pub min_transaction_cpu_usage: u32,
    pub max_transaction_lifetime: u32,
    pub deferred_trx_expiration_window: u32,
    pub max_transaction_delay: u32,
    pub max_inline_action_depth: u16,
    pub max_authority_depth: u16,
    pub max_ram_size: u64,
    pub total_ram_bytes_reserved: u64,
    pub total_ram_stake: i64,
    // TODO: pub last_producer_schedule_update:
    pub last_pervote_bucket_fill: u64,
    pub pervote_bucket: i64,
    pub total_unpaid_blocks: u32,
    pub total_activated_stake: i64,
    pub thresh_activated_stake_time: u64,
    pub last_producer_schedule_size: u16,
    pub total_producer_vote_weight: f64,
    // TODO: pub last_name_close:
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalState2 {}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoterInfo {}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResources {}

#[derive(Debug, Serialize, Deserialize)]
pub struct DelegatedBandwidth {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeState {}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameBid {}

#[derive(Debug, Serialize, Deserialize)]
pub struct BidRefund {}
