#! /bin/bash
echo "Building..."
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
pushd ${DIR}/..
cargo build --release --target=wasm32-unknown-unknown || exit 1
popd