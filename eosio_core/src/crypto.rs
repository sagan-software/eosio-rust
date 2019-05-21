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

#[derive(Read, Write, NumBytes, Default, Clone, Copy)]
#[eosio_bytes_root_path = "::eosio_bytes"]
pub struct Sha256([u8; 32_usize]);

#[derive(Read, Write, NumBytes, Clone, Copy)]
#[eosio_bytes_root_path = "::eosio_bytes"]
pub struct Sha512([u8; 64_usize]);
