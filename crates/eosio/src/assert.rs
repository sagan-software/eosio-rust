/// Aborts processing of this action and unwinds all pending changes if the test condition is true
#[inline]
pub fn eosio_assert(test: bool, msg: &str) {
    #[cfg(feature = "contract")]
    {
        if !test {
            let msg_ptr = msg.as_ptr();
            let msg_len = msg.len() as u32;
            unsafe { ::eosio_sys::eosio_assert_message(0, msg_ptr, msg_len) }
        }
    }
}

/// Aborts processing of this action and unwinds all pending changes if the test condition is true
#[inline]
pub fn eosio_assert_code<C>(test: bool, code: C)
where
    C: Into<u64>,
{
    #[cfg(feature = "contract")]
    {
        if !test {
            unsafe { ::eosio_sys::eosio_assert_code(0, code.into()) }
        }
    }
}

pub trait Assert<T> {
    fn assert(self, msg: &str) -> T;
}

impl<T, E> Assert<T> for Result<T, E> {
    #[inline]
    fn assert(self, msg: &str) -> T {
        if let Ok(t) = self {
            t
        } else {
            eosio_assert(false, msg);
            unreachable!();
        }
    }
}

impl<T> Assert<T> for Option<T> {
    #[inline]
    fn assert(self, msg: &str) -> T {
        if let Some(t) = self {
            t
        } else {
            eosio_assert(false, msg);
            unreachable!();
        }
    }
}

impl Assert<bool> for bool {
    #[inline]
    fn assert(self, msg: &str) -> Self {
        if self {
            true
        } else {
            eosio_assert(false, msg);
            unreachable!();
        }
    }
}
