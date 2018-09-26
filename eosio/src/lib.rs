// #![no_std]
#![feature(
    alloc,
    // core_intrinsics,
    // lang_items,
    // alloc_error_handler,
    proc_macro_non_items
)]

extern crate alloc;
extern crate core;
extern crate eosio_bytes;
extern crate eosio_derives;
extern crate eosio_macros;
extern crate eosio_sys;
extern crate eosio_types;
extern crate wee_alloc;

mod lib {
    #[cfg(not(feature = "std"))]
    pub use core::*;
    #[cfg(feature = "std")]
    pub use std::*;

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::string::{String, ToString};
    #[cfg(feature = "std")]
    pub use std::string::String;
}

pub mod action;
pub mod db;
pub mod print;

pub mod sys {
    pub use eosio_sys::*;
}

pub mod bytes {
    pub use eosio_bytes::*;
}

pub mod derives {
    pub use eosio_derives::*;
}

pub mod macros {
    pub use eosio_macros::*;
}

pub mod types {
    pub use eosio_types::*;
}

pub mod prelude {
    pub use super::action::*;
    pub use super::bytes::*;
    pub use super::db::*;
    pub use super::derives::*;
    pub use super::macros::*;
    pub use super::print::*;
    pub use super::types::*;
}

// ::eosio_macros::wee_alloc!();
