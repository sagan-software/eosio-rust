use eosio::{Checksum160, Checksum256, Checksum512};
use eosio_cdt_sys::{capi_checksum160, capi_checksum256, capi_checksum512};

/// Hashes `data` using RIPEMD160.
#[must_use]
#[inline]
pub fn ripemd160<T: AsRef<[u8]>>(data: T) -> Checksum160 {
    let data = data.as_ref();
    let data_ptr = data.as_ptr();
    #[allow(clippy::cast_possible_truncation)]
    let data_len = data.len() as u32;
    let mut c_hash = capi_checksum160::default();
    let c_hash_ptr: *mut capi_checksum160 =
        &mut c_hash as *mut _ as *mut capi_checksum160;
    unsafe { eosio_cdt_sys::ripemd160(data_ptr, data_len, c_hash_ptr) }
    c_hash.hash.into()
}

/// Tests if the RIPEMD160 hash generated from data matches the provided digest.
#[inline]
pub fn assert_ripemd160<C, T>(checksum: C, data: T)
where
    C: AsRef<Checksum160>,
    T: AsRef<[u8]>,
{
    let checksum = checksum.as_ref();
    let data = data.as_ref();
    let data_ptr = data.as_ptr();
    #[allow(clippy::cast_possible_truncation)]
    let data_len = data.len() as u32;
    let c_hash = capi_checksum160 {
        hash: checksum.to_bytes(),
        __bindgen_padding_0: [0_u32; 3],
    };
    let c_hash_ptr: *const capi_checksum160 =
        &c_hash as *const capi_checksum160;
    unsafe { eosio_cdt_sys::assert_ripemd160(data_ptr, data_len, c_hash_ptr) }
}

/// Hashes `data` using SHA1.
#[must_use]
#[inline]
pub fn sha1<T: AsRef<[u8]>>(data: T) -> Checksum160 {
    let data = data.as_ref();
    let data_ptr = data.as_ptr();
    #[allow(clippy::cast_possible_truncation)]
    let data_len = data.len() as u32;
    let mut c_hash = capi_checksum160::default();
    let c_hash_ptr: *mut capi_checksum160 =
        &mut c_hash as *mut _ as *mut capi_checksum160;
    unsafe { eosio_cdt_sys::sha1(data_ptr, data_len, c_hash_ptr) }
    c_hash.hash.into()
}

/// Tests if the SHA1 hash generated from data matches the provided digest.
#[inline]
pub fn assert_sha1<C, T>(checksum: C, data: T)
where
    C: AsRef<Checksum160>,
    T: AsRef<[u8]>,
{
    let checksum = checksum.as_ref();
    let data = data.as_ref();
    let data_ptr = data.as_ptr();
    #[allow(clippy::cast_possible_truncation)]
    let data_len = data.len() as u32;
    let c_hash = capi_checksum160 {
        hash: checksum.to_bytes(),
        __bindgen_padding_0: [0_u32; 3],
    };
    let c_hash_ptr: *const capi_checksum160 =
        &c_hash as *const capi_checksum160;
    unsafe { eosio_cdt_sys::assert_sha1(data_ptr, data_len, c_hash_ptr) }
}

/// Hashes `data` using SHA256.
#[must_use]
#[inline]
pub fn sha256<T: AsRef<[u8]>>(data: T) -> Checksum256 {
    let data = data.as_ref();
    let data_ptr = data.as_ptr();
    #[allow(clippy::cast_possible_truncation)]
    let data_len = data.len() as u32;
    let mut c_hash = capi_checksum256::default();
    let c_hash_ptr = &mut c_hash as *mut _ as *mut capi_checksum256;
    unsafe { eosio_cdt_sys::sha256(data_ptr, data_len, c_hash_ptr) }
    c_hash.hash.into()
}

/// Tests if the SHA256 hash generated from data matches the provided digest.
#[inline]
pub fn assert_sha256<C, T>(checksum: C, data: T)
where
    C: AsRef<Checksum256>,
    T: AsRef<[u8]>,
{
    let checksum = checksum.as_ref();
    let data = data.as_ref();
    let data_ptr = data.as_ptr();
    #[allow(clippy::cast_possible_truncation)]
    let data_len = data.len() as u32;
    let c_hash = capi_checksum256 {
        hash: checksum.to_bytes(),
    };
    let c_hash_ptr: *const capi_checksum256 =
        &c_hash as *const capi_checksum256;
    unsafe { eosio_cdt_sys::assert_sha256(data_ptr, data_len, c_hash_ptr) }
}

/// Hashes `data` using SHA512.
#[must_use]
#[inline]
pub fn sha512<T: AsRef<[u8]>>(data: T) -> Checksum512 {
    let data = data.as_ref();
    let data_ptr = data.as_ptr();
    #[allow(clippy::cast_possible_truncation)]
    let data_len = data.len() as u32;
    let mut c_hash = capi_checksum512::default();
    let c_hash_ptr: *mut capi_checksum512 =
        &mut c_hash as *mut _ as *mut capi_checksum512;
    unsafe { eosio_cdt_sys::sha512(data_ptr, data_len, c_hash_ptr) }
    c_hash.hash.into()
}

/// Tests if the SHA512 hash generated from data matches the provided digest.
#[inline]
pub fn assert_sha512<C, T>(checksum: C, data: T)
where
    C: AsRef<Checksum512>,
    T: AsRef<[u8]>,
{
    let checksum = checksum.as_ref();
    let data = data.as_ref();
    let data_ptr = data.as_ptr();
    #[allow(clippy::cast_possible_truncation)]
    let data_len = data.len() as u32;
    let c_hash = capi_checksum512 {
        hash: checksum.to_bytes(),
    };
    let c_hash_ptr: *const capi_checksum512 =
        &c_hash as *const capi_checksum512;
    unsafe { eosio_cdt_sys::assert_sha512(data_ptr, data_len, c_hash_ptr) }
}

// TODO recover_key
// TODO assert_recover_key
