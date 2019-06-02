extern crate eosio_numstr_macros;

use eosio_numstr_macros::s;

fn main() {
    let _ = s!(256, EOS);
}