#![cfg_attr(feature = "alloc", feature(alloc))]
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(proc_macro_non_items, try_from, custom_attribute)]

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

    pub use self::core::convert::TryInto;
    pub use self::core::marker::PhantomData;
    pub use self::core::ops::{BitAnd, BitOr, Mul, Shl, Shr};

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::string::{String, ToString};
    #[cfg(feature = "std")]
    pub use std::string::String;

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::vec::Vec;
    #[cfg(feature = "std")]
    pub use std::vec::Vec;
}

pub mod account;
pub mod action;
pub mod asset;
pub mod bytes;
pub mod db;
pub mod permission;
pub mod print;
pub mod scope;
pub mod symbol;
pub mod time;

pub mod sys {
    pub use eosio_sys::*;
}

pub mod macros {
    pub use eosio_macros::*;
}

pub mod prelude {
    pub use super::account::*;
    pub use super::action::*;
    pub use super::asset::*;
    pub use super::bytes::*;
    pub use super::db::*;
    pub use super::macros::*;
    pub use super::permission::*;
    pub use super::print::*;
    pub use super::scope::*;
    pub use super::symbol::*;
    pub use super::time::*;
}

mod eosio {
    pub use super::bytes;
    pub use super::print;
    pub use super::sys;
}
