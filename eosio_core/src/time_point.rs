//! TODO docs
use eosio_bytes::*;
use serde::Serialize;
use std::convert::{TryFrom, TryInto};

/// Time relative to unix epoch
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
#[eosio_bytes_root_path = "::eosio_bytes"]
pub struct TimePoint(i64);

impl TimePoint {
    /// Gets the nanoseconds
    #[inline]
    pub const fn as_i64(self) -> i64 {
        self.0
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
        match value.parse::<i64>() {
            Ok(n) => Ok(TimePoint(n)),
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
        t.as_i64().try_into()
    }
}

// // TODO: Duration ops similar to std::time::Duration

// #[derive(
//     Read, Write, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash,
// )]
// #[eosio_bytes_root_path = "::eosio_bytes"]
// pub struct Duration(i64);

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_constructors() {
//         assert_eq!(Time::UNIX_EPOCH.as_micros(), 0);
//         assert_eq!(Time::from_micros(1).as_micros(), 1);
//         assert_eq!(Time::from_millis(1).as_micros(), 1_000);
//         assert_eq!(Time::from_secs(1).as_micros(), 1_000_000);
//         assert_eq!(Time::from_mins(1).as_micros(), 60_000_000);
//         assert_eq!(Time::from_hours(1).as_micros(), 3_600_000_000);
//         assert_eq!(Time::from_days(1).as_micros(), 86_400_000_000);
//     }

//     #[test]
//     fn test_converters() {
//         assert_eq!(Time::from_millis(1).as_micros(), 1_000);
//         assert_eq!(Time::from_secs(1).as_millis(), 1_000);
//         assert_eq!(Time::from_mins(1).as_secs(), 60);
//         assert_eq!(Time::from_hours(1).as_mins(), 60);
//         assert_eq!(Time::from_days(1).as_hours(), 24);
//     }

//     #[test]
//     fn test_min_max() {
//         let t1 = Time::from_secs(1);
//         let t2 = Time::from_secs(2);
//         let t3 = Time::from_secs(3);
//         assert_eq!(t1.max(t2), t2);
//         assert_eq!(t1.min(t2), t1);
//         assert_eq!(t3.max(t2), t3);
//         assert_eq!(t3.min(t2), t2);
//     }

// }
