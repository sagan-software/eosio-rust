use alloc::vec::Vec;
use eosio::{
    AccountName, Action, DataStream, NumBytes, Read, ReadError, Transaction,
    TransactionId, Write, WriteError,
};

/// This method will abort execution of wasm without failing the contract. This
/// is used to bypass all cleanup / destructors that would normally be called.
#[inline]
pub fn eosio_exit<C>(code: C)
where
    C: Into<i32>,
{
    unsafe { eosio_cdt_sys::eosio_exit(code.into()) }
}

/// Sends an inline action.
///
/// # Errors
///
/// Will return `Err` if there was an issue serializing the action.
#[inline]
pub fn send_inline_action(action: &Action<Vec<u8>>) -> Result<(), WriteError> {
    let size = action.num_bytes();
    let mut bytes = vec![0_u8; size];
    let mut pos = 0;
    action.write(&mut bytes, &mut pos)?;
    let ptr = bytes[..].as_mut_ptr();
    unsafe { eosio_cdt_sys::send_inline(ptr, pos) }
    Ok(())
}

/// Sends a context free inline action.
///
/// # Errors
///
/// Will return `Err` if there was an issue serializing the action.
#[inline]
pub fn send_context_free_inline_action(
    action: &Action<Vec<u8>>,
) -> Result<(), WriteError> {
    let size = action.num_bytes();
    let mut bytes = vec![0_u8; size];
    let mut pos = 0;
    action.write(&mut bytes, &mut pos)?;
    let ptr = bytes[..].as_mut_ptr();
    unsafe { eosio_cdt_sys::send_context_free_inline(ptr, pos) }
    Ok(())
}

/// Sends a deferred transaction
///
/// # Errors
///
/// Will return `Err` if there was an issue serializing the transaction.
#[inline]
pub fn send_deferred<I, P, T>(
    id: I,
    payer: P,
    trx: T,
    replace_existing: bool,
) -> Result<(), WriteError>
where
    I: AsRef<TransactionId>,
    P: AsRef<AccountName>,
    T: AsRef<Transaction>,
{
    let bytes = trx.as_ref().pack()?;
    send_deferred_bytes(id, payer, bytes, replace_existing);
    Ok(())
}

/// Sends a deferred transaction from raw bytes.
#[inline]
pub fn send_deferred_bytes<I, P, T>(
    id: I,
    payer: P,
    bytes: T,
    replace_existing: bool,
) where
    I: AsRef<TransactionId>,
    P: AsRef<AccountName>,
    T: AsRef<[u8]>,
{
    let id = id.as_ref().as_u128();
    let id_ptr = &id as *const _ as *const u128;
    let bytes = bytes.as_ref();
    unsafe {
        eosio_cdt_sys::send_deferred(
            id_ptr,
            payer.as_ref().as_u64(),
            bytes.as_ptr(),
            bytes.len(),
            replace_existing.into(),
        )
    }
}

/// Cancels a deferred transaction
#[must_use]
#[inline]
pub fn cancel_deferred<I: AsRef<TransactionId>>(id: I) -> bool {
    let sender_id = id.as_ref().as_u128();
    let sender_id_ptr = &sender_id as *const _ as *const u128;
    let result = unsafe { eosio_cdt_sys::cancel_deferred(sender_id_ptr) };
    result == 1
}

/// Reads action data
///
/// # Errors
///
/// Will return `Err` if there was a problem reading the action data.
#[inline]
pub fn read_action_data<T: Read>() -> Result<T, ReadError> {
    let num_bytes = unsafe { eosio_cdt_sys::action_data_size() };
    let mut bytes = vec![0_u8; num_bytes as usize];
    let ptr: *mut eosio_cdt_sys::c_void =
        &mut bytes[..] as *mut _ as *mut eosio_cdt_sys::c_void;
    unsafe {
        eosio_cdt_sys::read_action_data(ptr, num_bytes);
    }
    let mut pos = 0;
    T::read(&bytes, &mut pos)
}

/// Get a `DataStream` of the current action data
#[must_use]
#[inline]
pub fn current_data_stream() -> DataStream {
    let num_bytes = unsafe { eosio_cdt_sys::action_data_size() };
    let mut bytes = vec![0_u8; num_bytes as usize];
    let ptr: *mut eosio_cdt_sys::c_void =
        &mut bytes[..] as *mut _ as *mut eosio_cdt_sys::c_void;
    unsafe {
        eosio_cdt_sys::read_action_data(ptr, num_bytes);
    }
    bytes.into()
}
