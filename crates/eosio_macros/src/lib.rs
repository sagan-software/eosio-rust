//! Macros for creating compile-time EOSIO names and symbols.
//!
//! Creating EOSIO names:
//!
//! ```
//! use eosio_macros::n;
//! assert_eq!(n!("test"), 14_605_613_396_213_628_928);
//! assert_eq!(n!("1234"), 614_248_767_926_829_056);
//! assert_eq!(n!("123451234512"), 614_251_535_012_020_768);
//! assert_eq!(n!("eosio.token"), 6_138_663_591_592_764_928);
//! ```
//!
//! Creating EOSIO symbols:
//!
//! ```
//! use eosio_macros::s;
//! assert_eq!(s!(4, "EOS"), 1397703940);
//! ```
#![no_std]
#![allow(clippy::missing_docs_in_private_items)]

use proc_macro_hack::proc_macro_hack;

/// Macro for converting EOSIO names into `u64` representations at compile
/// time.
///
/// # Examples
///
/// ```
/// use eosio_macros::n;
/// assert_eq!(n!("test"), 14_605_613_396_213_628_928);
/// assert_eq!(n!("1234"), 614_248_767_926_829_056);
/// assert_eq!(n!("123451234512"), 614_251_535_012_020_768);
/// assert_eq!(n!("eosio.token"), 6_138_663_591_592_764_928);
/// ```
#[proc_macro_hack]
pub use eosio_macros_internal::n;

/// Macro for converting EOSIO symbols into `u64` representations at
/// compile time.
///
/// # Examples
///
/// ```
/// use eosio_macros::s;
/// assert_eq!(s!(4, "EOS"), 1397703940);
/// ```
#[proc_macro_hack]
pub use eosio_macros_internal::s;

pub use eosio_macros_internal::{
    abi, action, table, NumBytes, Read, Table, Write,
};
