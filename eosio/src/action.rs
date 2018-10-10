use account::AccountName;
use bytes::{Read, ReadError, Write, WriteError};
use eosio_macros::*;
use permission::PermissionLevel;

/// This method will abort execution of wasm without failing the contract. This is used to bypass all cleanup / destructors that would normally be called.
pub fn eosio_exit<C>(code: C)
where
    C: Into<i32>,
{
    let code: i32 = code.into();
    unsafe { ::eosio_sys::eosio_exit(code) }
}

eosio_name!(ActionName);

#[derive(Clone, Debug)]
pub struct InlineAction<'a, Data>
where
    Data: Write,
{
    pub account: AccountName,
    pub name: ActionName,
    pub authorization: &'a [PermissionLevel],
    pub data: Data,
}

impl<'a, Data> Write for InlineAction<'a, Data>
where
    Data: Write,
{
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        let pos = self.account.write(bytes, pos)?;
        let pos = self.name.write(bytes, pos)?;
        let pos = self.authorization.write(bytes, pos)?;

        let mut data_bytes = [0u8; 1000]; // TODO don't hardcode?
        let data_size = self.data.write(&mut data_bytes, 0)?;

        let pos = (&data_bytes[..=data_size]).write(bytes, pos)?;
        Ok(pos)
    }
}

impl<'a, Data> InlineAction<'a, Data>
where
    Data: Write,
{
    pub fn send(&self) -> Result<(), WriteError> {
        let mut bytes = [0u8; 1000]; // TODO: don't hardcode this?
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

pub trait Action: Read + Write + Clone {
    const NAME: u64;

    fn execute(self);

    fn read_action_data() -> Result<(Self, usize), ReadError> {
        // TODO: set the length of this to a fixed size based on the action inputs
        // let mut bytes = [0u8; 8];
        let num_bytes = unsafe { ::eosio_sys::action_data_size() };
        let mut bytes = vec![0u8; num_bytes as usize];
        let ptr: *mut ::eosio_sys::c_void = &mut bytes[..] as *mut _ as *mut ::eosio_sys::c_void;
        unsafe {
            ::eosio_sys::read_action_data(ptr, num_bytes);
        }

        Self::read(&bytes, 0)
    }

    fn to_inline_action(self, authorization: &[PermissionLevel]) -> InlineAction<Self> {
        InlineAction {
            account: AccountName::receiver(),
            name: Self::NAME.into(),
            authorization,
            data: self,
        }
    }

    fn send(self, authorization: &[PermissionLevel]) -> Result<(), WriteError> {
        self.to_inline_action(authorization).send()
    }
}
