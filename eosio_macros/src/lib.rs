use proc_macro_hack::proc_macro_hack;

pub use eosio_macros_impl::abi;

#[proc_macro_hack(support_nested)]
pub use eosio_macros_impl::print;

#[proc_macro_hack]
pub use eosio_macros_impl::n;

#[proc_macro_hack]
pub use eosio_macros_impl::s;

pub use eosio_macros_impl::{action, name, table, NumBytes, Read, TableRow, Write};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n() {
        assert_eq!(n!(test), 14_605_613_396_213_628_928u64);
        assert_eq!(n!(1234), 614_248_767_926_829_056u64);
        assert_eq!(n!(123451234512), 614_251_535_012_020_768u64);
        assert_eq!(n!(eosio.token), 6_138_663_591_592_764_928u64);
    }

    #[test]
    fn test_s() {
        assert_eq!(s!(0, TGFT), 361_956_332_544);
        assert_eq!(s!(4, EOS), 1_397_703_940);
        assert_eq!(s!(0, EOS), 1_397_703_936);
        assert_eq!(s!(1, EDNA), 280_485_971_201);
    }

}
