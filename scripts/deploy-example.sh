#!/usr/bin/env bash

set -e

SOURCE="${BASH_SOURCE[0]}"
DIR="$(dirname "$SOURCE")"
ROOT=$DIR/..

function cleos() {
    docker exec docker_keosd_1 cleos \
        --url http://nodeosd:8888 \
        --wallet-url http://127.0.0.1:8900 \
        "$@"
}

function deploy_example_contract {
    printf "========= Deploying example contract: %s =========\n" $1
	cleos set abi $2 mnt/dev/examples/$1/$1.abi.json
	cleos set code $2 mnt/dev/release/$1_gc.wasm
}

. $DIR/build.sh
deploy_example_contract addressbook addressbook
deploy_example_contract hello hello
deploy_example_contract hello_bare hellobare
deploy_example_contract tictactoe tictactoe

function deploy_eosio_contract {
    printf "========= Deploying eosio contract: %s =========\n" $1
	cleos set code $2 mnt/dev/release/$1_gc.wasm
}

deploy_eosio_contract eosio_token eosio.token
deploy_eosio_contract eosio_wrap eosio.wrap