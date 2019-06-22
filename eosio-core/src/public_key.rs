//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/crypto.hpp#L22-L48>
use crate::{NumBytes, Read, UnsignedInt, Write};

/// EOSIO Public Key
#[derive(Read, Write, NumBytes, Clone)]
#[eosio_core_root_path = "crate"]
pub struct PublicKey {
    /// Type of the public key, could be either K1 or R1
    pub type_: UnsignedInt,
    /// Bytes of the public key
    pub data: [u8; 34],
}

impl PublicKey {
    /// TODO docs.
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// TODO docs.
    pub const fn to_bytes(&self) -> [u8; 34] {
        self.data
    }
}

impl Default for PublicKey {
    fn default() -> Self {
        Self {
            type_: UnsignedInt::default(),
            data: [0_u8; 34],
        }
    }
}
