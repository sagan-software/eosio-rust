use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack(support_nested)]
pub use eosio_cdt_macros_impl::print;

pub use eosio_cdt_macros_impl::{abi, action, table, TableRow};
