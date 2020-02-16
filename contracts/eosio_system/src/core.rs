use crate::{RAMCORE_SYMBOL, RAMMARKET, SELF};
use eosio::*;
use eosio_cdt::*;
use lazy_static::lazy_static;

/// Defines new global state parameters.
/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L120-L145>
#[derive(Table, Read, Write, NumBytes)]
#[eosio(table_name = "global", singleton)]
pub struct EosioGlobalState {
    pub blockchain_parameters: BlockchainParameters,
    pub max_ram_size: u64,
    pub total_ram_bytes_reserved: u64,
    pub total_ram_stake: i64,
    pub last_producer_schedule_update: BlockTimestamp,
    pub last_pervote_bucket_fill: TimePoint,
    pub pervote_bucket: i64,
    pub perblock_bucket: i64,
    /// all blocks which have been produced but not paid
    pub total_unpaid_blocks: u32,
    pub total_activated_stake: i64,
    pub thresh_activated_stake_time: TimePoint,
    pub last_producer_schedule_size: u16,
    /// the sum of all producer votes
    pub total_producer_vote_weight: f64,
    pub last_name_close: BlockTimestamp,
}

impl Default for EosioGlobalState {
    fn default() -> Self {
        Self {
            blockchain_parameters: get_blockchain_parameters().expect("read"),
            max_ram_size: 64 * 1024 * 1024 * 1024,
            total_ram_bytes_reserved: 0,
            total_ram_stake: 0,
            last_producer_schedule_update: 0.into(),
            last_pervote_bucket_fill: 0.into(),
            pervote_bucket: 0,
            perblock_bucket: 0,
            total_unpaid_blocks: 0,
            total_activated_stake: 0,
            thresh_activated_stake_time: 0.into(),
            last_producer_schedule_size: 0,
            total_producer_vote_weight: 0.0,
            last_name_close: 0.into(),
        }
    }
}

/// Defines new global state parameters added after version 1.0
/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L147-L159>
#[derive(Table, Read, Write, NumBytes, Default)]
#[eosio(table_name = "global2", singleton)]
pub struct EosioGlobalState2 {
    pub new_ram_per_block: u16,
    pub last_ram_increase: BlockTimestamp,
    pub last_block_num: BlockTimestamp,
    pub total_producer_votepay_share: f64,
    pub revision: u8,
}

/// Defines new global state parameters added after version 1.3.0
/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L161-L168>
#[derive(Table, Read, Write, NumBytes)]
#[eosio(table_name = "global3", singleton)]
pub struct EosioGlobalState3 {
    pub last_vpay_state_update: TimePoint,
    pub total_vpay_share_change_rate: f64,
}

#[derive(Table, Read, Write, NumBytes)]
#[eosio(table_name = "global4", singleton)]
pub struct EosioGlobalState4 {
    pub continuous_rate: f64,
    pub inflation_pay_factor: i64,
    pub votepay_factor: i64,
}

lazy_static! {
    pub static ref GLOBAL: SingletonIndex<EosioGlobalState> =
        EosioGlobalState::singleton(*SELF, *SELF);
    pub static ref GLOBAL2: SingletonIndex<EosioGlobalState2> =
        EosioGlobalState2::singleton(*SELF, *SELF);
    pub static ref GLOBAL3: SingletonIndex<EosioGlobalState3> =
        EosioGlobalState3::singleton(*SELF, *SELF);
    pub static ref GLOBAL4: SingletonIndex<EosioGlobalState4> =
        EosioGlobalState4::singleton(*SELF, *SELF);
}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/eosio.system.cpp#L66-L84>
#[eosio::action]
pub fn setram(max_ram_size: u64) {
    require_auth(*SELF);

    let mut gstate = GLOBAL.get_or_default().expect("read");

    // decreasing ram might result market maker issues
    assert!(
        gstate.max_ram_size < max_ram_size,
        "ram may only be increased"
    );
    assert!(
        max_ram_size < 1024 * 1024 * 1024 * 1024 * 1024,
        "ram size is unrealistic"
    );
    assert!(
        max_ram_size > gstate.total_ram_bytes_reserved,
        "attempt to set max below reserved"
    );

    // Increase the amount of ram for sale based upon the change in max ram
    // size.
    let cursor = RAMMARKET
        .find(RAMCORE_SYMBOL)
        .expect("failed to find RAMCORE market");
    let mut market = cursor.get().expect("read");
    market.base.balance.amount +=
        max_ram_size.saturating_sub(gstate.max_ram_size) as i64;
    cursor.modify(Payer::Same, market).expect("write");

    gstate.max_ram_size = max_ram_size;
    GLOBAL.set(&gstate, *SELF).expect("write");
}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/eosio.system.cpp#L104-L109>
#[eosio::action]
pub fn setramrate(bytes_per_block: u16) {
    require_auth(*SELF);

    let mut gstate = GLOBAL.get_or_default().expect("read");
    let mut gstate2 = GLOBAL2.get_or_default().expect("read");

    let cbt = current_block_time();
    if cbt <= gstate2.last_ram_increase {
        return;
    }

    let cursor = RAMMARKET
        .find(RAMCORE_SYMBOL)
        .expect("failed to find RAMCORE market");
    let mut market = cursor.get().expect("read");
}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/eosio.system.cpp#L111-L116>
#[eosio::action]
pub fn setparams(params: BlockchainParameters) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/eosio.system.cpp#L118-L121>
#[eosio::action]
pub fn setpriv(account: AccountName, is_priv: u8) {}

/// Set account limits action sets the resource limits of an account
///
/// @param account - name of the account whose resource limit to be set,
/// @param ram_bytes - ram limit in absolute bytes,
/// @param net_weight - fractionally proportionate net limit of available
/// resources based on (weight / total_weight_of_all_accounts),
/// @param cpu_weight - fractionally proportionate cpu limit of available
/// resources based on (weight / total_weight_of_all_accounts).
///
/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/eosio.system.cpp#L123-L139>
#[eosio::action]
pub fn setalimits(
    account: AccountName,
    ram_bytes: i64,
    net_weight: i64,
    cpu_weight: i64,
) {
}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/eosio.system.cpp#L141-L184>
#[eosio::action]
pub fn setacctram(account: AccountName, ram_bytes: Option<i64>) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/eosio.system.cpp#L186-L228>
#[eosio::action]
pub fn setacctnet(account: AccountName, net_weight: Option<i64>) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/eosio.system.cpp#L230-L272>
#[eosio::action]
pub fn setacctcpu(account: AccountName, cpu_weight: Option<i64>) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/eosio.system.cpp#L274-L277>
#[eosio::action]
pub fn activate(feature_digest: Checksum256) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/eosio.system.cpp#L279-L286>
#[eosio::action]
pub fn rmvproducer(producer: AccountName) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/eosio.system.cpp#L288-L295>
#[eosio::action]
pub fn updtrevision(revision: u8) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/eosio.system.cpp#L297-L310>
#[eosio::action]
pub fn setinflation(
    annual_rate: i64,
    inflation_pay_factor: i64,
    votepay_factor: i64,
) {
}

/// The Init action initializes the system contract for a version and a symbol.
/// Only succeeds when:
/// - version is 0 and
/// - symbol is found and
/// - system token supply is greater than 0,
/// - and system contract wasn’t already been initialized.
///
/// @param version - the version, has to be 0,
/// @param core - the system symbol.
/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/eosio.system.cpp#L375-L397>
#[eosio::action]
pub fn init(version: UnsignedInt, core: Symbol) {}
