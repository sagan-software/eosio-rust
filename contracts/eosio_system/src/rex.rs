use eosio::*;
use std::collections::VecDeque;

/// <https://github.com/EOSIO/eosio.contracts/blob/c046863a65d7e98424312ee8009f0acb493e6231/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L216-L227>
#[derive(Read, Write, NumBytes, Debug, Clone)]
pub struct RexPool {
    pub version: u8,
    /// total amount of CORE_SYMBOL in open rex_loans
    pub total_lent: Asset,
    /// total amount of CORE_SYMBOL available to be lent (connector)
    pub total_unlent: Asset,
    /// fees received in exchange for lent  (connector)
    pub total_rent: Asset,
    /// total amount of CORE_SYMBOL that have been lent (total_unlent +
    /// total_lent)
    pub total_lendable: Asset,
    /// total number of REX shares allocated to contributors to total_lendable
    pub total_rex: Asset,
    /// the amount of CORE_SYMBOL to be transferred from namebids to REX pool
    pub namebid_proceeds: Asset,
    /// increments with each new loan
    pub loan_num: u64,
}

impl Table for RexPool {
    type Row = Self;

    const NAME: TableName = TableName::new(n!("rexpool"));

    fn primary_key(_row: &Self::Row) -> u64 {
        0
    }
}

/// <https://github.com/EOSIO/eosio.contracts/blob/c046863a65d7e98424312ee8009f0acb493e6231/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L231-L237>
#[derive(Table, Read, Write, NumBytes, Debug, Clone)]
#[eosio(table_name = "rexfund")]
pub struct RexFund {
    pub version: u8,
    /// owner of the rex fund
    #[eosio(primary_key)]
    pub owner: AccountName,
    /// balance of the fund.
    pub balance: Asset,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/c046863a65d7e98424312ee8009f0acb493e6231/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L241-L250>
#[derive(Table, Read, Write, NumBytes, Debug, Clone)]
#[eosio(table_name = "rexbal")]
pub struct RexBalance {
    pub version: u8,
    /// owner of the rex fund
    #[eosio(primary_key)]
    pub owner: AccountName,
    /// the amount of CORE_SYMBOL currently included in owner's vote
    pub vote_stake: Asset,
    /// the amount of REX owned by owner
    pub rex_balance: Asset,
    /// matured REX available for selling
    pub matured_rex: i64,
    /// REX daily maturity buckets
    pub rex_maturities: VecDeque<(TimePointSec, i64)>,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/c046863a65d7e98424312ee8009f0acb493e6231/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L254-L267>
#[derive(Read, Write, NumBytes, Debug, Clone)]
pub struct RexLoan {
    pub version: u8,
    pub from: AccountName,
    pub receiver: AccountName,
    pub payment: Asset,
    pub balance: Asset,
    pub total_staked: Asset,
    pub loan_num: u64,
    pub expiration: TimePoint,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/c046863a65d7e98424312ee8009f0acb493e6231/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L279-L291>
#[derive(Read, Write, NumBytes, Debug, Clone)]
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
pub struct RexOrderOutcome {
    pub success: bool,
    pub proceeds: Asset,
    pub stake_change: Asset,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/rex.cpp#L10-L22>
#[eosio::action]
pub fn deposit(owner: AccountName, amount: Asset) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/rex.cpp#L24-L37>
#[eosio::action]
pub fn withdraw(owner: AccountName, amount: Asset) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/rex.cpp#L39-L54>
#[eosio::action]
pub fn buyrex(from: AccountName, amount: Asset) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/rex.cpp#L56-L94>
#[eosio::action]
pub fn unstaketorex(
    owner: AccountName,
    receiver: AccountName,
    from_net: Asset,
    from_cpu: Asset,
) {
}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/rex.cpp#L96-L144>
#[eosio::action]
pub fn sellrex(from: AccountName, rex: Asset) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/rex.cpp#L146-L153>
#[eosio::action]
pub fn cnclrexorder(owner: AccountName) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/rex.cpp#L155-L162>
#[eosio::action]
pub fn rentcpu(
    from: AccountName,
    receiver: AccountName,
    loan_payment: Asset,
    loan_fund: Asset,
) {
}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/rex.cpp#L164-L171>
#[eosio::action]
pub fn rentnet(
    from: AccountName,
    receiver: AccountName,
    loan_payment: Asset,
    loan_fund: Asset,
) {
}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/rex.cpp#L173-L179>
#[eosio::action]
pub fn fundcpuloan(from: AccountName, loan_num: u64, payment: Asset) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/rex.cpp#L181-L187>
#[eosio::action]
pub fn fundnetloan(from: AccountName, loan_num: u64, payment: Asset) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/rex.cpp#L189-L195>
#[eosio::action]
pub fn defcpuloan(from: AccountName, loan_num: u64, amount: Asset) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/rex.cpp#L197-L203>
#[eosio::action]
pub fn defnetloan(from: AccountName, loan_num: u64, amount: Asset) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/rex.cpp#L205-L229>
#[eosio::action]
pub fn updaterex(owner: AccountName) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/rex.cpp#L231-L241>
#[eosio::action]
pub fn setrex(balance: Asset) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/rex.cpp#L243-L248>
#[eosio::action]
pub fn rexexec(user: AccountName, max: u16) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/rex.cpp#L250-L259>
#[eosio::action]
pub fn consolidate(owner: AccountName) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/rex.cpp#L261-L293>
#[eosio::action]
pub fn mvtosavings(owner: AccountName, rex: Asset) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/rex.cpp#L295-L316>
#[eosio::action]
pub fn mvfrsavings(owner: AccountName, rex: Asset) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/rex.cpp#L318-L353>
#[eosio::action]
pub fn closerex(owner: AccountName) {}
