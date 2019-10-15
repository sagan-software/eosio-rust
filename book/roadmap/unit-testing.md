# Unit Testing

_Tracking this feature in [issue #4](https://github.com/sagan-software/eosio-rust/issues/4)_

A proper test suite is crucial for developers to build secure and correct smart contracts.

EOS already supports unit tests for smart contracts (see [`eosio.contracts`](https://github.com/EOSIO/eosio.contracts/tree/master/tests) for an example), so to support this in Rust we will likely need to:

1. Generate more FFI bindings for [EOS libraries](https://github.com/EOSIO/eos/tree/master/libraries/chain/).
2. Create a new `eosio_test` crate that will be a test harness, similar to how [`wasm-bindgen`](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html) uses [`wasm-bindgen-test`](https://github.com/rustwasm/wasm-bindgen/tree/master/crates/test) to support testing in headless browsers.
