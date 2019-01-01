use eosio::*;

#[test]
fn basic_symbol_tests() {
    let symbol = Symbol::from(361_956_332_546);
    assert_eq!(symbol.precision(), 2);

    let name = symbol.code();
    let num: u64 = name.into();
    assert_eq!(num, 1_413_891_924);
}

#[test]
fn test_is_valid() {
    let symbol = Symbol::from(361_956_332_546);
    assert_eq!(symbol.is_valid(), true);
}

#[test]
fn test_symbol_to_string() {
    let symbol = Symbol::from(361_956_332_546);
    assert_eq!(symbol.to_string(), "2,TGFT".to_string());
}

#[test]
fn test_symbol_name_to_string() {
    let symbol = Symbol::from(361_956_332_546);
    assert_eq!(symbol.code().to_string(), "TGFT".to_string());
}
