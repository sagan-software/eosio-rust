#![recursion_limit = "512"]
// #![feature(proc_macro_hygiene, proc_macro_diagnostic, proc_macro_quote)]
// #![feature(proc_macro_diagnostic)]

extern crate proc_macro;

mod abi;
mod action;
mod derive_table_row;
mod print;
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

#[proc_macro_hack]
pub fn print(input: TokenStream) -> TokenStream {
    crate::print::expand(input)
}

#[proc_macro_attribute]
pub fn table(args: TokenStream, input: TokenStream) -> TokenStream {
    crate::table::expand(args, input)
}

#[proc_macro_derive(
    TableRow,
    attributes(table_name, primary, secondary, singleton)
)]
pub fn derive_table_row(input: TokenStream) -> TokenStream {
    crate::derive_table_row::expand(input)
}
