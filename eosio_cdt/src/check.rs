/// Aborts processing of this action and unwinds all pending changes if the test condition is true
#[inline]
pub fn check(pred: bool, msg: &str) {
    if !pred {
        let msg_ptr = msg.as_ptr();
        let msg_len = msg.len() as u32;
        unsafe { ::eosio_cdt_sys::eosio_assert_message(0, msg_ptr, msg_len) }
    }
}

/// Aborts processing of this action and unwinds all pending changes if the test condition is true
#[inline]
pub fn check_code<C>(pred: bool, code: C)
where
    C: Into<u64>,
{
    if !pred {
        unsafe { ::eosio_cdt_sys::eosio_assert_code(0, code.into()) }
    }
}

pub trait Check<T> {
    fn check(self, msg: &str) -> T;
}

impl<T, E> Check<T> for Result<T, E> {
    #[inline]
    fn check(self, msg: &str) -> T {
        if let Ok(t) = self {
            t
        } else {
            check(false, msg);
            unreachable!();
        }
    }
}

impl<T> Check<T> for Option<T> {
    #[inline]
    fn check(self, msg: &str) -> T {
        if let Some(t) = self {
            t
        } else {
            check(false, msg);
            unreachable!();
        }
    }
}

impl Check<bool> for bool {
    #[inline]
    fn check(self, msg: &str) -> Self {
        if self {
            true
        } else {
            check(false, msg);
            unreachable!();
        }
    }
}
