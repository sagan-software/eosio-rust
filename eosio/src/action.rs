use account::AccountName;
use bytes::{Write, WriteError};
use eosio_macros::*;
use permission::{PermissionLevel, PermissionName};

/// Aborts processing of this action and unwinds all pending changes if the test condition is true
pub fn eosio_assert(test: bool, msg: &str) {
    let test = if test { 1 } else { 0 };
    let msg_ptr = msg.as_ptr();
    unsafe { ::eosio_sys::eosio_assert(test, msg_ptr) }
}

/// Aborts processing of this action and unwinds all pending changes if the test condition is true
pub fn eosio_assert_code<C>(test: bool, code: C)
where
    C: Into<u64>,
{
    let test = if test { 1 } else { 0 };
    let code: u64 = code.into();
    unsafe { ::eosio_sys::eosio_assert_code(test, code) }
}

/// This method will abort execution of wasm without failing the contract. This is used to bypass all cleanup / destructors that would normally be called.
pub fn eosio_exit<C>(code: C)
where
    C: Into<i32>,
{
    let code: i32 = code.into();
    unsafe { ::eosio_sys::eosio_exit(code) }
}

/// Get the current receiver of the action.
pub fn current_receiver() -> AccountName {
    let name = unsafe { ::eosio_sys::current_receiver() };
    name.into()
}

/// Verifies that `name` name has auth.
pub fn has_auth(name: AccountName) -> bool {
    unsafe { ::eosio_sys::has_auth(name.into()) }
}

/// Verifies that `name` is an account.
pub fn is_account(name: AccountName) -> bool {
    unsafe { ::eosio_sys::is_account(name.into()) }
}

/// Verifies that `name` exists in the set of provided auths on a action. Throws if not found.
pub fn require_auth(name: AccountName) {
    unsafe { ::eosio_sys::require_auth(name.into()) }
}

/// Verifies that `name` exists in the set of provided auths on a action. Throws if not found.
pub fn require_auth2(name: AccountName, permission: PermissionName) {
    unsafe { ::eosio_sys::require_auth2(name.into(), permission.into()) }
}

/// Add the specified account to set of accounts to be notified
pub fn require_recipient(name: AccountName) {
    unsafe { ::eosio_sys::require_recipient(name.into()) }
}

/// Verifies that `name` exists in the set of read locks held on a action. Throws if not found
pub fn require_read_lock(name: AccountName) {
    unsafe { ::eosio_sys::require_read_lock(name.into()) }
}

/// Verifies that `name` exists in the set of write locks held on a action. Throws if not found
pub fn require_write_lock(name: AccountName) {
    unsafe { ::eosio_sys::require_write_lock(name.into()) }
}

eosio_name!(ActionName);

pub struct Action<'a, Data>
where
    Data: Write,
{
    pub account: AccountName,
    pub name: ActionName,
    pub authorization: &'a [PermissionLevel],
    pub data: Data,
}

impl<'a, Data> Write for Action<'a, Data>
where
    Data: Write,
{
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        let pos = self.account.write(bytes, pos)?;
        let pos = self.name.write(bytes, pos)?;
        let pos = self.authorization.write(bytes, pos)?;

        let mut data_bytes = [0u8; 10000]; // TODO don't hardcode?
        let data_size = self.data.write(&mut data_bytes, 0)?;

        let pos = (&data_bytes[..=data_size]).write(bytes, pos)?;
        Ok(pos)
    }
}

impl<'a, Data> Action<'a, Data>
where
    Data: Write,
{
    pub fn send(&self) -> Result<(), WriteError> {
        let mut bytes = [0u8; 10000]; // TODO: don't hardcode this?
        let pos = self.write(&mut bytes, 0)?;
        let ptr = bytes[..].as_mut_ptr();
        if self.authorization.is_empty() {
            unsafe { ::eosio_sys::send_context_free_inline(ptr, pos) }
        } else {
            unsafe { ::eosio_sys::send_inline(ptr, pos) }
        }
        Ok(())
    }
}
