use super::TimePoint;
use crate::bytes::{NumBytes, Read, Write};
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
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
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
