//! TODO docs
use crate::{AccountName, ActionName, Authorization};
use eosio_bytes::{NumBytes, Read, Write};
use serde::{Deserialize, Serialize};

/// TODO docs
#[derive(
    Clone, Debug, Serialize, Deserialize, Read, Write, NumBytes, Default,
)]
#[eosio_bytes_root_path = "::eosio_bytes"]
pub struct Action {
    /// TODO docs
    pub account: AccountName,
    /// TODO docs
    pub name: ActionName,
    /// TODO docs
    pub authorization: Vec<Authorization>,
    /// TODO docs
    pub data: Vec<u8>,
}

/// TODO docs
pub trait ToAction: Write + NumBytes {
    /// TODO docs
    const NAME: u64;

    /// TODO docs
    #[inline]
    fn to_action(
        &self,
        account: AccountName,
        authorization: Vec<Authorization>,
    ) -> Action {
        let mut data = vec![0_u8; self.num_bytes()];
        self.write(&mut data, &mut 0).expect("write");

        Action {
            account,
            name: Self::NAME.into(),
            authorization,
            data,
        }
    }
}
