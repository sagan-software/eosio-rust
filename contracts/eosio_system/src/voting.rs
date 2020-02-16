use eosio::*;

/// Defines `producer_info` structure to be stored in `producer_info` table,
/// added after version 1.0 <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L180-L200>
#[derive(Read, Write, NumBytes, Debug, Clone)]
pub struct ProducerInfo {
    pub owner: AccountName,
    pub total_votes: f64,
    pub producer_key: PublicKey,
    pub is_active: bool,
    pub url: String,
    pub unpaid_blocks: u32,
    pub last_claim_time: TimePoint,
    pub location: u16,
    /// Added in version 1.9.0
    pub producer_authority: BinaryExtension<BlockSigningAuthority>,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/c046863a65d7e98424312ee8009f0acb493e6231/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L154-L163>
#[derive(Table, Read, Write, NumBytes, Debug, Clone)]
#[eosio(table_name = "producers2")]
pub struct ProducerInfo2 {
    #[eosio(primary_key)]
    pub owner: AccountName,
    pub votepay_share: f64,
    pub last_votepay_share_update: TimePoint,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/c046863a65d7e98424312ee8009f0acb493e6231/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L165-L200>
#[derive(Table, Read, Write, NumBytes, Debug, Clone)]
#[eosio(table_name = "voters")]
pub struct VoterInfo {
    /// The voter
    #[eosio(primary_key)]
    pub owner: AccountName,
    /// The proxy set by the voter, if any
    pub proxy: AccountName,
    /// The producers approved by this voter if no proxy set
    pub producers: Vec<AccountName>,
    pub staked: i64,
    /// The vote weight cast the last time the vote was updated.
    ///
    /// Every time a vote is cast we must first "undo" the last vote weight,
    /// before casting the new vote weight.  Vote weight is calculated as:
    ///
    ///     stated.amount * 2 ^ ( weeks_since_launch/weeks_per_year)
    pub last_vote_weight: f64,
    /// Total vote weight delegated to this voter.
    pub proxied_vote_weight: f64,
    /// Wether the voter is a proxy for others.
    pub is_proxy: bool,
    pub flags1: u32,
    pub reserved2: u32,
    pub reserved3: Asset,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/voting.cpp#L83-L88>
#[eosio::action]
pub fn regproducer(
    producer: AccountName,
    producer_key: PublicKey,
    url: String,
    location: u16,
) {
}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/voting.cpp#L90-L99>
#[eosio::action]
pub fn regproducer2(
    producer: AccountName,
    producer_authority: BlockSigningAuthority,
    url: String,
    location: u16,
) {
}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/voting.cpp#L101-L108>
#[eosio::action]
pub fn unregprod(producer: AccountName) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/voting.cpp#L207-L215>
#[eosio::action]
pub fn voteproducer(
    voter: AccountName,
    proxy: AccountName,
    producers: Vec<AccountName>,
) {
}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/voting.cpp#L342-L359>
#[eosio::action]
pub fn regproxy(proxy: AccountName, isproxy: bool) {}
