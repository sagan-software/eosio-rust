#!/bin/bash

command_exists () {
  type "$1" &> /dev/null ;
}

if ! command_exists bindgen; then
    cargo install bindgen
fi

bindgen \
    --distrust-clang-mangling \
    --no-layout-tests \
    --output src/bindings.rs \
    --ctypes-prefix ::ctypes \
    wrapper.hpp \
    -- \
    -I ../external/eosio.cdt/libraries/boost/include \
    -I ../external/eosio.cdt/libraries/libc++/libcxx/include \
    -I ../external/eosio.cdt/libraries/libc/musl/include \
    -I ../external/eosio.cdt/libraries \
    --std=c++14