/// Aborts processing of this action and unwinds all pending changes if the test
/// condition is true
#[inline]
pub fn check(pred: bool, msg: &str) {
    if !pred {
        let msg_ptr = msg.as_ptr();
        #[allow(clippy::cast_possible_truncation)]
        let msg_len = msg.len() as u32;
        unsafe { eosio_cdt_sys::eosio_assert_message(0, msg_ptr, msg_len) }
    }
}

/// Aborts processing of this action and unwinds all pending changes if the test
/// condition is true
#[inline]
pub fn check_code<C>(pred: bool, code: C)
where
    C: Into<u64>,
{
    if !pred {
        unsafe { eosio_cdt_sys::eosio_assert_code(0, code.into()) }
    }
}

pub trait Check {
    type Output;
    fn check(self, msg: &str) -> Self::Output;
}

impl<T, E> Check for Result<T, E> {
    type Output = T;

    #[inline]
    fn check(self, msg: &str) -> Self::Output {
        if let Ok(t) = self {
            t
        } else {
            check(false, msg);
            unreachable!();
        }
    }
}

impl<T> Check for Option<T> {
    type Output = T;

    #[inline]
    fn check(self, msg: &str) -> Self::Output {
        if let Some(t) = self {
            t
        } else {
            check(false, msg);
            unreachable!();
        }
    }
}

impl Check for bool {
    type Output = Self;

    #[inline]
    fn check(self, msg: &str) -> Self::Output {
        check(self, msg);
        self
    }
}
