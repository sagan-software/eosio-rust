#![feature(proc_macro_non_items)]
extern crate eosio;

use eosio::prelude::*;

#[no_mangle]
pub extern "C" fn apply(_receiver: u64, _code: u64, _action: u64) {
    unsafe { prints(cstr!("Hello World")) }
}
