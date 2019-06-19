//! TODO docs
use eosio_bytes::{Read, ReadError, Write, WriteError};
use std::ops::Deref;

/// A stream of bytes
pub struct DataStream {
    /// TODO docs
    bytes: Vec<u8>,
    /// TODO docs
    pos: usize,
}

impl DataStream {
    /// Creates a new DataStream
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: vec![0_u8; capacity],
            pos: 0,
        }
    }

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
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes.get(self.pos..).unwrap_or_else(|| &self.bytes)
    }

    /// Gets remaining number of bytes
    #[inline]
    pub fn remaining(&self) -> usize {
        self.bytes.len() - self.pos
    }

    /// Resets the data stream position
    #[inline]
    pub fn reset(&mut self) {
        self.pos = 0;
    }
}

impl From<Vec<u8>> for DataStream {
    fn from(bytes: Vec<u8>) -> Self {
        Self { bytes, pos: 0 }
    }
}

impl Deref for DataStream {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}
