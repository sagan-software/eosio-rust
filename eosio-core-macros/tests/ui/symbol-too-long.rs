extern crate eosio_core_macros;

use eosio_core_macros::s;

fn main() {
    let _ = s!(4, ABCDEFGH);
}