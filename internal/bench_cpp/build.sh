#!/bin/sh

set -e

mkdir -p ./build

winpty docker run --entrypoint eosio-cpp --rm -it sagansoftware/eosio.cdt:1.7.0 -v ../bench_contract_cpp:/bench_contract_cpp \
    -o=/build/bench_cpp.wasm \
    -abigen \
    -abigen_output=/build/bench_cpp.json \
    -contract=hello \
    /src/bench_contract.cpp