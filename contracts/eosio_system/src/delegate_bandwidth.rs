use eosio::*;

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L270-L281>
#[derive(Table, Read, Write, NumBytes, Debug, Clone)]
#[eosio(table_name = "userres")]
pub struct UserResources {
    #[eosio(primary_key)]
    pub owner: AccountName,
    pub net_weight: Asset,
    pub cpu_weight: Asset,
    pub ram_bytes: i64,
}

/// Every user `from` has a scope/table that uses every receipient 'to' as the
/// primary key. <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L283-L296>
#[derive(Table, Read, Write, NumBytes, Debug, Clone)]
#[eosio(table_name = "delband")]
pub struct DelegatedBandwidth {
    pub from: AccountName,
    #[eosio(primary_key)]
    pub to: AccountName,
    pub net_weight: Asset,
    pub cpu_weight: Asset,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L298-L309>
#[derive(Table, Read, Write, NumBytes, Debug, Clone)]
#[eosio(table_name = "refunds")]
pub struct RefundRequest {
    #[eosio(primary_key)]
    pub owner: AccountName,
    pub request_time: TimePointSec,
    pub net_amount: Asset,
    pub cpu_amount: Asset,
}

/// This action will buy an exact amount of ram and bill the payer the current
/// market price.
///
/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/delegate_bandwidth.cpp#L25-L35>
#[eosio::action]
pub fn buyrambytes(payer: AccountName, receiver: AccountName, bytes: u32) {}

/// When buying ram the payer irreversiblly transfers quant to system contract
/// and only the receiver may reclaim the tokens via the sellram action. The
/// receiver pays for the storage of all database records associated with this
/// action.
///
/// RAM is a scarce resource whose supply is defined by global properties
/// max_ram_size. RAM is priced using the bancor algorithm such that
/// price-per-byte with a constant reserve ratio of 100:1.
///
/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/delegate_bandwidth.cpp#L38-L106>
#[eosio::action]
pub fn buyram(payer: AccountName, receiver: AccountName, quant: Asset) {}

/// The system contract now buys and sells RAM allocations at prevailing market
/// prices. This may result in traders buying RAM today in anticipation of
/// potential shortages tomorrow. Overall this will result in the market
/// balancing the supply and demand for RAM over time.
///
/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/delegate_bandwidth.cpp#L108-L162>
#[eosio::action]
pub fn sellram(account: AccountName, bytes: i64) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/delegate_bandwidth.cpp#L375-L386>
#[eosio::action]
pub fn delegatebw(
    from: AccountName,
    receiver: AccountName,
    stake_net_quantity: Asset,
    stake_cpu_quantity: Asset,
    transfer: bool,
) {
}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/delegate_bandwidth.cpp#L388-L399>
#[eosio::action]
pub fn undelegatebw(
    from: AccountName,
    receiver: AccountName,
    unstake_net_quantity: Asset,
    unstake_cpu_quantity: Asset,
) {
}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/delegate_bandwidth.cpp#L402-L413>
#[eosio::action]
pub fn refund(owner: AccountName) {}
