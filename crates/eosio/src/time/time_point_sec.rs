use super::TimePoint;
use crate::bytes::{NumBytes, Read, Write};
use core::convert::TryInto;
use core::fmt;
use core::ops::Add;

/// A lower resolution `TimePoint` accurate only to seconds from 1970
/// <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/time.hpp#L79-L132>
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
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[eosio(crate_path = "crate::bytes")]
pub struct TimePointSec(u32);

impl TimePointSec {
    /// Create a new `TimePointSec`
    #[inline]
    #[must_use]
    pub const fn from_secs(secs: u32) -> Self {
        Self(secs)
    }

    /// Gets the seconds
    #[inline]
    #[must_use]
    pub const fn as_secs(self) -> u32 {
        self.0
    }
}

#[cfg(feature = "serde")]
struct TimePointSecVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for TimePointSecVisitor {
    type Value = TimePointSec;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a second timestamp as a number or string")
    }

    #[cfg(feature = "chrono")]
    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value.parse::<chrono::NaiveDateTime>() {
            Ok(n) => n
                .timestamp()
                .try_into()
                .map(TimePointSec)
                .map_err(serde::de::Error::custom),
            Err(e) => Err(serde::de::Error::custom(e)),
        }
    }

    #[inline]
    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(TimePointSec(value))
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::de::Deserialize<'de> for TimePointSec {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_any(TimePointSecVisitor)
    }
}

impl From<u32> for TimePointSec {
    #[inline]
    #[must_use]
    fn from(i: u32) -> Self {
        Self(i)
    }
}

impl From<TimePointSec> for u32 {
    #[inline]
    #[must_use]
    fn from(t: TimePointSec) -> Self {
        t.0
    }
}

impl From<TimePoint> for TimePointSec {
    #[inline]
    #[must_use]
    fn from(t: TimePoint) -> Self {
        Self((t.as_micros() as u32) / 1_000_000_u32)
    }
}

impl Add<u32> for TimePointSec {
    type Output = Self;

    #[must_use]
    fn add(self, rhs: u32) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Add<TimePointSec> for u32 {
    type Output = TimePointSec;

    #[must_use]
    fn add(self, rhs: TimePointSec) -> Self::Output {
        TimePointSec(rhs.0 + self)
    }
}
