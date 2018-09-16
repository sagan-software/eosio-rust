#![no_std]
#![feature(proc_macro_non_items)]

extern crate eosio;

use eosio::prelude::*;

#[no_mangle]
pub extern "C" fn apply(_receiver: u64, _code: u64, action: u64) {
    match action {
        n!(hi) => hi(),
        _ => {
            // eosio_assert!(false, "bad action");
            unsafe { eosio_assert(0, cstr!("bad action")) };
        }
    }
}

#[action]
fn hi(name1: Name, name2: Name) {
    unsafe {
        // print!("Hello, ", name1, "! ", "Hello, ", name2, "! ");
        prints(cstr!("Hello, "));
        printn(name1.as_u64());
        prints(cstr!("! "));
        prints(cstr!("Hello, "));
        printn(name2.as_u64());
        prints(cstr!("! "));
    }
}
