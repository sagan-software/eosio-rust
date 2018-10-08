/// Aborts processing of this action and unwinds all pending changes if the test condition is true
pub fn eosio_assert(test: bool, msg: &str) {
    let test = if test { 1 } else { 0 };
    let msg_ptr = msg.as_ptr();
    unsafe { ::eosio_sys::eosio_assert(test, msg_ptr) }
}

/// Aborts processing of this action and unwinds all pending changes if the test condition is true
pub fn eosio_assert_code<C>(test: bool, code: C)
where
    C: Into<u64>,
{
    let test = if test { 1 } else { 0 };
    let code: u64 = code.into();
    unsafe { ::eosio_sys::eosio_assert_code(test, code) }
}

pub trait Assert<T> {
    fn assert(self, msg: &str) -> T;
}

impl<T, E> Assert<T> for Result<T, E> {
    fn assert(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(_) => {
                let msg_ptr = msg.as_ptr();
                let len = msg.len() as u32;
                unsafe { ::eosio_sys::eosio_assert_message(0, msg_ptr, len) }
                panic!("unreachable");
            }
        }
    }
}

impl<T> Assert<T> for Option<T> {
    fn assert(self, msg: &str) -> T {
        match self {
            Some(t) => t,
            None => {
                let msg_ptr = msg.as_ptr();
                let len = msg.len() as u32;
                unsafe { ::eosio_sys::eosio_assert_message(0, msg_ptr, len) }
                panic!("unreachable");
            }
        }
    }
}

// impl Assert<bool> for bool {
//     fn assert(self, msg: &str) -> bool {
//         if self {
//             true
//         } else {
//             let msg_ptr = msg.as_ptr();
//             let len = msg.len() as u32;
//             unsafe { ::eosio_sys::eosio_assert_message(0, msg_ptr, len) }
//             panic!("unreachable");
//         }
//     }
// }
