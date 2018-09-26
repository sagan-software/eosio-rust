#![feature(proc_macro_non_items)]

extern crate eosio_macros;

use eosio_macros::{n, s};

#[test]
fn test_n() {
    assert_eq!(n!(test), 14_605_613_396_213_628_928 as u64);
    assert_eq!(n!(1234), 614_248_767_926_829_056 as u64);
    assert_eq!(n!(123451234512), 614_251_535_012_020_768 as u64);
}

#[test]
fn test_s() {
    assert_eq!(s!(0, TGFT), 361956332544);
    assert_eq!(s!(4, EOS), 1397703940);
    assert_eq!(s!(0, EOS), 1397703936);
    assert_eq!(s!(1, EDNA), 280485971201);
}

#[test]
fn test_derive_write_read_struct_named_fields() {
    #[derive(Readable, Writeable, PartialEq, Debug)]
    struct Thing {
        a: u64,
        b: u64,
        c: u32,
    }

    let thing1 = Thing { a: 1, b: 2, c: 3 };

    let mut bytes = [0u8; 100];
    let p1 = thing1.write(&mut bytes, 0).unwrap();

    let (thing2, p2) = Thing::read(&bytes, 0).unwrap();

    assert_eq!(thing1, thing2);
    assert_eq!(p1, p2);
    assert_eq!(p1, 20);
    assert_eq!(thing1.a, 1);
    assert_eq!(thing1.b, 2);
    assert_eq!(thing1.c, 3);
}

#[test]
fn test_derive_write_read_struct_unnamed_fields() {
    #[derive(Readable, Writeable, PartialEq, Debug)]
    struct Thing(u64, u64, u32);

    let thing1 = Thing(1, 2, 3);

    let mut bytes = [0u8; 100];
    let p1 = thing1.write(&mut bytes, 0).unwrap();

    let (thing2, p2) = Thing::read(&bytes, 0).unwrap();

    assert_eq!(thing1, thing2);
    assert_eq!(p1, p2);
    assert_eq!(p1, 20);
    assert_eq!(thing1.0, 1);
    assert_eq!(thing1.1, 2);
    assert_eq!(thing1.2, 3);
}
