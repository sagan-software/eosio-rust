use account::{AccountName, Permission};
use bytes::{Read, ReadError, Write, WriteError};
use eosio_macros::*;
use serde::{Deserialize, Serialize};

/// This method will abort execution of wasm without failing the contract. This is used to bypass all cleanup / destructors that would normally be called.
pub fn eosio_exit<C>(code: C)
where
    C: Into<i32>,
{
    let code: i32 = code.into();
    unsafe { ::eosio_sys::eosio_exit(code) }
}

eosio_name!(ActionName);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Action<Data>
where
    Data: Write + Sized,
{
    pub account: AccountName,
    pub name: ActionName,
    pub authorization: Vec<Permission>,
    pub data: Data,
}

impl<Data> Write for Action<Data>
where
    Data: Write,
{
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        let pos = self.account.write(bytes, pos)?;
        let pos = self.name.write(bytes, pos)?;
        let pos = self.authorization.write(bytes, pos)?;

        let data_size = ::lib::size_of_val(&self.data);
        let mut data_bytes = vec![0u8; data_size];
        let data_size = self.data.write(&mut data_bytes, 0)?;

        let pos = (&data_bytes[..=data_size]).write(bytes, pos)?;
        Ok(pos)
    }
}

#[derive(Clone, Debug)]
pub struct ActionId(u128);

impl<Data> Action<Data>
where
    Data: Write,
{
    pub fn send_inline(&self) -> Result<(), WriteError> {
        let size = ::lib::size_of_val(self);
        let mut bytes = vec![0u8; size];
        let pos = self.write(&mut bytes, 0)?;
        let ptr = bytes[..].as_mut_ptr();
        if self.authorization.is_empty() {
            unsafe { ::eosio_sys::send_context_free_inline(ptr, pos) }
        } else {
            unsafe { ::eosio_sys::send_inline(ptr, pos) }
        }
        Ok(())
    }

    pub fn send_deferred<P>(
        &self,
        id: ActionId,
        payer: P,
        replace_existing: bool,
    ) -> Result<(), WriteError>
    where
        P: Into<u64>,
    {
        // TODO
        Ok(())
    }

    pub fn cancel_deferred(id: ActionId) -> Result<(), ()> {
        // TODO
        Ok(())
    }
}

pub trait ActionFn: Read + Write + Clone {
    const NAME: u64;

    fn execute(self);

    fn read_data() -> Result<(Self, usize), ReadError> {
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

    fn to_action(self, authorization: Vec<Permission>) -> Action<Self> {
        Action {
            account: AccountName::receiver(),
            name: Self::NAME.into(),
            authorization,
            data: self,
        }
    }

    fn send_inline(self, authorization: Vec<Permission>) -> Result<(), WriteError> {
        self.to_action(authorization).send_inline()
    }
}
