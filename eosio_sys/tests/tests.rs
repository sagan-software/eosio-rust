extern crate eosio_sys;

use eosio_sys::*;

macro_rules! test_string_to_name {
    ($($n:ident, $i:expr, $o:expr)*) => ($(
        #[test]
        fn $n() {
            assert_eq!(string_to_name($i), $o);
        }
    )*)
}

test_string_to_name!(
    string_to_name_empty, "", Err(ToNameError::IsEmpty)
    string_to_name_single_char, "a", Ok(3_458_764_513_820_540_928)
    string_to_name_bad_number, "123456789012", Err(ToNameError::BadChar('6'))
    string_to_name_only_numbers, "123451234512", Ok(614_251_535_012_020_768)
    string_to_name_too_long, "1234512345123", Err(ToNameError::TooLong)
    string_to_name_uppercase, "TEST", Err(ToNameError::BadChar('T'))
    string_to_name_only_letters, "test", Ok(14_605_613_396_213_628_928)
    string_to_name_special_char, "test!", Err(ToNameError::BadChar('!'))
);

macro_rules! test_name_to_string {
    ($($n:ident, $i:expr, $o:expr)*) => ($(
        #[test]
        fn $n() {
            assert_eq!(name_to_string($i), $o);
        }
    )*)
}

test_name_to_string!(
    name_to_string_single_char, 3_458_764_513_820_540_928, "a"
    name_to_string_only_numbers, 614_251_535_012_020_768, "123451234512"
    name_to_string_only_letters, 14_605_613_396_213_628_928, "test"
    name_to_string_zero, 0, ""
);

macro_rules! test_symbol_name_length {
    ($($n:ident, $i:expr, $o:expr)*) => ($(
        #[test]
        fn $n() {
            assert_eq!(symbol_name_length($i), $o);
        }
    )*)
}

test_symbol_name_length!(
    symbol_name_length_zero, 0, 0
    symbol_name_length_three, 1_397_703_940, 3
    symbol_name_length_four, 361_956_332_544, 4
);
