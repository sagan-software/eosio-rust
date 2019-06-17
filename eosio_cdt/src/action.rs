use eosio_bytes::{NumBytes, Read, ReadError, Write, WriteError};
use eosio_core::{
    AccountName, Action, DeferredTransactionId, ToAction, Transaction,
};

/// This method will abort execution of wasm without failing the contract. This is used to bypass all cleanup / destructors that would normally be called.
#[inline]
pub fn eosio_exit<C>(code: C)
where
    C: Into<i32>,
{
    unsafe { ::eosio_cdt_sys::eosio_exit(code.into()) }
}

/// TODO docs.
#[inline]
pub fn send_inline_action(action: &Action) -> Result<(), WriteError> {
    let size = action.num_bytes();
    let mut bytes = vec![0_u8; size];
    let mut pos = 0;
    action.write(&mut bytes, &mut pos)?;
    let ptr = bytes[..].as_mut_ptr();
    unsafe { ::eosio_cdt_sys::send_inline(ptr, pos) }
    Ok(())
}

/// TODO docs.
#[inline]
pub fn send_context_free_inline_action(
    action: &Action,
) -> Result<(), WriteError> {
    let size = action.num_bytes();
    let mut bytes = vec![0_u8; size];
    let mut pos = 0;
    action.write(&mut bytes, &mut pos)?;
    let ptr = bytes[..].as_mut_ptr();
    unsafe { ::eosio_cdt_sys::send_context_free_inline(ptr, pos) }
    Ok(())
}

/// TODO docs
#[inline]
pub fn send_deferred<P>(
    id: &DeferredTransactionId,
    payer: P,
    trx: &Transaction,
    replace_existing: bool,
) -> Result<(), WriteError>
where
    P: Into<AccountName>,
{
    let sender_id = id.as_u128();
    let sender_id_ptr = &sender_id as *const _ as *const u128;
    let size = trx.num_bytes();
    let serialized_transaction = {
        let mut bytes = vec![0_u8; size];
        let mut pos = 0;
        trx.write(&mut bytes, &mut pos)?;
        bytes
    };
    let serialized_transaction_ptr = serialized_transaction[..].as_ptr();
    unsafe {
        ::eosio_cdt_sys::send_deferred(
            sender_id_ptr,
            payer.into().into(),
            serialized_transaction_ptr,
            size,
            replace_existing.into(),
        )
    }
    Ok(())
}

/// TODO docs
#[inline]
pub fn cancel_deferred(id: &DeferredTransactionId) -> bool {
    let sender_id = id.as_u128();
    let sender_id_ptr = &sender_id as *const _ as *const u128;
    let result = unsafe { ::eosio_cdt_sys::cancel_deferred(sender_id_ptr) };
    result == 1
}

/// TODO docs.
pub trait ActionFn: ToAction + Read + Write + NumBytes + Clone {
    /// TODO docs.
    fn execute(self);

    /// TODO docs.
    #[inline]
    fn read_data() -> Result<Self, ReadError> {
        let num_bytes = unsafe { ::eosio_cdt_sys::action_data_size() };
        let mut bytes = vec![0_u8; num_bytes as usize];
        let ptr: *mut ::eosio_cdt_sys::c_void =
            &mut bytes[..] as *mut _ as *mut ::eosio_cdt_sys::c_void;
        unsafe {
            ::eosio_cdt_sys::read_action_data(ptr, num_bytes);
        }
        let mut pos = 0;
        Self::read(&bytes, &mut pos)
    }
}
