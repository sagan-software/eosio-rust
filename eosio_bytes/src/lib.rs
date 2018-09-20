#![cfg_attr(feature = "alloc", feature(alloc))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod lib {
    mod core {
        #[cfg(not(feature = "std"))]
        pub use core::*;
        #[cfg(feature = "std")]
        pub use std::*;
    }

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::string::{String, ToString};
    #[cfg(feature = "std")]
    pub use std::string::String;

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::vec::Vec;
    #[cfg(feature = "std")]
    pub use std::vec::Vec;
}

mod readable;
mod writeable;

pub use self::readable::*;
pub use self::writeable::*;
