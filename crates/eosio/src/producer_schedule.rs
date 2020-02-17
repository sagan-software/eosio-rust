//! <https://github.com/EOSIO/eosio.cdt/blob/796ff8bee9a0fc864f665a0a4d018e0ff18ac383/libraries/eosiolib/contracts/eosio/producer_schedule.hpp#L54-L69>
use crate::{AccountName, NumBytes, PublicKey, Read, Write};
use alloc::{vec, vec::Vec};

/// Maps producer with its signing key, used for producer schedule
/// <https://github.com/EOSIO/eosio.cdt/blob/796ff8bee9a0fc864f665a0a4d018e0ff18ac383/libraries/eosiolib/contracts/eosio/producer_schedule.hpp#L15-L45>
#[derive(Read, Write, NumBytes, Clone, Default, Debug)]
#[eosio(crate_path = "crate::bytes")]
pub struct ProducerKey {
    /// Name of the producer
    pub producer_name: AccountName,
    /// Block signing key used by this producer
    pub block_signing_key: PublicKey,
}

/// Defines both the order, account name, and signing keys of the active set
/// of producers.
#[derive(Read, Write, NumBytes, Clone, Default, Debug)]
#[eosio(crate_path = "crate::bytes")]
pub struct ProducerSchedule {
    /// Version number of the schedule. It is sequentially incrementing
    /// version number.
    pub version: u32,
    /// List of producers for this schedule, including its signing key
    pub producers: Vec<ProducerKey>,
}

/// pairs a public key with an integer weight
#[derive(Read, Write, NumBytes, Clone, Default, Debug)]
#[eosio(crate_path = "crate::bytes")]
pub struct KeyWeight {
    /// public key used in a weighted threshold multi-sig authority
    pub key: PublicKey,
    /// weight associated with a signature from the private key associated with
    /// the accompanying public key
    pub weight: u64,
}

impl From<PublicKey> for KeyWeight {
    fn from(key: PublicKey) -> Self {
        Self { key, weight: 1 }
    }
}

/// block signing authority version 0
/// this authority allows for a weighted threshold multi-sig per-producer
#[derive(Read, Write, NumBytes, Clone, Default, Debug)]
#[eosio(crate_path = "crate::bytes")]
pub struct BlockSigningAuthority {
    /// minimum threshold of accumulated weights from component keys that
    /// satisfies this authority
    pub threshold: u32,
    /// component keys and their associated weights
    pub keys: Vec<KeyWeight>,
}

impl From<PublicKey> for BlockSigningAuthority {
    #[inline]
    fn from(key: PublicKey) -> Self {
        Self {
            threshold: 1,
            keys: vec![key.into()],
        }
    }
}

/// Maps producer with its signing key, used for producer schedule
#[derive(Read, Write, NumBytes, Clone, Default, Debug)]
#[eosio(crate_path = "crate::bytes")]
pub struct ProducerAuthority {
    /// Name of the producer
    pub producer_name: AccountName,
    /// The block signing authority used by this producer
    pub authority: BlockSigningAuthority,
}
