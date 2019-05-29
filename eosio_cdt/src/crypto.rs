use crate::Check;
use eosio_cdt_sys::{capi_checksum160, capi_checksum256, capi_checksum512};
use eosio_core::{Ripemd160, Sha1, Sha256, Sha512};

pub trait Hasher: Check<()> {
    fn new(data: &str) -> Self;
}

impl Hasher for Ripemd160 {
    #[inline]
    fn new(data: &str) -> Self {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let mut c_hash = capi_checksum160::default();
        let c_hash_ptr: *mut capi_checksum160 =
            &mut c_hash as *mut _ as *mut capi_checksum160;
        unsafe { ::eosio_cdt_sys::ripemd160(data_ptr, data_len, c_hash_ptr) }
        c_hash.hash.into()
    }
}

impl Check<()> for Ripemd160 {
    #[inline]
    fn check(self, data: &str) {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let c_hash = capi_checksum160 {
            hash: self.into(),
            __bindgen_padding_0: [0_u32; 3],
        };
        let c_hash_ptr: *const capi_checksum160 =
            &c_hash as *const capi_checksum160;
        unsafe {
            ::eosio_cdt_sys::assert_ripemd160(data_ptr, data_len, c_hash_ptr)
        }
    }
}

impl Hasher for Sha1 {
    #[inline]
    fn new(data: &str) -> Self {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let mut c_hash = capi_checksum160::default();
        let c_hash_ptr: *mut capi_checksum160 =
            &mut c_hash as *mut _ as *mut capi_checksum160;
        unsafe { ::eosio_cdt_sys::sha1(data_ptr, data_len, c_hash_ptr) }
        c_hash.hash.into()
    }
}

impl Check<()> for Sha1 {
    #[inline]
    fn check(self, data: &str) {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let c_hash = capi_checksum160 {
            hash: self.into(),
            __bindgen_padding_0: [0_u32; 3],
        };
        let c_hash_ptr: *const capi_checksum160 =
            &c_hash as *const capi_checksum160;
        unsafe { ::eosio_cdt_sys::assert_sha1(data_ptr, data_len, c_hash_ptr) }
    }
}

impl Hasher for Sha256 {
    #[inline]
    fn new(data: &str) -> Self {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let mut c_hash = capi_checksum256::default();
        let c_hash_ptr: *mut capi_checksum256 =
            &mut c_hash as *mut _ as *mut capi_checksum256;
        unsafe { ::eosio_cdt_sys::sha256(data_ptr, data_len, c_hash_ptr) }
        c_hash.hash.into()
    }
}

impl Check<()> for Sha256 {
    #[inline]
    fn check(self, data: &str) {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let c_hash = capi_checksum256 { hash: self.into() };
        let c_hash_ptr: *const capi_checksum256 =
            &c_hash as *const capi_checksum256;
        unsafe {
            ::eosio_cdt_sys::assert_sha256(data_ptr, data_len, c_hash_ptr)
        }
    }
}

impl Hasher for Sha512 {
    #[inline]
    fn new(data: &str) -> Self {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let mut c_hash = capi_checksum512::default();
        let c_hash_ptr: *mut capi_checksum512 =
            &mut c_hash as *mut _ as *mut capi_checksum512;
        unsafe { ::eosio_cdt_sys::sha512(data_ptr, data_len, c_hash_ptr) }
        c_hash.hash.into()
    }
}

impl Check<()> for Sha512 {
    #[inline]
    fn check(self, data: &str) {
        let data_ptr = data.as_ptr();
        let data_len = data.len() as u32;
        let c_hash = capi_checksum512 { hash: self.into() };
        let c_hash_ptr: *const capi_checksum512 =
            &c_hash as *const capi_checksum512;
        unsafe {
            ::eosio_cdt_sys::assert_sha512(data_ptr, data_len, c_hash_ptr)
        }
    }
}
