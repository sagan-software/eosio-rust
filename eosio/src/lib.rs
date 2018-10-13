#![cfg_attr(feature = "alloc", feature(alloc))]
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(
    proc_macro_non_items,
    try_from,
    custom_attribute,
    concat_idents
)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
extern crate eosio_macros;
extern crate eosio_sys;

mod lib {
    mod core {
        #[cfg(not(feature = "std"))]
        pub use core::*;
        #[cfg(feature = "std")]
        pub use std::*;
    }

    pub use self::core::convert::{TryFrom, TryInto};
    pub use self::core::marker::PhantomData;
    pub use self::core::mem::{size_of, size_of_val};
    pub use self::core::ops::*;
    pub use self::core::str::FromStr;

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::string::{String, ToString};
    #[cfg(feature = "std")]
    pub use std::string::String;

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::vec::Vec;
    #[cfg(feature = "std")]
    pub use std::vec::Vec;

}

mod account;
mod action;
mod assert;
mod asset;
mod bytes;
mod crypto;
mod print;
mod symbol;
mod table;
mod table_primary;
mod table_secondary;
mod time;

pub use self::account::*;
pub use self::action::*;
pub use self::assert::*;
pub use self::asset::*;
pub use self::bytes::*;
pub use self::crypto::*;
pub use self::print::*;
pub use self::symbol::*;
pub use self::table::*;
pub use self::table_primary::*;
pub use self::table_secondary::*;
pub use self::time::*;
pub use eosio_macros::*;

mod eosio {
    pub use super::bytes::*;
    pub use super::print::*;
    pub use eosio_sys::*;
}
