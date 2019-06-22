//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/crypto.hpp#L93-L120>
use crate::{NumBytes, Read, UnsignedInt, Write};

/// EOSIO Signature
#[derive(Read, Write, NumBytes, Clone)]
#[eosio_core_root_path = "crate"]
pub struct Signature {
    /// Type of the signature, could be either K1 or R1
    pub type_: UnsignedInt,
    /// Bytes of the signature
    pub data: [u8; 66],
}

impl Signature {
    /// TODO docs.
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
    /// TODO docs.
    pub const fn to_bytes(&self) -> [u8; 66] {
        self.data
    }
}
