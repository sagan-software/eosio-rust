#! /bin/bash
echo "Linting..."
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
pushd ${DIR}/..
touch crates/eosio/src/lib.rs || exit 1
touch crates/eosio_macros/src/lib.rs || exit 1
touch crates/eosio_macros_impl/src/lib.rs || exit 1
touch crates/eosio_rpc/src/lib.rs || exit 1
touch crates/eosio_sys/src/lib.rs || exit 1
touch crates/eosio_token/src/lib.rs || exit 1
cargo clippy --release --target=wasm32-unknown-unknown || exit 1
popd