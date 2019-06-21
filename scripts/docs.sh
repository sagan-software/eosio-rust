#!/usr/bin/env bash

set -e

SOURCE="${BASH_SOURCE[0]}"
DIR="$(dirname "$SOURCE")"
ROOT=$DIR/..
GH_PAGES=$ROOT/gh-pages
TARGET=$ROOT/target

git worktree remove --force $GH_PAGES || exit 0
git worktree add $GH_PAGES gh-pages
rm -Rf $GH_PAGES/*
mdbook build $ROOT/book
cargo doc \
    --all \
    --exclude addressbook \
    --exclude hello \
    --exclude hello_bare \
    --exclude tictactoe \
    --exclude eosio_numstr_macros_impl \
    --exclude eosio_cdt_macros_impl \
    --exclude benchmarks \
    --no-deps

cp -rf $TARGET/doc/* $GH_PAGES/

function build_readme {
    printf "========= Building README.md: %s =========\n" $1
    cargo readme \
        --project-root $1 \
        --output README.md \
        --no-badges \
        --no-license \
        --no-title
}

build_readme eosio
build_readme eosio_bytes
build_readme eosio_bytes_derive
build_readme eosio_core
build_readme eosio_cdt_sys
build_readme eosio_numstr
build_readme eosio_numstr_macros
build_readme eosio_numstr_macros_impl

# cargo bench -p eosio_numstr
# cp -rf $TARGET/criterion gh-pages/benchmarks