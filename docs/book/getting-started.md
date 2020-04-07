# Getting Started

In this section we will walk through writing the [`hello` C++ example](https://github.com/EOSIO/eosio.cdt/tree/master/examples/hello) in Rust. In this example you will learn how to setup and optimize a basic smart contract, accept an input, and print to the console.

#### Creating the project

First, let's create a new project with Cargo and change directories:

```sh
cargo +nightly new hello --lib
cd hello
```

You should now have a directory that looks like this:

```
src/
  lib.rs
Cargo.toml
```

#### Configuring Cargo

The `Cargo.toml` file is used by Rust to manage dependencies and other configuration options. If you open this file now it should look similar to this:

```toml
[package]
name = "hello"
version = "0.1.0"
authors = []
edition = "2018"

[dependencies]
```

Let's change this to add `eosio` as a dependency, and change the crate type so that Rust generates a `.wasm` file:

```toml
[package]
name = "hello"
version = "0.1.0"
authors = []
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
eosio = "0.3"
eosio_cdt = "0.3"
```

#### Generating and Optimizing a WASM File

At this point we can compile our project to produce a `.wasm` file:

```sh
cargo build --release --target=wasm32-unknown-unknown
```

You should now see a generated file at `target/wasm32-unknown-unknown/release/hello.wasm`:

```
$ ls -lh target/wasm32-unknown-unknown/release | grep wasm
-rwxr-xr-x 2 sagan sagan 1.9M Oct 25 16:34 hello.wasm
```

This file will be huge at almost 2MB! But we can _significantly_ reduce file size by enabling link-time optimization. Add this to the bottom of our `Cargo.toml` file:

```toml
[profile.release]
lto = true
```

Now if we rebuild the project we should see a much smaller `.wasm` file:

```
$ cargo build --release --target=wasm32-unknown-unknown
$ ls -lh target/wasm32-unknown-unknown/release | grep wasm
-rwxr-xr-x 2 sagan sagan  52K Oct 25 16:48 hello_world.wasm
```

That's better, but 52KB is still heavy for an empty smart contract. Luckily we can use `wasm-gc` and `wasm-opt` to reduce the file size even further:

```sh
wasm-gc target/wasm32-unknown-unknown/release/hello.wasm hello_gc.wasm
wasm-opt hello_gc.wasm --output hello_gc_opt.wasm -Oz
```

```
$ ls -lh | grep wasm
-rw-r--r-- 1 sagan sagan  109 Oct 25 16:57 hello_gc_opt.wasm
-rw-r--r-- 1 sagan sagan  116 Oct 25 16:56 hello_gc.wasm
```

By using `wasm-gc` and `wasm-opt` we are able to get the file size down to just over 100 bytes! But this is before we've added any code. Realistically you can expect simple contracts to be under 15KB.

#### Writing the Smart Contract

Now that we know how to prepare the `.wasm` file, let's start coding. Open up `src/lib.rs` and replace its contents with this:

```rust
use eosio::AccountName;
use eosio_cdt::{
    abi,
    print
};

#[eosio::action]                  // Mark this function as an action
fn hi(name: AccountName) {
    print!("Hello, {:?}", name);  // Print to the console
}

abi!(hi);                         // Create the 'apply' function
```

See the [API documentation](https://sagan-software.github.io/eosio-rust/) for more details on what this code is doing.

Let's recompile our project and minify the the WASM file again:

```sh
cargo build --release --target=wasm32-unknown-unknown
wasm-gc target/wasm32-unknown-unknown/release/hello.wasm hello_gc.wasm
wasm-opt hello_gc.wasm --output hello_gc_opt.wasm -Oz
```

#### Creating the ABI File

In the future ABI files will be [automatically generated](#abi-generation), but for now they must be typed out manually. Copy this code into a file called `hello.abi.json`:

```json
{
  "version": "eosio::abi/1.0",
  "types": [],
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
      "type": "hi",
      "ricardian_contract": ""
    }
  ],
  "tables": [],
  "ricardian_clauses": [],
  "abi_extensions": []
}
```

#### Deploying, First Attempt

At this point we have our WASM and ABI files, so we're ready to deploy our smart contract.

For this you will need to have access to an EOS node. Please see the [Installing EOS](#installing-eos) section for instructions. The code below will assume that you've started `nodeos` and `keosd` containers using Docker Compose.

First create an alias for `cleos`:

```sh
alias cleos='docker-compose exec keosd cleos --url http://nodeosd:8888 --wallet-url http://127.0.0.1:8900'
```

Then create a wallet, import a private key, and create the `hello` account:

```sh
PUBKEY=EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV
PRIVKEY=5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3
cleos wallet create --to-console
cleos wallet import --private-key $PRIVKEY
cleos create account eosio hello $PUBKEY $PUBKEY
```

Deploy the ABI:

```sh
cleos set abi hello /mnt/dev/project/hello.abi.json
```

Deploy the WASM:

```sh
cleos set code hello /mnt/dev/project/hello_gc_opt.wasm
```

...but this will **fail** with an error!

```
$ cleos set code hello /mnt/dev/project/hello_gc_opt.wasm
Reading WASM from /mnt/dev/project/hello_gc_opt.wasm...
Setting Code...
Error 3070002: Runtime Error Processing WASM
```

In the `nodeos` console log you will see this error message:

```
error 2018-10-26T03:46:12.176 thread-0  http_plugin.cpp:580           handle_exception     ] FC Exception encountered while processing chain.push_transaction
debug 2018-10-26T03:46:12.176 thread-0  http_plugin.cpp:581           handle_exception     ] Exception Details: 3070002 wasm_execution_error: Runtime Error Processing WASM
Smart contract data segments must lie in first 64KiB
     {"k":64}
     thread-0  wasm_eosio_validation.cpp:30 validate
pending console output:
     {"console":""}
     thread-0  apply_context.cpp:72 exec_one
```

This is happening because Rust by default reserves 1MB for the stack, but EOS expects data to be [within the first 64KB](https://github.com/EOSIO/eos/issues/5604).

#### Deploying, Second Attempt

We can fix this by telling the Rust compiler to reserve less than 64KB for the stack. Create a new file at `.cargo/config` with these contents:

```toml
[target.wasm32-unknown-unknown]
rustflags = [
  "-C", "link-args=-z stack-size=48000"
]
```

48KB seems to be a reasonable number, but feel free to experiment.

Now let's try to rebuild and redeploy our contract:

```sh
cargo build --release --target=wasm32-unknown-unknown
wasm-gc target/wasm32-unknown-unknown/release/hello.wasm hello_gc.wasm
wasm-opt hello_gc.wasm --output hello_gc_opt.wasm -Oz
cleos set code hello /mnt/dev/project/hello_gc_opt.wasm
```

Finally, say hello:

```sh
cleos push action hello hi '["world"]' -p 'hello@active'
```

#### Success!

If all went well you should see `Hello, world!` in the console. Otherwise, if the transaction was sent successfully but you don't see any output, you may need to use the `--contract-console` option with `nodeos`