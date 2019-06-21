use eosio_bytes::{NumBytes, Read, ReadError, Write, WriteError};
use eosio_core::{
    AccountName, BlockchainParameters, CpuWeight, NetWeight, ProducerKey,
    RamBytes,
};

/// Check if an account is privileged
#[inline]
pub fn is_privileged<A: Into<AccountName>>(account: A) -> bool {
    let a = account.into();
    unsafe { ::eosio_cdt_sys::is_privileged(a.into()) }
}

/// Get the resource limits of an account
#[inline]
pub fn get_resource_limits<A: Into<AccountName>>(
    account: A,
) -> (RamBytes, NetWeight, CpuWeight) {
    let mut ram_bytes = 0_i64;
    let ram_bytes_ptr = &mut ram_bytes as *mut _ as *mut i64;
    let mut net_weight = 0_i64;
    let net_weight_ptr = &mut net_weight as *mut _ as *mut i64;
    let mut cpu_weight = 0_i64;
    let cpu_weight_ptr = &mut cpu_weight as *mut _ as *mut i64;
    let a = account.into();
    unsafe {
        ::eosio_cdt_sys::get_resource_limits(
            a.into(),
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
pub fn set_resource_limits(
    account: AccountName,
    ram_bytes: i64,
    net_weight: i64,
    cpu_weight: i64,
) {
    unsafe {
        ::eosio_cdt_sys::set_resource_limits(
            account.into(),
            ram_bytes,
            net_weight,
            cpu_weight,
        )
    }
}

/// Set the privileged status of an account
#[inline]
pub fn set_privileged(account: AccountName, is_priv: bool) {
    unsafe { ::eosio_cdt_sys::set_privileged(account.into(), is_priv) }
}

/// Set the blockchain parameters
#[inline]
pub fn set_blockchain_parameters(
    params: &BlockchainParameters,
) -> Result<(), WriteError> {
    let size = params.num_bytes();
    let mut buf = vec![0_u8; size];
    params.write(&mut buf, &mut 0)?;
    let buf_ptr = &mut buf as *mut _ as *mut u8;
    unsafe {
        ::eosio_cdt_sys::set_blockchain_parameters_packed(buf_ptr, size as u32)
    }
    Ok(())
}

/// Retrieve the blolckchain parameters
#[inline]
pub fn get_blockchain_parameters() -> Result<BlockchainParameters, ReadError> {
    let expected_size = std::mem::size_of::<BlockchainParameters>();
    let mut buf = vec![0_u8; expected_size];
    let buf_ptr = &mut buf as *mut _ as *mut u8;
    let actual_size = unsafe {
        ::eosio_cdt_sys::get_blockchain_parameters_packed(
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
#[inline]
pub fn set_proposed_producers(prods: &[ProducerKey]) -> Option<u64> {
    let size = prods.num_bytes();
    let mut buf = vec![0_u8; size];
    let buf_ptr = &mut buf as *mut _ as *mut u8;
    let result = unsafe {
        ::eosio_cdt_sys::set_proposed_producers(buf_ptr, size as u32)
    };
    if result >= 0 {
        Some(result as u64)
    } else {
        None
    }
}
