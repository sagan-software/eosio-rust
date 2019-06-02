#!/usr/bin/env bash

set -e

SOURCE="${BASH_SOURCE[0]}"
DIR="$(dirname "$SOURCE")"
ROOT=$DIR/..

cargo fmt --all
touch $ROOT/eosio_abi/src/lib.rs
touch $ROOT/eosio_bytes_derive/src/lib.rs
touch $ROOT/eosio_bytes/src/lib.rs
touch $ROOT/eosio_cdt_macros/src/lib.rs
touch $ROOT/eosio_cdt_macros_impl/src/lib.rs
touch $ROOT/eosio_cdt/src/lib.rs
touch $ROOT/eosio_cli/src/lib.rs
touch $ROOT/eosio_contracts/eosio_system/src/lib.rs
touch $ROOT/eosio_contracts/eosio_token/src/lib.rs
touch $ROOT/eosio_core/src/lib.rs
touch $ROOT/eosio_numstr/src/lib.rs
touch $ROOT/eosio_numstr_macros/src/lib.rs
touch $ROOT/eosio_numstr_macros_impl/src/lib.rs
touch $ROOT/eosio_rpc/src/lib.rs
touch $ROOT/eosio/src/lib.rs
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
    -W clippy::dbg_macro \
    -W clippy::else_if_without_else \
    -W clippy::float_cmp_const \
    -D clippy::indexing_slicing \
    -W clippy::mem_forget \
    -W clippy::missing_docs_in_private_items \
    -W clippy::missing_inline_in_public_items \
    -D clippy::option_unwrap_used \
    -D clippy::result_unwrap_used \
    -D clippy::unimplemented \
    -W clippy::use_debug \
    -D clippy::wrong_pub_self_convention \
    -D clippy::wrong_self_convention