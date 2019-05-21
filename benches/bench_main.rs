#[macro_use]
extern crate criterion;

mod benchmarks;

criterion_main! {
    benchmarks::eosio_numstr::benches,
}
