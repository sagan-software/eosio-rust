# Quick Start

In this quick-start tutorial we will create a simple EOSIO smart contract in Rust that accepts an account name and prints a greeting message.

Create a new Rust library:

```sh
cargo new hello --lib
```

Edit `Cargo.toml`:

```toml
{{#include ../examples/hello/Cargo.toml}}
```

Edit `src/lib.rs`:

```rust,no_run,noplaypen
{{#include ../examples/hello/src/lib.rs}}
```

Compile with the following command:

```sh
RUSTFLAGS="-C link-args=-zstack-size=48000" \
cargo build --release -target=wasm32-unknown-unknown
```

The smart contract should now be built at `target/wasm32-unknown-unknown/release/hello.wasm`

## Deploying


Create a new file called `abi.json` (in future versions this will be automatically generated):

```json
{
    "version": "eosio::abi/1.0",
    "structs": [
        {
            "name": "hi",
            "base": "",
            "fields": [
                {
                    "name": "name",
                    "type": "name"
                }
            ]
        }
    ],
    "actions": [
        {
            "name": "hi",
            "type": "hi"
        }
    ]
}
```

Assuming you have `cleos` setup and have created the `hello` account:

```sh
cleos set abi hello abi.json
cleos set code hello target/wasm32-unknown-unknown/release/hello.wasm
```

## Say Hello

Finally, say hello:

```sh
cleos push action hello hi '["world"]' -p 'hello@active'
```

If all went well you should see `Hello, world` in the console. Otherwise, if the transaction was sent successfully but you don't see any output, you may need to use the `--contract-console` option with `nodeos`.