# Roadmap

_See the [1.0 milestone](https://github.com/sagan-software/eosio-rust/milestone/1) for a full list of fixes and features planned for 1.0._

Listed below are features that are planned for the 1.0 release. The goal is to have a 1.0 release candidate with all these features by Q1 2019, but this may change depending on community feedback and support.

### Unit Testing

_Tracking this feature in [issue #4](https://github.com/sagan-software/eosio-rust/issues/4)_

A proper test suite is crucial for developers to build secure and correct smart contracts.

EOS already supports unit tests for smart contracts (see [`eosio.contracts`](https://github.com/EOSIO/eosio.contracts/tree/master/tests) for an example), so to support this in Rust we will likely need to:

1. Generate more FFI bindings for [EOS libraries](https://github.com/EOSIO/eos/tree/master/libraries/chain/).
2. Create a new `eosio_test` crate that will be a test harness, similar to how [`wasm-bindgen`](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html) uses [`wasm-bindgen-test`](https://github.com/rustwasm/wasm-bindgen/tree/master/crates/test) to support testing in headless browsers.

### ABI Generation

_Tracking this feature in [issue #5](https://github.com/sagan-software/eosio-rust/issues/5)_

Hand-written ABI files are unnecessary and expose developers to risk if they aren't kept updated.

Since we already have `#[eosio::action]` and `#[eosio::table]` attributes, it should be fairly straightforward to implement this feature by detecting these attributes and generating a JSON file.

### ABI to Rust

_Tracking this feature in [issue #6](https://github.com/sagan-software/eosio-rust/issues/6)_

It would be nice to have a CLI command that would generate Rust code from on-chain ABIs. This would make it significantly easier to interact with external contracts through inline actions.

Implementing this feature would require fetching the ABI JSON from an EOS node and creating a Rust file containing the generated tables and actions.

### Schema Migrations

_Tracking this feature in [issue #7](https://github.com/sagan-software/eosio-rust/issues/7)_

Making changes to EOS table fields is currently not a pleasant experience. It can be a fragile error-prone process that involves duplicating code to work with multiple versions of structs. We believe that a better solution can be found by taking inspiration from projects like [Diesel](http://diesel.rs/) and [Django migrations](https://docs.djangoproject.com/en/2.1/topics/migrations/).

Implementing this feature will require significant effort and discovery. This may be a 1.0+ feature.

### RPC API

_Tracking this feature in [issue #8](https://github.com/sagan-software/eosio-rust/issues/8)_

All EOS apps need a way to talk to EOS nodes, to fetch table rows and to send transactions. In order for full-stack Rust-based EOS applications to come to fruition, there needs to be a solid RPC API. In Javascript there is `eosjs`, and something similar should exist for Rust.

Implementing this will be tricky since we need to support browser and server environments.

- For browsers we need to support [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) and [`stdweb`](https://github.com/koute/stdweb/)
- For servers we need to support [`hyper`](https://hyper.rs/guides/client/basic/)

This could get even more complicated if we decide to optionally support [futures](https://github.com/rust-lang-nursery/futures-rs). For the initial release futures will probably be mandatory.

There are a lot of things to consider so this may be a 1.0+ feature.

### `eosio-rust` CLI

_Tracking this feature in [issue #9](https://github.com/sagan-software/eosio-rust/issues/9)_

We already have several features that need CLIs. Consolidating all our CLIs under one CLI will make things simpler for developers and allow us to add new commands later on.

Commands should be implemented to:

- Create a new `eosio-rust` project, e.g. `eosio-rust new`
- Generate an ABI file, e.g. `eosio-rust to-abi`
- Generate Rust from an ABI, e.g. `eosio-rust from-abi`
- Manage table schemas, e.g. `eosio-rust schema`
- Run unit tests, e.g. `eosio-rust test`

### `wasm-bindgen` and `stdweb` Support

_Tracking this feature in [issue #10](https://github.com/sagan-software/eosio-rust/issues/10)_

A big selling point of Rust is its first-class support for WebAssembly and the possibility of writing full-stack web applications in one highly performant language. It would be great if we could use the same structs and functions from our smart contracts in our frontend code as well.

Implementing this may require rethinking some things, specifically traits that are implemented on primitive types like `SecondaryTableKey` seem to be causing some issues.

### `serde` Support

_Tracking this feature in [issue #11](https://github.com/sagan-software/eosio-rust/issues/11)_

Serde is the defacto standard when it comes to serializing and deserializing data. It will be necessary for table structs to support Serde's `Serialize`/`Deserialize` traits in order to implement the RPC API later on.

Implementing this will require writing custom serializers/deserializers for EOS types, for example:

- Booleans are 0 or 1
- Large numbers can sometimes be integers, sometimes be strings
