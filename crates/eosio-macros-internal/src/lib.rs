//! Internal use only.
//!
//! This crate is an implementation detail that will hopefully go away
//! once the [`proc_macro_hygiene`] feature is stabilized. In the meantime
//! we must use this crate (and [`proc_macro_hack`]) to allow for
//! function-like procedural macros in expression positions.
//!
//! [`proc_macro_hygiene`]: https://doc.rust-lang.org/beta/unstable-book/language-features/proc-macro-hygiene.html
//! [`proc_macro_hack`]: https://github.com/dtolnay/proc-macro-hack
//!
#![allow(
    clippy::unimplemented,
    clippy::missing_inline_in_public_items,
    clippy::missing_docs_in_private_items
)]
extern crate proc_macro;

mod action;
mod derive_num_bytes;
mod derive_read;
mod derive_table;
mod derive_write;
mod n;
mod s;
mod table;

use crate::proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_hack::proc_macro_hack;
use syn::{DeriveInput, Lit, LitStr, Meta, Path};

#[proc_macro_hack]
pub fn n(input: TokenStream) -> TokenStream {
    crate::n::expand(input)
}

#[proc_macro_hack]
pub fn s(input: TokenStream) -> TokenStream {
    crate::s::expand(input)
}

#[proc_macro_attribute]
pub fn action(args: TokenStream, input: TokenStream) -> TokenStream {
    crate::action::expand(args, input)
}

#[proc_macro_attribute]
pub fn table(args: TokenStream, input: TokenStream) -> TokenStream {
    crate::table::expand(args, input)
}

/// Derive the `Write` trait
#[inline]
#[proc_macro_derive(Write, attributes(__eosio_path))]
pub fn derive_write(input: TokenStream) -> TokenStream {
    crate::derive_write::expand(input)
}

/// Derive the `Read` trait
#[inline]
#[proc_macro_derive(Read, attributes(__eosio_path))]
pub fn derive_read(input: TokenStream) -> TokenStream {
    crate::derive_read::expand(input)
}

/// Derive the `NumBytes` trait
#[inline]
#[proc_macro_derive(NumBytes, attributes(__eosio_path))]
pub fn derive_num_bytes(input: TokenStream) -> TokenStream {
    crate::derive_num_bytes::expand(input)
}

/// TODO docs
#[inline]
#[proc_macro_derive(
    Table,
    attributes(table_name, primary, secondary, singleton)
)]
pub fn derive_table(input: TokenStream) -> TokenStream {
    crate::derive_table::expand(input)
}

/// The default root path using the `eosio` crate.
const DEFAULT_ROOT_PATH: &str = "::eosio";

/// Get the root path for types/traits.
pub(crate) fn root_path(input: &DeriveInput) -> Path {
    let litstr = input
        .attrs
        .iter()
        .fold(None, |acc, attr| match attr.parse_meta() {
            Ok(meta) => {
                let name = meta.name();
                if name == "__eosio_path" {
                    match meta {
                        Meta::NameValue(meta) => match meta.lit {
                            Lit::Str(s) => Some(s),
                            _ => panic!("eosio_path must be a lit str"),
                        },
                        _ => acc,
                    }
                } else {
                    acc
                }
            }
            Err(_) => acc,
        })
        .unwrap_or_else(|| LitStr::new(DEFAULT_ROOT_PATH, Span::call_site()));
    litstr
        .parse_with(Path::parse_mod_style)
        .expect("bad path for __eosio_path")
}
