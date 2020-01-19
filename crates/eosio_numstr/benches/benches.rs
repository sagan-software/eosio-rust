#[macro_use]
extern crate criterion;
extern crate eosio_numstr;

use criterion::{black_box, Criterion};

fn name_from_bytes(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "name_from_bytes",
        |b, input| {
            b.iter(|| eosio_numstr::name_from_bytes(black_box(input.bytes())))
        },
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

fn name_to_bytes(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "name_to_bytes",
        |b, input| b.iter(|| eosio_numstr::name_to_bytes(black_box(*input))),
        vec![0, 3_458_764_513_820_540_928, 614_251_535_012_020_768],
    );
}

fn symbol_code_from_bytes(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "symbol_code_from_bytes",
        |b, input| {
            b.iter(|| {
                eosio_numstr::symbol_code_from_bytes(black_box(input.bytes()))
            })
        },
        vec!["", "A", "AB", "ABC", "a", "ab", "abc"],
    );
}

fn symbol_code_to_bytes(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "symbol_code_to_bytes",
        |b, input| {
            b.iter(|| eosio_numstr::symbol_code_to_bytes(black_box(*input)))
        },
        vec![0, 1_397_703_940, 5_138_124_851_399_447_552],
    );
}

criterion_group!(
    benches,
    name_from_bytes,
    name_to_bytes,
    symbol_code_from_bytes,
    symbol_code_to_bytes
);
criterion_main!(benches);
