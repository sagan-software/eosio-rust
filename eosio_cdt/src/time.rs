use eosio_core::Time;

/// Gets the current time
#[inline]
pub fn current_time() -> Time {
    Time::from(unsafe { ::eosio_cdt_sys::current_time() } as i64)
}

/// Gets the publication time
#[inline]
pub fn publication() -> Time {
    Time::from(unsafe { ::eosio_cdt_sys::publication_time() } as i64)
}

/// Gets the expiration time
#[inline]
pub fn expiration() -> Time {
    let seconds = unsafe { ::eosio_cdt_sys::expiration() };
    Time::from_secs(i64::from(seconds))
}

impl crate::print::Print for Time {
    #[inline]
    fn print(&self) {
        "Time(".print();
        self.as_micros().print();
        ")".print();
    }
}
