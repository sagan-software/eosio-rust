# Tic-Tac-Toe

This example shows how to create a smart contract that reads and writes tables.

## Usage

```sh
cleos push action tictactoe create '["alice","bob"]' -p 'alice@active'
cleos push action tictactoe makemove '["alice","bob",1,0,1]' -p 'alice@active'
cleos push action tictactoe restart '["alice","bob",1]' -p 'alice@active'
cleos push action tictactoe close '["alice","bob"]' -p 'alice@active'
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
