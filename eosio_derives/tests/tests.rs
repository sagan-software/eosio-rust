#![feature(proc_macro_non_items)]

extern crate eosio_bytes;
extern crate eosio_derives;

use eosio_bytes::*;
use eosio_derives::*;

#[test]
fn test_write_read_struct_named_fields() {
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
fn test_write_read_struct_unnamed_fields() {
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
