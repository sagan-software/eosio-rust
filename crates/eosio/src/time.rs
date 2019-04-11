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

    pub const UNIX_EPOCH: Self = Time(0);

    /// Gets the current time
    #[cfg(all(feature = "contract", not(any(feature = "stdweb", feature = "js-sys"))))]
    #[inline]
    pub fn now() -> Self {
        Time(unsafe { ::eosio_sys::current_time() } as i64)
    }

    /// Gets the current time
    #[cfg(all(feature = "stdweb", not(any(feature = "contract", feature = "js-sys"))))]
    #[inline]
    pub fn now() -> Self {
        let microseconds = ::stdweb::web::Date::now() * 1_000.0;
        Time(microseconds as i64)
    }

    /// Gets the current time
    #[cfg(all(feature = "js-sys", not(any(feature = "contract", feature = "stdweb"))))]
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
        Self::from_secs(i64::from(seconds))
    }

    /// Creates a `Time` from microseconds
    #[inline]
    pub const fn from_micros(micros: i64) -> Self {
        Time(micros)
    }

    /// Creates a `Time` from milliseconds
    #[inline]
    pub fn from_millis(millis: i64) -> Self {
        Time(millis.saturating_mul(Self::MILLISECOND))
    }

    /// Creates a `Time` from seconds
    #[inline]
    pub fn from_secs(secs: i64) -> Self {
        Time(secs.saturating_mul(Self::SECOND))
    }

    /// Creates a `Time` from minutes
    #[inline]
    pub fn from_mins(mins: i64) -> Self {
        Time(mins.saturating_mul(Self::MINUTE))
    }

    /// Creates a `Time` from hours
    #[inline]
    pub fn from_hours(hours: i64) -> Self {
        Time(hours.saturating_mul(Self::HOUR))
    }

    /// Creates a `Time` from days
    #[inline]
    pub fn from_days(days: i64) -> Self {
        Time(days.saturating_mul(Self::DAY))
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
//         check(other.0 != 0, "divide by zero");
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
//         check(other.0 != 0, "divide by zero");
//         self.0 = self.0.checked_div(other.0).assert("division overflow");
//     }
// }

// TODO: Duration ops similar to std::time::Duration

#[derive(Read, Write, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub struct Duration(i64);
