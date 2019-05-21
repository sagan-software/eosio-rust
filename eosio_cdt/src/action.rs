use eosio_bytes::{NumBytes, Read, ReadError, Write, WriteError};
use eosio_core::{Action, ToAction};

/// This method will abort execution of wasm without failing the contract. This is used to bypass all cleanup / destructors that would normally be called.
#[inline]
pub fn eosio_exit<C>(code: C)
where
    C: Into<i32>,
{
    unsafe { ::eosio_cdt_sys::eosio_exit(code.into()) }
}

#[derive(Clone, Debug)]
pub struct DeferredId(u128);

#[inline]
pub fn send_inline_action<Data>(action: Action<Data>) -> Result<(), WriteError>
where
    Data: Write + NumBytes,
{
    let size = action.num_bytes().saturating_add(1); // 1 extra byte is needed
    let mut bytes = vec![0_u8; size];
    let mut pos = 0;
    action.write(&mut bytes, &mut pos)?;
    let ptr = bytes[..].as_mut_ptr();
    if action.authorization.is_empty() {
        unsafe { ::eosio_cdt_sys::send_context_free_inline(ptr, pos) }
    } else {
        unsafe { ::eosio_cdt_sys::send_inline(ptr, pos) }
    }
    Ok(())
}

// #[inline]
// pub fn send_deferred<P>(
//     &self,
//     _id: DeferredId,
//     _payer: P,
//     _replace_existing: bool,
// ) -> Result<(), WriteError>
// where
//     P: Into<u64>,
// {
//     // TODO
//     Ok(())
// }

// #[inline]
// pub fn cancel_deferred(_id: DeferredId) -> Result<(), ()> {
//     // TODO
//     Ok(())
// }

pub trait ActionFn: ToAction + Read + Write + NumBytes + Clone {
    fn execute(self);

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
