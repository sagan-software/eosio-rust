//! Internal use only.
//!
//! This crate is an implementation detail that will hopefully go away
//! once the [`proc_macro_hygiene`] feature is stabilized. In the meantime
//! we must use this crate (and [`proc_macro_hack`]) to allow for
//! function-like procedural macros in expression positions.
//!
//! [`proc_macro_hygiene`]: https://doc.rust-lang.org/beta/unstable-book/language-features/proc-macro-hygiene.html
//! [`proc_macro_hack`]: https://github.com/dtolnay/proc-macro-hack
#![allow(
    clippy::unimplemented,
    clippy::missing_inline_in_public_items,
    clippy::missing_docs_in_private_items
)]
extern crate proc_macro;

mod abi;
mod action;
mod derive_num_bytes;
mod derive_read;
mod derive_table;
mod derive_write;
mod internal;
mod n;
mod s;
mod table;

use crate::proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn};

#[proc_macro]
pub fn abi(input: TokenStream) -> TokenStream {
    crate::abi::expand(input)
}

#[proc_macro_hack]
pub fn n(input: TokenStream) -> TokenStream {
    use crate::n::EosioName;
    let item = parse_macro_input!(input as EosioName);
    quote!(#item).into()
}

#[proc_macro_hack]
pub fn s(input: TokenStream) -> TokenStream {
    use crate::s::EosioSymbol;
    let item = parse_macro_input!(input as EosioSymbol);
    quote!(#item).into()
}

#[proc_macro_attribute]
pub fn action(args: TokenStream, input: TokenStream) -> TokenStream {
    use crate::action::{ActionArgs, ActionFn};
    let args = parse_macro_input!(args as ActionArgs);
    let item = parse_macro_input!(input as ItemFn);
    let action = ActionFn::new(args, item);
    quote!(#action).into()
}

#[proc_macro_attribute]
pub fn table(args: TokenStream, input: TokenStream) -> TokenStream {
    use crate::table::{Table, TableArgs};
    let args = parse_macro_input!(args as TableArgs);
    let input = parse_macro_input!(input as DeriveInput);
    let table = Table::new(args, input);
    quote!(#table).into()
}

/// Derive the `Write` trait
#[inline]
#[proc_macro_derive(Write, attributes(eosio))]
pub fn derive_write(input: TokenStream) -> TokenStream {
    use crate::derive_write::DeriveWrite;
    let item = parse_macro_input!(input as DeriveWrite);
    quote!(#item).into()
}

/// Derive the `Read` trait
#[inline]
#[proc_macro_derive(Read, attributes(eosio))]
pub fn derive_read(input: TokenStream) -> TokenStream {
    use crate::derive_read::DeriveRead;
    let item = parse_macro_input!(input as DeriveRead);
    quote!(#item).into()
}

/// Derive the `NumBytes` trait
#[inline]
#[proc_macro_derive(NumBytes, attributes(eosio))]
pub fn derive_num_bytes(input: TokenStream) -> TokenStream {
    use crate::derive_num_bytes::DeriveNumBytes;
    let item = parse_macro_input!(input as DeriveNumBytes);
    quote!(#item).into()
}

/// TODO docs
#[inline]
#[proc_macro_derive(Table, attributes(eosio))]
pub fn derive_table(input: TokenStream) -> TokenStream {
    use crate::derive_table::DeriveTable;
    let item = parse_macro_input!(input as DeriveTable);
    quote!(#item).into()
}
