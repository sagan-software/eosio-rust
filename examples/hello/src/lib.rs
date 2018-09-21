#![no_std]
#![feature(proc_macro_non_items)]

extern crate eosio;
extern crate eosio_bytes;
extern crate eosio_sys;
extern crate eosio_types;

use eosio::prelude::*;

#[eosio_action]
fn hi(name: Name) {
    print!("Hello, ", name);
}

eosio_abi!(hi);
