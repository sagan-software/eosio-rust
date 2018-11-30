#! /bin/bash
echo "Testing..."
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
pushd ${DIR}/..
cargo test -p eosio -p eosio_macros -p eosio_sys
popd