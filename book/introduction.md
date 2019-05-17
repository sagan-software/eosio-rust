# Introduction

> **DISCLAIMER:** _This project is in early development and we looking for feedback on all APIs and features. All APIs and features should be considered unstable and insecure until version `1.0` is released. This code is not yet suitable for production environments where user funds are at risk. Thank you._

This project intends to enable developers to write full-stack EOSIO applications using the Rust programming language. We believe Rust is an excellent choice for EOSIO smart contract development with its focus on safety, speed, and WebAssembly. Furthermore, projects like [wasm-bindgen][wasm_bindgen] and [stdweb] make it possible to write full-stack Rust web applications, limiting the need for Javascript and enabling code reuse between browsers, servers, and smart contracts.

The primary goals of this project are to provide Rust crates that:

- Enable developers to write secure EOSIO smart contracts.
- Streamline the development of full-stack EOSIO web applications.
- Simplify managing and updating EOSIO table schemas.
- Allow developers to publish reusable smart contract code.

[wasm_bindgen]: https://github.com/rustwasm/wasm-bindgen/
[stdweb]: https://github.com/koute/stdweb/
[eosio]: https://sagan-software.github.io/rust-eos/eosio/
[eosio_macros]: https://sagan-software.github.io/rust-eos/eosio_macros/
[eosio_sys]: https://sagan-software.github.io/rust-eos/eosio_sys/
