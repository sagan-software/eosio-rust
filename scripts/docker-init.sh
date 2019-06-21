#! /usr/bin/env bash

set -e

SOURCE="${BASH_SOURCE[0]}"
DIR="$(dirname "$SOURCE")"
ROOT=$DIR/..
PUBKEY="EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV"
PRIVKEY="5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3"
TELOS_PUBKEY="TLOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV"

function cleos() {
    docker exec docker_keosd_1 cleos \
        --url http://nodeosd:8888 \
        --wallet-url http://127.0.0.1:8900 \
        "$@"
}

# https://developers.eos.io/eosio-nodeos/docs/bios-boot-sequence
cleos wallet create --to-console
cleos wallet import --private-key $PRIVKEY
cleos create account eosio eosio.bpay $PUBKEY $PUBKEY
cleos create account eosio eosio.msig $PUBKEY $PUBKEY
cleos create account eosio eosio.names $PUBKEY $PUBKEY
cleos create account eosio eosio.ram $PUBKEY $PUBKEY
cleos create account eosio eosio.ramfee $PUBKEY $PUBKEY
cleos create account eosio eosio.saving $PUBKEY $PUBKEY
cleos create account eosio eosio.stake $PUBKEY $PUBKEY
cleos create account eosio eosio.token $PUBKEY $PUBKEY
cleos create account eosio eosio.vpay $PUBKEY $PUBKEY
cleos create account eosio eosio.rex $PUBKEY $PUBKEY
cleos create account eosio eosio.wrap $PUBKEY $PUBKEY
cleos set contract eosio.token eosio.contracts/build/contracts/eosio.token
cleos set contract eosio.msig eosio.contracts/build/contracts/eosio.msig
cleos set contract eosio.wrap eosio.contracts/build/contracts/eosio.wrap
cleos push action eosio.token create '[ "eosio", "1000000000.0000 EOS" ]' -p eosio.token
cleos push action eosio.token issue '[ "eosio", "1000000000.0000 EOS", "memo" ]' -p eosio
cleos set contract eosio eosio.contracts/build/contracts/eosio.system
cleos push action eosio setpriv '[ "eosio.msig", 1 ]' -p eosio@active
cleos push action eosio setpriv '[ "eosio.wrap", 1 ]' -p eosio@active
cleos push action eosio init '[ 0, "4,EOS" ]' -p eosio

function create_account() {
    cleos system newaccount eosio --transfer $1 $PUBKEY \
        --stake-net "100.0000 EOS" \
        --stake-cpu "100.0000 EOS" \
        --buy-ram-kbytes 1000
}

create_account hello
create_account hellobare
create_account hellocpp
create_account tictactoe
create_account alice
create_account bob
create_account carol
create_account dan
create_account addressbook
create_account eosiotkncpp