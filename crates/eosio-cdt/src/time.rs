//! TODO module docs.

use eosio::{TimePoint, TimePointSec};

/// Gets the current time
#[inline]
pub fn current_time_point() -> TimePoint {
    let micros = unsafe { eosio_cdt_sys::current_time() } as i64;
    TimePoint::from_micros(micros)
}

/// Gets the current time
#[inline]
pub fn current_time_point_sec() -> TimePointSec {
    current_time_point().as_time_point_sec()
}

/// Gets the publication time
#[inline]
pub fn publication() -> TimePoint {
    let micros = unsafe { eosio_cdt_sys::publication_time() } as i64;
    TimePoint::from_micros(micros)
}

/// Gets the expiration time
#[inline]
pub fn expiration() -> TimePointSec {
    let secs = unsafe { eosio_cdt_sys::expiration() };
    TimePointSec::from_secs(secs)
}
