use crate::{
    capi_checksum160, capi_checksum256, capi_checksum512, capi_name, int128_t,
    uint128_t,
};
use std::default::Default;

pub unsafe fn read_action_data(msg: *mut crate::c_void, len: u32) -> u32 {
    Default::default()
}

pub unsafe fn action_data_size() -> u32 {
    Default::default()
}

pub unsafe fn require_recipient(name: capi_name) {}

pub unsafe fn require_auth(name: capi_name) {}

pub unsafe fn has_auth(name: capi_name) -> bool {
    Default::default()
}

pub unsafe fn require_auth2(name: capi_name, permission: capi_name) {}

pub unsafe fn is_account(name: capi_name) -> bool {
    Default::default()
}

pub unsafe fn send_inline(serialized_action: *mut crate::c_char, size: usize) {}

pub unsafe fn send_context_free_inline(
    serialized_action: *mut crate::c_char,
    size: usize,
) {
}

pub unsafe fn publication_time() -> u64 {
    Default::default()
}

pub unsafe fn current_receiver() -> capi_name {
    Default::default()
}

pub unsafe fn get_active_producers(
    producers: *mut capi_name,
    datalen: u32,
) -> u32 {
    Default::default()
}

pub unsafe fn assert_sha256(
    data: *const crate::c_char,
    length: u32,
    hash: *const capi_checksum256,
) {
}

pub unsafe fn assert_sha1(
    data: *const crate::c_char,
    length: u32,
    hash: *const capi_checksum160,
) {
}

pub unsafe fn assert_sha512(
    data: *const crate::c_char,
    length: u32,
    hash: *const capi_checksum512,
) {
}

pub unsafe fn assert_ripemd160(
    data: *const crate::c_char,
    length: u32,
    hash: *const capi_checksum160,
) {
}

pub unsafe fn sha256(
    data: *const crate::c_char,
    length: u32,
    hash: *mut capi_checksum256,
) {
}

pub unsafe fn sha1(
    data: *const crate::c_char,
    length: u32,
    hash: *mut capi_checksum160,
) {
}

pub unsafe fn sha512(
    data: *const crate::c_char,
    length: u32,
    hash: *mut capi_checksum512,
) {
}

pub unsafe fn ripemd160(
    data: *const crate::c_char,
    length: u32,
    hash: *mut capi_checksum160,
) {
}

pub unsafe fn recover_key(
    digest: *const capi_checksum256,
    sig: *const crate::c_char,
    siglen: usize,
    pub_: *mut crate::c_char,
    publen: usize,
) -> crate::c_int {
    Default::default()
}

pub unsafe fn assert_recover_key(
    digest: *const capi_checksum256,
    sig: *const crate::c_char,
    siglen: usize,
    pub_: *const crate::c_char,
    publen: usize,
) {
}

pub unsafe fn db_store_i64(
    scope: u64,
    table: capi_name,
    payer: capi_name,
    id: u64,
    data: *const crate::c_void,
    len: u32,
) -> i32 {
    Default::default()
}

pub unsafe fn db_update_i64(
    iterator: i32,
    payer: capi_name,
    data: *const crate::c_void,
    len: u32,
) {
}

pub unsafe fn db_remove_i64(iterator: i32) {}

pub unsafe fn db_get_i64(
    iterator: i32,
    data: *const crate::c_void,
    len: u32,
) -> i32 {
    Default::default()
}

pub unsafe fn db_next_i64(iterator: i32, primary: *mut u64) -> i32 {
    Default::default()
}

pub unsafe fn db_previous_i64(iterator: i32, primary: *mut u64) -> i32 {
    Default::default()
}

pub unsafe fn db_find_i64(
    code: capi_name,
    scope: u64,
    table: capi_name,
    id: u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_lowerbound_i64(
    code: capi_name,
    scope: u64,
    table: capi_name,
    id: u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_upperbound_i64(
    code: capi_name,
    scope: u64,
    table: capi_name,
    id: u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_end_i64(code: capi_name, scope: u64, table: capi_name) -> i32 {
    Default::default()
}

pub unsafe fn db_idx64_store(
    scope: u64,
    table: capi_name,
    payer: capi_name,
    id: u64,
    secondary: *const u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx64_update(
    iterator: i32,
    payer: capi_name,
    secondary: *const u64,
) {
}

pub unsafe fn db_idx64_remove(iterator: i32) {}

pub unsafe fn db_idx64_next(iterator: i32, primary: *mut u64) -> i32 {
    Default::default()
}

pub unsafe fn db_idx64_previous(iterator: i32, primary: *mut u64) -> i32 {
    Default::default()
}

pub unsafe fn db_idx64_find_primary(
    code: capi_name,
    scope: u64,
    table: capi_name,
    secondary: *mut u64,
    primary: u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx64_find_secondary(
    code: capi_name,
    scope: u64,
    table: capi_name,
    secondary: *const u64,
    primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx64_lowerbound(
    code: capi_name,
    scope: u64,
    table: capi_name,
    secondary: *mut u64,
    primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx64_upperbound(
    code: capi_name,
    scope: u64,
    table: capi_name,
    secondary: *mut u64,
    primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx64_end(
    code: capi_name,
    scope: u64,
    table: capi_name,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx128_store(
    scope: u64,
    table: capi_name,
    payer: capi_name,
    id: u64,
    secondary: *const uint128_t,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx128_update(
    iterator: i32,
    payer: capi_name,
    secondary: *const uint128_t,
) {
}

pub unsafe fn db_idx128_remove(iterator: i32) {}

pub unsafe fn db_idx128_next(iterator: i32, primary: *mut u64) -> i32 {
    Default::default()
}

pub unsafe fn db_idx128_previous(iterator: i32, primary: *mut u64) -> i32 {
    Default::default()
}

pub unsafe fn db_idx128_find_primary(
    code: capi_name,
    scope: u64,
    table: capi_name,
    secondary: *mut uint128_t,
    primary: u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx128_find_secondary(
    code: capi_name,
    scope: u64,
    table: capi_name,
    secondary: *const uint128_t,
    primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx128_lowerbound(
    code: capi_name,
    scope: u64,
    table: capi_name,
    secondary: *mut uint128_t,
    primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx128_upperbound(
    code: capi_name,
    scope: u64,
    table: capi_name,
    secondary: *mut uint128_t,
    primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx128_end(
    code: capi_name,
    scope: u64,
    table: capi_name,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx256_store(
    scope: u64,
    table: capi_name,
    payer: capi_name,
    id: u64,
    data: *const uint128_t,
    data_len: u32,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx256_update(
    iterator: i32,
    payer: capi_name,
    data: *const uint128_t,
    data_len: u32,
) {
}

pub unsafe fn db_idx256_remove(iterator: i32) {}

pub unsafe fn db_idx256_next(iterator: i32, primary: *mut u64) -> i32 {
    Default::default()
}

pub unsafe fn db_idx256_previous(iterator: i32, primary: *mut u64) -> i32 {
    Default::default()
}

pub unsafe fn db_idx256_find_primary(
    code: capi_name,
    scope: u64,
    table: capi_name,
    data: *mut uint128_t,
    data_len: u32,
    primary: u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx256_find_secondary(
    code: capi_name,
    scope: u64,
    table: capi_name,
    data: *const uint128_t,
    data_len: u32,
    primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx256_lowerbound(
    code: capi_name,
    scope: u64,
    table: capi_name,
    data: *mut uint128_t,
    data_len: u32,
    primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx256_upperbound(
    code: capi_name,
    scope: u64,
    table: capi_name,
    data: *mut uint128_t,
    data_len: u32,
    primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx256_end(
    code: capi_name,
    scope: u64,
    table: capi_name,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_double_store(
    scope: u64,
    table: capi_name,
    payer: capi_name,
    id: u64,
    secondary: *const f64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_double_update(
    iterator: i32,
    payer: capi_name,
    secondary: *const f64,
) {
}

pub unsafe fn db_idx_double_remove(iterator: i32) {}

pub unsafe fn db_idx_double_next(iterator: i32, primary: *mut u64) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_double_previous(iterator: i32, primary: *mut u64) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_double_find_primary(
    code: capi_name,
    scope: u64,
    table: capi_name,
    secondary: *mut f64,
    primary: u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_double_find_secondary(
    code: capi_name,
    scope: u64,
    table: capi_name,
    secondary: *const f64,
    primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_double_lowerbound(
    code: capi_name,
    scope: u64,
    table: capi_name,
    secondary: *mut f64,
    primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_double_upperbound(
    code: capi_name,
    scope: u64,
    table: capi_name,
    secondary: *mut f64,
    primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_double_end(
    code: capi_name,
    scope: u64,
    table: capi_name,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_long_double_store(
    scope: u64,
    table: capi_name,
    payer: capi_name,
    id: u64,
    secondary: *const f64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_long_double_update(
    iterator: i32,
    payer: capi_name,
    secondary: *const f64,
) {
}

pub unsafe fn db_idx_long_double_remove(iterator: i32) {}

pub unsafe fn db_idx_long_double_next(iterator: i32, primary: *mut u64) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_long_double_previous(
    iterator: i32,
    primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_long_double_find_primary(
    code: capi_name,
    scope: u64,
    table: capi_name,
    secondary: *mut f64,
    primary: u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_long_double_find_secondary(
    code: capi_name,
    scope: u64,
    table: capi_name,
    secondary: *const f64,
    primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_long_double_lowerbound(
    code: capi_name,
    scope: u64,
    table: capi_name,
    secondary: *mut f64,
    primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_long_double_upperbound(
    code: capi_name,
    scope: u64,
    table: capi_name,
    secondary: *mut f64,
    primary: *mut u64,
) -> i32 {
    Default::default()
}

pub unsafe fn db_idx_long_double_end(
    code: capi_name,
    scope: u64,
    table: capi_name,
) -> i32 {
    Default::default()
}

pub unsafe fn check_transaction_authorization(
    trx_data: *const crate::c_char,
    trx_size: u32,
    pubkeys_data: *const crate::c_char,
    pubkeys_size: u32,
    perms_data: *const crate::c_char,
    perms_size: u32,
) -> i32 {
    Default::default()
}

pub unsafe fn check_permission_authorization(
    account: capi_name,
    permission: capi_name,
    pubkeys_data: *const crate::c_char,
    pubkeys_size: u32,
    perms_data: *const crate::c_char,
    perms_size: u32,
    delay_us: u64,
) -> i32 {
    Default::default()
}

pub unsafe fn get_permission_last_used(
    account: capi_name,
    permission: capi_name,
) -> i64 {
    Default::default()
}

pub unsafe fn get_account_creation_time(account: capi_name) -> i64 {
    Default::default()
}

pub unsafe fn prints(cstr: *const crate::c_char) {}

pub unsafe fn prints_l(cstr: *const crate::c_char, len: u32) {}

pub unsafe fn printi(value: i64) {}

pub unsafe fn printui(value: u64) {}

pub unsafe fn printi128(value: *const int128_t) {}

pub unsafe fn printui128(value: *const uint128_t) {}

pub unsafe fn printsf(value: f32) {}

pub unsafe fn printdf(value: f64) {}

pub unsafe fn printqf(value: *const f64) {}

pub unsafe fn printn(name: u64) {}

pub unsafe fn printhex(data: *const crate::c_void, datalen: u32) {}

pub unsafe fn get_resource_limits(
    account: capi_name,
    ram_bytes: *mut i64,
    net_weight: *mut i64,
    cpu_weight: *mut i64,
) {
}

pub unsafe fn set_resource_limits(
    account: capi_name,
    ram_bytes: i64,
    net_weight: i64,
    cpu_weight: i64,
) {
}

pub unsafe fn set_proposed_producers(
    producer_data: *mut crate::c_char,
    producer_data_size: u32,
) -> i64 {
    Default::default()
}

pub unsafe fn is_privileged(account: capi_name) -> bool {
    Default::default()
}

pub unsafe fn set_privileged(account: capi_name, is_priv: bool) {}

pub unsafe fn set_blockchain_parameters_packed(
    data: *mut crate::c_char,
    datalen: u32,
) {
}

pub unsafe fn get_blockchain_parameters_packed(
    data: *mut crate::c_char,
    datalen: u32,
) -> u32 {
    Default::default()
}

pub unsafe fn eosio_assert(test: u32, msg: *const crate::c_char) {}

pub unsafe fn eosio_assert_message(
    test: u32,
    msg: *const crate::c_char,
    msg_len: u32,
) {
}

pub unsafe fn eosio_assert_code(test: u32, code: u64) {}

pub unsafe fn eosio_exit(code: i32) {}

pub unsafe fn current_time() -> u64 {
    Default::default()
}

pub unsafe fn send_deferred(
    sender_id: *const uint128_t,
    payer: capi_name,
    serialized_transaction: *const crate::c_char,
    size: usize,
    replace_existing: u32,
) {
}

pub unsafe fn cancel_deferred(sender_id: *const uint128_t) -> crate::c_int {
    Default::default()
}

pub unsafe fn read_transaction(
    buffer: *mut crate::c_char,
    size: usize,
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
    type_: u32,
    index: u32,
    buff: *mut crate::c_char,
    size: usize,
) -> crate::c_int {
    Default::default()
}

pub unsafe fn get_context_free_data(
    index: u32,
    buff: *mut crate::c_char,
    size: usize,
) -> crate::c_int {
    Default::default()
}
