// #[cfg(feature = "contract")]
// use crate::assert::*;
use eosio_macros::*;

/// Time relative to unix epoch
#[derive(
    Read, Write, NumBytes, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash, Default,
)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct Time(i64);

impl Time {
    /// One microsecond
    pub const MICROSECOND: i64 = 1;
    /// One millisecond
    pub const MILLISECOND: i64 = Self::MICROSECOND * 1_000;
    /// One second
    pub const SECOND: i64 = Self::MILLISECOND * 1_000;
    /// One minute
    pub const MINUTE: i64 = Self::SECOND * 60;
    /// One hour
    pub const HOUR: i64 = Self::MINUTE * 60;
    /// One day
    pub const DAY: i64 = Self::HOUR * 24;

    /// Gets the current time
    #[cfg(feature = "contract")]
    #[inline]
    pub fn now() -> Self {
        Time(unsafe { ::eosio_sys::current_time() } as i64)
    }

    /// Gets the current time
    #[cfg(feature = "stdweb")]
    #[inline]
    pub fn now() -> Self {
        let microseconds = ::stdweb::web::Date::now() * 1_000.0;
        Time(microseconds as i64)
    }

    /// Gets the current time
    #[cfg(feature = "js-sys")]
    #[inline]
    pub fn now() -> Self {
        let microseconds = ::js_sys::Date::now() * 1_000.0;
        Time(microseconds as i64)
    }

    /// Gets the publication time
    #[cfg(feature = "contract")]
    #[inline]
    pub fn publication() -> Self {
        Time(unsafe { ::eosio_sys::publication_time() } as i64)
    }

    /// Gets the expiration time
    #[cfg(feature = "contract")]
    #[inline]
    pub fn expiration() -> Self {
        let seconds = unsafe { ::eosio_sys::expiration() };
        Self::from_seconds(seconds as i32)
    }

    /// Gets the zero time
    #[inline]
    pub fn zero() -> Self {
        Time(0)
    }

    /// Returns true if 0
    #[inline]
    pub fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Gets the microseconds
    #[inline]
    pub fn microseconds(self) -> i64 {
        self.0
    }

    /// Creates a `Time` from microseconds
    #[inline]
    pub fn from_microseconds(microseconds: i64) -> Self {
        Time(microseconds)
    }

    /// Gets the milliseconds
    #[inline]
    pub fn milliseconds(self) -> i64 {
        self.0 / Self::MILLISECOND
    }

    /// Creates a `Time` from milliseconds
    #[inline]
    pub fn from_milliseconds(milliseconds: i64) -> Self {
        Time(milliseconds.saturating_mul(Self::MILLISECOND))
    }

    /// Gets the seconds
    #[inline]
    pub fn seconds(self) -> i32 {
        (self.0 / Self::SECOND) as i32
    }

    /// Creates a `Time` from seconds
    #[inline]
    pub fn from_seconds(seconds: i32) -> Self {
        Time(i64::from(seconds).saturating_mul(Self::SECOND))
    }

    /// Gets the minutes
    #[inline]
    pub fn minutes(self) -> i32 {
        (self.0 / Self::MINUTE) as i32
    }

    /// Creates a `Time` from minutes
    #[inline]
    pub fn from_minutes(minutes: i32) -> Self {
        Time(i64::from(minutes).saturating_mul(Self::MINUTE))
    }

    /// Gets the hours
    #[inline]
    pub fn hours(self) -> i32 {
        (self.0 / Self::HOUR) as i32
    }

    /// Creates a `Time` from hours
    #[inline]
    pub fn from_hours(hours: i32) -> Self {
        Time(i64::from(hours).saturating_mul(Self::HOUR))
    }

    /// Gets the days
    #[inline]
    pub fn days(self) -> i32 {
        (self.0 / Self::HOUR) as i32
    }

    /// Creates a `Time` from days
    #[inline]
    pub fn from_days(days: i32) -> Self {
        Time(i64::from(days).saturating_mul(Self::DAY))
    }

    /// Gets the max `Time` of two values
    #[inline]
    pub fn max(self, other: Self) -> Self {
        if self >= other {
            self
        } else {
            other
        }
    }

    /// Gets the min `Time` of two values
    #[inline]
    pub fn min(self, other: Self) -> Self {
        if self <= other {
            self
        } else {
            other
        }
    }
}

#[cfg(feature = "contract")]
impl crate::print::Print for Time {
    #[inline]
    fn print(&self) {
        "Time(".print();
        self.0.print();
        ")".print();
    }
}

#[cfg(feature = "serde")]
struct TimeVisitor;

#[cfg(feature = "serde")]
impl<'de> ::serde::de::Visitor<'de> for TimeVisitor {
    type Value = Time;

    #[inline]
    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter.write_str("a microsecond timestamp as a number or string")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        match value.parse::<i64>() {
            Ok(n) => Ok(Time(n)),
            Err(e) => Err(::serde::de::Error::custom(e)),
        }
    }

    #[inline]
    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        Ok(Time(value))
    }
}

#[cfg(feature = "serde")]
impl<'de> ::serde::de::Deserialize<'de> for Time {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_any(TimeVisitor)
    }
}

// TODO: TimeSpan ops similar to std::time::Duration

#[derive(Read, Write, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub struct TimeSpan(u64);

impl From<u64> for Time {
    #[inline]
    fn from(i: u64) -> Self {
        Time(i as i64)
    }
}

impl From<Time> for u64 {
    #[inline]
    fn from(t: Time) -> Self {
        t.0 as Self
    }
}

impl From<i64> for Time {
    #[inline]
    fn from(i: i64) -> Self {
        Time(i)
    }
}

impl From<Time> for i64 {
    #[inline]
    fn from(t: Time) -> Self {
        t.0
    }
}

impl From<Time> for i32 {
    #[inline]
    fn from(t: Time) -> Self {
        t.seconds()
    }
}

// impl Add for Time {
//     type Output = Self;
//     fn add(self, other: Self) -> Self {
//         Time(self.0.checked_add(other.0).assert("addition overflow"))
//     }
// }

// impl AddAssign for Time {
//     fn add_assign(&mut self, other: Self) {
//         self.0 = self.0.checked_add(other.0).assert("addition overflow");
//     }
// }

// impl Sub for Time {
//     type Output = Self;
//     fn sub(self, other: Self) -> Self {
//         Time(self.0.checked_sub(other.0).assert("subtraction overflow"))
//     }
// }

// impl SubAssign for Time {
//     fn sub_assign(&mut self, other: Self) {
//         self.0 = self.0.checked_sub(other.0).assert("subtraction overflow");
//     }
// }

// impl Mul for Time {
//     type Output = Self;
//     fn mul(self, other: Self) -> Self {
//         Time(
//             self.0
//                 .checked_mul(other.0)
//                 .assert("multiplication overflow"),
//         )
//     }
// }

// impl MulAssign for Time {
//     fn mul_assign(&mut self, other: Self) {
//         self.0 = self
//             .0
//             .checked_mul(other.0)
//             .assert("multiplication overflow");
//     }
// }

// impl Div for Time {
//     type Output = Self;
//     fn div(self, other: Self) -> Self {
//         Time(self.0.checked_sub(other.0).assert("division overflow"))
//     }
// }

// impl DivAssign for Time {
//     fn div_assign(&mut self, other: Self) {
//         eosio_assert(other.0 != 0, "divide by zero");
//         self.0 = self.0.checked_div(other.0).assert("division overflow");
//     }
// }

// impl Add for Time {
//     type Output = Self;
//     fn add(self, other: Self) -> Self {
//         Time(self.0.checked_add(other.0).assert("addition overflow"))
//     }
// }

// impl AddAssign for Time {
//     fn add_assign(&mut self, other: Self) {
//         self.0 = self.0.checked_add(other.0).assert("addition overflow");
//     }
// }

// impl Sub for Time {
//     type Output = Self;
//     fn sub(self, other: Self) -> Self {
//         Time(self.0.checked_sub(other.0).assert("subtraction overflow"))
//     }
// }

// impl SubAssign for Time {
//     fn sub_assign(&mut self, other: Self) {
//         self.0 = self.0.checked_sub(other.0).assert("subtraction overflow");
//     }
// }

// impl Mul for Time {
//     type Output = Self;
//     fn mul(self, other: Self) -> Self {
//         Time(
//             self.0
//                 .checked_mul(other.0)
//                 .assert("multiplication overflow"),
//         )
//     }
// }

// impl MulAssign for Time {
//     fn mul_assign(&mut self, other: Self) {
//         self.0 = self
//             .0
//             .checked_mul(other.0)
//             .assert("multiplication overflow");
//     }
// }

// impl Div for Time {
//     type Output = Self;
//     fn div(self, other: Self) -> Self {
//         Time(self.0.checked_sub(other.0).assert("division overflow"))
//     }
// }

// impl DivAssign for Time {
//     fn div_assign(&mut self, other: Self) {
//         eosio_assert(other.0 != 0, "divide by zero");
//         self.0 = self.0.checked_div(other.0).assert("division overflow");
//     }
// }
