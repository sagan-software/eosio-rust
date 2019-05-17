use eosio::{AccountName, Authorization, PermissionName};
use serde_derive::{Deserialize, Serialize};

crate::builder!("/v1/chain/get_account", GetAccountParams, GetAccount);

#[derive(Serialize, Clone)]
pub struct GetAccountParams {
    account_name: AccountName,
}

pub const fn get_account(account_name: AccountName) -> GetAccountParams {
    GetAccountParams { account_name }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAccount {
    pub account_name: AccountName,
    pub head_block_num: i64,
    pub head_block_time: String,
    pub privileged: bool,
    pub last_code_update: String,
    pub created: String,
    pub core_liquid_balance: Option<String>,
    pub ram_quota: i64,
    pub net_weight: i64,
    pub cpu_weight: i64,
    pub net_limit: Limit,
    pub cpu_limit: Limit,
    pub ram_usage: i64,
    pub permissions: Vec<Permission>,
    pub total_resources: Option<TotalResources>,
    pub self_delegated_bandwidth: Option<SelfDelegatedBandwidth>,
    pub refund_request: Option<RefundRequest>,
    pub voter_info: Option<VoterInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Limit {
    pub used: i64,
    pub available: i64,
    pub max: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Permission {
    pub perm_name: PermissionName,
    pub parent: PermissionName,
    pub required_auth: RequiredAuth,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequiredAuth {
    pub threshold: u32,
    pub keys: Vec<KeyWeight>,
    pub accounts: Vec<PermissionLevelWeight>,
    pub waits: Vec<WaitWeight>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionLevelWeight {
    pub permission: Authorization,
    pub weight: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaitWeight {
    pub wait_sec: u32,
    pub weight: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyWeight {
    pub key: String,
    pub weight: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalResources {
    pub owner: AccountName,
    pub net_weight: String,
    pub cpu_weight: String,
    pub ram_bytes: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfDelegatedBandwidth {
    pub from: AccountName,
    pub to: AccountName,
    pub net_weight: String,
    pub cpu_weight: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefundRequest {
    pub owner: String,
    pub request_time: String,
    pub net_amount: String,
    pub cpu_amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoterInfo {
    pub owner: AccountName,
    pub proxy: AccountName,
    pub producers: Vec<AccountName>,
    pub staked: u64,
    pub last_vote_weight: String,
    pub proxied_vote_weight: String,
    pub is_proxy: u8,
}
