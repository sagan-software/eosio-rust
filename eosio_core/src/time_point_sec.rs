//! TODO docs
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
pub struct TimePointSec(u32);

impl TimePointSec {
    /// Gets the nanoseconds
    #[inline]
    pub const fn as_u32(self) -> u32 {
        self.0
    }
}

/// TODO docs
struct TimePointSecVisitor;

impl<'de> ::serde::de::Visitor<'de> for TimePointSecVisitor {
    type Value = TimePointSec;

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
            Ok(n) => Ok(TimePointSec(n)),
            Err(e) => Err(::serde::de::Error::custom(e)),
        }
    }

    #[inline]
    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        Ok(TimePointSec(value))
    }
}

impl<'de> ::serde::de::Deserialize<'de> for TimePointSec {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_any(TimePointSecVisitor)
    }
}

impl From<u32> for TimePointSec {
    #[inline]
    fn from(i: u32) -> Self {
        Self(i)
    }
}

impl From<TimePointSec> for u32 {
    #[inline]
    fn from(t: TimePointSec) -> Self {
        t.0
    }
}
