<img src="logo.svg" alt="rust-eos" width="300"/>

# rust-eos [![Build Status](https://travis-ci.org/paritytech/parity-wasm.svg?branch=master)](https://travis-ci.org/paritytech/parity-wasm) [![crates.io link](https://img.shields.io/crates/v/eosio.svg)](https://crates.io/crates/eosio)

**Rust crates for building full-stack smart contract applications on EOSIO blockchains.**

| Crate                          | Description                                                  |
| ------------------------------ | ------------------------------------------------------------ |
| [`eosio`][eosio]               | Library for building EOSIO smart contracts.                  |
| [`eosio_macros`][eosio_macros] | Procedural macros for EOSIO smart contract development.      |
| [`eosio_sys`][eosio_sys]       | Low-level FFI bindings for EOSIO smart contract development. |

> **DISCLAIMER:** _This project is in early development and we looking for feedback on all APIs and features. All APIs and features should be considered unstable and insecure until version `1.0` is released. This code is not yet suitable for production environments where user funds are at risk. Thank you._

This project enables developers to write full-stack EOSIO applications using the Rust programming language. We believe Rust is an excellent choice for EOSIO smart contract development with its focus on safety, speed, and WebAssembly. Furthermore, projects like [wasm-bindgen][wasm_bindgen] and [stdweb] make it possible to write full-stack Rust web applications, limiting the need for Javascript and enabling code reuse between browsers, servers, and smart contracts.

The primary goals of this project are to provide Rust crates that:

- Enable developers to write secure EOSIO smart contracts.
- Streamline the development of full-stack EOSIO web applications.
- Simplify managing and updating EOSIO table schemas.
- Allow developers to publish reusable smart contract code.

For a detailed look at planned features please see our [roadmap](#roadmap).

[wasm_bindgen]: https://github.com/rustwasm/wasm-bindgen/
[stdweb]: https://github.com/koute/stdweb/
[eosio]: https://sagan-software.github.io/rust-eos/eosio/
[eosio_macros]: https://sagan-software.github.io/rust-eos/eosio_macros/
[eosio_sys]: https://sagan-software.github.io/rust-eos/eosio_sys/

## Table of Contents

- [Getting Help](#getting-help)
- [Installation](#installation)
  - [Installing Rust](#installing-rust)
  - [Installing EOS](#installing-eos)
  - [Optional Dependencies](#optional-dependencies)
- [Quickstart](#quickstart)
- [Getting Started](#getting-started)
- [Examples](#examples)
- [Roadmap](#roadmap)
  - [Unit Testing](#unit-testing)
  - [ABI Generation](#abi-generation)
  - [ABI to Rust](#abi-to-rust)
  - [Schema Migration](#schema-migration)
  - [RPC API](#rpc-api)
  - [`rust-eos` CLI](#rust-eos-cli)
  - [`wasm-bindgen` and `stdweb` Support](#wasm-bindgen-and-stdweb-support)
  - [`serde` Support](#serde-support)
- [License](#license)
- [Contributing](#contributing)

---

## Getting Help

If you find a bug or think of an improvement please [open an issue] and let us know!

Otherwise if you are stuck on something or run into problems, here are some resources that could help:

- For questions about this project:
  - [Join our Telegram group][telegram]
  - [Open an issue]
- For questions about EOS development:
  - [Join the EOS Developers Telegram group](https://t.me/joinchat/Esi1OkPktgcFeJ3Lmlcrqg)
  - [Ask on the EOS.IO Stack Exchange](https://eosio.stackexchange.com/)
  - [Ask on r/eosdev](https://www.reddit.com/r/eosdev)
- For questions about Rust:
  - [Join Mozilla's IRC server](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-beginners)
  - [Ask on the Rust user forums](https://users.rust-lang.org/)
  - [Ask on r/rust](https://www.reddit.com/r/rust)

[open an issue]: https://github.com/sagan-software/rust-eos/issues/new

## Installation

### Installing Rust

Install Rust with `rustup` per the [official instructions](https://www.rust-lang.org/en-US/install.html):

```sh
curl https://sh.rustup.rs -sSf | sh
```

This project requires nightly Rust and the `wasm32-unknown-unknown` target to be available, which can be installed with `rustup`:

```sh
rustup install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
rustup default nightly
```

### Installing EOS

An EOS node is required to deploy and test smart contracts. The easiest way to setup a node is to use Docker. See the [official Docker quickstart guide](https://developers.eos.io/eosio-nodeos/docs/docker-quickstart) for instructions.

We recommend using `docker-compose` to manage `nodeos` and `keosd` containers. You can download the official [`docker-compose-latest.yml`](https://raw.githubusercontent.com/EOSIO/eos/master/Docker/docker-compose-latest.yml) file and start the containers using these commands:

```sh
wget https://raw.githubusercontent.com/EOSIO/eos/master/Docker/docker-compose-latest.yml
docker volume create --name=nodeos-data-volume
docker volume create --name=keosd-data-volume
docker-compose -f docker-compose-latest.yml up
```

**Note #1!** If you are using `cleos` within a Docker container, you need to mount your project directory as a volume so that `cleos` can deploy your files. If you're using Docker Compose, add your project directory to the `volumes` section of the `keosd` container like so (abbreviated):

```yaml
services:
  keosd:
    volumes:
      - ./:mnt/dev/project:ro
```

**Note #2!** If you are expecting to see console output from `nodeos` then be sure to add `--contracts-console` to the end of the `nodeosd` command like so (abbreviated):

```yaml
services:
  nodeosd:
    command: /opt/eosio/bin/nodeosd.sh ... --contracts-console
```

### Optional Dependencies

#### wasm-gc

[wasm-gc](https://github.com/alexcrichton/wasm-gc) is a command-line tool that removes unused code in WASM files. It can be installed with Cargo:

```sh
cargo install wasm-gc
```

#### Binaryen

[Binaryen](https://github.com/WebAssembly/binaryen) comes with a command-line tool called `wasm-opt` that optimizes WASM file sizes. Binaryen can be installed with most system package managers.

#### WebAssembly Binary Toolkit (WABT)

[WABT](https://github.com/WebAssembly/wabt) comes with a command-line tool `wasm2wat` that can be used to create textual representations of WASM files, which can be useful for debugging. WABT can be installed with most system package managers.

## Quickstart

_See the [Getting Started](#getting-started) section for a more thorough explanation of the code below._

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
eosio = "0.1"

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
#![feature(proc_macro_hygiene)]

extern crate eosio;

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

## Getting Started

In this section we will walk through writing the [`hello` C++ example](https://github.com/EOSIO/eosio.cdt/tree/master/examples/hello) in Rust. In this example you will learn how to setup and optimize a basic smart contract, accept an input, and print to the console.

#### Create the project

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

#### Configure Cargo

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
name = "hello-world"
version = "0.1.0"
authors = []
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
eosio = "0.1"
```

#### Generate and Optimize a WASM File

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

By using `wasm-gc` and `wasm-opt` we are able to get the file size down to just over 100 bytes, but this is before we've added any code. Realistically you can expect simple contracts to be under 15KB.

#### Writing the Smart Contract

Now that we know how to prepare the `.wasm` file, let's start coding. Open up `src/lib.rs` and replace its contents with this:

```rust
#![feature(proc_macro_hygiene)]     // Required for procedural macros

extern crate eosio;                 // Declare that we are using the eosio crate

use eosio::*;                       // Include everything from the eosio crate

#[eosio_action]                     // Mark this function as an action
fn hi(name: AccountName) {
    eosio_print!("Hello, ", name);  // Print to the console
}

eosio_abi!(hi);                     // Create the 'apply' function
```

See the [API documentation](https://sagan-software.github.io/rust-eos/) for more details on what this code is doing.

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

If all went well you should see `Hello, world` in the console. Otherwise, if the transaction was sent successfully but you don't see any output, you may need to use the `--contract-console` option with `nodeos`.

## Examples

Examples can be found in the [`examples`](examples) directory. The equivalent C++ code has been provided where possible.

| Directory                             | Description                                                            |
| ------------------------------------- | ---------------------------------------------------------------------- |
| [`hello`](examples/hello)             | The most basic contract using the `eosio` crate.                       |
| [`hello_bare`](examples/hello_bare)   | A bare bones version of the `hello` contract without any dependencies. |
| [`tictactoe`](examples/tictactoe)     | An example of how to interact with EOSIO tables.                       |
| [`addressbook`](examples/addressbook) | An example of how to interact with tables using secondary indexes.     |
| [`eosio_token`](examples/eosio_token) | The standard `eosio.token` contract ported to Rust.                    |

## Roadmap

_See the [1.0 milestone](https://github.com/sagan-software/rust-eos/milestone/1) for a full list of fixes and features planned for 1.0._

Listed below are features that are planned for the 1.0 release. The goal is to have a 1.0 release candidate with all these features by Q1 2019, but this may change depending on community feedback.

### Unit Testing

_Tracking this feature in [issue #4](https://github.com/sagan-software/rust-eos/issues/4)_

A proper test suite is crucial for developers to build secure and correct smart contracts.

EOS already supports unit tests for smart contracts (see [`eosio.contracts`](https://github.com/EOSIO/eosio.contracts/tree/master/tests) for an example), so to support this in Rust we will likely need to:

1. Generate more FFI bindings for [EOS libraries](https://github.com/EOSIO/eos/tree/master/libraries/chain/).
2. Create a new `eosio_test` crate that will be a test harness, similar to how [`wasm-bindgen`](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html) uses [`wasm-bindgen-test`](https://github.com/rustwasm/wasm-bindgen/tree/master/crates/test) to support testing in headless browsers.

### ABI Generation

_Tracking this feature in [issue #5](https://github.com/sagan-software/rust-eos/issues/5)_

Hand-written ABI files are unnecessary and expose developers to risk if they aren't kept up-to-date.

Since we already have `#[eosio_action]` and `#[eosio_table]` attributes, it should be fairly straightforward to implement this feature by detecting these attributes and generating a JSON file.

### ABI to Rust

_Tracking this feature in [issue #6](https://github.com/sagan-software/rust-eos/issues/6)_

It would be nice to have a CLI command that would generate Rust code from on-chain ABIs. This would make it significantly easier to interact with external contracts through inline actions.

Implementing this feature would require fetching the ABI JSON from an EOS node and creating a Rust file containing the generated tables and actions.

### Schema Migrations

_Tracking this feature in [issue #7](https://github.com/sagan-software/rust-eos/issues/7)_

Making changes to EOS table fields is currently not a pleasant experience. It can be a fragile error-prone process that involves duplicating code to work with multiple versions of structs. We believe that a better solution can be found by taking inspiration from projects like [Diesel](http://diesel.rs/) and [Django migrations](https://docs.djangoproject.com/en/2.1/topics/migrations/).

Implementing this feature will require significant effort and discovery. This may be a 1.0+ feature.

### RPC API

_Tracking this feature in [issue #8](https://github.com/sagan-software/rust-eos/issues/8)_

All EOS apps need a way to talk to EOS nodes, to fetch table rows and to send transactions. In order for full-stack Rust-based EOS applications to come to fruition, there needs to be a solid RPC API. In Javascript there is `eosjs`, and something similar should exist for Rust.

Implementing this will be a little tricky since we need to support browsers and server environments. In the browser we will need to support [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) and [`stdweb`](https://github.com/koute/stdweb/), and on the server we should support [`hyper`](https://hyper.rs/guides/client/basic/). This could get even more complicated if we decide to optionally support futures. This may be a 1.0+ feature.

### `rust-eos` CLI

_Tracking this feature in [issue #9](https://github.com/sagan-software/rust-eos/issues/9)_

We already have several features that need CLIs. Consolidating all our CLIs under one CLI will make things simpler for developers and allow us to add new commands later on.

Commands should be implemented to:

- Create a new `rust-eos` project, e.g. `rust-eos new`
- Generate an ABI file, e.g. `rust-eos to-abi`
- Generate Rust from an ABI, e.g. `rust-eos from-abi`
- Manage table schemas, e.g. `rust-eos schema`
- Run unit tests, e.g. `rust-eos test`

### `wasm-bindgen` and `stdweb` Support

_Tracking this feature in [issue #10](https://github.com/sagan-software/rust-eos/issues/10)_

A big selling point of Rust is its first-class support for WebAssembly and the possibility of writing full-stack web applications in one highly performant language. It would be great if we could use the same structs and functions from our smart contracts in our frontend code as well.

Implementing this may require rethinking some things, specifically traits that are implemented on primitive types like `SecondaryTableKey` seem to be causing some issues.

### `serde` Support

_Tracking this feature in [issue #11](https://github.com/sagan-software/rust-eos/issues/11)_

Serde is the defacto standard when it comes to serializing and deserializing data. It will be necessary for table structs to support Serde's `Serialize`/`Deserialize` traits in order to implement the RPC API later on.

Implementing this will require writing custom serializers/deserializers for EOS types, for example:

- Booleans are 0 or 1
- Large numbers can sometimes be integers, sometimes be strings

## License

Licensed under either of these:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  https://opensource.org/licenses/MIT)

## Contributing

Unless you explicitly state otherwise, any contribution you intentionally submit
for inclusion in the work, as defined in the Apache-2.0 license, shall be
dual-licensed as above, without any additional terms or conditions.

[guide]: https://sagan-software.github.io/rust-eos/
[telegram]: https://t.me/SaganSoftware
[website]: https://sagan-software.github.io/rust-eos/
[docs]: https://sagan-software.github.io/rust-eos/docs/
