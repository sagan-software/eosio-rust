use core::convert::TryInto;
use eosio::{
    AccountName, BlockchainParameters, Checksum256, CpuWeight, NetWeight,
    NumBytes, ProducerKey, RamBytes, Read, ReadError, Write, WriteError,
};

/// Check if an account is privileged
#[inline]
pub fn is_privileged<A: AsRef<AccountName>>(account: A) -> bool {
    let a = account.as_ref().as_u64();
    unsafe { eosio_cdt_sys::is_privileged(a) }
}

/// Get the resource limits of an account
#[inline]
pub fn get_resource_limits<A: AsRef<AccountName>>(
    account: A,
) -> (RamBytes, NetWeight, CpuWeight) {
    let mut ram_bytes = 0_i64;
    let ram_bytes_ptr = &mut ram_bytes as *mut _ as *mut i64;
    let mut net_weight = 0_i64;
    let net_weight_ptr = &mut net_weight as *mut _ as *mut i64;
    let mut cpu_weight = 0_i64;
    let cpu_weight_ptr = &mut cpu_weight as *mut _ as *mut i64;
    let a = account.as_ref().as_u64();
    unsafe {
        eosio_cdt_sys::get_resource_limits(
            a,
            ram_bytes_ptr,
            net_weight_ptr,
            cpu_weight_ptr,
        )
    };
    (
        RamBytes::from(ram_bytes),
        NetWeight::from(net_weight),
        CpuWeight::from(cpu_weight),
    )
}

/// Set the resource limits of an account
#[inline]
pub fn set_resource_limits<A: AsRef<AccountName>>(
    account: A,
    ram_bytes: i64,
    net_weight: i64,
    cpu_weight: i64,
) {
    let a = account.as_ref().as_u64();
    unsafe {
        eosio_cdt_sys::set_resource_limits(a, ram_bytes, net_weight, cpu_weight)
    }
}

/// Set the privileged status of an account
#[inline]
pub fn set_privileged<A: AsRef<AccountName>>(account: A, is_priv: bool) {
    let a = account.as_ref().as_u64();
    unsafe { eosio_cdt_sys::set_privileged(a, is_priv) }
}

/// Set the blockchain parameters
///
/// # Errors
///
/// Returns an error if there was a problem serializing the parameters.
#[inline]
pub fn set_blockchain_parameters<T: AsRef<BlockchainParameters>>(
    params: T,
) -> Result<(), WriteError> {
    let params = params.as_ref();
    let size = params.num_bytes();
    let mut buf = vec![0_u8; size];
    params.write(&mut buf, &mut 0)?;
    let buf_ptr = &mut buf as *mut _ as *mut u8;
    #[allow(clippy::cast_possible_truncation)]
    unsafe {
        eosio_cdt_sys::set_blockchain_parameters_packed(buf_ptr, size as u32)
    }
    Ok(())
}

/// Retrieve the blolckchain parameters
///
/// # Errors
///
/// Returns an error if there was a problem reading the parameters.
#[inline]
pub fn get_blockchain_parameters() -> Result<BlockchainParameters, ReadError> {
    let expected_size = BlockchainParameters::default().num_bytes();
    let mut buf = vec![0_u8; expected_size];
    let buf_ptr = &mut buf as *mut _ as *mut u8;
    #[allow(clippy::cast_possible_truncation)]
    let actual_size = unsafe {
        eosio_cdt_sys::get_blockchain_parameters_packed(
            buf_ptr,
            expected_size as u32,
        )
    } as usize;
    if actual_size <= expected_size {
        Err(ReadError::NotEnoughBytes)
    } else {
        BlockchainParameters::read(&buf, &mut 0)
    }
}

/// Proposes a schedule change
#[must_use]
#[inline]
pub fn set_proposed_producers<T: AsRef<[ProducerKey]>>(
    prods: T,
) -> Option<u64> {
    let prods = prods.as_ref();
    let size = prods.num_bytes();
    let mut buf = vec![0_u8; size];
    let buf_ptr = &mut buf as *mut _ as *mut u8;
    #[allow(clippy::cast_possible_truncation)]
    let result =
        unsafe { eosio_cdt_sys::set_proposed_producers(buf_ptr, size as u32) };
    if result >= 0 {
        result.try_into().ok()
    } else {
        None
    }
}

#[must_use]
#[inline]
pub fn is_feature_activated<T: AsRef<Checksum256>>(feature_digest: T) -> bool {
    use eosio_cdt_sys::capi_checksum256;
    let hash = feature_digest.as_ref().to_bytes();
    let checksum = capi_checksum256 { hash };
    let ptr = &checksum as *const capi_checksum256;
    unsafe { eosio_cdt_sys::is_feature_activated(ptr) }
}

#[inline]
pub fn preactivate_feature<T: AsRef<Checksum256>>(feature_digest: T) {
    use eosio_cdt_sys::capi_checksum256;
    let hash = feature_digest.as_ref().to_bytes();
    let checksum = capi_checksum256 { hash };
    let ptr = &checksum as *const capi_checksum256;
    unsafe { eosio_cdt_sys::preactivate_feature(ptr) }
}
