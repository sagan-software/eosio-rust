#![feature(proc_macro_hygiene, try_from, custom_attribute, concat_idents)]

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
pub mod json;
mod print;
mod symbol;
mod table;
mod table_primary;
mod table_secondary;
mod time;

pub mod sys {
    pub use eosio_sys::*;
}

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

pub use eosio_macros::{
    eosio_abi, eosio_action, eosio_name, eosio_print, eosio_table, n, s, Read, TableRow, Write,
};

pub use eosio_sys::{ParseNameError, ParseSymbolError};
