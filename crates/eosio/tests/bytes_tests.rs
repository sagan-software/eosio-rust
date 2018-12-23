use eosio::*;

macro_rules! test_type {
    ($($i:ident, $t:ty, $e:expr)*) => ($(
        #[test]
        fn $i() {
            let expected_pos = $e.num_bytes();
            let mut bytes = [0u8; 100];
            let thing: $t = $e;

            let mut write_pos = 0;
            thing.write(&mut bytes, &mut write_pos).unwrap();
            assert_eq!(expected_pos, write_pos);

            // let mut read_pos = 0;
            // let result = <$t as ::eosio::Read>::read(&bytes, &mut read_pos).unwrap();
            // assert_eq!(expected_pos, read_pos);

            // assert_eq!($e, result);
            // assert_eq!(write_pos, read_pos);
        }
    )*)
}

test_type!(
    test_u8, u8, 1_u8
    test_u16, u16, 1_u16
    test_u32, u32, 1_u32
    test_u64, u64, 1_u64
    // test_i8, i8, 1i8
    test_i16, i16, -1_i16
    test_i32, i32, -1_i32
    test_i64, i64, -1_i64
    test_typle2, (u8, u16), (1u8, 1u16)
    // test_typle3, (u8, u16, u32), (1u8, 1u16, 1u32)
    test_bool_true, bool, true
    test_bool_false, bool, false
    // test_option_none, Option<u8>, None
    // test_option_some, Option<u8>, Some(1)
    test_string, String, "neat".to_string()
    test_vec, Vec<u8>, vec![1_u8, 2_u8, 3_u8]
    test_tuple2, (u8, u16), (1_u8, 2_u16)
    test_tuple3, (u8, u16, u32), (1_u8, 2_u16, 3_u32)
    test_tuple4, (u8, u16, u32, u64), (1_u8, 2_u16, 3_u32, 4_u64)
    test_array1, [u8; 1], [1u8; 1]
    test_array2, [u8; 2], [1u8; 2]
    test_array3, [u8; 3], [1u8; 3]
    test_array4, [u8; 4], [1u8; 4]
    test_array5, [u8; 5], [1u8; 5]
    test_array6, [u8; 6], [1u8; 6]
    test_array7, [u8; 7], [1u8; 7]
    test_array8, [u8; 8], [1u8; 8]
    test_array9, [u8; 9], [1u8; 9]
    test_array10, [u8; 10], [1u8; 10]
    test_array11, [u8; 11], [1u8; 11]
    test_array12, [u8; 12], [1u8; 12]
    test_array13, [u8; 13], [1u8; 13]
    test_array14, [u8; 14], [1u8; 14]
    test_array15, [u8; 15], [1u8; 15]
    test_array16, [u8; 16], [1u8; 16]
    test_array17, [u8; 17], [1u8; 17]
    test_array18, [u8; 18], [1u8; 18]
    test_array19, [u8; 19], [1u8; 19]
    test_array20, [u8; 20], [1u8; 20]
    test_account_name, AccountName, AccountName::from(n!(test))
    test_time, Time, Time::UNIX_EPOCH
    test_f32, f32, -0.12345_f32
    test_f64, f64, -0.12345_f64
);

#[test]
fn test_struct_named_fields() {
    #[derive(Read, Write, PartialEq, Debug)]
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
fn test_struct_unnamed_fields() {
    #[derive(Read, Write, PartialEq, Debug)]
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

#[test]
fn test_read_pos() {
    let bytes = &[
        10, 9, 0, 1, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 20, 4, 3, 2, 1, 1, 1, 1, 1,
    ];

    let mut pos = 0;
    u8::read(bytes, &mut pos).unwrap();
    assert_eq!(pos, 1);

    u8::read(bytes, &mut pos).unwrap();
    assert_eq!(pos, 2);

    u16::read(bytes, &mut pos).unwrap();
    assert_eq!(pos, 4);

    u32::read(bytes, &mut pos).unwrap();
    assert_eq!(pos, 8);

    u64::read(bytes, &mut pos).unwrap();
    assert_eq!(pos, 16);

    u64::read(bytes, &mut pos).unwrap();
    assert_eq!(pos, 24);

    pos = 2;
    u64::read(bytes, &mut pos).unwrap();
    assert_eq!(pos, 10);

    pos = 10;
    u64::read(bytes, &mut pos).unwrap();
    assert_eq!(pos, 18);
}

#[test]
fn test_write_pos() {
    let bytes = &mut [0u8; 1000];

    let mut pos = 0;
    1u8.write(bytes, &mut pos).unwrap();
    assert_eq!(pos, 1);

    1_u16.write(bytes, &mut pos).unwrap();
    assert_eq!(pos, 3);

    1_u32.write(bytes, &mut pos).unwrap();
    assert_eq!(pos, 7);

    1_u64.write(bytes, &mut pos).unwrap();
    assert_eq!(pos, 15);
}
