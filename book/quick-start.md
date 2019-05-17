# Quick Start

Create the project:

```sh
cargo +nightly new hello --lib
cd hello
```

File `Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
eosio = "0.2"

[profile.release]
lto = true
```

File `.cargo/config`:

```toml
[target.wasm32-unknown-unknown]
rustflags = [
  "-C", "link-args=-z stack-size=48000"
]
```

File `src/lib.rs`:

```rust
use eosio::*;

#[eosio_action]
fn hi(name: AccountName) {
    eosio_print!("Hello, ", name);
}

eosio_abi!(hi);
```

File `hello.abi.json`:

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

Compile and minify the smart contract (requires [optional dependencies](#optional-dependencies)):

```sh
cargo build --release --target=wasm32-unknown-unknown
wasm-gc target/wasm32-unknown-unknown/release/hello.wasm hello_gc.wasm
wasm-opt hello_gc.wasm --output hello_gc_opt.wasm -Oz
```

Deploying the smart contract will depend on how you have your EOSIO node setup. Assuming you followed the `docker-compose` instructions above, run these commands in a terminal:

```sh
alias cleos='docker-compose exec keosd cleos --url http://nodeosd:8888 --wallet-url http://127.0.0.1:8900'

# Create a wallet and the 'hello' account
PUBKEY=EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV
PRIVKEY=5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3
cleos wallet create --to-console
cleos wallet import --private-key $PRIVKEY
cleos create account eosio hello $PUBKEY $PUBKEY

# Deploy the ABI and WASM files
cleos set abi hello /mnt/dev/project/hello.abi.json
cleos set code hello /mnt/dev/project/hello_gc_opt.wasm

# Say hello
cleos push action hello hi '["world"]' -p 'hello@active'
```

If all went well you should see `Hello, world` in the console.
