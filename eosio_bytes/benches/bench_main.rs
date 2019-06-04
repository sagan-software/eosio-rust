#[macro_use]
extern crate criterion;
extern crate eosio_bytes;

use criterion::{black_box, Criterion};
use eosio_bytes::*;

fn u8_write(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "u8::write",
        |b, input| b.iter(|| black_box(input).write(&mut [0_u8; 1], &mut 0)),
        vec![0_u8, 100, 255],
    );
}

fn vec_u8_write(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "Vec<u8>::write",
        |b, input| b.iter(|| black_box(input).write(&mut [0_u8; 4], &mut 0)),
        vec![vec![], vec![0_u8], vec![0_u8, 100], vec![0_u8, 100, 255]],
    );
}

fn str_write(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "&str::write",
        |b, input| b.iter(|| black_box(input).write(&mut [0_u8; 13], &mut 0)),
        vec!["", "Hello", "Hello World!"],
    );
}

criterion_group!(benches, u8_write, vec_u8_write, str_write,);
criterion_main!(benches);
