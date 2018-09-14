extern crate eosio;

use eosio::prelude::*;

#[action]
fn hi(user: AccountName) {
    eosio_print!("Hello, ", user);
}
