#![cfg_attr(feature = "alloc", feature(alloc))]
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(try_from)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
extern crate eosio_sys;

mod lib {
    mod core {
        #[cfg(not(feature = "std"))]
        pub use core::*;
        #[cfg(feature = "std")]
        pub use std::*;
    }

    pub use self::core::ops::{BitAnd, BitOr, Mul, Shl, Shr};

    pub use self::core::convert::TryInto;

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::string::{String, ToString};
    #[cfg(feature = "std")]
    pub use std::string::String;

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::vec::Vec;
    #[cfg(feature = "std")]
    pub use std::vec::Vec;
}

mod fixed_size;
mod readable;
mod writeable;

pub use self::fixed_size::*;
pub use self::readable::*;
pub use self::writeable::*;
