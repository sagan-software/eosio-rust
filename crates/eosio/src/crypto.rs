#[cfg(feature = "contract")]
use crate::assert::Assert;
use eosio_macros::*;
#[cfg(feature = "contract")]
use eosio_sys::{capi_checksum160, capi_checksum256, capi_checksum512};

#[cfg(feature = "contract")]
pub trait Hasher: Assert<()> {
    fn new(data: &str) -> Self;
}

#[derive(
    Read, Write, NumBytes, Default, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct Ripemd160([u8; 20usize]);

#[cfg(feature = "contract")]
impl Hasher for Ripemd160 {
    fn new(data: &str) -> Self {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let mut c_hash = capi_checksum160::default();
        let c_hash_ptr: *mut capi_checksum160 = &mut c_hash as *mut _ as *mut capi_checksum160;
        unsafe { ::eosio_sys::ripemd160(data_ptr, data_len, c_hash_ptr) }
        Ripemd160(c_hash.hash)
    }
}

#[cfg(feature = "contract")]
impl Assert<()> for Ripemd160 {
    fn assert(self, data: &str) {
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

#[derive(
    Read, Write, NumBytes, Default, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct Sha1([u8; 20usize]);

#[cfg(feature = "contract")]
impl Hasher for Sha1 {
    fn new(data: &str) -> Self {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let mut c_hash = capi_checksum160::default();
        let c_hash_ptr: *mut capi_checksum160 = &mut c_hash as *mut _ as *mut capi_checksum160;
        unsafe { ::eosio_sys::sha1(data_ptr, data_len, c_hash_ptr) }
        Sha1(c_hash.hash)
    }
}

#[cfg(feature = "contract")]
impl Assert<()> for Sha1 {
    fn assert(self, data: &str) {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let c_hash = capi_checksum160 {
            hash: self.0,
            __bindgen_padding_0: [0u32; 3],
        };
        let c_hash_ptr: *const capi_checksum160 = &c_hash as *const capi_checksum160;
        unsafe { ::eosio_sys::assert_sha1(data_ptr, data_len, c_hash_ptr) }
    }
}

#[derive(Read, Write, NumBytes, Default, Clone, Copy)]
pub struct Sha256([u8; 32usize]);

#[cfg(feature = "contract")]
impl Hasher for Sha256 {
    fn new(data: &str) -> Self {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let mut c_hash = capi_checksum256::default();
        let c_hash_ptr: *mut capi_checksum256 = &mut c_hash as *mut _ as *mut capi_checksum256;
        unsafe { ::eosio_sys::sha256(data_ptr, data_len, c_hash_ptr) }
        Sha256(c_hash.hash)
    }
}

#[cfg(feature = "contract")]
impl Assert<()> for Sha256 {
    fn assert(self, data: &str) {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let c_hash = capi_checksum256 { hash: self.0 };
        let c_hash_ptr: *const capi_checksum256 = &c_hash as *const capi_checksum256;
        unsafe { ::eosio_sys::assert_sha256(data_ptr, data_len, c_hash_ptr) }
    }
}

#[derive(Read, Write, NumBytes, Clone, Copy)]
pub struct Sha512([u8; 64usize]);

#[cfg(feature = "contract")]
impl Hasher for Sha512 {
    fn new(data: &str) -> Self {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let mut c_hash = capi_checksum512::default();
        let c_hash_ptr: *mut capi_checksum512 = &mut c_hash as *mut _ as *mut capi_checksum512;
        unsafe { ::eosio_sys::sha512(data_ptr, data_len, c_hash_ptr) }
        Sha512(c_hash.hash)
    }
}

#[cfg(feature = "contract")]
impl Assert<()> for Sha512 {
    fn assert(self, data: &str) {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let c_hash = capi_checksum512 { hash: self.0 };
        let c_hash_ptr: *const capi_checksum512 = &c_hash as *const capi_checksum512;
        unsafe { ::eosio_sys::assert_sha512(data_ptr, data_len, c_hash_ptr) }
    }
}
