extern crate eosio;

use eosio::prelude::*;

#[test]
fn basic_symbol_tests() {
    let symbol = Symbol::from(361_956_332_546);
    assert_eq!(symbol.precision(), 2);

    let name = symbol.name();
    let num: u64 = name.into();
    assert_eq!(num, 1_413_891_924);
}
