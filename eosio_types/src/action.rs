use eosio_bytes::*;
use eosio_derives::*;
use names::*;

#[cfg(any(feature = "std", feature = "alloc"))]
use lib::Vec;

#[cfg(any(feature = "std", feature = "alloc"))]
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

        // let pos = data_size.write(bytes, pos)?;
        let pos = (&data_bytes[..=data_size]).write(bytes, pos)?;
        Ok(pos)
    }
}

#[derive(Readable, Writeable, Clone, Debug)]
#[readable_path = "::eosio_bytes::Readable"]
#[writeable_path = "::eosio_bytes::Writeable"]
pub struct PermissionLevel {
    pub actor: AccountName,
    pub permission: PermissionName,
}
