#! /usr/bin/env bash

set -e

docker-compose --file ./docker/docker-compose.yml down
docker volume rm nodeos-data-volume
docker volume rm keosd-data-volume

docker volume create --name=nodeos-data-volume
docker volume create --name=keosd-data-volume
docker-compose --file ./docker/docker-compose.yml up --abort-on-container-exit
