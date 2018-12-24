use crate::Builder;
use eosio::{AccountName, Authorization};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GetAccountBuilder {
    account_name: AccountName,
}

impl Builder for GetAccountBuilder {
    const PATH: &'static str = "/v1/chain/get_account";
    type Output = GetAccount;
}

pub fn get_account(account_name: AccountName) -> GetAccountBuilder {
    GetAccountBuilder { account_name }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAccount {
    pub account_name: AccountName,
    pub head_block_num: u64,
    pub head_block_time: String,
    pub privileged: bool,
    pub last_code_update: String,
    pub created: String,
    pub core_liquid_balance: String,
    pub ram_quota: u64,
    pub net_weight: u64,
    pub cpu_weight: u64,
    pub net_limit: Limit,
    pub cpu_limit: Limit,
    pub ram_usage: u64,
    pub permissions: Vec<Permission>,
    pub total_resources: TotalResources,
    pub self_delegated_bandwidth: SelfDelegatedBandwidth,
    pub refund_request: Option<RefundRequest>,
    pub voter_info: VoterInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Limit {
    pub used: u64,
    pub available: u64,
    pub max: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Permission {
    pub perm_name: String,
    pub parent: String,
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
    pub owner: String,
    pub net_weight: String,
    pub cpu_weight: String,
    pub ram_bytes: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfDelegatedBandwidth {
    pub from: String,
    pub to: String,
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
    pub owner: String,
    pub proxy: String,
    pub producers: Vec<String>,
    pub staked: u64,
    pub last_vote_weight: String,
    pub proxied_vote_weight: String,
    pub is_proxy: u8,
}
