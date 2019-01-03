#![recursion_limit = "512"]
// #![feature(proc_macro_hygiene, proc_macro_diagnostic, proc_macro_quote)]
// #![feature(proc_macro_diagnostic)]

extern crate proc_macro;

mod abi;
mod action;
mod derive_num_bytes;
mod derive_print;
mod derive_read;
mod derive_table_row;
mod derive_write;
mod n;
mod name;
mod print;
mod s;
mod table;

mod paths {
    #[cfg(feature = "internal")]
    pub fn eosio() -> ::proc_macro2::TokenStream {
        ::quote::quote!(crate)
    }

    #[cfg(not(feature = "internal"))]
    pub fn eosio() -> ::proc_macro2::TokenStream {
        ::quote::quote!(::eosio)
    }
}

use crate::proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;

#[proc_macro]
pub fn abi(input: TokenStream) -> TokenStream {
    crate::abi::expand(input)
}

#[proc_macro_attribute]
pub fn action(args: TokenStream, input: TokenStream) -> TokenStream {
    crate::action::expand(args, input)
}

#[proc_macro]
pub fn name(input: TokenStream) -> TokenStream {
    crate::name::expand(input)
}

#[proc_macro_hack]
pub fn print(input: TokenStream) -> TokenStream {
    crate::print::expand(input)
}

#[proc_macro_attribute]
pub fn table(args: TokenStream, input: TokenStream) -> TokenStream {
    crate::table::expand(args, input)
}

#[proc_macro_hack]
pub fn n(input: TokenStream) -> TokenStream {
    crate::n::expand(input)
}

#[proc_macro_hack]
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
