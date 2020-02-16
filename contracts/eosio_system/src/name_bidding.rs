use eosio::*;
use eosio_cdt::*;
use std::{collections::VecDeque, marker::PhantomData};

/// A name bid, which consists of:
/// - a `newname` name that the bid is for
/// - a `high_bidder` account name that is the one with the highest bid so far
/// - the `high_bid` which is amount of highest bid
/// - and `last_bid_time` which is the time of the highest bid
///
/// [Reference implementation](https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L90-L103)
pub struct NameBid {
    /// name that the bid is for
    pub newname: AccountName,
    /// account name that is the one with the highest bid so far
    pub high_bidder: AccountName,
    /// amount of highest bid. negative high_bid == closed auction waiting to
    /// be claimed
    pub high_bid: i64,
    /// time of the highest bid
    pub last_bid_time: TimePoint,
}

/// A bid refund, which is defined by:
/// - the `bidder` account name owning the refund
/// - the `amount` to be refunded
/// [Reference implementation](https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/include/eosio.system/eosio.system.hpp#L105-L113)
pub struct BidRefund {
    /// account name owning the refund
    pub bidder: AccountName,
    /// to be refunded
    pub amount: Asset,
}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/name_bidding.cpp#L11-L68>
#[eosio::action]
pub fn bidname(bidder: AccountName, newname: AccountName, bid: Asset) {}

/// <https://github.com/EOSIO/eosio.contracts/blob/v1.9.0-rc3/contracts/eosio.system/src/name_bidding.cpp#L70-L78>
#[eosio::action]
pub fn bidrefund(bidder: AccountName, newname: AccountName) {}
