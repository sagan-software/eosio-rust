//! This crate provides three derive macros for [`eosio_core`] traits.
//!
//! # Examples
//!
//! ```
//! use eosio_core::{Read, Write, NumBytes};
//!
//! #[derive(Read, Write, NumBytes, PartialEq, Debug)]
//! #[eosio_core_root_path = "::eosio_core"]
//! struct Thing(u8);
//!
//! let thing = Thing(30);
//!
//! // Number of bytes
//! assert_eq!(thing.num_bytes(), 1);
//!
//! // Read bytes
//! assert_eq!(thing, Thing::read(&mut [30_u8], &mut 0).unwrap());
//!
//! // Write bytes
//! let mut bytes = vec![0_u8; 1];
//! thing.write(&mut bytes, &mut 0).unwrap();
//! assert_eq!(vec![30], bytes);
//! ```
//!
//! [`eosio_core`]: https://crates.io/crates/eosio_core
#![allow(clippy::unimplemented)]
extern crate proc_macro;

mod derive_num_bytes;
mod derive_read;
mod derive_write;

use crate::proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{DeriveInput, Lit, LitStr, Meta, Path};

/// Derive the `Write` trait
#[inline]
#[proc_macro_derive(Write, attributes(eosio_core_root_path))]
pub fn derive_write(input: TokenStream) -> TokenStream {
    crate::derive_write::expand(input)
}

/// Derive the `Read` trait
#[inline]
#[proc_macro_derive(Read, attributes(eosio_core_root_path))]
pub fn derive_read(input: TokenStream) -> TokenStream {
    crate::derive_read::expand(input)
}

/// Derive the `NumBytes` trait
#[inline]
#[proc_macro_derive(NumBytes, attributes(eosio_core_root_path))]
pub fn derive_num_bytes(input: TokenStream) -> TokenStream {
    crate::derive_num_bytes::expand(input)
}

/// The default root path using the `eosio` crate.
#[cfg(feature = "internal-use-only-root-path-is-eosio")]
const DEFAULT_ROOT_PATH: &str = "::eosio";

/// The default root path using the `eosio_core` crate.
#[cfg(not(feature = "internal-use-only-root-path-is-eosio"))]
const DEFAULT_ROOT_PATH: &str = "::eosio_core";

/// Get the root path for types/traits.
pub(crate) fn root_path(input: &DeriveInput) -> Path {
    let litstr = input
        .attrs
        .iter()
        .fold(None, |acc, attr| match attr.parse_meta() {
            Ok(meta) => {
                let name = meta.name();
                if name == "eosio_core_root_path" {
                    match meta {
                        Meta::NameValue(meta) => match meta.lit {
                            Lit::Str(s) => Some(s),
                            _ => panic!("eosio_core_path must be a lit str"),
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
        .expect("bad path for eosio_core_root_path")
}
