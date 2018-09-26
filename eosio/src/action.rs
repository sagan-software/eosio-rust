use account::AccountName;
use eosio_macros::*;
use permission::{PermissionLevel, PermissionName};
use write::{WriteError, Writeable};

eosio_name!(ActionName);

#[derive(Clone, Debug)]
pub struct Action<'a, Data>
where
    Data: Writeable,
{
    pub account: AccountName,
    pub name: ActionName,
    pub authorization: &'a [PermissionLevel],
    pub data: Data,
}

impl<'a, Data> Writeable for Action<'a, Data>
where
    Data: Writeable,
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

pub fn eosio_assert(test: bool, msg: &str) {
    let test = if test { 1 } else { 0 };
    let msg_ptr = msg.as_ptr();
    unsafe { ::eosio_sys::eosio_assert(test, msg_ptr) }
}

pub fn eosio_assert_code<C>(test: bool, code: C)
where
    C: Into<u64>,
{
    let test = if test { 1 } else { 0 };
    let code: u64 = code.into();
    unsafe { ::eosio_sys::eosio_assert_code(test, code) }
}

pub fn eosio_exit<C>(code: C)
where
    C: Into<i32>,
{
    let code: i32 = code.into();
    unsafe { ::eosio_sys::eosio_exit(code) }
}

pub fn current_receiver() -> AccountName {
    let name = unsafe { ::eosio_sys::current_receiver() };
    name.into()
}

pub fn has_auth(name: AccountName) -> bool {
    unsafe { ::eosio_sys::has_auth(name.into()) }
}

pub fn is_account(name: AccountName) -> bool {
    unsafe { ::eosio_sys::is_account(name.into()) }
}

pub fn require_auth(name: AccountName) {
    unsafe { ::eosio_sys::require_auth(name.into()) }
}

pub fn require_auth2(name: AccountName, permission: PermissionName) {
    unsafe { ::eosio_sys::require_auth2(name.into(), permission.into()) }
}

pub fn require_read_lock(name: AccountName) {
    unsafe { ::eosio_sys::require_read_lock(name.into()) }
}

pub fn require_recipient(name: AccountName) {
    unsafe { ::eosio_sys::require_recipient(name.into()) }
}

pub fn require_write_lock(name: AccountName) {
    unsafe { ::eosio_sys::require_write_lock(name.into()) }
}

pub enum InlineError {
    WriteError,
}

pub fn send_inline<T>(action: &Action<T>) -> Result<(), InlineError>
where
    T: Writeable,
{
    let mut bytes = [0u8; 10000]; // TODO: don't hardcode this?
    let pos = action
        .write(&mut bytes, 0)
        .map_err(|_| InlineError::WriteError)?;
    let ptr = bytes[..].as_mut_ptr();
    unsafe { ::eosio_sys::send_inline(ptr, pos) }
    Ok(())
}

pub enum ContextFreeInlineError {
    NoAuthorizationsAllowed,
    WriteError,
}

pub fn send_context_free_inline<T>(action: &Action<T>) -> Result<(), ContextFreeInlineError>
where
    T: Writeable,
{
    if action.authorization.is_empty() {
        return Err(ContextFreeInlineError::NoAuthorizationsAllowed);
    }
    let mut bytes = [0u8; 10000]; // TODO: don't hardcode this?
    let pos = action
        .write(&mut bytes, 0)
        .map_err(|_| ContextFreeInlineError::WriteError)?;
    let ptr = bytes[..pos].as_mut_ptr();
    unsafe { ::eosio_sys::send_context_free_inline(ptr, pos) }
    Ok(())
}
