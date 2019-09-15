#!/usr/bin/env bash

set -e

SOURCE="${BASH_SOURCE[0]}"
DIR="$(dirname "$SOURCE")"
ROOT=$DIR/..
TARGET_DIR=$ROOT/target/wasm32-unknown-unknown/release

function build_contract {
    local OUT="${1//-/_}"
    printf "========= Building package: %s (%s) =========\n" $1 ${OUT}
    rm -f \
        ${TARGET_DIR}/${OUT}.wasm \
        ${TARGET_DIR}/${OUT}_gc.wasm \
        ${TARGET_DIR}/${OUT}_gc_opt.wasm \
        ${TARGET_DIR}/${OUT}_gc_opt.wat
    RUSTFLAGS="-C link-args=-zstack-size=48000" \
    cargo build \
        --release \
        --target=wasm32-unknown-unknown \
        --features contract \
        -p $1
    wasm-gc \
        ${TARGET_DIR}/${OUT}.wasm \
        ${TARGET_DIR}/${OUT}_gc.wasm
	# wasm-opt \
    #     -Oz \
    #     --output $TARGET_DIR/$1_gc_opt.wasm \
    #     $TARGET_DIR/$1_gc.wasm
	# wasm2wat \
    #     $TARGET_DIR/$1_gc_opt.wasm \
    #     -o $TARGET_DIR/$1_gc_opt.wat \
    #     --generate-names
    ls -lah ${TARGET_DIR} | grep -e ${OUT}.wasm -e ${OUT}_gc.wasm -e ${OUT}_gc_opt.wasm
}

cargo fmt --all
build_contract addressbook
build_contract hello
build_contract hello-bare
build_contract tictactoe
build_contract eosio-bios
build_contract eosio-forum
build_contract eosio-msig
build_contract eosio-system
build_contract eosio-token
build_contract eosio-wrap
