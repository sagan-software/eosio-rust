use super::TimePointSec;
use crate::bytes::{NumBytes, Read, Write};
use core::{
    convert::{TryFrom, TryInto},
    num::TryFromIntError,
};

/// High resolution time point in microseconds
/// <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/time.hpp#L49-L77>
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
pub struct TimePoint(i64);

impl TimePoint {
    #[inline]
    #[must_use]
    pub const fn from_micros(micros: i64) -> Self {
        Self(micros)
    }

    #[inline]
    #[must_use]
    pub const fn from_millis(millis: i64) -> Self {
        Self::from_micros(millis * 1_000)
    }

    /// Gets the microseconds
    #[inline]
    #[must_use]
    pub const fn as_micros(&self) -> i64 {
        self.0
    }

    /// Gets the milliseconds
    #[inline]
    #[must_use]
    pub const fn as_millis(&self) -> i64 {
        self.0 / 1_000
    }

    #[inline]
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn as_secs(&self) -> i32 {
        (self.0 / 1_000_000) as i32
    }

    #[inline]
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub const fn as_time_point_sec(&self) -> TimePointSec {
        TimePointSec::from_secs(self.as_secs() as u32)
    }
}

impl From<i64> for TimePoint {
    #[inline]
    #[must_use]
    fn from(i: i64) -> Self {
        Self(i)
    }
}

impl From<TimePoint> for i64 {
    #[inline]
    #[must_use]
    fn from(t: TimePoint) -> Self {
        t.0
    }
}

impl TryFrom<u64> for TimePoint {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(i: u64) -> Result<Self, Self::Error> {
        Ok(i64::try_from(i)?.into())
    }
}

impl TryFrom<TimePoint> for u64 {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(t: TimePoint) -> Result<Self, Self::Error> {
        t.as_micros().try_into()
    }
}
