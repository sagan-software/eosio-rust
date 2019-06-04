#!/usr/bin/env bash

set -e

SOURCE="${BASH_SOURCE[0]}"
DIR="$(dirname "$SOURCE")"

. $DIR/build.sh

cargo test \
    -p eosio_bytes \
    -p eosio_bytes_derive \
    -p eosio_cdt_sys \
    -p eosio_core \
    -p eosio_numstr \
    -p eosio_numstr_macros

docker run \
    --rm \
    --volume /$(pwd)/target/wasm32-unknown-unknown/release/eosio_token_gc.wasm:/eosio.contracts/build/contracts/eosio.token/eosio.token.wasm:ro \
    --entrypoint //eosio.contracts/build/tests/unit_test \
    sagansoftware/eos:latest \
    --show_progress=yes --run_test=eosio_token_tests