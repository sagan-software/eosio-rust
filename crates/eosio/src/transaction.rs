//! TODO docs
use crate::action::Action;
use crate::bytes::{NumBytes, Read, Write};
use crate::time::TimePointSec;
use crate::varint::UnsignedInt;
use serde::{Deserialize, Serialize};

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
    Serialize,
    Deserialize,
)]
#[__eosio_path = "crate::bytes"]
pub struct Extension(u16, Vec<char>);

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
    Serialize,
    Deserialize,
)]
#[__eosio_path = "crate::bytes"]
pub struct TransactionHeader {
    /// TODO docs
    pub expiration: TimePointSec,
    /// TODO docs
    pub ref_block_num: u16,
    /// TODO docs
    pub ref_block_prefix: u32,
    /// number of 8 byte words this transaction can serialize into after compressions
    pub max_net_usage_words: UnsignedInt,
    /// number of CPU usage units to bill transaction for
    pub max_cpu_usage_ms: u8,
    /// number of seconds to delay transaction, default: 0
    pub delay_sec: UnsignedInt,
}

/// TODO docs
#[derive(
    Clone, Debug, Serialize, Deserialize, Read, Write, NumBytes, Default,
)]
#[__eosio_path = "crate::bytes"]
pub struct Transaction<T: Default + Clone> {
    /// TODO docs
    pub header: TransactionHeader,
    /// TODO docs
    pub context_free_actions: Vec<Action<T>>,
    /// TODO docs
    pub actions: Vec<Action<T>>,
    /// TODO docs
    pub transaction_extensions: Vec<Extension>,
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
