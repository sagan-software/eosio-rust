use assert::Assert;
use eosio_macros::*;
use eosio_sys::{capi_checksum160, capi_checksum256, capi_checksum512};

pub trait Hasher: Assert<()> {
    fn new(data: &str) -> Self;
}

#[derive(Read, Write, Default, Clone, Copy)]
pub struct Checksum160([u8; 20usize]);

impl Hasher for Checksum160 {
    fn new(data: &str) -> Self {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let mut c_hash = capi_checksum160::default();
        let c_hash_ptr: *mut capi_checksum160 = &mut c_hash as *mut _ as *mut capi_checksum160;
        unsafe { ::eosio_sys::ripemd160(data_ptr, data_len, c_hash_ptr) }
        Checksum160(c_hash.hash)
    }
}

impl Assert<()> for Checksum160 {
    fn assert(self, data: &str) -> () {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let c_hash = capi_checksum160 {
            hash: self.0,
            __bindgen_padding_0: [0u32; 3],
        };
        let c_hash_ptr: *const capi_checksum160 = &c_hash as *const capi_checksum160;
        unsafe { ::eosio_sys::assert_ripemd160(data_ptr, data_len, c_hash_ptr) }
    }
}

#[derive(Read, Write, Default, Clone, Copy)]
pub struct Checksum256([u8; 32usize]);

impl Hasher for Checksum256 {
    fn new(data: &str) -> Self {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let mut c_hash = capi_checksum256::default();
        let c_hash_ptr: *mut capi_checksum256 = &mut c_hash as *mut _ as *mut capi_checksum256;
        unsafe { ::eosio_sys::sha256(data_ptr, data_len, c_hash_ptr) }
        Checksum256(c_hash.hash)
    }
}

impl Assert<()> for Checksum256 {
    fn assert(self, data: &str) -> () {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let c_hash = capi_checksum256 { hash: self.0 };
        let c_hash_ptr: *const capi_checksum256 = &c_hash as *const capi_checksum256;
        unsafe { ::eosio_sys::assert_sha256(data_ptr, data_len, c_hash_ptr) }
    }
}

#[derive(Read, Write, Clone, Copy)]
pub struct Checksum512([u8; 64usize]);

impl Hasher for Checksum512 {
    fn new(data: &str) -> Self {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let mut c_hash = capi_checksum512::default();
        let c_hash_ptr: *mut capi_checksum512 = &mut c_hash as *mut _ as *mut capi_checksum512;
        unsafe { ::eosio_sys::sha512(data_ptr, data_len, c_hash_ptr) }
        Checksum512(c_hash.hash)
    }
}

impl Assert<()> for Checksum512 {
    fn assert(self, data: &str) -> () {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let c_hash = capi_checksum512 { hash: self.0 };
        let c_hash_ptr: *const capi_checksum512 = &c_hash as *const capi_checksum512;
        unsafe { ::eosio_sys::assert_sha512(data_ptr, data_len, c_hash_ptr) }
    }
}
