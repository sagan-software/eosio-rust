<img src="logo.svg" alt="eosio-rs" width="300"/>

# eosio-rs

**Rust crates for building full-stack smart contract applications on EOSIO blockchains.**

> **DISCLAIMER:** _This project is in early development. All APIs and features should be considered unstable until version `1.0` is released. This code has not been security audited and is not yet suitable for production environments. Thank you._

[![Build Status](https://travis-ci.org/paritytech/parity-wasm.svg?branch=master)](https://travis-ci.org/paritytech/parity-wasm)
[![crates.io link](https://img.shields.io/crates/v/eosio.svg)](https://crates.io/crates/eosio)

- [Website][website]
- [Documentation][docs]
- [Telegram][telegram]

## Quick Start

`Cargo.toml`:

```toml
[dependencies]
eosio = "0.1"
```

`src/lib.rs`:

```rust
#![feature(proc_macro_hygiene)]

extern crate eosio;

use eosio::*;

#[eosio_action]
fn hi(name: AccountName) {
    eosio_print!("Hi, ", name);
}

eosio_abi!(hi);
```

## Getting Started

To get started writing EOSIO smart contracts in Rust, please read our [getting started guide][guide] and/or look at examples in the [`examples`](examples) directory.

## Getting Help

If you run into problems please join our [Telegram group][telegram] and ask for help.

## License

Licensed under either of these:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  https://opensource.org/licenses/MIT)

### Contributing

Unless you explicitly state otherwise, any contribution you intentionally submit
for inclusion in the work, as defined in the Apache-2.0 license, shall be
dual-licensed as above, without any additional terms or conditions.

[guide]: https://sagan-software.github.io/rust-eos/
[telegram]: https://t.me/SaganSoftware
[website]: https://sagan-software.github.io/rust-eos/
[docs]: https://sagan-software.github.io/rust-eos/docs/
