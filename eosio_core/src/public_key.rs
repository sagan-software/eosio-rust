//! TODO docs
use eosio_bytes::{NumBytes, Read, Write};

/// TODO docs
#[derive(Read, Write, NumBytes, Clone, Copy)]
#[eosio_bytes_root_path = "::eosio_bytes"]
pub struct PublicKey([u8; 34]);

impl PublicKey {
    /// TODO docs.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// TODO docs.
    pub const fn to_bytes(&self) -> [u8; 34] {
        self.0
    }
}

impl From<[u8; 34]> for PublicKey {
    #[inline]
    fn from(value: [u8; 34]) -> Self {
        Self(value)
    }
}

impl From<PublicKey> for [u8; 34] {
    #[inline]
    fn from(value: PublicKey) -> Self {
        value.0
    }
}
