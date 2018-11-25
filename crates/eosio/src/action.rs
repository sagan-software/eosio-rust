use crate::account::{AccountName, Authorization};
use crate::bytes::{NumBytes, Read, ReadError, Write, WriteError};
use eosio_macros::*;

/// This method will abort execution of wasm without failing the contract. This is used to bypass all cleanup / destructors that would normally be called.
#[cfg(feature = "contract")]
pub fn eosio_exit<C>(code: C)
where
    C: Into<i32>,
{
    let code: i32 = code.into();
    unsafe { ::eosio_sys::eosio_exit(code) }
}

eosio_name!(ActionName);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Action<Data> {
    pub account: AccountName,
    pub name: ActionName,
    pub authorization: Vec<Authorization>,
    pub data: Data,
}

impl<Data> NumBytes for Action<Data>
where
    Data: NumBytes,
{
    fn num_bytes(&self) -> usize {
        self.account.num_bytes()
            + self.name.num_bytes()
            + self.authorization.num_bytes()
            + self.data.num_bytes()
    }
}

#[cfg(feature = "contract")]
impl<Data> Write for Action<Data>
where
    Data: Write + NumBytes,
{
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        let pos = self.account.write(bytes, pos)?;
        let pos = self.name.write(bytes, pos)?;
        let pos = self.authorization.write(bytes, pos)?;

        let data_size = self.data.num_bytes();
        let mut data_bytes = vec![0u8; data_size];
        self.data.write(&mut data_bytes, 0)?;

        let pos = (&data_bytes[..]).write(bytes, pos)?;
        Ok(pos)
    }
}

#[derive(Clone, Debug)]
pub struct ActionId(u128);

impl<Data> Action<Data>
where
    Data: Write + NumBytes,
{
    #[cfg(feature = "contract")]
    pub fn send_inline(&self) -> Result<(), WriteError> {
        let size = self.num_bytes() + 1; // 1 extra byte is needed
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

    #[cfg(feature = "contract")]
    pub fn send_deferred<P>(
        &self,
        _id: ActionId,
        _payer: P,
        _replace_existing: bool,
    ) -> Result<(), WriteError>
    where
        P: Into<u64>,
    {
        // TODO
        Ok(())
    }

    #[cfg(feature = "contract")]
    pub fn cancel_deferred(_id: ActionId) -> Result<(), ()> {
        // TODO
        Ok(())
    }
}

pub trait ToAction: Sized {
    const NAME: u64;

    fn to_action(self, account: AccountName, authorization: Vec<Authorization>) -> Action<Self> {
        Action {
            account,
            name: Self::NAME.into(),
            authorization,
            data: self,
        }
    }
}

pub trait ActionFn: ToAction + Read + Write + NumBytes + Clone {
    #[cfg(feature = "contract")]
    fn execute(self);

    #[cfg(feature = "contract")]
    fn read_data() -> Result<(Self, usize), ReadError> {
        let num_bytes = unsafe { ::eosio_sys::action_data_size() };
        let mut bytes = vec![0u8; num_bytes as usize];
        let ptr: *mut ::eosio_sys::c_void = &mut bytes[..] as *mut _ as *mut ::eosio_sys::c_void;
        unsafe {
            ::eosio_sys::read_action_data(ptr, num_bytes);
        }

        Self::read(&bytes, 0)
    }

    #[cfg(feature = "contract")]
    fn send_inline(self, authorization: Vec<Authorization>) -> Result<(), WriteError> {
        self.to_action(AccountName::receiver(), authorization)
            .send_inline()
    }
}
