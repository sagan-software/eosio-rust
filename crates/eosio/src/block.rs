//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/time.hpp#L134-L210>
use crate::bytes::{NumBytes, Read, Write};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::num::{NonZeroU64, ParseIntError};
use std::str::FromStr;

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
pub struct BlockId(String);

impl fmt::Display for BlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
    Copy,
    Hash,
    Serialize,
    Deserialize,
)]
#[__eosio_path = "crate::bytes"]
pub struct BlockNum(NonZeroU64);

impl fmt::Display for BlockNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for BlockNum {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<NonZeroU64>().map(Self)
    }
}

/// TODO docs
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Serialize, Deserialize,
)]
#[serde(untagged)]
pub enum BlockNumOrId {
    /// TODO docs
    Id(BlockId),
    /// TODO docs
    Num(BlockNum),
}

impl FromStr for BlockNumOrId {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<BlockNum>() {
            Ok(num) => Ok(Self::Num(num)),
            Err(_) => Ok(Self::Id(BlockId(s.to_owned()))),
        }
    }
}

impl fmt::Display for BlockNumOrId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(num) => write!(f, "{}", num),
            Self::Id(id) => write!(f, "{}", id),
        }
    }
}

/// This class is used in the block headers to represent the block time
/// It is a parameterised class that takes an Epoch in milliseconds and
/// and an interval in milliseconds and computes the number of slots.
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
    Copy,
    Hash,
    Default,
    Serialize,
)]
#[__eosio_path = "crate::bytes"]
pub struct BlockTimestamp(u32);

impl BlockTimestamp {
    /// Time between blocks.
    pub const BLOCK_INTERVAL_MS: i32 = 500;
    /// Epoch is 2000-01-01T00:00.000Z.
    pub const BLOCK_TIMESTAMP_EPOCH: i64 = 946_684_800_000;

    /// Gets the milliseconds
    #[inline]
    pub const fn as_u32(self) -> u32 {
        self.0
    }
}

/// TODO docs
struct BlockTimestampVisitor;

impl<'de> ::serde::de::Visitor<'de> for BlockTimestampVisitor {
    type Value = BlockTimestamp;

    #[inline]
    fn expecting(
        &self,
        formatter: &mut ::std::fmt::Formatter,
    ) -> ::std::fmt::Result {
        formatter.write_str("a second timestamp as a number or string")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        match value.parse::<u32>() {
            Ok(n) => Ok(BlockTimestamp(n)),
            Err(e) => Err(::serde::de::Error::custom(e)),
        }
    }

    #[inline]
    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        Ok(BlockTimestamp(value))
    }
}

impl<'de> ::serde::de::Deserialize<'de> for BlockTimestamp {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_any(BlockTimestampVisitor)
    }
}

impl From<u32> for BlockTimestamp {
    #[inline]
    fn from(i: u32) -> Self {
        Self(i)
    }
}

impl From<BlockTimestamp> for u32 {
    #[inline]
    fn from(t: BlockTimestamp) -> Self {
        t.0
    }
}
