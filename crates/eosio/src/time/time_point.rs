use super::{BlockTimestamp, TimePointSec};
use crate::bytes::{NumBytes, Read, Write};
use core::convert::{TryFrom, TryInto};
use core::fmt;
use core::num::TryFromIntError;

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
    pub const fn as_secs(&self) -> i32 {
        (self.0 / 1_000_000) as i32
    }

    #[inline]
    #[must_use]
    pub const fn as_time_point_sec(&self) -> TimePointSec {
        TimePointSec::from_secs(self.as_secs() as u32)
    }

    #[inline]
    #[must_use]
    pub const fn as_block_timestamp(&self) -> BlockTimestamp {
        BlockTimestamp::new(self.as_millis() as u32)
    }
}

#[cfg(feature = "serde")]
struct TimePointVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for TimePointVisitor {
    type Value = TimePoint;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a microsecond timestamp as a number or string")
    }

    #[cfg(feature = "chrono")]
    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value.parse::<chrono::NaiveDateTime>() {
            Ok(n) => Ok(TimePoint(n.timestamp_nanos() / 1000)),
            Err(e) => Err(serde::de::Error::custom(e)),
        }
    }

    #[inline]
    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(TimePoint(value))
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::de::Deserialize<'de> for TimePoint {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_any(TimePointVisitor)
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
