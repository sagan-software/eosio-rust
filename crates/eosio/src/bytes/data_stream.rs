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
    #[inline]
    pub fn read<T: Read>(&mut self) -> Result<T, ReadError> {
        let bytes = self
            .bytes
            .get(self.pos..)
            .ok_or(ReadError::NotEnoughBytes)?;
        T::read(bytes, &mut self.pos)
    }

    /// Write something to the stream
    #[inline]
    pub fn write<T: Write>(&mut self, thing: &T) -> Result<(), WriteError> {
        let bytes = self
            .bytes
            .get_mut(self.pos..)
            .ok_or(WriteError::NotEnoughSpace)?;
        thing.write(bytes, &mut self.pos)
    }

    /// Gets bytes as slice
    #[inline]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes.get(self.pos..).unwrap_or_else(|| &self.bytes)
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

impl Deref for DataStream {
    type Target = [u8];
    #[must_use]
    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}
