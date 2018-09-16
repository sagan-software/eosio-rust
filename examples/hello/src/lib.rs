#![no_std]
#![feature(proc_macro_non_items)]

extern crate eosio;

use eosio::prelude::*;

#[no_mangle]
pub extern "C" fn apply(_receiver: u64, _code: u64, action: u64) {
    match action {
        n!(hi) => hi(),
        _ => {
            eosio_assert!(false, "bad action");
        }
    }
}

#[action]
fn hi(name1: Name, name2: Name) {
    print!("Hello, ", name1, "! ", "Hello, ", name2, "!");
}
