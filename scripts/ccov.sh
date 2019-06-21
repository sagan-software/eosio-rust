#!/usr/bin/env bash

set -e

SOURCE="${BASH_SOURCE[0]}"
DIR="$(dirname "$SOURCE")"
ROOT=$DIR/..

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
cargo +nightly build --verbose
cargo +nightly test --verbose
rm -f ccov.zip
zip -0 ccov.zip `find $ROOT \( -name "eosio*.gc*" \) -print`
grcov ccov.zip -s . -t lcov --llvm --branch --ignore-not-existing --ignore-dir "/*" -o lcov.info
bash <(curl -s https://codecov.io/bash) -f lcov.info