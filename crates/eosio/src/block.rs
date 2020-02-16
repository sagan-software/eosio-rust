//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/time.hpp#L134-L210>
use crate::bytes::{NumBytes, Read, Write};
use alloc::string::{String, ToString};
use core::{
    fmt,
    num::{NonZeroU64, ParseIntError},
    str::FromStr,
};

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
)]
#[eosio(crate_path = "crate::bytes")]
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash)]
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
            Err(_) => Ok(Self::Id(BlockId(s.to_string()))),
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
