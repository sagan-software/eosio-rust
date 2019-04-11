#![recursion_limit = "128"]
#![warn(
    clippy::all,
    clippy::complexity,
    clippy::style,
    clippy::perf,
    clippy::nursery,
    clippy::cargo
)]

#[macro_use]
extern crate mashup;

/// Docs
mod lib {
    /// Docs
    mod core {
        #[cfg(not(feature = "std"))]
        pub use core::*;
        #[cfg(feature = "std")]
        pub use std::*;
    }

    pub use self::core::marker::PhantomData;
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
mod asset;
mod bytes;
mod check;
mod crypto;
mod print;
mod symbol;
mod table;
mod time;

#[cfg(feature = "serde")]
pub mod json;

#[cfg(feature = "contract")]
mod table_primary;

#[cfg(feature = "contract")]
mod table_secondary;

/// Docs
pub mod sys {
    pub use eosio_sys::*;
}

pub use self::account::*;
pub use self::action::*;
pub use self::asset::*;
pub use self::bytes::*;
pub use self::check::*;
pub use self::crypto::*;
pub use self::print::*;
pub use self::symbol::*;
pub use self::table::*;
#[cfg(feature = "contract")]
pub use self::table_primary::*;
#[cfg(feature = "contract")]
pub use self::table_secondary::*;
pub use self::time::*;
pub use eosio_macros::*;
pub use eosio_sys::{ParseNameError, ParseSymbolError};

/// Docs
#[cfg(all(feature = "serde", feature = "stdweb"))]
mod stdweb_serializers {
    use super::*;
    use stdweb::*;
    js_serializable!(Authorization);
    js_deserializable!(Authorization);
}

/// Docs
#[cfg(all(feature = "serde", feature = "stdweb"))]
pub use self::stdweb_serializers::*;
