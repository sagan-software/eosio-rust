#!/usr/bin/env bash

set -e

SOURCE="${BASH_SOURCE[0]}"
DIR="$(dirname "$SOURCE")"
ROOT=$DIR/..
TARGET_DIR=$ROOT/target/wasm32-unknown-unknown/release

function build_contract {
    printf "========= Building package: %s =========\n" $1
    rm -f \
        $TARGET_DIR/$1.wasm \
        $TARGET_DIR/$1_gc.wasm \
        $TARGET_DIR/$1_gc_opt.wasm \
        $TARGET_DIR/$1_gc_opt.wat
    RUSTFLAGS="-C link-args=-zstack-size=48000" \
    cargo build \
        --release \
        --target=wasm32-unknown-unknown \
        --verbose \
        --features contract \
        -p $1
    wasm-gc \
        $TARGET_DIR/$1.wasm \
        $TARGET_DIR/$1_gc.wasm
	# wasm-opt \
    #     -Oz \
    #     --output $TARGET_DIR/$1_gc_opt.wasm \
    #     $TARGET_DIR/$1_gc.wasm
	# wasm2wat \
    #     $TARGET_DIR/$1_gc_opt.wasm \
    #     -o $TARGET_DIR/$1_gc_opt.wat \
    #     --generate-names
    ls -lah $TARGET_DIR | grep -e $1.wasm -e $1_gc.wasm -e $1_gc_opt.wasm
}

cargo fmt --all
build_contract addressbook
build_contract hello
build_contract hello_bare
build_contract tictactoe
build_contract eosio_token
build_contract eosio_wrap
