use eosio::{PermissionLevel, PublicKey, Transaction, Write, WriteError};

/// Checks if a transaction is authorized by a provided set of keys and
/// permissions
///
/// # Errors
///
/// Returns `Err` if there as a problem serializing the public keys or
/// permission levels.
#[inline]
pub fn has_transaction_authority<T, K, L>(
    trx: T,
    public_keys: K,
    permission_levels: L,
) -> Result<bool, WriteError>
where
    T: AsRef<Transaction>,
    K: AsRef<[PublicKey]>,
    L: AsRef<[PermissionLevel]>,
{
    let trx = trx.as_ref().pack()?;
    has_transaction_authority_bytes(trx, public_keys, permission_levels)
}

/// Checks if a transaction is authorized by a provided set of keys and
/// permissions
///
/// # Errors
///
/// Returns `Err` if there as a problem serializing the public keys or
/// permission levels.
#[inline]
#[allow(clippy::cast_possible_truncation)]
pub fn has_transaction_authority_bytes<T, K, L>(
    trx: T,
    public_keys: K,
    permission_levels: L,
) -> Result<bool, WriteError>
where
    T: AsRef<[u8]>,
    K: AsRef<[PublicKey]>,
    L: AsRef<[PermissionLevel]>,
{
    let trx = trx.as_ref();
    let public_keys = public_keys.as_ref().pack()?;
    let permission_levels = permission_levels.as_ref().pack()?;
    let result = unsafe {
        eosio_cdt_sys::check_transaction_authorization(
            trx.as_ptr(),
            trx.len() as u32,
            public_keys.as_ptr(),
            public_keys.len() as u32,
            permission_levels.as_ptr(),
            permission_levels.len() as u32,
        )
    };
    Ok(result == 1)
}
