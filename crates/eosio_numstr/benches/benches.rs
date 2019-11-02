#[macro_use]
extern crate criterion;
extern crate eosio_numstr;

use criterion::{black_box, Criterion};

fn name_from_str(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "name_from_str",
        |b, input| b.iter(|| eosio_numstr::name_from_str(black_box(input))),
        vec![
            "",
            "aaaaaa",
            "111111",
            "......",
            "aaaaaaaaaaaa",
            "111111111111",
            "............",
            "AAAAAAAAAAAA",
            "666666666666",
            "aaaaaaaaaaaaa",
        ],
    );
}

fn name_to_string(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "name_to_string",
        |b, input| b.iter(|| eosio_numstr::name_to_string(black_box(*input))),
        vec![0, 3_458_764_513_820_540_928, 614_251_535_012_020_768],
    );
}

fn symbol_from_str(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "symbol_from_str",
        |b, input| b.iter(|| eosio_numstr::name_from_str(black_box(input))),
        vec!["", "A", "AB", "ABC", "a", "ab", "abc"],
    );
}

fn symbol_to_string(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "symbol_to_string",
        |b, input| b.iter(|| eosio_numstr::symbol_to_string(black_box(*input))),
        vec![0, 1_397_703_940, 5_138_124_851_399_447_552],
    );
}

criterion_group!(
    benches,
    name_from_str,
    name_to_string,
    symbol_from_str,
    symbol_to_string
);
criterion_main!(benches);
