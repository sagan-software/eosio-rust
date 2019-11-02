/// Aborts processing of this action and unwinds all pending changes if the test condition is true
#[inline]
pub fn check(pred: bool, msg: &str) {
    if !pred {
        let msg_ptr = msg.as_ptr();
        #[allow(clippy::cast_possible_truncation)]
        let msg_len = msg.len() as u32;
        unsafe { eosio_cdt_sys::eosio_assert_message(0, msg_ptr, msg_len) }
    }
}

/// Aborts processing of this action and unwinds all pending changes if the test condition is true
#[inline]
pub fn check_code<C>(pred: bool, code: C)
where
    C: Into<u64>,
{
    if !pred {
        unsafe { eosio_cdt_sys::eosio_assert_code(0, code.into()) }
    }
}
