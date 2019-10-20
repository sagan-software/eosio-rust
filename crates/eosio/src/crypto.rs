//! TODO docs
use crate::account::AccountName;
use crate::bytes::{NumBytes, Read, Write};
use crate::varint::UnsignedInt;

/// TODO docs
/// TODO Read, Write, `NumBytes` needs a custom implementation based on `fixed_bytes`
#[derive(
    Read,
    Write,
    NumBytes,
    Default,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
#[__eosio_path = "crate::bytes"]
pub struct Checksum160([u8; 20]);

impl Checksum160 {
    /// TODO docs.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// TODO docs.
    pub const fn to_bytes(&self) -> [u8; 20] {
        self.0
    }
}

impl From<[u8; 20]> for Checksum160 {
    #[inline]
    fn from(value: [u8; 20]) -> Self {
        Self(value)
    }
}

impl From<Checksum160> for [u8; 20] {
    #[inline]
    fn from(value: Checksum160) -> Self {
        value.0
    }
}

/// TODO docs
// TODO Read, Write, NumBytes needs a custom implementation based on fixed_bytes
#[derive(
    Read,
    Write,
    NumBytes,
    Default,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
#[__eosio_path = "crate::bytes"]
pub struct Checksum256([u8; 32]);

impl Checksum256 {
    /// TODO docs.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// TODO docs.
    pub const fn to_bytes(&self) -> [u8; 32] {
        self.0
    }
}

impl From<[u8; 32]> for Checksum256 {
    #[inline]
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

impl From<Checksum256> for [u8; 32] {
    #[inline]
    fn from(value: Checksum256) -> Self {
        value.0
    }
}

/// TODO docs
/// TODO Read, Write, `NumBytes` needs a custom implementation based on `fixed_bytes`
#[derive(Read, Write, NumBytes, Clone, Copy)]
#[__eosio_path = "crate::bytes"]
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

/// Maps producer with its signing key, used for producer schedule
/// <https://github.com/EOSIO/eosio.cdt/blob/796ff8bee9a0fc864f665a0a4d018e0ff18ac383/libraries/eosiolib/contracts/eosio/producer_schedule.hpp#L15-L45>
#[derive(Read, Write, NumBytes, Clone, Default)]
#[__eosio_path = "crate::bytes"]
pub struct ProducerKey {
    /// Name of the producer
    pub producer_name: AccountName,
    /// Block signing key used by this producer
    pub block_signing_key: PublicKey,
}

/// EOSIO Public Key
/// <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/crypto.hpp#L22-L48>
#[derive(Read, Write, NumBytes, Clone)]
#[__eosio_path = "crate::bytes"]
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

impl PartialEq for PublicKey {
    fn eq(&self, other: &Self) -> bool {
        self.type_ == other.type_ && self.as_bytes() == other.as_bytes()
    }
}

impl std::fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.type_, f)?;
        std::fmt::Debug::fmt(self.as_bytes(), f)
    }
}

/// EOSIO Signature
/// <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/crypto.hpp#L93-L120>
#[derive(Read, Write, NumBytes, Clone)]
#[__eosio_path = "crate::bytes"]
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
