use eosio_macros::*;

#[derive(Read, Write, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct Time(u64);

impl Time {
    pub fn now() -> Self {
        Time(unsafe { ::eosio_sys::current_time() })
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
