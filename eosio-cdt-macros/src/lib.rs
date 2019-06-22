use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack(support_nested)]
pub use eosio_cdt_macros_internal::print;

pub use eosio_cdt_macros_internal::{abi, action, table, TableRow};
