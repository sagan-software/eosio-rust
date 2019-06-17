//! TODO docs
use eosio_bytes::{NumBytes, Read, ReadError, Write, WriteError};
use std::marker::PhantomData;

/// TODO docs
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Default)]
pub struct Ignore<T>(PhantomData<T>);

impl<T> NumBytes for Ignore<T> {
    #[inline]
    fn num_bytes(&self) -> usize {
        0
    }
}

impl<T> Read for Ignore<T> {
    #[inline]
    fn read(_bytes: &[u8], _pos: &mut usize) -> Result<Self, ReadError> {
        Ok(Self(PhantomData))
    }
}

impl<T> Write for Ignore<T> {
    #[inline]
    fn write(
        &self,
        _bytes: &mut [u8],
        _pos: &mut usize,
    ) -> Result<(), WriteError> {
        Ok(())
    }
}
