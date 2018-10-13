#![feature(proc_macro_non_items)]

extern crate eosio;

use eosio::*;

#[eosio_action]
fn crypto(data: String) {
    Checksum160::new(&data).assert(&data);
    Checksum256::new(&data).assert(&data);
}

eosio_abi!(crypto);
