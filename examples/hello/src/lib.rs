#![feature(proc_macro_hygiene)]

use eosio::*;

#[eosio_action]
fn hi(name: AccountName) {
    eosio_print!("Hi, ", name);
}

eosio_abi!(hi);
