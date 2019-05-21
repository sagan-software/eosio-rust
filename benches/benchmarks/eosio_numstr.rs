use criterion::{black_box, Criterion, ParameterizedBenchmark};

fn string_to_name(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "eosio_numstr::string_to_name",
        |b, input| b.iter(|| eosio_numstr::string_to_name(black_box(input))),
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
        "eosio_numstr::name_to_string",
        |b, input| b.iter(|| eosio_numstr::name_to_string(black_box(*input))),
        vec![0, 3_458_764_513_820_540_928, 614_251_535_012_020_768],
    );
}

fn string_to_symbol(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "eosio_numstr::string_to_symbol",
        |b, input| b.iter(|| eosio_numstr::string_to_name(black_box(input))),
        vec!["", "A", "AB", "ABC", "a", "ab", "abc"],
    );
}

criterion_group!(benches, string_to_name, name_to_string, string_to_symbol);
