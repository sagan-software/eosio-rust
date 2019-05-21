extern crate proc_macro;

mod derive_num_bytes;
mod derive_read;
mod derive_write;

use crate::proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{DeriveInput, Lit, LitStr, Meta, Path};

#[cfg(feature = "internal-use-only-root-path-is-eosio")]
const DEFAULT_ROOT_PATH: &str = "::eosio";

#[cfg(not(feature = "internal-use-only-root-path-is-eosio"))]
const DEFAULT_ROOT_PATH: &str = "::eosio_bytes";

pub(crate) fn root_path(input: &DeriveInput) -> Path {
    let litstr = input
        .attrs
        .iter()
        .fold(None, |acc, attr| match attr.parse_meta() {
            Ok(meta) => {
                let name = meta.name();
                if name == "eosio_bytes_root_path" {
                    match meta {
                        Meta::NameValue(meta) => match meta.lit {
                            Lit::Str(s) => Some(s),
                            _ => panic!("eosio_bytes_path must be a lit str"),
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
        .expect("bad path for eosio_bytes_root_path")
}

#[proc_macro_derive(Write, attributes(eosio_bytes_root_path))]
pub fn derive_write(input: TokenStream) -> TokenStream {
    crate::derive_write::expand(input)
}

#[proc_macro_derive(Read, attributes(eosio_bytes_root_path))]
pub fn derive_read(input: TokenStream) -> TokenStream {
    crate::derive_read::expand(input)
}

#[proc_macro_derive(NumBytes, attributes(eosio_bytes_root_path))]
pub fn derive_num_bytes(input: TokenStream) -> TokenStream {
    crate::derive_num_bytes::expand(input)
}

// #[proc_macro_attribute]
// pub fn eosio_bytes_root_path(
//     _args: TokenStream,
//     input: TokenStream,
// ) -> TokenStream {
//     input
// }
