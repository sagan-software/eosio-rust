#![no_std]
#![feature(proc_macro_non_items)]

extern crate eosio;

use eosio::prelude::*;

#[no_mangle]
pub extern "C" fn apply(_receiver: u64, _code: u64, _action: u64) {
    let mut bytes = [0u8; 8];
    let ptr: *mut c_void = &mut bytes[..] as *mut _ as *mut c_void;
    unsafe {
        read_action_data(ptr, action_data_size());
    }

    let user = ((bytes[0] as u64) << 0)
        | ((bytes[1] as u64) << 8)
        | ((bytes[2] as u64) << 16)
        | ((bytes[3] as u64) << 24)
        | ((bytes[4] as u64) << 32)
        | ((bytes[5] as u64) << 40)
        | ((bytes[6] as u64) << 48)
        | ((bytes[7] as u64) << 56);

    unsafe {
        prints(cstr!("Hello, "));
        printn(user);
    }
}
