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
    --ctypes-prefix crate::ctypes \
    --with-derive-default \
    --with-derive-eq \
    --with-derive-hash \
    --with-derive-ord \
    --with-derive-partialeq \
    --with-derive-partialord \
    --whitelist-function action_data_size \
    --whitelist-function current_receiver \
    --whitelist-function has_auth \
    --whitelist-function is_account \
    --whitelist-function publication_time \
    --whitelist-function read_action_data \
    --whitelist-function require_auth \
    --whitelist-function require_auth2 \
    --whitelist-function require_read_lock \
    --whitelist-function require_recipient \
    --whitelist-function require_write_lock \
    --whitelist-function send_context_free_inline \
    --whitelist-function send_inline \
    --whitelist-function get_active_producers \
    --whitelist-function assert_recover_key \
    --whitelist-function assert_ripemd160 \
    --whitelist-function assert_sha1 \
    --whitelist-function assert_sha256 \
    --whitelist-function assert_sha512 \
    --whitelist-function recover_key \
    --whitelist-function ripemd160 \
    --whitelist-function sha1 \
    --whitelist-function sha256 \
    --whitelist-function sha512 \
    --whitelist-function db_.+ \
    --whitelist-function check_permission_authorization \
    --whitelist-function check_transaction_authorization \
    --whitelist-function get_account_creation_time \
    --whitelist-function get_permission_last_used \
    --whitelist-function print.* \
    --whitelist-function activate_feature \
    --whitelist-function get_blockchain_parameters_packed \
    --whitelist-function get_resource_limits \
    --whitelist-function is_privileged \
    --whitelist-function set_active_producers \
    --whitelist-function set_blockchain_parameters_packed \
    --whitelist-function set_privileged \
    --whitelist-function set_proposed_producers \
    --whitelist-function set_resource_limits \
    --whitelist-function current_time \
    --whitelist-function eosio_.+ \
    --whitelist-function cancel_deferred \
    --whitelist-function expiration \
    --whitelist-function get_action \
    --whitelist-function get_context_free_data \
    --whitelist-function read_transaction \
    --whitelist-function send_deferred \
    --whitelist-function tapos_.+ \
    --whitelist-function transaction_size \
    --whitelist-type account_name \
    --whitelist-type account_permission \
    --whitelist-type action_name \
    --whitelist-type block_id_type \
    --whitelist-type checksum160 \
    --whitelist-type checksum256 \
    --whitelist-type checksum512 \
    --whitelist-type permission_name \
    --whitelist-type public_key \
    --whitelist-type scope_name \
    --whitelist-type signature \
    --whitelist-type table_name \
    --whitelist-type time \
    --whitelist-type transaction_id_type \
    --whitelist-type weight_type \
    wrapper.hpp \
    -- \
    -I ../external/eosio.cdt/libraries/boost/include \
    -I ../external/eosio.cdt/libraries/libc++/libcxx/include \
    -I ../external/eosio.cdt/libraries/libc/musl/include \
    -I ../external/eosio.cdt/libraries \
    --std=c++14
