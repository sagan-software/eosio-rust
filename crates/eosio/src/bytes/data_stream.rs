use super::{Read, ReadError, Write, WriteError};
use alloc::vec::Vec;
use core::ops::Deref;

/// A stream of bytes
pub struct DataStream {
    /// TODO docs
    bytes: Vec<u8>,
    /// TODO docs
    pos: usize,
}

impl DataStream {
    /// Read something from the stream
    ///
    /// # Errors
    ///
    /// Will return `Err` if there was a problem reading the data.
    #[inline]
    pub fn read<T: Read>(&mut self) -> Result<T, ReadError> {
        T::read(&self.bytes, &mut self.pos)
    }

    /// Write something to the stream
    ///
    /// # Errors
    ///
    /// Will return `Err` if there was a problem writing the data.
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn write<T: Write>(&mut self, thing: T) -> Result<(), WriteError> {
        thing.write(&mut self.bytes, &mut self.pos)
    }

    /// Gets the remaining number of bytes
    #[inline]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Gets remaining bytes as slice
    #[inline]
    #[must_use]
    pub fn as_remaining_bytes(&self) -> Option<&[u8]> {
        self.bytes.get(self.pos..)
    }

    /// Resets the data stream position
    #[inline]
    pub fn reset(&mut self) {
        self.pos = 0;
    }

    /// Get the current position
    #[inline]
    #[must_use]
    pub const fn position(&self) -> usize {
        self.pos
    }

    /// Gets the remaining number of bytes
    #[inline]
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.bytes.len() - self.pos
    }
}

impl From<Vec<u8>> for DataStream {
    #[must_use]
    fn from(bytes: Vec<u8>) -> Self {
        Self { bytes, pos: 0 }
    }
}

impl From<&[u8]> for DataStream {
    #[must_use]
    fn from(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.to_vec(),
            pos: 0,
        }
    }
}

impl Deref for DataStream {
    type Target = [u8];

    #[must_use]
    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}

impl AsRef<[u8]> for DataStream {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::DataStream;
    use crate::{n, AccountName, Name};

    #[test]
    fn read_write() {
        let account = AccountName::from(n!("eosio.token"));
        let name = Name::from(n!("alice"));
        let mut ds1 = DataStream::from(vec![0; 16]);
        assert_eq!(ds1.position(), 0);
        assert_eq!(ds1.remaining(), 16);
        ds1.write(account).expect("failed to write account");
        assert_eq!(ds1.position(), 8);
        assert_eq!(ds1.remaining(), 8);
        assert_eq!(
            ds1.as_remaining_bytes()
                .expect("failed to get remaining bytes")
                .len(),
            8
        );
        ds1.write(name).expect("failed to write name");
        assert_eq!(ds1.position(), 16);
        assert_eq!(ds1.remaining(), 0);
        let bytes = ds1.as_bytes();
        let mut ds2 = DataStream::from(bytes);
        assert_eq!(ds2.position(), 0);
        assert_eq!(ds2.remaining(), 16);
        let account2 =
            ds2.read::<AccountName>().expect("failed to read account");
        assert_eq!(ds2.position(), 8);
        assert_eq!(ds2.remaining(), 8);
        let name2 = ds2.read::<Name>().expect("failed to read name");
        assert_eq!(ds2.position(), 16);
        assert_eq!(ds2.remaining(), 0);
        assert_eq!(account, account2);
        assert_eq!(name, name2);
    }
}
