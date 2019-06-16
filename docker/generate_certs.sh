#! /usr/bin/env bash

set -e

openssl req \
    -new \
    -x509 \
    -sha256 \
    -newkey rsa:2048 \
    -nodes \
    -keyout key.pem \
    -days 99999 \
    -out cert.pem \
    -subj '//CN=localhost'