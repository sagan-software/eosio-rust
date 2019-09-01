//! TODO docs
use crate::{NumBytes, Read, Write};

/// TODO docs
/// TODO Read, Write, `NumBytes` needs a custom implementation based on `fixed_bytes`
#[derive(Read, Write, NumBytes, Clone, Copy)]
#[eosio_core_root_path = "crate"]
pub struct Checksum512([u8; 64]);

impl Checksum512 {
    /// TODO docs.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// TODO docs.
    pub const fn to_bytes(&self) -> [u8; 64] {
        self.0
    }
}

impl From<[u8; 64]> for Checksum512 {
    #[inline]
    fn from(value: [u8; 64]) -> Self {
        Self(value)
    }
}

impl From<Checksum512> for [u8; 64] {
    #[inline]
    fn from(value: Checksum512) -> Self {
        value.0
    }
}
