use proc_macro_hack::proc_macro_hack;

// #[proc_macro_hack(support_nested)]
pub use eosio_macros_impl::eosio_abi;

#[proc_macro_hack(support_nested)]
pub use eosio_macros_impl::eosio_print;

#[proc_macro_hack]
pub use eosio_macros_impl::n;

#[proc_macro_hack]
pub use eosio_macros_impl::s;

pub use eosio_macros_impl::{
    eosio_action, eosio_name, eosio_table, NumBytes, Read, TableRow, Write,
};
