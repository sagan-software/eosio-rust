use crate::account::{AccountName, Authorization};
use crate::bytes::NumBytes;
#[cfg(feature = "contract")]
use crate::bytes::{Read, ReadError, Write, WriteError};
use eosio_macros::*;

/// This method will abort execution of wasm without failing the contract. This is used to bypass all cleanup / destructors that would normally be called.
#[cfg(feature = "contract")]
#[inline]
pub fn eosio_exit<C>(code: C)
where
    C: Into<i32>,
{
    unsafe { ::eosio_sys::eosio_exit(code.into()) }
}

name!(ActionName);

/// Docs
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Action<Data> {
    /// Docs
    pub account: AccountName,
    /// Docs
    pub name: ActionName,
    /// Docs
    pub authorization: Vec<Authorization>,
    /// Docs
    pub data: Data,
}

impl<Data> NumBytes for Action<Data>
where
    Data: NumBytes,
{
    /// Docs
    #[inline]
    fn num_bytes(&self) -> usize {
        self.account
            .num_bytes()
            .saturating_add(self.name.num_bytes())
            .saturating_add(self.authorization.num_bytes())
            .saturating_add(self.data.num_bytes())
    }
}

#[cfg(feature = "contract")]
impl<Data> Write for Action<Data>
where
    Data: Write + NumBytes,
{
    #[inline]
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        self.account.write(bytes, pos)?;
        self.name.write(bytes, pos)?;
        self.authorization.write(bytes, pos)?;

        let data_size = self.data.num_bytes();
        let mut data_bytes = vec![0_u8; data_size];
        let mut data_pos = 0;
        self.data.write(&mut data_bytes, &mut data_pos)?;

        (&data_bytes[..]).write(bytes, pos)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Id(u128);

#[cfg(feature = "contract")]
impl<Data> Action<Data>
where
    Data: Write + NumBytes,
{
    #[inline]
    pub fn send_inline(&self) -> Result<(), WriteError> {
        let size = self.num_bytes().saturating_add(1); // 1 extra byte is needed
        let mut bytes = vec![0_u8; size];
        let mut pos = 0;
        self.write(&mut bytes, &mut pos)?;
        let ptr = bytes[..].as_mut_ptr();
        if self.authorization.is_empty() {
            unsafe { ::eosio_sys::send_context_free_inline(ptr, pos) }
        } else {
            unsafe { ::eosio_sys::send_inline(ptr, pos) }
        }
        Ok(())
    }

    #[inline]
    pub fn send_deferred<P>(
        &self,
        _id: Id,
        _payer: P,
        _replace_existing: bool,
    ) -> Result<(), WriteError>
    where
        P: Into<u64>,
    {
        // TODO
        Ok(())
    }

    #[inline]
    pub fn cancel_deferred(_id: Id) -> Result<(), ()> {
        // TODO
        Ok(())
    }
}

pub trait ToAction: Sized {
    const NAME: u64;

    #[inline]
    fn to_action(self, account: AccountName, authorization: Vec<Authorization>) -> Action<Self> {
        Action {
            account,
            name: Self::NAME.into(),
            authorization,
            data: self,
        }
    }
}

#[cfg(feature = "contract")]
pub trait ActionFn: ToAction + Read + Write + NumBytes + Clone {
    fn execute(self);

    #[inline]
    fn read_data() -> Result<Self, ReadError> {
        let num_bytes = unsafe { ::eosio_sys::action_data_size() };
        let mut bytes = vec![0_u8; num_bytes as usize];
        let ptr: *mut ::eosio_sys::c_void = &mut bytes[..] as *mut _ as *mut ::eosio_sys::c_void;
        unsafe {
            ::eosio_sys::read_action_data(ptr, num_bytes);
        }

        let mut pos = 0;
        Self::read(&bytes, &mut pos)
    }

    #[inline]
    fn send_inline(self, authorization: Vec<Authorization>) -> Result<(), WriteError> {
        self.to_action(AccountName::receiver(), authorization)
            .send_inline()
    }
}
