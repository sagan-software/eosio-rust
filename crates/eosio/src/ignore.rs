//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/ignore.hpp#L12-L20>
use crate::{NumBytes, Read, ReadError, Write, WriteError};
use std::marker::PhantomData;

/// Tells the datastream to ignore this type, but allows the abi generator to
/// add the correct type.
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
