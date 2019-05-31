use eosio_core::TimePoint;

/// Gets the current time
#[inline]
pub fn current_time() -> TimePoint {
    TimePoint::from(unsafe { ::eosio_cdt_sys::current_time() } as i64)
}

/// Gets the publication time
#[inline]
pub fn publication() -> TimePoint {
    TimePoint::from(unsafe { ::eosio_cdt_sys::publication_time() } as i64)
}

/// Gets the expiration time
#[inline]
pub fn expiration() -> TimePoint {
    let seconds = unsafe { ::eosio_cdt_sys::expiration() };
    TimePoint::from_secs(i64::from(seconds))
}

impl crate::print::Print for TimePoint {
    #[inline]
    fn print(&self) {
        "TimePoint(".print();
        self.as_micros().print();
        ")".print();
    }
}
