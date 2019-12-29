use eosio::*;

// TODO this relies on eosio_bios::BlockHeader
// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/producer_pay.cpp#L10-L66>
// pub fn onblock(header: PhantomData<BlockHeader>) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/producer_pay.cpp#L68-L189>
#[eosio::action]
pub fn claimrewards(owner: AccountName) {}
