use eosio_bytes::*;
use serde::Serialize;

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
    /// One microsecond
    const MICROSECOND: i64 = 1;
    /// One millisecond
    const MILLISECOND: i64 = Self::MICROSECOND * 1_000;
    /// One second
    const SECOND: i64 = Self::MILLISECOND * 1_000;
    /// One minute
    const MINUTE: i64 = Self::SECOND * 60;
    /// One hour
    const HOUR: i64 = Self::MINUTE * 60;
    /// One day
    const DAY: i64 = Self::HOUR * 24;

    pub const UNIX_EPOCH: Self = TimePoint(0);

    /// Creates a `TimePoint` from microseconds
    #[inline]
    pub const fn from_micros(micros: i64) -> Self {
        Self(micros)
    }

    /// Creates a `TimePoint` from milliseconds
    #[inline]
    pub fn from_millis(millis: i64) -> Self {
        Self(millis.saturating_mul(Self::MILLISECOND))
    }

    /// Creates a `TimePoint` from seconds
    #[inline]
    pub fn from_secs(secs: i64) -> Self {
        Self(secs.saturating_mul(Self::SECOND))
    }

    /// Creates a `TimePoint` from minutes
    #[inline]
    pub fn from_mins(mins: i64) -> Self {
        Self(mins.saturating_mul(Self::MINUTE))
    }

    /// Creates a `TimePoint` from hours
    #[inline]
    pub fn from_hours(hours: i64) -> Self {
        Self(hours.saturating_mul(Self::HOUR))
    }

    /// Creates a `TimePoint` from days
    #[inline]
    pub fn from_days(days: i64) -> Self {
        Self(days.saturating_mul(Self::DAY))
    }

    /// Gets the nanoseconds
    #[inline]
    pub const fn as_micros(self) -> i64 {
        self.0
    }

    /// Gets the milliseconds
    #[inline]
    pub const fn as_millis(self) -> i64 {
        self.0 / Self::MILLISECOND
    }

    /// Gets the seconds
    #[inline]
    pub const fn as_secs(self) -> i64 {
        self.0 / Self::SECOND
    }

    /// Gets the minutes
    #[inline]
    pub const fn as_mins(self) -> i64 {
        self.0 / Self::MINUTE
    }

    /// Gets the hours
    #[inline]
    pub const fn as_hours(self) -> i64 {
        self.0 / Self::HOUR
    }

    /// Gets the days
    #[inline]
    pub const fn as_days(self) -> i64 {
        self.0 / Self::DAY
    }

    /// Gets the max `TimePoint` of two values
    #[inline]
    pub fn max(self, other: Self) -> Self {
        if self >= other {
            self
        } else {
            other
        }
    }

    /// Gets the min `TimePoint` of two values
    #[inline]
    pub fn min(self, other: Self) -> Self {
        if self <= other {
            self
        } else {
            other
        }
    }
}

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

impl From<u64> for TimePoint {
    #[inline]
    fn from(i: u64) -> Self {
        Self(i as i64)
    }
}

impl From<TimePoint> for u64 {
    #[inline]
    fn from(t: TimePoint) -> Self {
        t.0 as Self
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
