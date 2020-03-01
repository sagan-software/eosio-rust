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
    slot: u32,
}

impl BlockTimestamp {
    /// Epoch is 2000-01-01T00:00.000Z.
    pub const EPOCH: i64 = 946_684_800_000;
    /// Milliseconds between blocks.
    pub const INTERVAL_MS: i32 = 500;
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
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn from(tp: TimePoint) -> Self {
        let millis = tp.as_millis() - Self::EPOCH;
        let slot = millis.max(0) / i64::from(Self::INTERVAL_MS);
        let slot = slot as u32;
        Self { slot }
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

impl From<TimePointSec> for BlockTimestamp {
    #[inline]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn from(tps: TimePointSec) -> Self {
        let millis = i64::from(tps.as_secs()) * 1_000 - Self::EPOCH;
        let slot = millis / i64::from(Self::INTERVAL_MS);
        let slot = slot as u32;
        Self { slot }
    }
}

impl From<BlockTimestamp> for TimePointSec {
    #[inline]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn from(bt: BlockTimestamp) -> Self {
        let millis = i64::from(bt.slot)
            * i64::from(BlockTimestamp::INTERVAL_MS)
            + BlockTimestamp::EPOCH;
        let secs = millis / 1_000;
        Self::from_secs(secs as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockTimestamp, TimePoint, TimePointSec};

    #[test]
    fn from_time_point() {
        let test_cases = vec![
            (0, 0),
            (BlockTimestamp::EPOCH, 0),
            (
                BlockTimestamp::EPOCH
                    + i64::from(BlockTimestamp::INTERVAL_MS) * 10,
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

    #[test]
    fn to_time_point() {
        let test_cases = vec![
            (0, BlockTimestamp::EPOCH),
            (
                1,
                BlockTimestamp::EPOCH + i64::from(BlockTimestamp::INTERVAL_MS),
            ),
            (
                10,
                BlockTimestamp::EPOCH
                    + i64::from(BlockTimestamp::INTERVAL_MS) * 10,
            ),
        ];
        for (slot, millis) in test_cases {
            let bt = BlockTimestamp { slot };
            assert_eq!(TimePoint::from_millis(millis), TimePoint::from(bt));
        }
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    #[test]
    fn to_time_point_sec() {
        let test_cases = vec![
            (0, BlockTimestamp::EPOCH),
            (
                1,
                BlockTimestamp::EPOCH + i64::from(BlockTimestamp::INTERVAL_MS),
            ),
            (
                10,
                BlockTimestamp::EPOCH
                    + i64::from(BlockTimestamp::INTERVAL_MS) * 10,
            ),
        ];
        for (slot, millis) in test_cases {
            let bt = BlockTimestamp { slot };
            let secs = millis / 1_000;
            let secs = secs as u32;
            assert_eq!(TimePointSec::from_secs(secs), TimePointSec::from(bt));
        }
    }
}
