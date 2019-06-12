//! TODO docs

use eosio_bytes::{NumBytes, Read, Write};

/// TODO docs
#[derive(Read, Write, NumBytes, Clone, Copy)]
#[eosio_bytes_root_path = "::eosio_bytes"]
pub struct Signature([u8; 66]);

impl Signature {
    /// TODO docs.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    /// TODO docs.
    pub fn to_bytes(&self) -> [u8; 66] {
        &self.0
    }
}

impl From<[u8; 66]> for Signature {
    #[inline]
    fn from(value: [u8; 66]) -> Self {
        Self(value)
    }
}

impl From<Signature> for [u8; 66] {
    #[inline]
    fn from(value: Signature) -> Self {
        value.0
    }
}
