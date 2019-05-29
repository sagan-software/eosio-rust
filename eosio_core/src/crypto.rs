use eosio_bytes::{NumBytes, Read, Write};

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
#[eosio_bytes_root_path = "::eosio_bytes"]
pub struct Ripemd160([u8; 20_usize]);

impl Ripemd160 {
    pub const fn new(value: [u8; 20]) -> Self {
        Self(value)
    }
}

impl From<Ripemd160> for [u8; 20] {
    fn from(value: Ripemd160) -> Self {
        value.0
    }
}

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
#[eosio_bytes_root_path = "::eosio_bytes"]
pub struct Sha1([u8; 20_usize]);

impl Sha1 {
    pub const fn new(value: [u8; 20]) -> Self {
        Self(value)
    }
}

impl From<Sha1> for [u8; 20] {
    fn from(value: Sha1) -> Self {
        value.0
    }
}

#[derive(Read, Write, NumBytes, Default, Clone, Copy)]
#[eosio_bytes_root_path = "::eosio_bytes"]
pub struct Sha256([u8; 32_usize]);

impl Sha256 {
    pub const fn new(value: [u8; 32]) -> Self {
        Self(value)
    }
}

impl From<Sha256> for [u8; 32] {
    fn from(value: Sha256) -> Self {
        value.0
    }
}

#[derive(Read, Write, NumBytes, Clone, Copy)]
#[eosio_bytes_root_path = "::eosio_bytes"]
pub struct Sha512([u8; 64_usize]);

impl Sha512 {
    pub const fn new(value: [u8; 64]) -> Self {
        Self(value)
    }
}

impl From<Sha512> for [u8; 64] {
    fn from(value: Sha512) -> Self {
        value.0
    }
}
