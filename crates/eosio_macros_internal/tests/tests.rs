use eosio::*;

#[test]
fn struct_named_fields() {
    #[derive(Read, Write, NumBytes, PartialEq, Debug)]
    struct Thing {
        a: u64,
        b: u64,
        c: u32,
    }

    let thing1 = Thing { a: 1, b: 2, c: 3 };

    let mut bytes = [0u8; 100];
    let mut write_pos = 0;
    thing1.write(&mut bytes, &mut write_pos).unwrap();
    assert_eq!(write_pos, 20);

    let mut read_pos = 0;
    let thing2 = Thing::read(&bytes, &mut read_pos).unwrap();
    assert_eq!(read_pos, write_pos);

    assert_eq!(thing1, thing2);
    assert_eq!(thing1.a, 1);
    assert_eq!(thing1.b, 2);
    assert_eq!(thing1.c, 3);
}

#[test]
fn struct_unnamed_fields() {
    #[derive(Read, Write, NumBytes, PartialEq, Debug)]
    struct Thing(u64, u64, u32);

    let thing1 = Thing(1, 2, 3);

    let mut bytes = [0u8; 100];

    let mut write_pos = 0;
    thing1.write(&mut bytes, &mut write_pos).unwrap();
    assert_eq!(write_pos, 20);

    let mut read_pos = 0;
    let thing2 = Thing::read(&bytes, &mut read_pos).unwrap();
    assert_eq!(read_pos, write_pos);

    assert_eq!(thing1, thing2);
    assert_eq!(thing1.0, 1);
    assert_eq!(thing1.1, 2);
    assert_eq!(thing1.2, 3);
}
