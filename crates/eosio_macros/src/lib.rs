#![recursion_limit = "256"]
#![feature(proc_macro_hygiene, proc_macro_diagnostic, proc_macro_quote)]

extern crate eosio_sys;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

mod derive_num_bytes;
mod derive_print;
mod derive_read;
mod derive_table_row;
mod derive_write;
mod eosio_abi;
mod eosio_action;
mod eosio_name;
mod eosio_print;
mod eosio_table;
mod n;
mod s;

mod paths {
    #[cfg(feature = "internal")]
    pub fn eosio() -> ::proc_macro2::TokenStream {
        quote!(crate)
    }

    #[cfg(not(feature = "internal"))]
    pub fn eosio() -> ::proc_macro2::TokenStream {
        quote!(::eosio)
    }
}

use crate::proc_macro::TokenStream;

#[proc_macro]
pub fn eosio_abi(input: TokenStream) -> TokenStream {
    crate::eosio_abi::expand(input)
}

#[proc_macro_attribute]
pub fn eosio_action(args: TokenStream, input: TokenStream) -> TokenStream {
    crate::eosio_action::expand(args, input)
}

#[proc_macro]
pub fn eosio_name(input: TokenStream) -> TokenStream {
    crate::eosio_name::expand(input)
}

#[proc_macro]
pub fn eosio_print(input: TokenStream) -> TokenStream {
    crate::eosio_print::expand(input)
}

#[proc_macro_attribute]
pub fn eosio_table(args: TokenStream, input: TokenStream) -> TokenStream {
    crate::eosio_table::expand(args, input)
}

#[proc_macro]
pub fn n(input: TokenStream) -> TokenStream {
    crate::n::expand(input)
}

#[proc_macro]
pub fn s(input: TokenStream) -> TokenStream {
    crate::s::expand(input)
}

#[proc_macro_derive(Write)]
pub fn derive_write(input: TokenStream) -> TokenStream {
    crate::derive_write::expand(input)
}

#[proc_macro_derive(Read)]
pub fn derive_read(input: TokenStream) -> TokenStream {
    crate::derive_read::expand(input)
}

#[proc_macro_derive(TableRow, attributes(table_name, primary, secondary))]
pub fn derive_table_row(input: TokenStream) -> TokenStream {
    crate::derive_table_row::expand(input)
}

#[proc_macro_derive(NumBytes)]
pub fn derive_num_bytes(input: TokenStream) -> TokenStream {
    crate::derive_num_bytes::expand(input)
}
