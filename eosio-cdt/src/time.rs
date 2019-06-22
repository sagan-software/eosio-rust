use eosio_core::{TimePoint, TimePointSec};

/// Gets the current time
#[inline]
pub fn current_time_point() -> TimePoint {
    TimePoint::from(unsafe { ::eosio_cdt_sys::current_time() } as i64)
}

/// Gets the current time
#[inline]
pub fn current_time_point_sec() -> TimePointSec {
    current_time_point().into()
}

/// Gets the publication time
#[inline]
pub fn publication() -> TimePoint {
    TimePoint::from(unsafe { ::eosio_cdt_sys::publication_time() } as i64)
}

/// Gets the expiration time
#[inline]
pub fn expiration() -> TimePointSec {
    let seconds = unsafe { ::eosio_cdt_sys::expiration() };
    seconds.into()
}

impl crate::print::Print for TimePoint {
    #[inline]
    fn print(&self) {
        "TimePoint(".print();
        self.as_i64().print();
        ")".print();
    }
}
