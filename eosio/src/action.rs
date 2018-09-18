use eosio_types::*;

pub fn current_receiver() -> AccountName {
    let name = unsafe { ::eosio_sys::action::current_receiver() };
    Name::new(name)
}

pub fn has_auth(name: &AccountName) -> bool {
    unsafe { ::eosio_sys::action::has_auth(name.as_u64()) }
}

pub fn is_account(name: &AccountName) -> bool {
    unsafe { ::eosio_sys::action::is_account(name.as_u64()) }
}

pub fn require_auth(name: &AccountName) {
    unsafe { ::eosio_sys::action::require_auth(name.as_u64()) }
}

pub fn require_auth2(name: &AccountName, permission: &PermissionName) {
    unsafe { ::eosio_sys::action::require_auth2(name.as_u64(), permission.as_u64()) }
}

pub fn require_read_lock(name: &AccountName) {
    unsafe { ::eosio_sys::action::require_read_lock(name.as_u64()) }
}

pub fn require_recipient(name: &AccountName) {
    unsafe { ::eosio_sys::action::require_recipient(name.as_u64()) }
}

pub fn require_write_lock(name: &AccountName) {
    unsafe { ::eosio_sys::action::require_write_lock(name.as_u64()) }
}
