use eosio::{AccountName, Authorization};
use serde_derive::{Deserialize, Serialize};

const PATH: &str = "/v1/chain/get_account";

#[derive(Serialize)]
struct Params {
    account_name: AccountName,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAccount {
    account_name: AccountName,
    head_block_num: u64,
    head_block_time: String,
    privileged: bool,
    last_code_update: String,
    created: String,
    core_liquid_balance: String,
    ram_quota: u64,
    net_weight: u64,
    cpu_weight: u64,
    net_limit: Limit,
    cpu_limit: Limit,
    ram_usage: u64,
    permissions: Vec<Permission>,
    total_resources: TotalResources,
    self_delegated_bandwidth: SelfDelegatedBandwidth,
    refund_request: Option<RefundRequest>,
    voter_info: VoterInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Limit {
    used: u64,
    available: u64,
    max: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Permission {
    perm_name: String,
    parent: String,
    required_auth: RequiredAuth,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequiredAuth {
    threshold: u32,
    keys: Vec<KeyWeight>,
    accounts: Vec<PermissionLevelWeight>,
    waits: Vec<WaitWeight>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionLevelWeight {
    permission: Authorization,
    weight: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaitWeight {
    wait_sec: u32,
    weight: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyWeight {
    key: String,
    weight: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalResources {
    owner: String,
    net_weight: String,
    cpu_weight: String,
    ram_bytes: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfDelegatedBandwidth {
    from: String,
    to: String,
    net_weight: String,
    cpu_weight: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefundRequest {
    owner: String,
    request_time: String,
    net_amount: String,
    cpu_amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoterInfo {
    owner: String,
    proxy: String,
    producers: Vec<String>,
    staked: u64,
    last_vote_weight: String,
    proxied_vote_weight: String,
    is_proxy: u8,
}

pub fn get_account<A>(
    node: &str,
    account_name: A,
) -> impl ::futures::Future<Item = GetAccount, Error = crate::Error>
where
    A: Into<AccountName>,
{
    crate::http::post(
        node,
        PATH,
        Params {
            account_name: account_name.into(),
        },
    )
}
