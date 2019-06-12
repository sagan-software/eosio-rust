//! TODO docs
use crate::{AccountName, ActionName, Authorization};
use eosio_bytes::{NumBytes, Write, WriteError};
use serde::{Deserialize, Serialize};

/// TODO docs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Action<Data> {
    /// TODO docs
    pub account: AccountName,
    /// TODO docs
    pub name: ActionName,
    /// TODO docs
    pub authorization: Vec<Authorization>,
    /// TODO docs
    pub data: Data,
}

impl<Data> NumBytes for Action<Data>
where
    Data: NumBytes,
{
    /// TODO docs
    #[inline]
    fn num_bytes(&self) -> usize {
        self.account
            .num_bytes()
            .saturating_add(self.name.num_bytes())
            .saturating_add(self.authorization.num_bytes())
            .saturating_add(self.data.num_bytes())
    }
}

impl<Data> Write for Action<Data>
where
    Data: Write + NumBytes,
{
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
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

/// TODO docs
#[derive(Clone, Debug)]
pub struct DeferredTransactionId(u128);

/// TODO docs
pub trait ToAction: Sized + Write + NumBytes {
    /// TODO docs
    const NAME: u64;

    /// TODO docs
    #[inline]
    fn to_action(
        self,
        account: AccountName,
        authorization: Vec<Authorization>,
    ) -> Action<Self> {
        Action {
            account,
            name: Self::NAME.into(),
            authorization,
            data: self,
        }
    }
}
