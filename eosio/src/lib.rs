#![no_std]
#![feature(
    alloc,
    core_intrinsics,
    panic_handler,
    lang_items,
    alloc_error_handler,
    proc_macro_non_items
)]

extern crate alloc;
extern crate eosio_macros;
extern crate eosio_sys;
extern crate eosio_types;
extern crate wee_alloc;

pub mod bytes;
pub mod print;

pub mod types {
    pub use eosio_types::*;
}

pub mod sys {
    pub use eosio_sys::*;
}

pub mod macros {
    pub use eosio_macros::*;
}

pub mod prelude {
    pub use super::bytes::*;
    pub use super::macros::*;
    pub use super::print::*;
    pub use super::sys::prelude::*;
    pub use super::types::*;
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &::core::panic::PanicInfo) -> ! {
    unsafe {
        ::core::intrinsics::abort();
    }
}

#[alloc_error_handler]
#[no_mangle]
pub extern "C" fn oom(_: ::core::alloc::Layout) -> ! {
    unsafe {
        ::core::intrinsics::abort();
    }
}
