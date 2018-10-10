use assert::*;
use eosio_macros::*;
use lib::*;

#[derive(Read, Write, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub struct Time(u64);

impl Time {
    pub fn now() -> Self {
        Time(unsafe { ::eosio_sys::current_time() })
    }

    pub fn publication() -> Self {
        Time(unsafe { ::eosio_sys::publication_time() })
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

    pub fn milliseconds(self) -> u64 {
        self.microseconds() / 1_000
    }

    pub fn seconds(self) -> u64 {
        self.milliseconds() / 1_000
    }
}

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

impl From<u32> for Time {
    fn from(i: u32) -> Self {
        Time(u64::from(i) * 1_000_000)
    }
}

impl From<Time> for u32 {
    fn from(t: Time) -> Self {
        t.seconds() as u32
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
