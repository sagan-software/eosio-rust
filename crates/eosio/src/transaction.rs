//! TODO docs
use crate::{
    action::Action,
    bytes::{NumBytes, Read, Write},
    time::TimePointSec,
    varint::UnsignedInt,
};
use alloc::vec::Vec;

/// TODO docs
#[derive(
    Read,
    Write,
    NumBytes,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
    Clone,
    Hash,
    Default,
)]
#[eosio(crate_path = "crate::bytes")]
pub struct TransactionExtension(u16, Vec<char>);

/// TODO docs
#[derive(
    Read,
    Write,
    NumBytes,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
    Clone,
    Hash,
    Default,
)]
#[eosio(crate_path = "crate::bytes")]
pub struct TransactionHeader {
    /// TODO docs
    pub expiration: TimePointSec,
    /// TODO docs
    pub ref_block_num: u16,
    /// TODO docs
    pub ref_block_prefix: u32,
    /// number of 8 byte words this transaction can serialize into after
    /// compressions
    pub max_net_usage_words: UnsignedInt,
    /// number of CPU usage units to bill transaction for
    pub max_cpu_usage_ms: u8,
    /// number of seconds to delay transaction, default: 0
    pub delay_sec: UnsignedInt,
}

/// TODO docs
#[derive(Clone, Debug, Read, Write, NumBytes, Default)]
#[eosio(crate_path = "crate::bytes")]
pub struct Transaction<T: Default + Clone = Vec<u8>> {
    /// TODO docs
    pub header: TransactionHeader,
    /// TODO docs
    pub context_free_actions: Vec<Action<T>>,
    /// TODO docs
    pub actions: Vec<Action<T>>,
    /// TODO docs
    pub transaction_extensions: Vec<TransactionExtension>,
}

/// TODO docs
/// TODO represet this as a String for RPC
#[derive(Clone, Debug)]
pub struct TransactionId(u128);

impl TransactionId {
    /// TODO docs
    #[must_use]
    pub const fn as_u128(&self) -> u128 {
        self.0
    }
}

impl From<u128> for TransactionId {
    #[must_use]
    fn from(value: u128) -> Self {
        Self(value)
    }
}

impl AsRef<TransactionId> for TransactionId {
    fn as_ref(&self) -> &Self {
        self
    }
}
