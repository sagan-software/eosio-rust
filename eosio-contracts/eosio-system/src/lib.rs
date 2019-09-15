use eosio::*;
use serde_derive::{Deserialize, Serialize};

pub struct NameBid {
    pub new_name: AccountName,
    pub high_bidder: AccountName,
    pub high_bid: i64,
    pub last_bid_time: TimePoint,
}

pub struct BidRefund {
    pub bidder: AccountName,
    pub amount: Asset,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/c046863a65d7e98424312ee8009f0acb493e6231/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L84-L108>
#[derive(Serialize, Deserialize, Table, Read, Write, NumBytes)]
#[table_name = "global"]
#[singleton]
pub struct EosioGlobalState {
    pub blockchain_parameters: BlockchainParameters,
    #[serde(deserialize_with = "eosio::u64_or_string")]
    pub max_ram_size: u64,
    #[serde(deserialize_with = "eosio::u64_or_string")]
    pub total_ram_bytes_reserved: u64,
    pub total_ram_stake: i64,
    pub last_producer_schedule_update: BlockTimestamp,
    pub last_pervote_bucket_fill: TimePoint,
    pub pervote_bucket: i64,
    pub perblock_bucket: i64,
    pub total_unpaid_blocks: u32,
    pub total_activated_stake: i64,
    pub thresh_activated_stake_time: TimePoint,
    pub last_producer_schedule_size: u16,
    #[serde(deserialize_with = "eosio::f64_or_string")]
    pub total_producer_vote_weight: f64,
    pub last_name_close: BlockTimestamp,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/c046863a65d7e98424312ee8009f0acb493e6231/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L110-L124>
#[table_name = "global2"]
#[singleton]
#[derive(Serialize, Deserialize, Table, Read, Write, NumBytes)]
pub struct EosioGlobalState2 {
    pub new_ram_per_block: u16,
    pub last_ram_increase: BlockTimestamp,
    pub last_block_num: BlockTimestamp,
    #[serde(deserialize_with = "eosio::f64_or_string")]
    pub total_producer_votepay_share: f64,
    pub revision: u8,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/c046863a65d7e98424312ee8009f0acb493e6231/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L126-L132>
#[table_name = "global3"]
#[singleton]
#[derive(Serialize, Deserialize, Table, Read, Write, NumBytes)]
pub struct EosioGlobalState3 {
    pub last_vpay_state_update: TimePoint,
    #[serde(deserialize_with = "eosio::f64_or_string")]
    pub total_vpay_share_change_rate: f64,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/c046863a65d7e98424312ee8009f0acb493e6231/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L134-L152>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProducerInfo {
    pub owner: AccountName,
    #[serde(deserialize_with = "eosio::f64_or_string")]
    pub total_votes: f64,
    // pub producer_key: PublicKey,
    #[serde(deserialize_with = "eosio::bool_or_integer")]
    pub is_active: bool,
    pub url: String,
    pub unpaid_blocks: u32,
    pub last_claim_time: TimePoint,
    pub location: u16,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/c046863a65d7e98424312ee8009f0acb493e6231/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L154-L163>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProducerInfo2 {
    pub owner: AccountName,
    #[serde(deserialize_with = "eosio::f64_or_string")]
    pub votepay_share: f64,
    pub last_votepay_share_update: TimePoint,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/c046863a65d7e98424312ee8009f0acb493e6231/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L165-L200>
#[derive(Serialize, Deserialize)]
pub struct VoterInfo {
    /// The voter
    pub owner: AccountName,
    /// The proxy set by the voter, if any
    pub proxy: AccountName,
    /// The producers approved by this voter if no proxy set
    pub producers: Vec<AccountName>,
    pub staked: i64,
    /// The vote weight cast the last time the vote was updated.
    ///
    /// Every time a vote is cast we must first "undo" the last vote weight, before casting the
    /// new vote weight.  Vote weight is calculated as:
    ///
    ///     stated.amount * 2 ^ ( weeks_since_launch/weeks_per_year)
    #[serde(deserialize_with = "eosio::f64_or_string")]
    pub last_vote_weight: f64,
    /// Total vote weight delegated to this voter.
    #[serde(deserialize_with = "eosio::f64_or_string")]
    pub proxied_vote_weight: f64,
    /// Wether the voter is a proxy for others.
    pub is_proxy: bool,
    pub flags1: u32,
    pub reserved2: u32,
    pub reserved3: Asset,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/c046863a65d7e98424312ee8009f0acb493e6231/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L216-L227>
#[derive(Serialize, Deserialize)]
pub struct RexPool {
    pub version: u8,
    pub total_lent: Asset,
    pub total_unlent: Asset,
    pub total_rent: Asset,
    pub total_lendable: Asset,
    pub total_rex: Asset,
    pub namebid_proceeds: Asset,
    #[serde(deserialize_with = "eosio::u64_or_string")]
    pub loan_num: u64,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/c046863a65d7e98424312ee8009f0acb493e6231/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L231-L237>
#[derive(Serialize, Deserialize)]
pub struct RexFund {
    pub version: u8,
    pub owner: AccountName,
    pub balance: Asset,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/c046863a65d7e98424312ee8009f0acb493e6231/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L241-L250>
#[derive(Serialize, Deserialize)]
pub struct RexBalance {
    pub version: u8,
    pub owner: AccountName,
    pub vote_stake: Asset,
    pub rex_balance: Asset,
    pub matured_rex: i64,
    pub rex_maturities: std::collections::VecDeque<(TimePointSec, i64)>,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/c046863a65d7e98424312ee8009f0acb493e6231/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L254-L267>
#[derive(Serialize, Deserialize)]
pub struct RexLoan {
    pub version: u8,
    pub from: AccountName,
    pub receiver: AccountName,
    pub payment: Asset,
    pub balance: Asset,
    pub total_staked: Asset,
    #[serde(deserialize_with = "eosio::u64_or_string")]
    pub loan_num: u64,
    pub expiration: TimePoint,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/c046863a65d7e98424312ee8009f0acb493e6231/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L279-L291>
#[derive(Serialize, Deserialize)]
pub struct RexOrder {
    pub version: u8,
    pub owner: AccountName,
    pub rex_requested: Asset,
    pub proceeds: Asset,
    pub stake_change: Asset,
    pub order_time: TimePoint,
    pub is_open: bool,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/c046863a65d7e98424312ee8009f0acb493e6231/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L296-L300>
#[derive(Serialize, Deserialize)]
pub struct RexOrderOutcome {
    pub success: bool,
    pub proceeds: Asset,
    pub stake_change: Asset,
}

pub const ACTIVE_PERMISSION: PermissionName = PermissionName::new(n!(active));
pub const TOKEN_ACCOUNT: AccountName = AccountName::new(n!(eosio.token));
pub const RAM_ACCOUNT: AccountName = AccountName::new(n!(eosio.ram));
pub const RAMFEE_ACCOUNT: AccountName = AccountName::new(n!(eosio.ramfee));
pub const STAKE_ACCOUNT: AccountName = AccountName::new(n!(eosio.stake));
pub const BPAY_ACCOUNT: AccountName = AccountName::new(n!(eosio.bpay));
pub const VPAY_ACCOUNT: AccountName = AccountName::new(n!(eosio.vpay));
pub const NAMES_ACCOUNT: AccountName = AccountName::new(n!(eosio.names));
pub const SAVING_ACCOUNT: AccountName = AccountName::new(n!(eosio.saving));
pub const REX_ACCOUNT: AccountName = AccountName::new(n!(eosio.rex));
pub const NULL_ACCOUNT: AccountName = AccountName::new(n!(eosio.null));
pub const RAMCORE_SYMBOL: Symbol = Symbol::new(s!(4, RAMCORE));
pub const RAM_SYMBOL: Symbol = Symbol::new(s!(0, RAM));
pub const REX_SYMBOL: Symbol = Symbol::new(s!(4, REX));

pub fn init(version: UnsignedInt, core: Symbol) {}

// pub fn onblock(header: Ignore<BlockHeader>) {}

pub fn setalimits(
    account: AccountName,
    ram_bytes: i64,
    net_weight: i64,
    cpu_weight: i64,
) {
}
