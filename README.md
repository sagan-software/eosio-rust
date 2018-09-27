![eosiolib.rs](logo.png)

## Libraries for building EOSIO smart contracts in Rust

![Travis (.org)](https://img.shields.io/travis/rust-lang/rust.svg?style=flat-square)
![AppVeyor tests](https://img.shields.io/appveyor/tests/NZSmartie/coap-net-iu0to.svg?style=flat-square)
![Crates.io](https://img.shields.io/crates/v/rustc-serialize.svg?style=flat-square)

- [Homepage](#)
- [Documentation](#)
- [Telegram](#)

## Quickstart

```rust
#![feature(proc_macro_non_items)]

extern crate eosio;

use eosio::prelude::*;

#[eosio_action]
fn hi(name: AccountName) {
    eosio_print!("Hi, ", name);
}

eosio_abi!(hi);
```

## Getting Started

Guides and getting started documentation coming soon.

## Getting Help

If you run into problems please join our [Telegram group](https://t.me/SaganSoftware) and ask for help.

## License

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

### Contributing

Unless you explicitly state otherwise, any contribution you intentionally submit
for inclusion in the work, as defined in the Apache-2.0 license, shall be
dual-licensed as above, without any additional terms or conditions.