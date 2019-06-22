#!/usr/bin/env bash

set -e

SOURCE="${BASH_SOURCE[0]}"
DIR="$(dirname "$SOURCE")"

. $DIR/build.sh

cargo test \
    -p eosio-core \
    -p eosio-core-derive \
    -p eosio-core-macros

docker run \
    --rm \
    --volume /$(pwd)/target/wasm32-unknown-unknown/release/eosio_token_gc.wasm:/eosio.contracts/build/contracts/eosio.token/eosio.token.wasm:ro \
    --volume /$(pwd)/target/wasm32-unknown-unknown/release/eosio_wrap_gc.wasm:/eosio.contracts/build/contracts/eosio.wrap/eosio.wrap.wasm:ro \
    --entrypoint //eosio.contracts/build/tests/unit_test \
    sagansoftware/eos:latest \
    --show_progress=yes \
    --run_test=eosio_token_tests \
    --run_test=eosio_wrap_tests