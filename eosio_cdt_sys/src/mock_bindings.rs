use crate::{
    capi_checksum160, capi_checksum256, capi_checksum512, capi_name, int128_t,
    uint128_t,
};
use std::default::Default;

pub unsafe fn read_action_data(_msg: *mut crate::c_void, _len: u32) -> u32 {
    Default::default()
}

pub unsafe fn action_data_size() -> u32 {
    Default::default()
}

pub unsafe fn require_recipient(_name: capi_name) {}

pub unsafe fn require_auth(_name: capi_name) {}

pub unsafe fn has_auth(_name: capi_name) -> bool {
    Default::default()
}

pub unsafe fn require_auth2(_name: capi_name, _permission: capi_name) {}

pub unsafe fn is_account(_name: capi_name) -> bool {
    Default::default()
}

pub unsafe fn send_inline(
    _serialized_action: *mut crate::c_char,
    _size: usize,
) {
}

pub unsafe fn send_context_free_inline(
    _serialized_action: *mut crate::c_char,
    _size: usize,
) {
}

pub unsafe fn publication_time() -> u64 {
    Default::default()
}

pub unsafe fn current_receiver() -> capi_name {
    Default::default()
}

pub unsafe fn get_active_producers(
    _producers: *mut capi_name,
    _datalen: u32,
) -> u32 {
    Default::default()
}

pub unsafe fn assert_sha256(
    _data: *const crate::c_char,
    _length: u32,
    _hash: *const capi_checksum256,
) {
}

pub unsafe fn assert_sha1(
    _data: *const crate::c_char,
    _length: u32,
    _hash: *const capi_checksum160,
) {
}

pub unsafe fn assert_sha512(
    _data: *const crate::c_char,
    _length: u32,
    _hash: *const capi_checksum512,
) {
}

pub unsafe fn assert_ripemd160(
    _data: *const crate::c_char,
    _length: u32,
    _hash: *const capi_checksum160,
) {
}

pub unsafe fn sha256(
    _data: *const crate::c_char,
    _length: u32,
    _hash: *mut capi_checksum256,
) {
}

pub unsafe fn sha1(
    _data: *const crate::c_char,
    _length: u32,
    _hash: *mut capi_checksum160,
) {
}

pub unsafe fn sha512(
    _data: *const crate::c_char,
    _length: u32,
    _hash: *mut capi_checksum512,
) {
}

pub unsafe fn ripemd160(
    _data: *const crate::c_char,
    _length: u32,
    _hash: *mut capi_checksum160,
) {
}

pub unsafe fn recover_key(
    _digest: *const capi_checksum256,
    _sig: *const crate::c_char,
    _siglen: usize,
    _pub_: *mut crate::c_char,
    _publen: usize,
) -> crate::c_int {
    Default::default()
}

pub unsafe fn assert_recover_key(
    _digest: *const capi_checksum256,
    _sig: *const crate::c_char,
    _siglen: usize,
    _pub_: *const crate::c_char,
    _publen: usize,
) {
}

pub unsafe fn db_store_i64(
    _scope: u64,
    _table: capi_name,
    _payer: capi_name,
    _id: u64,
    _data: *const crate::c_void,
    _len: u32,
) -> i32 {
    Default::default()
}

pub unsafe fn db_update_i64(
    _iterator: i32,
    _payer: capi_name,
    _data: *const crate::c_void,
    _len: u32,
) {
}

pub unsafe fn db_remove_i64(_iterator: i32) {}

pub unsafe fn db_get_i64(
    _iterator: i32,
    _data: *const crate::c_void,
    _len: u32,
) -> i32 {
    Default::default()
}

pub unsafe fn db_next_i64(_iterator: i32, _primary: *mut u64) -> i32 {
    Default::default()
}

pub unsafe fn db_previous_i64(_iterator: i32, _primary: *mut u64) -> i32 {
    Default::default()
}

pub unsafe fn db_find_i64(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _id: u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_lowerbound_i64(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _id: u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_upperbound_i64(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _id: u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_end_i64(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx64_store(
    _scope: u64,
    _table: capi_name,
    _payer: capi_name,
    _id: u64,
    _secondary: *const u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx64_update(
    _iterator: i32,
    _payer: capi_name,
    _secondary: *const u64,
) {
}

pub unsafe fn db_idx64_remove(_iterator: i32) {}

pub unsafe fn db_idx64_next(_iterator: i32, _primary: *mut u64) -> i32 {
    Default::default()
}

pub unsafe fn db_idx64_previous(_iterator: i32, _primary: *mut u64) -> i32 {
    Default::default()
}

pub unsafe fn db_idx64_find_primary(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _secondary: *mut u64,
    _primary: u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx64_find_secondary(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _secondary: *const u64,
    _primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx64_lowerbound(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _secondary: *mut u64,
    _primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx64_upperbound(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _secondary: *mut u64,
    _primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx64_end(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx128_store(
    _scope: u64,
    _table: capi_name,
    _payer: capi_name,
    _id: u64,
    _secondary: *const uint128_t,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx128_update(
    _iterator: i32,
    _payer: capi_name,
    _secondary: *const uint128_t,
) {
}

pub unsafe fn db_idx128_remove(_iterator: i32) {}

pub unsafe fn db_idx128_next(_iterator: i32, _primary: *mut u64) -> i32 {
    Default::default()
}

pub unsafe fn db_idx128_previous(_iterator: i32, _primary: *mut u64) -> i32 {
    Default::default()
}

pub unsafe fn db_idx128_find_primary(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _secondary: *mut uint128_t,
    _primary: u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx128_find_secondary(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _secondary: *const uint128_t,
    _primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx128_lowerbound(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _secondary: *mut uint128_t,
    _primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx128_upperbound(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _secondary: *mut uint128_t,
    _primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx128_end(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx256_store(
    _scope: u64,
    _table: capi_name,
    _payer: capi_name,
    _id: u64,
    _data: *const uint128_t,
    _data_len: u32,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx256_update(
    _iterator: i32,
    _payer: capi_name,
    _data: *const uint128_t,
    _data_len: u32,
) {
}

pub unsafe fn db_idx256_remove(_iterator: i32) {}

pub unsafe fn db_idx256_next(_iterator: i32, _primary: *mut u64) -> i32 {
    Default::default()
}

pub unsafe fn db_idx256_previous(_iterator: i32, _primary: *mut u64) -> i32 {
    Default::default()
}

pub unsafe fn db_idx256_find_primary(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _data: *mut uint128_t,
    _data_len: u32,
    _primary: u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx256_find_secondary(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _data: *const uint128_t,
    _data_len: u32,
    _primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx256_lowerbound(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _data: *mut uint128_t,
    _data_len: u32,
    _primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx256_upperbound(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _data: *mut uint128_t,
    _data_len: u32,
    _primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx256_end(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_double_store(
    _scope: u64,
    _table: capi_name,
    _payer: capi_name,
    _id: u64,
    _secondary: *const f64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_double_update(
    _iterator: i32,
    _payer: capi_name,
    _secondary: *const f64,
) {
}

pub unsafe fn db_idx_double_remove(_iterator: i32) {}

pub unsafe fn db_idx_double_next(_iterator: i32, _primary: *mut u64) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_double_previous(
    _iterator: i32,
    _primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_double_find_primary(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _secondary: *mut f64,
    _primary: u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_double_find_secondary(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _secondary: *const f64,
    _primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_double_lowerbound(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _secondary: *mut f64,
    _primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_double_upperbound(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _secondary: *mut f64,
    _primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_double_end(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_long_double_store(
    _scope: u64,
    _table: capi_name,
    _payer: capi_name,
    _id: u64,
    _secondary: *const f64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_long_double_update(
    _iterator: i32,
    _payer: capi_name,
    _secondary: *const f64,
) {
}

pub unsafe fn db_idx_long_double_remove(_iterator: i32) {}

pub unsafe fn db_idx_long_double_next(
    _iterator: i32,
    _primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_long_double_previous(
    _iterator: i32,
    _primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_long_double_find_primary(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _secondary: *mut f64,
    _primary: u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_long_double_find_secondary(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _secondary: *const f64,
    _primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_long_double_lowerbound(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _secondary: *mut f64,
    _primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_long_double_upperbound(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
    _secondary: *mut f64,
    _primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_long_double_end(
    _code: capi_name,
    _scope: u64,
    _table: capi_name,
) -> i32 {
    Default::default()
}

pub unsafe fn check_transaction_authorization(
    _trx_data: *const crate::c_char,
    _trx_size: u32,
    _pubkeys_data: *const crate::c_char,
    _pubkeys_size: u32,
    _perms_data: *const crate::c_char,
    _perms_size: u32,
) -> i32 {
    Default::default()
}

pub unsafe fn check_permission_authorization(
    _account: capi_name,
    _permission: capi_name,
    _pubkeys_data: *const crate::c_char,
    _pubkeys_size: u32,
    _perms_data: *const crate::c_char,
    _perms_size: u32,
    _delay_us: u64,
) -> i32 {
    Default::default()
}

pub unsafe fn get_permission_last_used(
    _account: capi_name,
    _permission: capi_name,
) -> i64 {
    Default::default()
}

pub unsafe fn get_account_creation_time(_account: capi_name) -> i64 {
    Default::default()
}

pub unsafe fn prints(_cstr: *const crate::c_char) {}

pub unsafe fn prints_l(_cstr: *const crate::c_char, _len: u32) {}

pub unsafe fn printi(_value: i64) {}

pub unsafe fn printui(_value: u64) {}

pub unsafe fn printi128(_value: *const int128_t) {}

pub unsafe fn printui128(_value: *const uint128_t) {}

pub unsafe fn printsf(_value: f32) {}

pub unsafe fn printdf(_value: f64) {}

pub unsafe fn printqf(_value: *const f64) {}

pub unsafe fn printn(_name: u64) {}

pub unsafe fn printhex(_data: *const crate::c_void, _datalen: u32) {}

pub unsafe fn get_resource_limits(
    _account: capi_name,
    _ram_bytes: *mut i64,
    _net_weight: *mut i64,
    _cpu_weight: *mut i64,
) {
}

pub unsafe fn set_resource_limits(
    _account: capi_name,
    _ram_bytes: i64,
    _net_weight: i64,
    _cpu_weight: i64,
) {
}

pub unsafe fn set_proposed_producers(
    _producer_data: *mut crate::c_char,
    _producer_data_size: u32,
) -> i64 {
    Default::default()
}

pub unsafe fn is_privileged(_account: capi_name) -> bool {
    Default::default()
}

pub unsafe fn set_privileged(_account: capi_name, _is_priv: bool) {}

pub unsafe fn set_blockchain_parameters_packed(
    _data: *mut crate::c_char,
    _datalen: u32,
) {
}

pub unsafe fn get_blockchain_parameters_packed(
    _data: *mut crate::c_char,
    _datalen: u32,
) -> u32 {
    Default::default()
}

pub unsafe fn eosio_assert(_test: u32, _msg: *const crate::c_char) {}

pub unsafe fn eosio_assert_message(
    _test: u32,
    _msg: *const crate::c_char,
    _msg_len: u32,
) {
}

pub unsafe fn eosio_assert_code(_test: u32, _code: u64) {}

pub unsafe fn eosio_exit(_code: i32) {}

pub unsafe fn current_time() -> u64 {
    Default::default()
}

pub unsafe fn send_deferred(
    _sender_id: *const uint128_t,
    _payer: capi_name,
    _serialized_transaction: *const crate::c_char,
    _size: usize,
    _replace_existing: u32,
) {
}

pub unsafe fn cancel_deferred(_sender_id: *const uint128_t) -> crate::c_int {
    Default::default()
}

pub unsafe fn read_transaction(
    _buffer: *mut crate::c_char,
    _size: usize,
) -> usize {
    Default::default()
}

pub unsafe fn transaction_size() -> usize {
    Default::default()
}

pub unsafe fn tapos_block_num() -> crate::c_int {
    Default::default()
}

pub unsafe fn tapos_block_prefix() -> crate::c_int {
    Default::default()
}

pub unsafe fn expiration() -> u32 {
    Default::default()
}

pub unsafe fn get_action(
    _type_: u32,
    _index: u32,
    _buff: *mut crate::c_char,
    _size: usize,
) -> crate::c_int {
    Default::default()
}

pub unsafe fn get_context_free_data(
    _index: u32,
    _buff: *mut crate::c_char,
    _size: usize,
) -> crate::c_int {
    Default::default()
}
