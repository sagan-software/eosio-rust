use super::TimePointSec;
use crate::bytes::{NumBytes, Read, Write};
use serde::Serialize;
use std::convert::{TryFrom, TryInto};

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
    Serialize,
)]
#[__eosio_path = "crate::bytes"]
pub struct TimePoint(i64);

impl TimePoint {
    #[inline]
    pub const fn from_micros(micros: i64) -> Self {
        Self(micros)
    }

    /// Gets the microseconds
    #[inline]
    pub const fn as_micros(&self) -> i64 {
        self.0
    }

    #[inline]
    pub const fn as_secs(&self) -> i32 {
        (self.0 / 1_000_000) as i32
    }

    #[inline]
    pub const fn as_time_point_sec(&self) -> TimePointSec {
        TimePointSec::from_secs(self.as_secs() as u32)
    }
}

/// TODO docs
struct TimePointVisitor;

impl<'de> ::serde::de::Visitor<'de> for TimePointVisitor {
    type Value = TimePoint;

    #[inline]
    fn expecting(
        &self,
        formatter: &mut ::std::fmt::Formatter,
    ) -> ::std::fmt::Result {
        formatter.write_str("a microsecond timestamp as a number or string")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        match value.parse::<chrono::NaiveDateTime>() {
            Ok(n) => Ok(TimePoint(n.timestamp_nanos() / 1000)),
            Err(e) => Err(::serde::de::Error::custom(e)),
        }
    }

    #[inline]
    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        Ok(TimePoint(value))
    }
}

impl<'de> ::serde::de::Deserialize<'de> for TimePoint {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_any(TimePointVisitor)
    }
}

impl From<i64> for TimePoint {
    #[inline]
    fn from(i: i64) -> Self {
        Self(i)
    }
}

impl From<TimePoint> for i64 {
    #[inline]
    fn from(t: TimePoint) -> Self {
        t.0
    }
}

impl TryFrom<u64> for TimePoint {
    type Error = std::num::TryFromIntError;
    #[inline]
    fn try_from(i: u64) -> Result<Self, Self::Error> {
        Ok(i64::try_from(i)?.into())
    }
}

impl TryFrom<TimePoint> for u64 {
    type Error = std::num::TryFromIntError;
    #[inline]
    fn try_from(t: TimePoint) -> Result<Self, Self::Error> {
        t.as_micros().try_into()
    }
}