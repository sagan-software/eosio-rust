#![feature(proc_macro_non_items)]

extern crate eosio;
extern crate eosio_bytes;
extern crate eosio_sys;
extern crate eosio_types;

use eosio::prelude::*;

#[eosio_action]
fn hi(name: Name) {
    eosio_print!("Hi, ", name);
}

eosio_abi!(hi);
