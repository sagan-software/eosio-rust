use eosio::{BlockTimestamp, TimePoint, TimePointSec};

/// Returns the time in microseconds from 1970 of the current block as a
/// `TimePoint`
#[must_use]
#[inline]
#[allow(clippy::cast_possible_wrap)]
pub fn current_time_point() -> TimePoint {
    let micros = unsafe { eosio_cdt_sys::current_time() } as i64;
    TimePoint::from_micros(micros)
}

/// Gets the current time as seconds
#[must_use]
#[inline]
pub fn current_time_point_sec() -> TimePointSec {
    current_time_point().as_time_point_sec()
}

/// Gets the current time as a block timestamp
#[must_use]
#[inline]
pub fn current_block_time() -> BlockTimestamp {
    current_time_point().into()
}

/// Gets the publication time
#[must_use]
#[inline]
#[allow(clippy::cast_possible_wrap)]
pub fn publication() -> TimePoint {
    let micros = unsafe { eosio_cdt_sys::publication_time() } as i64;
    TimePoint::from_micros(micros)
}

/// Gets the expiration time
#[must_use]
#[inline]
pub fn expiration() -> TimePointSec {
    let secs = unsafe { eosio_cdt_sys::expiration() };
    TimePointSec::from_secs(secs)
}
