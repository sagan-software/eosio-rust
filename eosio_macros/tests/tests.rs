#![feature(proc_macro_non_items)]

extern crate eosio_macros;

use eosio_macros::*;

#[test]
fn test_n() {
    assert_eq!(n!(test), 14_605_613_396_213_628_928 as u64);
    assert_eq!(n!(1234), 614_248_767_926_829_056 as u64);
    assert_eq!(n!(123451234512), 614_251_535_012_020_768 as u64);
}

#[test]
fn test_s() {
    assert_eq!(s!(0, TGFT), 361_956_332_544);
    assert_eq!(s!(4, EOS), 1_397_703_940);
    assert_eq!(s!(0, EOS), 1_397_703_936);
    assert_eq!(s!(1, EDNA), 280_485_971_201);
}
