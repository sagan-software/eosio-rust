#![recursion_limit = "128"]
#![feature(
    proc_macro_non_items,
    proc_macro_diagnostic,
    proc_macro_quote
)]

extern crate eosio_sys;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

mod c;
mod derive_print;
mod derive_read;
mod derive_table_row;
mod derive_write;
mod eosio_abi;
mod eosio_action;
mod eosio_assert;
mod eosio_name;
mod eosio_print;
mod n;
mod s;

use proc_macro::TokenStream;

#[proc_macro]
pub fn c(input: TokenStream) -> TokenStream {
    ::c::expand(input)
}

#[proc_macro]
pub fn eosio_abi(input: TokenStream) -> TokenStream {
    ::eosio_abi::expand(input)
}

#[proc_macro_attribute]
pub fn eosio_action(args: TokenStream, input: TokenStream) -> TokenStream {
    ::eosio_action::expand(args, input)
}

#[proc_macro]
pub fn eosio_assert(input: TokenStream) -> TokenStream {
    ::eosio_assert::expand(input)
}

#[proc_macro]
pub fn eosio_name(input: TokenStream) -> TokenStream {
    ::eosio_name::expand(input)
}

#[proc_macro]
pub fn eosio_print(input: TokenStream) -> TokenStream {
    ::eosio_print::expand(input)
}

#[proc_macro]
pub fn n(input: TokenStream) -> TokenStream {
    ::n::expand(input)
}

#[proc_macro]
pub fn s(input: TokenStream) -> TokenStream {
    ::s::expand(input)
}

#[proc_macro_derive(Write)]
pub fn derive_write(input: TokenStream) -> TokenStream {
    ::derive_write::expand(input)
}

#[proc_macro_derive(Read)]
pub fn derive_read(input: TokenStream) -> TokenStream {
    ::derive_read::expand(input)
}

#[proc_macro_derive(TableRow, attributes(primary))]
pub fn derive_table_row(input: TokenStream) -> TokenStream {
    ::derive_table_row::expand(input)
}
