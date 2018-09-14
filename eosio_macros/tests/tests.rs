#![feature(proc_macro_non_items)]

extern crate eosio_macros;

use eosio_macros::n;

#[test]
fn test_n() {
    assert_eq!(n!(test), 14_605_613_396_213_628_928 as u64);
    assert_eq!(n!(1234), 614_248_767_926_829_056 as u64);
    assert_eq!(n!(123451234512), 614_251_535_012_020_768 as u64);
}
