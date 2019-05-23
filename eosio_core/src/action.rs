use crate::{AccountName, ActionName, Authorization};
use eosio_bytes::{NumBytes, Write, WriteError};
use serde::{Deserialize, Serialize};

/// Docs
#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug)]
pub struct DeferredId(u128);

pub trait ToAction: Sized + Write + NumBytes {
    const NAME: u64;

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
