use assert::*;
use eosio_macros::*;
use lib::*;

#[derive(Read, Write, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub struct Time(u64);

impl Time {
    pub const MICROSECOND: u64 = 1;
    pub const MILLISECOND: u64 = Self::MICROSECOND * 1_000;
    pub const SECOND: u64 = Self::MILLISECOND * 1_000;

    pub fn now() -> Self {
        Time(unsafe { ::eosio_sys::current_time() })
    }

    pub fn publication() -> Self {
        Time(unsafe { ::eosio_sys::publication_time() })
    }

    pub fn expiration() -> Self {
        let seconds = unsafe { ::eosio_sys::expiration() };
        Self::from_seconds(seconds)
    }

    pub fn zero() -> Self {
        Time(0)
    }

    pub fn is_zero(self) -> bool {
        self.0 == 0
    }

    pub fn microseconds(self) -> u64 {
        self.0
    }

    pub fn from_microseconds(microseconds: u64) -> Self {
        Time(microseconds)
    }

    pub fn milliseconds(self) -> u64 {
        self.0 / Self::MILLISECOND
    }

    pub fn from_milliseconds(milliseconds: u64) -> Self {
        Time(milliseconds * Self::MILLISECOND)
    }

    pub fn seconds(self) -> u32 {
        (self.0 / Self::SECOND) as u32
    }

    pub fn from_seconds(seconds: u32) -> Self {
        Time(u64::from(seconds) * Self::SECOND)
    }
}

// TODO: TimeSpan ops similar to std::time::Duration

#[derive(Read, Write, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub struct TimeSpan(u64);

impl From<u64> for Time {
    fn from(i: u64) -> Self {
        Time(i)
    }
}

impl From<Time> for u64 {
    fn from(t: Time) -> Self {
        t.0
    }
}

impl From<i64> for Time {
    fn from(i: i64) -> Self {
        Time(i as u64)
    }
}

impl From<Time> for i64 {
    fn from(t: Time) -> Self {
        t.0 as i64
    }
}

impl From<Time> for u32 {
    fn from(t: Time) -> Self {
        t.seconds()
    }
}

impl Add for Time {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Time(self.0.checked_add(other.0).assert("addition overflow"))
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0.checked_add(other.0).assert("addition overflow");
    }
}

impl Sub for Time {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Time(self.0.checked_sub(other.0).assert("subtraction overflow"))
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0.checked_sub(other.0).assert("subtraction overflow");
    }
}

impl Mul for Time {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Time(
            self.0
                .checked_mul(other.0)
                .assert("multiplication overflow"),
        )
    }
}

impl MulAssign for Time {
    fn mul_assign(&mut self, other: Self) {
        self.0 = self
            .0
            .checked_mul(other.0)
            .assert("multiplication overflow");
    }
}

impl Div for Time {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Time(self.0.checked_sub(other.0).assert("division overflow"))
    }
}

impl DivAssign for Time {
    fn div_assign(&mut self, other: Self) {
        eosio_assert(other.0 != 0, "divide by zero");
        self.0 = self.0.checked_div(other.0).assert("division overflow");
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
