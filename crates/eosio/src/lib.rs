#![feature(try_from, custom_attribute, concat_idents)]

use static_assertions::assert_cfg;

// TODO: wasm-bindgen/js-sys/web-sys
// TODO: neither contract or stdweb
assert_cfg!(
    all(
        any(feature = "contract", feature = "stdweb"),
        not(all(feature = "contract", feature = "stdweb"))
    ),
    "feature = 'contract' and feature = 'stdweb' cannot both be enabled"
);

mod lib {
    mod core {
        #[cfg(not(feature = "std"))]
        pub use core::*;
        #[cfg(feature = "std")]
        pub use std::*;
    }

    pub use self::core::convert::{TryFrom, TryInto};
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
mod assert;
mod asset;
mod bytes;
mod crypto;
#[cfg(feature = "serde")]
pub mod json;
mod print;
mod symbol;
mod table;
#[cfg(feature = "contract")]
mod table_primary;
#[cfg(feature = "contract")]
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
#[cfg(feature = "contract")]
pub use self::table_primary::*;
#[cfg(feature = "contract")]
pub use self::table_secondary::*;
pub use self::time::*;
pub use eosio_macros::*;
pub use eosio_sys::{ParseNameError, ParseSymbolError};

#[cfg(all(feature = "serde", feature = "stdweb"))]
mod stdweb_serializers {
    use super::*;
    use stdweb::*;
    js_serializable!(Authorization);
    js_deserializable!(Authorization);
}
#[cfg(all(feature = "serde", feature = "stdweb"))]
pub use self::stdweb_serializers::*;
