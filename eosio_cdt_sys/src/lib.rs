//! This crate provides low-level FFI bindings for EOSIO smart contract
//! development. Bindings are automatically generated with [`bindgen`]
//! from header files in the [`EOSIO/eosio.cdt`] repository.
//!
//! For more idiomatic Rust wrappers please see the [`eosio`] and
//! [`eosio_cdt`] crates.
//!
//! [`bindgen`]: https://github.com/rust-lang/rust-bindgen
//! [`EOSIO/eosio.cdt`]: https://github.com/EOSIO/eosio.cdt
//! [`eosio`]: https://crates.io/crates/eosio
//! [`eosio_cdt`]: https://crates.io/crates/eosio_cdt

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(dead_code)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::op_ref)]
#![allow(clippy::use_self)]
#![allow(clippy::unseparated_literal_suffix)]

mod bindings;

#[cfg(target_arch = "wasm32")]
pub use self::bindings::*;

#[cfg(not(target_arch = "wasm32"))]
pub use self::bindings::{
    capi_checksum160, capi_checksum256, capi_checksum512, capi_name,
    capi_public_key, capi_signature, int128_t, uint128_t,
};

#[cfg(not(target_arch = "wasm32"))]
mod mock_bindings;

#[cfg(not(target_arch = "wasm32"))]
pub use self::mock_bindings::*;

pub use std::ffi::c_void;
pub type c_char = u8;
pub type c_int = i32;
