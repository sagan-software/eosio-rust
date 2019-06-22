use eosio_core::{
    PermissionLevel, PublicKey, Read, ReadError, Transaction, Write, WriteError,
};

/// Checks if a transaction is authorized by a provided set of keys and permissions
#[inline]
pub fn check_transaction_authority(
    trx: &Transaction,
    public_keys: &[PublicKey],
    permission_levels: &[PermissionLevel],
) -> bool {
    // let mut trx_bytes =
    // let trx_ptr = trx as &
    // TODO
    false
}
