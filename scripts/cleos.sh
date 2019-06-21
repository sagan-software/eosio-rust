#!/usr/bin/env bash

set -e

docker exec docker_keosd_1 cleos \
    --url http://nodeosd:8888 \
    --wallet-url http://127.0.0.1:8900 \
    "$@"