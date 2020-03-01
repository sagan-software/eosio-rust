# Hello, World!

This example shows how to create a smart contract in Rust that prints a greeting, **without using the `eosio` crate**. This is used to demonstrate the code that is being abstracted away, and to compare the abstraction overhead.

## Usage

```sh
cleos push action hello hi '["world"]' -p 'hello@active'
```

## Cargo.toml

```toml
{{#include ../../examples/hello_bare/Cargo.toml}}
```

## Source

```rust,no_run,noplaypen
{{#include ../../examples/hello_bare/src/lib.rs}}
```

## ABI

```json
{{#include ../../examples/hello_bare/hello_bare.abi.json}}
```
