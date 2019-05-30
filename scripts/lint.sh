#!/usr/bin/env bash

set -e

SOURCE="${BASH_SOURCE[0]}"
DIR="$(dirname "$SOURCE")"
ROOT=$DIR/..

cargo fmt --all
touch $ROOT/eosio/src/lib.rs
touch $ROOT/eosio_abi/src/lib.rs
touch $ROOT/eosio_bytes/src/lib.rs
touch $ROOT/eosio_bytes_derive/src/lib.rs
touch $ROOT/eosio_cdt/src/lib.rs
touch $ROOT/eosio_cdt_macros/impl/src/lib.rs
touch $ROOT/eosio_cdt_macros/src/lib.rs
touch $ROOT/eosio_cli/src/lib.rs
touch $ROOT/eosio_contracts/eosio_system/src/lib.rs
touch $ROOT/eosio_contracts/eosio_token/src/lib.rs
touch $ROOT/eosio_core/src/lib.rs
touch $ROOT/eosio_numstr/src/lib.rs
touch $ROOT/eosio_numstr_macros/impl/src/lib.rs
touch $ROOT/eosio_numstr_macros/src/lib.rs
touch $ROOT/eosio_rpc/src/lib.rs
touch $ROOT/examples/addressbook/src/lib.rs
touch $ROOT/examples/hello_bare/src/lib.rs
touch $ROOT/examples/hello/src/lib.rs
touch $ROOT/examples/tictactoe/src/lib.rs
cargo clippy -- \
    -W clippy::complexity \
    -D clippy::correctness \
    -W clippy::pedantic \
    -W clippy::nursery \
    -W clippy::style \
    -W clippy::perf \
    -W clippy::cargo \
    -W clippy::restriction