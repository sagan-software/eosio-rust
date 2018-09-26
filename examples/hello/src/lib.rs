#![feature(proc_macro_non_items)]

extern crate eosio;

use eosio::prelude::*;

#[eosio_action]
fn hi(name: AccountName) {
    eosio_print!("Hi, ", name);
}

eosio_abi!(hi);
