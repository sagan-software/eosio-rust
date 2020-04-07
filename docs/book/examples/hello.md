# Hello, World!

This example shows how to create a smart contract in Rust that prints a greeting.

## Usage

```sh
cleos push action hello hi '["world"]' -p 'hello@active'
```

## Cargo.toml

```toml
{{#include ../../../examples/hello/Cargo.toml}}
```

## Source

```rust,no_run,noplaypen
{{#include ../../../examples/hello/src/lib.rs}}
```

## ABI

```json
{{#include ../../../examples/hello/hello.abi.json}}
```
