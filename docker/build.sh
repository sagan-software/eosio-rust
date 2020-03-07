set -e
docker build --tag sagansoftware/eos:2.0.3 --file ./eos.dockerfile .
docker build --tag sagansoftware/eosio.cdt:1.7.0 --file ./eosio.cdt.dockerfile .
docker build --tag sagansoftware/eosio.contracts:1.9.1 --file ./eosio.contracts.dockerfile .
