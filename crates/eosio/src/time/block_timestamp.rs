use crate::{
    time::{TimePoint, TimePointSec},
    NumBytes, Read, Write,
};

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
)]
#[eosio(crate_path = "crate::bytes")]
pub struct BlockTimestamp {
    pub slot: u32,
}

impl BlockTimestamp {
    /// Milliseconds between blocks.
    pub const INTERVAL_MS: i32 = 500;
    /// Epoch is 2000-01-01T00:00.000Z.
    pub const EPOCH: i64 = 946_684_800_000;
}

impl From<u32> for BlockTimestamp {
    #[inline]
    #[must_use]
    fn from(i: u32) -> Self {
        Self { slot: i }
    }
}

impl From<BlockTimestamp> for u32 {
    #[inline]
    #[must_use]
    fn from(t: BlockTimestamp) -> Self {
        t.slot
    }
}

impl From<TimePoint> for BlockTimestamp {
    #[inline]
    fn from(tp: TimePoint) -> Self {
        let millis = tp.as_millis() - Self::EPOCH;
        let slot = millis.max(0) / i64::from(Self::INTERVAL_MS);
        let slot = slot as u32;
        Self { slot }
    }
}

#[cfg(test)]
#[test]
fn test_from_time_point() {
    let test_cases = vec![
        (0, 0),
        (BlockTimestamp::EPOCH, 0),
        (
            BlockTimestamp::EPOCH + i64::from(BlockTimestamp::INTERVAL_MS) * 10,
            10,
        ),
        (
            BlockTimestamp::EPOCH
                + i64::from(BlockTimestamp::INTERVAL_MS) * 100,
            100,
        ),
        (
            BlockTimestamp::EPOCH
                + i64::from(BlockTimestamp::INTERVAL_MS) * 10
                + 255,
            10,
        ),
    ];
    for (millis, slot) in test_cases {
        let tp = TimePoint::from_millis(millis);
        assert_eq!(BlockTimestamp { slot }, BlockTimestamp::from(tp));
    }
}

impl From<BlockTimestamp> for TimePoint {
    #[inline]
    fn from(bt: BlockTimestamp) -> Self {
        let millis = i64::from(bt.slot)
            * i64::from(BlockTimestamp::INTERVAL_MS)
            + BlockTimestamp::EPOCH;
        Self::from_millis(millis)
    }
}

#[cfg(test)]
#[test]
fn test_to_time_point() {
    let test_cases = vec![
        (0, BlockTimestamp::EPOCH),
        (
            1,
            BlockTimestamp::EPOCH + i64::from(BlockTimestamp::INTERVAL_MS),
        ),
        (
            10,
            BlockTimestamp::EPOCH + i64::from(BlockTimestamp::INTERVAL_MS) * 10,
        ),
    ];
    for (slot, millis) in test_cases {
        let bt = BlockTimestamp { slot };
        assert_eq!(TimePoint::from_millis(millis), TimePoint::from(bt));
    }
}

impl From<TimePointSec> for BlockTimestamp {
    #[inline]
    fn from(tps: TimePointSec) -> Self {
        let millis = i64::from(tps.as_secs()) * 1_000 - Self::EPOCH;
        let slot = millis / i64::from(Self::INTERVAL_MS);
        let slot = slot as u32;
        Self { slot }
    }
}

#[cfg(test)]
#[test]
fn test_from_time_point_sec() {}

impl From<BlockTimestamp> for TimePointSec {
    #[inline]
    fn from(bt: BlockTimestamp) -> Self {
        let millis = i64::from(bt.slot)
            * i64::from(BlockTimestamp::INTERVAL_MS)
            + BlockTimestamp::EPOCH;
        let secs = millis * 1_000;
        Self::from_secs(secs as u32)
    }
}

#[cfg(test)]
#[test]
fn test_to_time_point_sec() {
    let test_cases = vec![
        (0, BlockTimestamp::EPOCH),
        (
            1,
            BlockTimestamp::EPOCH + i64::from(BlockTimestamp::INTERVAL_MS),
        ),
        (
            10,
            BlockTimestamp::EPOCH + i64::from(BlockTimestamp::INTERVAL_MS) * 10,
        ),
    ];
    for (slot, millis) in test_cases {
        let bt = BlockTimestamp { slot };
        let secs = millis / 1_000;
        let secs = secs as u32;
        assert_eq!(TimePointSec::from_secs(secs), TimePointSec::from(bt));
    }
}

// TODO docs
// struct BlockTimestampVisitor;

// impl<'de> ::serde::de::Visitor<'de> for BlockTimestampVisitor {
//     type Value = BlockTimestamp;

//     #[inline]
//     fn expecting(
//         &self,
//         formatter: &mut ::std::fmt::Formatter,
//     ) -> ::std::fmt::Result {
//         formatter.write_str("a second timestamp as a number or string")
//     }

//     #[inline]
//     fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
//     where
//         E: ::serde::de::Error,
//     {
//         match value.parse::<u32>() {
//             Ok(n) => Ok(BlockTimestamp(n)),
//             Err(e) => Err(::serde::de::Error::custom(e)),
//         }
//     }

//     #[inline]
//     fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
//     where
//         E: ::serde::de::Error,
//     {
//         Ok(BlockTimestamp(value))
//     }
// }

// impl<'de> ::serde::de::Deserialize<'de> for BlockTimestamp {
//     #[inline]
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: ::serde::de::Deserializer<'de>,
//     {
//         deserializer.deserialize_any(BlockTimestampVisitor)
//     }
// }
